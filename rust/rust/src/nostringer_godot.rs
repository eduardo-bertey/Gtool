use godot::prelude::*;
use godot::prelude::ToGodot;
use nostringer::{sign, verify, SignatureVariant};
use nostringer::blsag::sign_blsag_hex;
use nostr::base64::{engine::general_purpose, Engine as _};

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct Nostringer {
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for Nostringer {
    fn init(base: Base<RefCounted>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl Nostringer {
    #[func]
    pub fn generate_keypair(&self, variant: String) -> VarDictionary {
        let keypair = nostringer::generate_keypair_hex(&variant);
        let mut dict = VarDictionary::new();
        let _ = dict.insert("public_key", keypair.public_key_hex);
        let _ = dict.insert("private_key", keypair.private_key_hex);
        dict
    }

    #[func]
    pub fn sign_bin(&self, message: PackedByteArray, private_key_hex: String, ring_pubkeys: VarArray, variant_str: String) -> VarDictionary {
        let mut result = VarDictionary::new();
        result.insert("signature", PackedByteArray::new());
        result.insert("key_image", PackedByteArray::new());

        let ring: Vec<String> = ring_pubkeys.iter_shared().map(|v| v.to_string()).collect();
        if ring.len() < 2 {
            godot_error!("Nostringer: El anillo debe tener al menos 2 miembros.");
            return result;
        }

        let res_text = self.sign(message, private_key_hex, ring_pubkeys, variant_str);
        
        if let Some(sig_v) = res_text.get("signature") {
            let sig_str = sig_v.to_string();
            if sig_str.starts_with("ringA") {
                // USAMOS URL_SAFE_NO_PAD (Formato oficial nostringer)
                if let Ok(bytes) = general_purpose::URL_SAFE_NO_PAD.decode(&sig_str[5..]) {
                    result.insert("signature", PackedByteArray::from_iter(bytes));
                }
            }
        }

        if let Some(ki_v) = res_text.get("key_image") {
            if let Ok(ki_bytes) = hex::decode(ki_v.to_string()) {
                result.insert("key_image", PackedByteArray::from_iter(ki_bytes));
            }
        }

        result
    }

    #[func]
    pub fn verify_bin(&self, signature: PackedByteArray, message: PackedByteArray, ring_pubkeys: VarArray, key_image: PackedByteArray) -> bool {
        if signature.is_empty() { return false; }
        
        let b64 = general_purpose::URL_SAFE_NO_PAD.encode(signature.as_slice());
        let sig_str = format!("ringA{}", b64);
        let ki_hex = if key_image.is_empty() { String::new() } else { hex::encode(key_image.as_slice()) };
        
        self.verify(sig_str, message, ring_pubkeys, ki_hex)
    }

    #[func]
    pub fn sign(&self, message: PackedByteArray, private_key_hex: String, ring_pubkeys: VarArray, variant_str: String) -> VarDictionary {
        let ring: Vec<String> = ring_pubkeys.iter_shared().map(|v| v.to_string()).collect();
        let mut result = VarDictionary::new();
        let is_blsag = variant_str.to_lowercase() == "blsag";
        let variant = if is_blsag { SignatureVariant::Blsag } else { SignatureVariant::Sag };

        match sign(message.as_slice(), &private_key_hex, &ring, variant) {
            Ok(sig_str) => {
                result.insert("signature", sig_str);
                if is_blsag {
                    if let Ok((_s, ki)) = sign_blsag_hex(message.as_slice(), &private_key_hex, &ring) {
                        result.insert("key_image", ki);
                    }
                }
            },
            Err(e) => godot_error!("Nostringer sign error: {:?}", e),
        }
        result
    }

    #[func]
    pub fn verify(&self, signature: String, message: PackedByteArray, ring_pubkeys: VarArray, _ki: String) -> bool {
        let ring: Vec<String> = ring_pubkeys.iter_shared().map(|v| v.to_string()).collect();
        match verify(&signature, message.as_slice(), &ring) {
            Ok(v) => v,
            Err(e) => { godot_error!("Nostringer verify error: {:?}", e); false }
        }
    }
}
