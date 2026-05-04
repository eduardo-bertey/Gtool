use godot::prelude::*;
use yatorrent::metadata::metainfo::Metainfo;
use yatorrent::bencoding::Value;
use std::fs;
use std::sync::Arc;
use once_cell::sync::Lazy;

static RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime")
});


use std::sync::Mutex;


#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct InfoTorrent {
    base: Base<RefCounted>,
    metainfo: Option<Metainfo>,
    info_hash: Option<[u8; 20]>,
    announce_list: Vec<Vec<String>>,
    display_name: String,
    hash_type: String,
    exact_length: Option<u64>,
    web_seed: String,
    source: String,
    search_keywords: String,
    acceptable_source: String,
    manifest: String,
    fetched_data: Arc<Mutex<Option<Vec<u8>>>>,
    full_metadata_bytes: Arc<Mutex<Option<Vec<u8>>>>,
}

// Signals are now part of the main impl block below

#[godot_api]
impl IRefCounted for InfoTorrent {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            metainfo: None,
            info_hash: None,
            announce_list: Vec::new(),
            display_name: String::new(),
            hash_type: String::new(),
            exact_length: None,
            web_seed: String::new(),
            source: String::new(),
            search_keywords: String::new(),
            acceptable_source: String::new(),
            manifest: String::new(),
            fetched_data: Arc::new(Mutex::new(None)),
            full_metadata_bytes: Arc::new(Mutex::new(None)),
        }
    }
}

#[godot_api]
impl InfoTorrent {
    #[signal]
    fn metadata_loaded();

    #[func]
    pub fn load_from_magnet(&mut self, uri: GString) -> bool {
        let uri_str = uri.to_string();
        
        self.info_hash = None;
        self.metainfo = None;
        self.display_name = String::new();
        self.announce_list = Vec::new();
        self.exact_length = None;
        self.hash_type = String::new();

        if uri_str.len() == 40 && uri_str.chars().all(|c| c.is_ascii_hexdigit()) {
            if let Ok(hash_vec) = hex::decode(&uri_str) {
                if let Ok(hash_array) = hash_vec.try_into() {
                    self.info_hash = Some(hash_array);
                    self.hash_type = "btih".to_string();
                    return true;
                }
            }
        }

        if !uri_str.starts_with("magnet:?") { return false; }
        
        let params = &uri_str[8..];
        for part in params.split('&') {
            let kv: Vec<&str> = part.splitn(2, '=').collect();
            if kv.len() == 2 {
                let (k, v) = (kv[0], kv[1]);
                match k {
                    "xt" => {
                        if v.contains("btih:") {
                            let hex_str = v.split(':').last().unwrap_or("");
                            if let Ok(hash_vec) = hex::decode(hex_str) {
                                if let Ok(hash_array) = hash_vec.try_into() {
                                    self.info_hash = Some(hash_array);
                                    self.hash_type = "btih".to_string();
                                }
                            }
                        }
                    }
                    "dn" => self.display_name = v.replace("%20", " ").replace("+", " "),
                    "tr" => {
                        let decoded = v.replace("%3A", ":").replace("%2F", "/").replace("%3F", "?").replace("%3D", "=");
                        self.announce_list.push(vec![decoded]);
                    }
                    "xl" => self.exact_length = v.parse::<u64>().ok(),
                    "ws" => self.web_seed = v.to_string(),
                    "as" => self.acceptable_source = v.to_string(),
                    "xs" => self.source = v.to_string(),
                    "kt" => self.search_keywords = v.replace("+", " "),
                    "mt" => self.manifest = v.to_string(),
                    _ => {}
                }
            }
        }
        self.info_hash.is_some()
    }

    #[func]
    pub fn add_tracker(&mut self, url: GString) {
        self.announce_list.push(vec![url.to_string()]);
    }

