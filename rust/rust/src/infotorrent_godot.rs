use godot::prelude::*;
use yatorrent::metadata::metainfo::Metainfo;
use yatorrent::bencoding::Value;
use yatorrent::manager::torrent_manager::TorrentManager;
use tokio::sync::mpsc;
use std::fs;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct InfoTorrent {
    base: Base<RefCounted>,
    metainfo: Option<Metainfo>,
    info_hash: Option<[u8; 20]>,
    announce_list: Vec<Vec<String>>,
    dht_nodes: Vec<String>,
}

#[godot_api]
impl IRefCounted for InfoTorrent {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            metainfo: None,
            info_hash: None,
            announce_list: Vec::new(),
            dht_nodes: vec![
                "router.bittorrent.com:6881".to_string(),
                "router.utorrent.com:6881".to_string(),
                "dht.transmissionbt.com:6881".to_string(),
            ],
        }
    }
}

#[godot_api]
impl InfoTorrent {
    #[signal]
    fn metadata_loaded();

    #[func]
    pub fn load_from_file(&mut self, path: GString) -> bool {
        let path_str = path.to_string();
        let contents = match fs::read(&path_str) {
            Ok(c) => c,
            Err(e) => {
                godot_error!("Could not read torrent file: {}", e);
                return false;
            }
        };
        let torrent_content = Value::new(&contents);
        match Metainfo::new(&torrent_content, &contents) {
            Ok(m) => {
                self.info_hash = Some(m.info_hash);
                self.announce_list = m.announce_list.clone(); // Clone before move
                self.metainfo = Some(m);
                true
            }
            Err(e) => {
                godot_error!("Could not parse torrent file: {}", e);
                false
            }
        }
    }

    #[func]
    pub fn load_from_magnet(&mut self, uri: GString) -> bool {
        let uri_str = uri.to_string();
        
        // Intentar parsear como un hash hexadecimal directamente (40 caracteres)
        if uri_str.len() == 40 {
             if let Ok(hash_vec) = hex::decode(&uri_str) {
                 if let Ok(hash_array) = hash_vec.try_into() {
                     self.info_hash = Some(hash_array);
                     self.metainfo = None; 
                     return true;
                 }
             }
        }

        // Intentar parsear como magnet URI completo
        match yatorrent::magnet::Magnet::new(uri_str) {
            Ok(magnet) => {
                self.info_hash = Some(magnet.info_hash);
                self.metainfo = None; 
                self.announce_list = vec![magnet.tracker_urls];
                true
            }
            Err(e) => {
                godot_error!("Could not parse magnet/infohash: {}", e);
                false
            }
        }
    }

    #[func]
    pub fn fetch_metadata(&mut self) {
        let info_hash = match self.info_hash {
             Some(h) => h,
             None => {
                 godot_error!("No info hash set. Call load_from_tracker or load_from_magnet first.");
                 return;
             }
        };

        let announce_list = self.announce_list.clone();
        let dht_nodes = self.dht_nodes.clone();
        
        // El InstanceId es seguro para hilos (Send/Sync), a diferencia del objeto Gd o Callable.
        let instance_id = self.base().instance_id();
        
        std::thread::spawn(move || {
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build() {
                Ok(rt) => rt,
                Err(e) => {
                    godot_error!("Could not create tokio runtime: {}", e);
                    return;
                }
            };

            rt.block_on(async move {
                let temp_dir = std::env::temp_dir();
                let (tx, mut rx) = mpsc::unbounded_channel::<Vec<u8>>();
                
                let mut manager = TorrentManager::new(
                    info_hash,
                    &temp_dir,
                    6881,
                    announce_list,
                    None,
                    None,
                    6882,
                    dht_nodes,
                    Vec::new(),
                    true, // stop after metadata download
                    10,
                );
                
                manager.metadata_tx = Some(tx);

                // Ejecutamos el manager y escuchamos el canal en paralelo
                // SIN usar tokio::spawn para evitar errores de MutexGuard (!Send)
                tokio::select! {
                    _ = manager.start() => {},
                    Some(data) = rx.recv() => {
                        let bytes = PackedByteArray::from_iter(data);
                        // Recuperamos la instancia de forma segura y avisamos a Godot
                        let mut gd_instance = Gd::<InfoTorrent>::from_instance_id(instance_id);
                        gd_instance.call_deferred("emit_metadata_loaded_internal", &[bytes.to_variant()]);
                    }
                }
            });
        });
    }

    #[func]
    fn emit_metadata_loaded_internal(&mut self, metadata_bytes: PackedByteArray) {
        let bytes = metadata_bytes.to_vec();
        let torrent_content = Value::new(&bytes);
        match Metainfo::new(&torrent_content, &bytes) {
             Ok(m) => {
                 self.metainfo = Some(m);
                 // Corregido: sin .into() para evitar error E0283
                 self.base_mut().emit_signal("metadata_loaded", &[]);
             }
             Err(e) => {
                 godot_error!("Error al parsear metadata: {}", e);
             }
        }
    }

    #[func]
    pub fn get_files(&self) -> Array<VarDictionary> {
        let mut arr = Array::new();
        if let Some(m) = &self.metainfo {
            let piece_length = m.piece_length;
            let mut current_offset: u64 = 0;
            
            for (file_path, length) in m.get_files() {
                let start_offset = current_offset;
                let end_offset = current_offset + length;
                
                let start_piece = start_offset / piece_length;
                let end_piece = if length > 0 {
                    (end_offset - 1) / piece_length
                } else {
                    start_piece
                };
                
                let mut d = VarDictionary::new();
                d.insert("path", GString::from(&file_path));
                d.insert("size", length);
                d.insert("start_offset", start_offset);
                d.insert("end_offset", end_offset);
                d.insert("start_piece", start_piece as i64);
                d.insert("end_piece", end_piece as i64);
                
                arr.push(&d);
                current_offset += length;
            }
        }
        arr
    }

    #[func]
    pub fn get_info_hash(&self) -> GString {
        if let Some(h) = self.info_hash {
            GString::from(&hex::encode(h))
        } else if let Some(m) = &self.metainfo {
            GString::from(&hex::encode(m.info_hash))
        } else {
            "".into()
        }
    }

    #[func]
    pub fn get_piece_length(&self) -> u64 {
        if let Some(m) = &self.metainfo {
            m.piece_length
        } else {
            0
        }
    }

    #[func]
    pub fn get_piece_count(&self) -> i32 {
        if let Some(m) = &self.metainfo {
            m.pieces.len() as i32
        } else {
            0
        }
    }

    #[func]
    pub fn get_piece_hashes(&self) -> PackedByteArray {
        let mut pba = PackedByteArray::new();
        if let Some(m) = &self.metainfo {
            for piece in &m.pieces {
                for byte in piece {
                    pba.push(*byte);
                }
            }
        }
        pba
    }

    #[func]
    pub fn get_total_size(&self) -> u64 {
        if let Some(m) = &self.metainfo {
             m.get_files().iter().map(|f| f.1).sum()
        } else {
            0
        }
    }
    
    #[func]
    pub fn is_loaded(&self) -> bool {
        self.metainfo.is_some()
    }
}
