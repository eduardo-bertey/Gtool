use godot::prelude::*;
use godot::classes::Node;
use nostr::prelude::*;
use std::str::FromStr;

#[derive(GodotClass)]
#[class(base=Node)]
struct Keyl {
    base: Base<Node>
}

#[godot_api]
impl INode for Keyl {
    fn init(base: Base<Node>) -> Self {
        godot_print!("KeyTool (Keyl) initialized");
        Self { base }
    }
}

#[godot_api]
impl Keyl {
    /// Genera una nueva clave privada aleatoria y la devuelve como PackedByteArray (32 bytes)
    #[func]
    pub fn generate_key(&self) -> PackedByteArray {
        let keys = Keys::generate();
        let secret_bytes = keys.secret_key().secret_bytes();
        PackedByteArray::from_iter(secret_bytes.to_vec())
    }

    /// Convierte bytes de clave privada a formato nsec (Bech32)
    #[func]
    pub fn to_nsec(&self, secret_bytes: PackedByteArray) -> GString {
        if secret_bytes.len() != 32 {
            godot_error!("La clave privada debe tener 32 bytes");
            return "".into();
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(secret_bytes.as_slice());

        match SecretKey::from_slice(&bytes) {
            Ok(sk) => {
                let keys = Keys::new(sk);
                match keys.secret_key().to_bech32() {
                    Ok(nsec) => (&nsec).into(),
                    Err(e) => {
                        godot_error!("Error al convertir a nsec: {}", e);
                        "".into()
                    }
                }
            }
            Err(e) => {
                godot_error!("Clave privada inválida: {}", e);
                "".into()
            }
        }
    }

    /// Convierte bytes de clave privada a su npub correspondiente (Bech32)
    #[func]
    pub fn to_npub(&self, secret_bytes: PackedByteArray) -> GString {
        if secret_bytes.len() != 32 {
            godot_error!("La clave privada debe tener 32 bytes");
            return "".into();
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(secret_bytes.as_slice());

        match SecretKey::from_slice(&bytes) {
            Ok(sk) => {
                let keys = Keys::new(sk);
                match keys.public_key().to_bech32() {
                    Ok(npub) => (&npub).into(),
                    Err(e) => {
                        godot_error!("Error al convertir a npub: {}", e);
                        "".into()
                    }
                }
            }
            Err(e) => {
                godot_error!("Clave privada inválida: {}", e);
                "".into()
            }
        }
    }

    /// Convierte un npub (Bech32) a clave pública en formato Hex
    #[func]
    pub fn hex_npub(&self, npub: GString) -> GString {
        let s = npub.to_string();
        match PublicKey::from_bech32(&s) {
            Ok(pk) => (&pk.to_string()).into(),
            Err(e) => {
                godot_error!("npub inválido: {}", e);
                "".into()
            }
        }
    }

    /// Convierte un nsec (Bech32) a clave privada en formato Hex
    #[func]
    pub fn hex_nsec(&self, nsec: GString) -> GString {
        let s = nsec.to_string();
        match SecretKey::from_bech32(&s) {
            Ok(sk) => (&sk.to_secret_hex()).into(),
            Err(e) => {
                godot_error!("nsec inválido: {}", e);
                "".into()
            }
        }
    }

    /// Valida y formatea un npub (acepta hex o bech32, devuelve bech32)
    #[func]
    pub fn validate_npub(&self, input: GString) -> GString {
        let s = input.to_string();
        if let Ok(pk) = PublicKey::from_bech32(&s) {
            return (&pk.to_bech32().unwrap_or_default()).into();
        }
        if let Ok(pk) = PublicKey::from_str(&s) {
            return (&pk.to_bech32().unwrap_or_default()).into();
        }
        "".into()
    }

    /// Valida y formatea un nsec (acepta hex o bech32, devuelve bech32)
    #[func]
    pub fn validate_nsec(&self, input: GString) -> GString {
        let s = input.to_string();
        if let Ok(sk) = SecretKey::from_bech32(&s) {
            return (&sk.to_bech32().unwrap_or_default()).into();
        }
        if let Ok(sk) = SecretKey::from_str(&s) {
            return (&sk.to_bech32().unwrap_or_default()).into();
        }
        "".into()
    }

    /// Obtiene la clave pública (hex) desde bytes de clave privada
    #[func]
    pub fn get_pubkey_from_secret(&self, secret_bytes: PackedByteArray) -> GString {
        if secret_bytes.len() != 32 { return "".into(); }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(secret_bytes.as_slice());
        
        match SecretKey::from_slice(&bytes) {
            Ok(sk) => {
                let keys = Keys::new(sk);
                (&keys.public_key().to_string()).into()
            }
            Err(_) => "".into()
        }
    }
}