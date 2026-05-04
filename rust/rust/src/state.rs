use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;

pub static VEC_DATA: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(Vec::new()));

//use once_cell::sync::Lazy;
//use std::sync::Mutex;

pub static DOWNLOADED_DATA: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static GLOBAL_ARRAY: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(Vec::new()));


pub static GLOBAL_IPS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static GLOBAL_HTTP: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub static IP_IPFS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static ID_IPFS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));



// Diccionario: peer_id -> lista de IPs
pub static PEER_IPS: Lazy<Mutex<HashMap<String, Vec<String>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub fn add_global_peer(address: String) {
    if let Ok(mut ips) = GLOBAL_IPS.lock() {
        if !ips.contains(&address) {
            ips.push(address);
        }
    }
}

pub fn add_peer_address(id: String, address: String) {
    if let Ok(mut peers) = PEER_IPS.lock() {
        peers.entry(id).or_insert_with(Vec::new).push(address);
    }
}

pub fn try_clear_global_ips() -> bool {
    if let Ok(mut ips) = GLOBAL_IPS.try_lock() {
        ips.clear();
        true
    } else {
        false
    }
}
