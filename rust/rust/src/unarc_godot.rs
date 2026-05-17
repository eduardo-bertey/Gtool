use godot::prelude::*;
use unarc_rs::unified::ArchiveFormat;
use std::path::Path;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct Unarc {
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for Unarc {
    fn init(base: Base<RefCounted>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl Unarc {
    // Retorna una lista con la información (nombre, tamaño, si es directorio) de todas las entradas del archivo comprimido
    #[func]
    pub fn get_entries(&self, archive_path: String) -> Array<VarDictionary> {
        let mut entries_array = Array::new();
        let path = Path::new(&archive_path);
        
        if !path.exists() {
            godot_warn!("El archivo no existe: {}", archive_path);
            return entries_array;
        }

        match ArchiveFormat::open_path(path) {
            Ok(mut archive) => {
                loop {
                    match archive.next_entry() {
                        Ok(Some(entry)) => {
                            let mut dict = VarDictionary::new();
                            let name = entry.name().to_string();
                            let size = entry.original_size() as i64;
                            let is_dir = name.ends_with('/') || name.ends_with('\\');
                            
                            dict.insert("name", name);
                            dict.insert("size", size);
                            dict.insert("is_directory", is_dir);
                            entries_array.push(&dict);
                        }
                        Ok(None) => break,
                        Err(e) => {
                            godot_error!("Error leyendo la entrada del archivo comprimido: {:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                godot_error!("No se pudo abrir el archivo comprimido {}: {:?}", archive_path, e);
            }
        }

        entries_array
    }

    // Extrae todo el archivo comprimido a una carpeta destino
    #[func]
    pub fn extract_all(&self, archive_path: String, output_dir: String) -> bool {
        let path = Path::new(&archive_path);
        if !path.exists() {
            godot_warn!("El archivo no existe: {}", archive_path);
            return false;
        }

        let out_dir = Path::new(&output_dir);
        if let Err(e) = std::fs::create_dir_all(out_dir) {
            godot_error!("No se pudo crear el directorio de salida {}: {:?}", output_dir, e);
            return false;
        }

        match ArchiveFormat::open_path(path) {
            Ok(mut archive) => {
                loop {
                    match archive.next_entry() {
                        Ok(Some(entry)) => {
                            let entry_name = entry.name();
                            let target_path = out_dir.join(entry_name);

                            if entry_name.ends_with('/') || entry_name.ends_with('\\') {
                                if let Err(e) = std::fs::create_dir_all(&target_path) {
                                    godot_error!("No se pudo crear el directorio {:?}: {:?}", target_path, e);
                                    return false;
                                }
                            } else {
                                if let Some(parent) = target_path.parent() {
                                    if let Err(e) = std::fs::create_dir_all(parent) {
                                        godot_error!("No se pudo crear el directorio padre {:?}: {:?}", parent, e);
                                        return false;
                                    }
                                }
                                match archive.read(&entry) {
                                    Ok(data) => {
                                        if let Err(e) = std::fs::write(&target_path, &data) {
                                            godot_error!("No se pudo escribir el archivo {:?}: {:?}", target_path, e);
                                            return false;
                                        }
                                    }
                                    Err(e) => {
                                        godot_error!("Error leyendo la entrada {}: {:?}", entry_name, e);
                                        return false;
                                    }
                                }
                            }
                        }
                        Ok(None) => break,
                        Err(e) => {
                            godot_error!("Error iterando entradas: {:?}", e);
                            return false;
                        }
                    }
                }
                true
            }
            Err(e) => {
                godot_error!("No se pudo abrir el archivo comprimido: {:?}", e);
                false
            }
        }
    }

    // Extrae una sola entrada del archivo a una ruta específica
    #[func]
    pub fn extract_entry(&self, archive_path: String, entry_name: String, dest_path: String) -> bool {
        let path = Path::new(&archive_path);
        if !path.exists() {
            godot_warn!("El archivo no existe: {}", archive_path);
            return false;
        }

        match ArchiveFormat::open_path(path) {
            Ok(mut archive) => {
                loop {
                    match archive.next_entry() {
                        Ok(Some(entry)) => {
                            if entry.name() == entry_name {
                                match archive.read(&entry) {
                                    Ok(data) => {
                                        if let Some(parent) = Path::new(&dest_path).parent() {
                                            if let Err(e) = std::fs::create_dir_all(parent) {
                                                godot_error!("No se pudo crear el directorio {:?}: {:?}", parent, e);
                                                return false;
                                            }
                                        }
                                        if let Err(e) = std::fs::write(&dest_path, &data) {
                                            godot_error!("No se pudo escribir el archivo {}: {:?}", dest_path, e);
                                            return false;
                                        }
                                        return true;
                                    }
                                    Err(e) => {
                                        godot_error!("Error leyendo la entrada {}: {:?}", entry_name, e);
                                        return false;
                                    }
                                }
                            }
                        }
                        Ok(None) => break,
                        Err(e) => {
                            godot_error!("Error iterando entradas: {:?}", e);
                            break;
                        }
                    }
                }
                godot_warn!("Entrada no encontrada en el archivo: {}", entry_name);
            }
            Err(e) => {
                godot_error!("No se pudo abrir el archivo comprimido: {:?}", e);
            }
        }
        false
    }

    // Retorna los bytes sin comprimir de una sola entrada en el archivo
    #[func]
    pub fn read_entry_bytes(&self, archive_path: String, entry_name: String) -> PackedByteArray {
        let path = Path::new(&archive_path);
        if !path.exists() {
            godot_warn!("El archivo no existe: {}", archive_path);
            return PackedByteArray::new();
        }

        match ArchiveFormat::open_path(path) {
            Ok(mut archive) => {
                loop {
                    match archive.next_entry() {
                        Ok(Some(entry)) => {
                            if entry.name() == entry_name {
                                match archive.read(&entry) {
                                    Ok(data) => {
                                        return PackedByteArray::from_iter(data);
                                    }
                                    Err(e) => {
                                        godot_error!("Error leyendo la entrada {}: {:?}", entry_name, e);
                                        return PackedByteArray::new();
                                    }
                                }
                            }
                        }
                        Ok(None) => break,
                        Err(e) => {
                            godot_error!("Error iterando entradas: {:?}", e);
                            break;
                        }
                    }
                }
                godot_warn!("Entrada no encontrada en el archivo: {}", entry_name);
            }
            Err(e) => {
                godot_error!("No se pudo abrir el archivo comprimido: {:?}", e);
            }
        }
        PackedByteArray::new()
    }
}
