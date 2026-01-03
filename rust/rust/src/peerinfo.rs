
// prueba no se usa 
/*

prueba de torrent http 
señales de godot 

 */

use godot::prelude::*;
use godot::classes::{Node};
use godot::builtin::{GString};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::torrent::peer::{self, url_encode_bytes, extract_peers_bytes};
use crate::torrent::utils::{decode_bencoded_value, get_i64};
use reqwest;
use rand::{thread_rng, Rng, distributions::Alphanumeric};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct TPeer {
    base: Base<Node>,
    // Map URL -> List of IPs
    peers_map: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

#[godot_api]
impl INode for TPeer {
    fn init(base: Base<Node>) -> Self {
        godot_print!("TPeer init");
        Self {
            base,
            peers_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[godot_api]
impl TPeer {

    #[signal]
    fn ips_actualizadas(url: GString, ips: GString);

    // Implementation requesting HTTP tracker
    #[func]
    pub fn add_http_tracker(&self, url: GString, info_hash: GString) {
        let url_str = url.to_string();
        let hash_str = info_hash.to_string();
        let map_clone = self.peers_map.clone();

        thread::spawn(move || {
            let info_hash_bytes = match hex::decode(&hash_str) {
                Ok(b) => b,
                Err(e) => {
                    println!("Error decoding hash: {}", e);
                    return;
                }
            };
            let encoded_info_hash = url_encode_bytes(&info_hash_bytes);
            
            // Random peer_id
            let rand_str: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(12)
                .map(char::from)
                .collect();
            let peer_id = format!("-AY0001-{}", rand_str);

            loop {
                // Construct URL
                // We use defaults for downloaded/uploaded/left/port for simple peer discovery
                let tracker_url = format!(
                    "{}?info_hash={}&peer_id={}&port=6881&uploaded=0&downloaded=0&left=100&compact=1",
                    url_str,
                    encoded_info_hash,
                    peer_id
                );

                println!("Consulting tracker: {}", tracker_url);

                match reqwest::blocking::get(&tracker_url) {
                    Ok(resp) => {
                        if let Ok(bytes) = resp.bytes() {
                            // Decode to find interval
                            let decoded = decode_bencoded_value(&bytes);
                            let mut interval = get_i64(&decoded, "interval");
                            let min_interval = get_i64(&decoded, "min interval");
                            
                            if min_interval > 0 {
                                interval = min_interval;
                            }
                            if interval <= 0 {
                                interval = 1800; // Default 30 mins
                            }

                            // Extract peers
                            // We use catch_unwind or just try block logic if possible, 
                            // but extract_peers_bytes panics on failure. 
                            // For safety we might want to rewrite a safe version, but for now reuse.
                             let peers_result = std::panic::catch_unwind(|| {
                                extract_peers_bytes(&bytes)
                            });

                            if let Ok(peer_bytes) = peers_result {
                                let mut ip_list = Vec::new();
                                for chunk in peer_bytes.chunks(6) {
                                    if chunk.len() == 6 {
                                        let ip = format!("{}.{}.{}.{}", chunk[0], chunk[1], chunk[2], chunk[3]);
                                        let port = u16::from_be_bytes([chunk[4], chunk[5]]);
                                        let full_addr = format!("{}:{}", ip, port);
                                        ip_list.push(full_addr);
                                    }
                                }

                                // Update map
                                let mut map = map_clone.lock().unwrap();
                                map.insert(url_str.clone(), ip_list.clone());
                                println!("Updated {} IPs for {}", ip_list.len(), url_str);
                                
                                // Note: Emitting signals from thread is tricky in Rust/Godot. 
                                // We are updating the map. The main thread can poll this map or we can try defer call if we had checks.
                                // For now, we just update the internal state as requested "guardar las ip ... como un diccionario".
                            } else {
                                println!("Failed to extract peers from response");
                            }

                            println!("Sleeping for {} seconds", interval);
                            thread::sleep(Duration::from_secs(interval as u64));
                        } else {
                            println!("Failed to get bytes from response");
                            thread::sleep(Duration::from_secs(60));
                        }
                    },
                    Err(e) => {
                        println!("Request failed: {}", e);
                        thread::sleep(Duration::from_secs(60));
                    }
                }
            }
        });
    }

    #[func]
    fn get_ips(&self, url: GString) -> GString {
        let url_str = url.to_string();
        let map = self.peers_map.lock().unwrap();
        
        if let Some(ips) = map.get(&url_str) {
             let joined = ips.join(", ");
             GString::from(&joined)
        } else {
            GString::from("")
        }
    }
    
    #[func]
    fn get_all_trackers_info(&self) -> GString {
        let map = self.peers_map.lock().unwrap();
        let mut result = String::new();
        for (url, ips) in map.iter() {
            result.push_str(&format!("URL: {}\nIPs: {}\n\n", url, ips.join(", ")));
        }
        GString::from(&result)
    }
}