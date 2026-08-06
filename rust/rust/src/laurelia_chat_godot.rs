use godot::classes::ProjectSettings;
use godot::prelude::*;
use candle_core::{DType, Device, Result, Tensor};
use std::path::Path;
use std::sync::Mutex;

use crate::hf_godot::HFGodot;

use xlstm::blocks::laurelia::weights::Weights;
use xlstm::blocks::laurelia::{Config, LaureliaTokenizer, LLM};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct LaureliaChat {
    model: Mutex<Option<LLM>>,
    tokenizer: Mutex<Option<LaureliaTokenizer>>,
    device: Device,

    // Repo de HuggingFace (org/repo, revision y archivos)
    #[export] pub hf_org: GString,
    #[export] pub hf_repo: GString,
    #[export] pub hf_revision: GString,
    #[export] pub checkpoint_file: GString,
    #[export] pub tokenizer_file: GString,

    // Descarga o no automática
    #[export] pub auto_download: bool,

    // Parámetros de generación
    #[export] pub max_new_tokens: i64,
    #[export] pub cache_size: i64,
    #[export] pub temperature: f64,
    #[export] pub top_k: i64,
    #[export] pub top_p: f64,
    #[export] pub repetition_penalty: f64,

    base: Base<Node>,
}

#[godot_api]
impl INode for LaureliaChat {
    fn init(base: Base<Node>) -> Self {
        Self {
            model: Mutex::new(None),
            tokenizer: Mutex::new(None),
            device: Device::Cpu,
            hf_org: "ScortexIA".into(),
            hf_repo: "laurelia".into(),
            hf_revision: "laurelia-llm".into(),
            checkpoint_file: "checkpoint.pt".into(),
            tokenizer_file: "tokenizer.json".into(),
            auto_download: true,
            max_new_tokens: 50,
            cache_size: 1024,
            temperature: 0.7,
            top_k: 40,
            top_p: 0.9,
            repetition_penalty: 1.2,
            base,
        }
    }
}

#[godot_api]
impl LaureliaChat {
    /// Descarga tokenizer + checkpoint desde HuggingFace (si auto_download)
    /// y carga el modelo en memoria. Retorna true si tuvo éxito.
    #[func]
    pub fn load_model(&mut self) -> bool {
        let ckpt_path: String = self.checkpoint_file.to_string();
        let tok_path: String = self.tokenizer_file.to_string();

        // Resolver rutas: descarga automática o local
        let (weights_path, tokenizer_path) = if self.auto_download {
            match Self::download(
                &self.hf_org.to_string(),
                &self.hf_repo.to_string(),
                &ckpt_path,
                &tok_path,
            ) {
                Ok(p) => p,
                Err(e) => {
                    godot_error!("LaureliaChat: fallo al descargar desde HF: {}", e);
                    return false;
                }
            }
        } else {
            (ckpt_path, tok_path)
        };

        let dtype = DType::F32;

        godot_print!(
            "LaureliaChat: cargando modelo {} ({:?})",
            weights_path,
            dtype
        );
        let model = if weights_path.ends_with(".pt") || weights_path.ends_with(".pth") {
            match Weights::load_pth(&weights_path, &Config::default(), dtype, &self.device) {
                Ok(m) => m,
                Err(e) => {
                    godot_error!("LaureliaChat: error cargando .pt: {}", e);
                    return false;
                }
            }
        } else {
            match Weights::load(&weights_path, &Config::default(), dtype, &self.device) {
                Ok(m) => m,
                Err(e) => {
                    godot_error!("LaureliaChat: error cargando pesos: {}", e);
                    return false;
                }
            }
        };

        let tokenizer = match LaureliaTokenizer::from_file(&tokenizer_path) {
            Ok(t) => t,
            Err(e) => {
                godot_error!("LaureliaChat: error cargando tokenizer: {}", e);
                return false;
            }
        };

        godot_print!(
            "LaureliaChat: modelo cargado. dim={} heads={} layers={} vocab={}",
            model.config.dim,
            model.config.heads,
            model.config.layers,
            tokenizer.vocab_size()
        );

        *self.model.lock().unwrap() = Some(model);
        *self.tokenizer.lock().unwrap() = Some(tokenizer);
        true
    }

