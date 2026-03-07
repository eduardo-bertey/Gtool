use godot::prelude::*;
use godot::classes::{Node, RandomNumberGenerator, FileAccess, GDScript};
use godot::classes::file_access::ModeFlags;
use godot::global::Error;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ToolSec {
    base: Base<Node>,
}

#[godot_api]
impl INode for ToolSec {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl ToolSec {
    #[func]
    pub fn encode(&mut self, path: GString, seed: GString, overwrite: bool, run: bool) -> Variant {
        let mut rng = RandomNumberGenerator::new_gd();
        rng.set_seed(seed.hash_u32() as u64);

        if run {
            // Modo memoria: descifra todo para poder ejecutarlo como script
            let mut file = match FileAccess::open(&path, ModeFlags::READ) {
                Some(f) => f,
                None => {
                    godot_error!("ToolSec: No se pudo abrir el archivo para lectura: {}", path);
                    return Variant::nil();
                }
            };
            
            let size = file.get_length();
            let mut data = file.get_buffer(size as i64);
            
            // Procesamiento byte a byte en memoria
            for i in 0..data.len() {
                let rand_byte = rng.randi_range(0, 255) as u8;
                let original = data[i];
                data[i] = original ^ rand_byte;
            }
            file.close();

            // Si se pide sobreescribir, guardamos la versión procesada (cifrada/descifrada)
            if overwrite {
                let write_file = FileAccess::open(&path, ModeFlags::WRITE);
                if let Some(mut wf) = write_file {
                    wf.store_buffer(&data);
                    wf.close();
                } else {
                    godot_error!("ToolSec: No se pudo abrir para sobreescribir: {}", path);
                }
            }

            // Cargamos el script desde los datos descifrados en memoria
            return self.dynamic_load_from_data(data);
        } else {
            // Modo streaming: para archivos gigantes (hasta 100GB), procesa paso a paso sin saturar RAM
            if !overwrite {
                godot_print!("ToolSec: encode llamado sin run ni overwrite. Abortando.");
                return Variant::nil();
            }

            let mut file = match FileAccess::open(&path, ModeFlags::READ_WRITE) {
                Some(f) => f,
                None => {
                    godot_error!("ToolSec: No se pudo abrir el archivo en modo READ_WRITE: {}", path);
                    return Variant::nil();
                }
            };

            let total_size = file.get_length();
            let chunk_size = 1024 * 1024; // 1MB por paso

            while file.get_position() < total_size {
                let current_pos = file.get_position();
                let remaining = total_size - current_pos;
                let to_process = if remaining < chunk_size as u64 { remaining as usize } else { chunk_size };

                let chunk = file.get_buffer(to_process as i64);
                let mut modified = PackedByteArray::new();
                for i in 0..to_process {
                    let rand_byte = rng.randi_range(0, 255) as u8;
                    modified.push(chunk[i] ^ rand_byte);
                }

                file.seek(current_pos);
                file.store_buffer(&modified);
            }
            file.close();
            godot_print!("ToolSec: Archivo procesado dinámicamente (paso a paso): {}", path);
            return Variant::from(true);
        }
    }

    #[func]
    fn dynamic_load_from_data(&mut self, data: PackedByteArray) -> Variant {
        // Convertir PackedByteArray a GString de forma correcta para que Godot lo entienda como código
        let vec = data.to_vec();
        let code = String::from_utf8_lossy(&vec);
        let mut script = GDScript::new_gd();
        script.set_source_code(&GString::from(code.as_ref()));

        if script.reload() == Error::OK {
            let obj = script.instantiate(&[]);
            if obj.is_nil() {
                godot_error!("ToolSec: Error al instanciar el script.");
                return Variant::nil();
            }

            // Agregamos al SceneTree si es un Nodo, de forma genérica
            if let Ok(node) = obj.try_to::<Gd<Node>>() {
                 let engine = godot::classes::Engine::singleton();
                 if let Some(main_loop) = engine.get_main_loop() {
                    if let Ok(tree) = main_loop.try_cast::<godot::classes::SceneTree>() {
                        if let Some(mut root) = tree.get_root() {
                             root.call_deferred("add_child", &[node.to_variant()]);
                             godot_print!("ToolSec: Script añadido al SceneTree (Node detectado via Engine).");
                        }
                    }
                 }
            } else {
                godot_print!("ToolSec: Script instanciado como objeto genérico (No es un Nodo).");
            }
            
            return obj;
        } else {
            godot_error!("ToolSec: Error de compilación en el script descifrado.");
            return Variant::nil();
        }
    }
}
