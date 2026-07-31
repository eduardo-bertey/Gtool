use godot::prelude::*;
use tokenizers::tokenizer::Tokenizer as HFTokenizer;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct Tokenizer {
    tokenizer: Option<HFTokenizer>,
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for Tokenizer {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            tokenizer: None,
            base,
        }
    }
}

#[godot_api]
impl Tokenizer {
    /// Carga un tokenizer de Hugging Face desde un archivo JSON.
    /// Usa el tokenizer tal cual viene definido en el JSON (pre-tokenizer, decoder, etc.).
    /// Retorna true si la carga fue exitosa.
    #[func]
    pub fn load_from_file(&mut self, path: String) -> bool {
        match HFTokenizer::from_file(&path) {
            Ok(tok) => {
                self.tokenizer = Some(tok);
                true
            }
            Err(e) => {
                godot_error!("Tokenizer: fallo al cargar {}: {}", path, e);
                false
            }
        }
    }

    /// Convierte texto en IDs de tokens.
    #[func]
    pub fn encode(&self, text: String) -> PackedInt64Array {
        let mut out = PackedInt64Array::new();
        if let Some(tok) = &self.tokenizer {
            match tok.encode(text, false) {
                Ok(encoding) => {
                    for &id in encoding.get_ids() {
                        out.push(id as i64);
                    }
                }
                Err(e) => godot_error!("Tokenizer: error al codificar: {}", e),
            }
        }
        out
    }

    /// Convierte IDs de tokens en texto.
    #[func]
    pub fn decode(&self, ids: PackedInt64Array) -> String {
        if let Some(tok) = &self.tokenizer {
            let indices: Vec<u32> = ids.to_vec().iter().map(|&id| id as u32).collect();
            match tok.decode(&indices, true) {
                Ok(text) => return text,
                Err(e) => godot_error!("Tokenizer: error al decodificar: {}", e),
            }
        }
        String::new()
    }

    /// Devuelve el tamaño del vocabulario, o 0 si no hay tokenizer cargado.
    #[func]
    pub fn vocab_size(&self) -> i64 {
        match &self.tokenizer {
            Some(tok) => tok.get_vocab_size(true) as i64,
            None => 0,
        }
    }

    /// Devuelve el token correspondiente a un id, o vacío si no existe.
    #[func]
    pub fn id_to_token(&self, id: i64) -> String {
        match &self.tokenizer {
            Some(tok) => tok.id_to_token(id as u32).unwrap_or_default(),
            None => String::new(),
        }
    }

    #[func]
    pub fn is_loaded(&self) -> bool {
        self.tokenizer.is_some()
    }
}