    /// Genera texto a partir de un prompt. Usa los parámetros exportados.
    #[func]
    pub fn generate(&self, prompt: GString) -> GString {
        let model_guard = self.model.lock().unwrap();
        let model = match model_guard.as_ref() {
            Some(m) => m,
            None => {
                godot_error!("LaureliaChat: modelo no cargado. Llamá load_model() primero.");
                return GString::new();
            }
        };
        let tokenizer_guard = self.tokenizer.lock().unwrap();
        let tokenizer = match tokenizer_guard.as_ref() {
            Some(t) => t,
            None => return GString::new(),
        };

        let ids = match tokenizer.encode(&prompt.to_string()) {
            Ok(ids) => ids,
            Err(e) => {
                godot_error!("LaureliaChat: error al codificar prompt: {}", e);
                return GString::new();
            }
        };
        if ids.is_empty() {
            return GString::new();
        }

        let input = match Tensor::from_vec(ids.clone(), (1, ids.len()), model.device()) {
            Ok(t) => t,
            Err(e) => {
                godot_error!("LaureliaChat: error creando tensor: {}", e);
                return GString::new();
            }
        };

        let max_new = self.max_new_tokens.max(1) as usize;
        let temp = self.temperature as f32;
        let top_k = self.top_k.max(1) as usize;
        let top_p = self.top_p as f32;
        let rep = self.repetition_penalty as f32;

        let out = match model.generate(&input, max_new, temp, top_k, top_p, rep, None) {
            Ok(o) => o,
            Err(e) => {
                godot_error!("LaureliaChat: error en generate: {}", e);
                return GString::new();
            }
        };

        let ids_out = match out.reshape((out.elem_count(),)) {
            Ok(t) => match t.to_vec1() {
                Ok(v) => v,
                Err(e) => {
                    godot_error!("LaureliaChat: error en to_vec1: {}", e);
                    return GString::new();
                }
            },
            Err(e) => {
                godot_error!("LaureliaChat: error reshape: {}", e);
                return GString::new();
            }
        };

        match tokenizer.decode(&ids_out) {
            Ok(text) => text.as_str().into(),
            Err(e) => {
                godot_error!("LaureliaChat: error decodificando: {}", e);
                GString::new()
            }
        }
    }

    #[func]
    pub fn is_loaded(&self) -> bool {
        self.model.lock().unwrap().is_some()
    }

    /// Libera el modelo de memoria.
    #[func]
    pub fn unload_model(&self) {
        *self.model.lock().unwrap() = None;
        *self.tokenizer.lock().unwrap() = None;
    }
}

impl LaureliaChat {
    fn download(
        org: &str,
        repo: &str,
        ckpt_file: &str,
        tok_file: &str,
    ) -> Result<(String, String)> {
        let local_dir = ProjectSettings::singleton()
            .globalize_path("user://hf_models/laurelia")
            .to_string();
        if !Path::new(&local_dir).exists() {
            std::fs::create_dir_all(&local_dir).map_err(|e| {
                candle_core::Error::Msg(format!("creando dir {local_dir}: {e}"))
            })?;
        }

        let repo_id: GString = format!("{org}/{repo}").into();
        let mut hf = HFGodot::new_gd();
        hf.init_client("".into());

        let ckpt = hf.download_file(
            repo_id.clone(),
            ckpt_file.into(),
            local_dir.clone().into(),
            "model".into(),
        );
        let tok = hf.download_file(
            repo_id,
            tok_file.into(),
            local_dir.clone().into(),
            "model".into(),
        );

        if ckpt.is_empty() || tok.is_empty() {
            return Err(candle_core::Error::Msg(
                "HFGodot: fallo la descarga del modelo".into(),
            ));
        }

        Ok((ckpt.to_string(), tok.to_string()))
    }
}