    #[func]
    pub fn set_trackers(&mut self, urls: PackedStringArray) {
        self.announce_list.clear();
        for url in urls.as_slice() {
            self.announce_list.push(vec![url.to_string()]);
        }
    }

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
                self.announce_list = m.announce_list.clone();
                match &m.file {
                    yatorrent::metadata::infodict::MetainfoFile::SingleFile(f) => self.display_name = f.name.clone(),
                    yatorrent::metadata::infodict::MetainfoFile::MultiFile(f) => self.display_name = f.name.clone(),
                }
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
        if let Some(m) = &self.metainfo { m.piece_length } else { 0 }
    }

    #[func]
    pub fn get_piece_count(&self) -> i32 {
        if let Some(m) = &self.metainfo { m.pieces.len() as i32 } else { 0 }
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
    pub fn get_piece_hash(&self, piece_index: i32) -> PackedByteArray {
        if let Some(m) = &self.metainfo {
            if let Some(piece) = m.pieces.get(piece_index as usize) {
                return PackedByteArray::from_iter(piece.iter().copied());
            }
        }
        PackedByteArray::new()
    }

    #[func]
    pub fn get_total_size(&self) -> u64 {
        if let Some(m) = &self.metainfo {
             m.get_files().iter().map(|f| f.1).sum()
        } else {
            self.exact_length.unwrap_or(0)
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
                let end_piece = if length > 0 { (end_offset - 1) / piece_length } else { start_piece };
                
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

    #[func] pub fn get_display_name(&self) -> GString { GString::from(&self.display_name) }
    #[func] pub fn get_trackers(&self) -> Array<Variant> {
        let mut outer = Array::new();
        for tier in &self.announce_list {
            let mut inner = Array::<GString>::new();
            for tracker in tier { inner.push(&GString::from(tracker)); }
            outer.push(&inner.to_variant());
        }
        outer
    }
    #[func] pub fn get_exact_length(&self) -> Variant {
        match self.exact_length {
            Some(l) => l.to_variant(),
            None => Variant::nil(),
        }
    }
    #[func] pub fn is_loaded(&self) -> bool { self.metainfo.is_some() }
    #[func] pub fn get_manifest(&self) -> GString { GString::from(&self.manifest) }
    #[func] pub fn get_web_seed(&self) -> GString { GString::from(&self.web_seed) }
    #[func] pub fn get_search_keywords(&self) -> GString { GString::from(&self.search_keywords) }
    #[func] pub fn get_source(&self) -> GString { GString::from(&self.source) }
    #[func] pub fn get_hash_type(&self) -> GString { GString::from(&self.hash_type) }

    #[func]
    pub fn poll_metadata(&mut self) -> bool {
        let data = if let Ok(mut lock) = self.fetched_data.try_lock() {
            lock.take()
        } else {
            None
        };
        
        if let Some(d) = data {
            self.on_metadata_fetched(d.into());
            return true;
        }
        false
    }

    #[func]
    pub fn save_torrent_file(&self, path: GString) -> bool {
        if let Ok(lock) = self.full_metadata_bytes.lock() {
            if let Some(bytes) = lock.as_ref() {
                return fs::write(path.to_string(), bytes).is_ok();
            }
        }
        false
    }

    #[func]
    pub fn get_peers(&self) -> PackedStringArray {
        match crate::state::GLOBAL_IPS.try_lock() {
            Ok(ips) => PackedStringArray::from_iter(ips.iter().map(GString::from)),
            Err(_) => {
                let mut arr = PackedStringArray::new();
                arr.push(&GString::from("await"));
                arr
            }
        }
    }

    #[func]
    pub fn get_peer_ips_dict(&self) -> VarDictionary {
        let mut d = VarDictionary::new();
        match crate::state::PEER_IPS.try_lock() {
            Ok(peers) => {
                for (id, ips) in peers.iter() {
                    let mut arr = PackedStringArray::new();
                    for ip in ips { arr.push(&GString::from(ip)); }
                    d.insert(GString::from(id), arr);
                }
            }
            Err(_) => {
                d.insert("status", "await");
            }
        }
        d
    }

    #[func]
    pub fn clear_peers(&self) -> GString {
        let global_cleared = crate::state::try_clear_global_ips();
        let mut peers_cleared = false;
        
        if let Ok(mut peers) = crate::state::PEER_IPS.try_lock() {
            peers.clear();
            peers_cleared = true;
        }
        
        if global_cleared && peers_cleared {
            "ok".into()
        } else {
            "await".into()
        }
    }

    #[func]
    pub fn fetch_metadata(&mut self) {
        let hash_array = match self.info_hash {
            Some(h) => h,
            None => {
                godot_error!("Cannot fetch metadata: No info hash set.");
                return;
            }
        };

        let mut trackers = Vec::new();
        for tier in &self.announce_list {
            for tr_url in tier {
                if let Ok(tr) = tr_url.parse::<demagnetize::tracker::Tracker>() {
                    trackers.push(Arc::new(tr));
                }
            }
        }

        // Add fallback trackers if none found to increase success rate
        if trackers.is_empty() {
            let fallbacks = [
                "udp://tracker.opentrackr.org:1337/announce",
                "udp://9.rarbg.com:2810/announce",
                "udp://tracker.openbittorrent.com:6969/announce",
                "udp://exodus.desync.com:6969/announce",
                "udp://www.torrent.eu.org:451/announce",
            ];
            for tr_url in fallbacks {
                if let Ok(tr) = tr_url.parse::<demagnetize::tracker::Tracker>() {
                    trackers.push(Arc::new(tr));
                }
            }
        }

        let magnet = demagnetize::magnet::Magnet {
            info_hash: demagnetize::types::InfoHash(hash_array),
            display_name: if self.display_name.is_empty() { None } else { Some(self.display_name.clone()) },
            trackers,
        };

        let fetched_data = self.fetched_data.clone();
        let full_metadata_bytes = self.full_metadata_bytes.clone();
        
        RUNTIME.spawn(async move {
            let cfg = demagnetize::config::Config::default();
            let mut rng = rand::thread_rng();
            let mut app_obj = demagnetize::app::App::new(cfg, rng);
            
            // Set the callback to capture discovered peers
            app_obj.on_peer_discovered = Some(Box::new(|addr, id_opt| {
                let addr_str = addr.to_string(); // Esto devuelve "IP:PUERTO"
                crate::state::add_global_peer(addr_str.clone());
                if let Some(id_bytes) = id_opt {
                    crate::state::add_peer_address(hex::encode(id_bytes), addr_str);
                }
            }));
            
            let app = Arc::new(app_obj);
            
            match magnet.get_torrent_file(app.clone()).await {
                Ok(torrent_file) => {
                    let info_bytes = torrent_file.info.data.to_vec();
                    let full_bytes = bytes::Bytes::from(torrent_file).to_vec();
                    
                    if let Ok(mut lock) = fetched_data.lock() {
                        *lock = Some(info_bytes);
                    }
                    if let Ok(mut lock) = full_metadata_bytes.lock() {
                        *lock = Some(full_bytes);
                    }
                }
                Err(e) => {
                    godot_error!("Failed to fetch metadata: {}", e);
                }
            }
            app.shutdown().await;
        });
    }

    #[func]
    fn on_metadata_fetched(&mut self, bytes: PackedByteArray) {
        let contents = bytes.to_vec();
        godot_print!("Received metadata: {} bytes", contents.len());
        // godot_print!("Hex: {}", hex::encode(&contents));

        // Metainfo::new expects a full torrent dict with an "info" key.
        // We wrap the received info dict: d4:info<contents>e
        let mut wrapped = Vec::new();
        wrapped.extend_from_slice(b"d4:info");
        wrapped.extend_from_slice(&contents);
        wrapped.push(b'e');

        let torrent_content = Value::new(&wrapped);
        match Metainfo::new(&torrent_content, &wrapped) {
            Ok(m) => {
                self.info_hash = Some(m.info_hash);
                self.announce_list = m.announce_list.clone();
                match &m.file {
                    yatorrent::metadata::infodict::MetainfoFile::SingleFile(f) => self.display_name = f.name.clone(),
                    yatorrent::metadata::infodict::MetainfoFile::MultiFile(f) => self.display_name = f.name.clone(),
                }
                self.metainfo = Some(m);
                self.base_mut().emit_signal("metadata_loaded", &[]);
                godot_print!("Metadata loaded successfully for {}", self.display_name);
            }
            Err(e) => {
                godot_error!("Could not parse fetched metadata: {}", e);
            }
        }
    }
}
