/// KEM (Key Encapsulation Mechanism) Godot Node
///
/// Expone todos los algoritmos de libcrux_kem a Godot:
/// - X25519: x25519 ECDH KEM
/// - Secp256r1: NIST P256 ECDH KEM
/// - MlKem512: ML-KEM 512 (FIPS 203)
/// - MlKem768: ML-KEM 768 (FIPS 203)
/// - MlKem1024: ML-KEM 1024 (FIPS 203)
/// - X25519MlKem768Draft00: Hybrid x25519 + ML-KEM 768 draft
/// - XWingKemDraft06: Hybrid x25519 + ML-KEM 768 X-Wing draft 06

use godot::prelude::*;
use godot::classes::RefCounted;
use libcrux_kem::{self, Algorithm, PublicKey, Ct, Ss, PrivateKey};

/// Convierte string a Algorithm
fn parse_algorithm(name: &str) -> Option<Algorithm> {
    match name {
        "X25519" => Some(Algorithm::X25519),
        "Secp256r1" => Some(Algorithm::Secp256r1),
        "MlKem512" => Some(Algorithm::MlKem512),
        "MlKem768" => Some(Algorithm::MlKem768),
        "MlKem1024" => Some(Algorithm::MlKem1024),
        "X25519MlKem768Draft00" => Some(Algorithm::X25519MlKem768Draft00),
        "XWingKemDraft06" => Some(Algorithm::XWingKemDraft06),
        _ => None,
    }
}

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct KemTool {
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for KemTool {
    fn init(base: Base<RefCounted>) -> Self {
        godot_print!("KemTool initialized — post-quantum KEM (libcrux)");
        Self { base }
    }
}

#[godot_api]
impl KemTool {
    /// Lista todos los algoritmos disponibles
    #[func]
    pub fn list_algorithms(&self) -> PackedStringArray {
        let names = [
            "X25519",
            "Secp256r1",
            "MlKem512",
            "MlKem768",
            "MlKem1024",
            "X25519MlKem768Draft00",
            "XWingKemDraft06",
        ];
        let mut arr = PackedStringArray::new();
        for n in &names {
            arr.push(&GString::from(*n));
        }
        arr
    }

    /// Genera un par de claves (private_key, public_key) para el algoritmo dado.
    /// Retorna un Dictionary { "private_key": PackedByteArray, "public_key": PackedByteArray }
    /// o Variant::nil() en caso de error.
    #[func]
    pub fn key_gen(&self, algorithm: GString) -> Variant {
        let alg_str = algorithm.to_string();
        let alg = match parse_algorithm(&alg_str) {
            Some(a) => a,
            None => {
                godot_error!("KemTool::key_gen — algoritmo desconocido: {}", alg_str);
                return Variant::nil();
            }
        };

        use rand_09::rngs::OsRng;
        use rand_09::TryRngCore;
        let mut os_rng = OsRng;
        let mut rng = os_rng.unwrap_mut();

        match libcrux_kem::key_gen(alg, &mut rng) {
            Ok((sk, pk)) => {
                let sk_bytes = sk.encode();
                let pk_bytes = pk.encode();

                let mut dict = Dictionary::new();
                dict.set("private_key", PackedByteArray::from_iter(sk_bytes).to_variant());
                dict.set("public_key", PackedByteArray::from_iter(pk_bytes).to_variant());
                dict.to_variant()
            }
            Err(e) => {
                godot_error!("KemTool::key_gen error ({:?}): {:?}", alg, e);
                Variant::nil()
            }
        }
    }

    /// Encapsula: genera un shared secret + ciphertext a partir de una public key.
    /// Retorna Dictionary { "shared_secret": PackedByteArray, "ciphertext": PackedByteArray }
    #[func]
    pub fn encapsulate(&self, algorithm: GString, public_key_bytes: PackedByteArray) -> Variant {
        let alg_str = algorithm.to_string();
        let alg = match parse_algorithm(&alg_str) {
            Some(a) => a,
            None => {
                godot_error!("KemTool::encapsulate — algoritmo desconocido: {}", alg_str);
                return Variant::nil();
            }
        };

        let pk = match PublicKey::decode(alg, public_key_bytes.as_slice()) {
            Ok(pk) => pk,
            Err(e) => {
                godot_error!("KemTool::encapsulate — error decodificando public key: {:?}", e);
                return Variant::nil();
            }
        };

        use rand_09::rngs::OsRng;
        use rand_09::TryRngCore;
        let mut os_rng = OsRng;
        let mut rng = os_rng.unwrap_mut();

        match pk.encapsulate(&mut rng) {
            Ok((ss, ct)) => {
                let mut dict = Dictionary::new();
                dict.set("shared_secret", PackedByteArray::from_iter(ss.encode()).to_variant());
                dict.set("ciphertext", PackedByteArray::from_iter(ct.encode()).to_variant());
                dict.to_variant()
            }
            Err(e) => {
                godot_error!("KemTool::encapsulate error: {:?}", e);
                Variant::nil()
            }
        }
    }

    /// Desencapsula: recupera el shared secret a partir de un ciphertext y una private key.
    /// Retorna PackedByteArray con el shared secret, o vacío en caso de error.
    #[func]
    pub fn decapsulate(
        &self,
        algorithm: GString,
        ciphertext_bytes: PackedByteArray,
        private_key_bytes: PackedByteArray,
    ) -> PackedByteArray {
        let alg_str = algorithm.to_string();
        let alg = match parse_algorithm(&alg_str) {
            Some(a) => a,
            None => {
                godot_error!("KemTool::decapsulate — algoritmo desconocido: {}", alg_str);
                return PackedByteArray::new();
            }
        };

        let ct = match Ct::decode(alg, ciphertext_bytes.as_slice()) {
            Ok(ct) => ct,
            Err(e) => {
                godot_error!("KemTool::decapsulate — error decodificando ciphertext: {:?}", e);
                return PackedByteArray::new();
            }
        };

        let sk = match PrivateKey::decode(alg, private_key_bytes.as_slice()) {
            Ok(sk) => sk,
            Err(e) => {
                godot_error!("KemTool::decapsulate — error decodificando private key: {:?}", e);
                return PackedByteArray::new();
            }
        };

        match ct.decapsulate(&sk) {
            Ok(ss) => PackedByteArray::from_iter(ss.encode()),
            Err(e) => {
                godot_error!("KemTool::decapsulate error: {:?}", e);
                PackedByteArray::new()
            }
        }
    }

    /// Prueba completa de un round-trip KEM:
    /// key_gen → encapsulate → decapsulate → verifica que ambos shared secrets son iguales.
    /// Retorna Dictionary con todos los datos y el resultado de la verificación.
    #[func]
    pub fn test_roundtrip(&self, algorithm: GString) -> Variant {
        let alg_str = algorithm.to_string();
        let alg = match parse_algorithm(&alg_str) {
            Some(a) => a,
            None => {
                godot_error!("KemTool::test_roundtrip — algoritmo desconocido: {}", alg_str);
                return Variant::nil();
            }
        };

        let start = std::time::Instant::now();

        use rand_09::rngs::OsRng;
        use rand_09::TryRngCore;
        let mut os_rng = OsRng;
        let mut rng = os_rng.unwrap_mut();

        // 1. Key Generation
        let keygen_start = std::time::Instant::now();
        let (sk_a, pk_a) = match libcrux_kem::key_gen(alg, &mut rng) {
            Ok(kp) => kp,
            Err(e) => {
                godot_error!("test_roundtrip key_gen error: {:?}", e);
                return Variant::nil();
            }
        };
        let keygen_ms = keygen_start.elapsed().as_secs_f64() * 1000.0;

        // Encode/Decode public key (simula transmisión)
        let received_pk = pk_a.encode();
        let pk = match PublicKey::decode(alg, &received_pk) {
            Ok(pk) => pk,
            Err(e) => {
                godot_error!("test_roundtrip decode pk error: {:?}", e);
                return Variant::nil();
            }
        };

        // 2. Encapsulation (lado B)
        let encaps_start = std::time::Instant::now();
        let (ss_b, ct_b) = match pk.encapsulate(&mut rng) {
            Ok(result) => result,
            Err(e) => {
                godot_error!("test_roundtrip encapsulate error: {:?}", e);
                return Variant::nil();
            }
        };
        let encaps_ms = encaps_start.elapsed().as_secs_f64() * 1000.0;

        // Encode/Decode ciphertext (simula transmisión)
        let received_ct = ct_b.encode();
        let ct_a = match Ct::decode(alg, &received_ct) {
            Ok(ct) => ct,
            Err(e) => {
                godot_error!("test_roundtrip decode ct error: {:?}", e);
                return Variant::nil();
            }
        };

        // 3. Decapsulation (lado A)
        let decaps_start = std::time::Instant::now();
        let ss_a = match ct_a.decapsulate(&sk_a) {
            Ok(ss) => ss,
            Err(e) => {
                godot_error!("test_roundtrip decapsulate error: {:?}", e);
                return Variant::nil();
            }
        };
        let decaps_ms = decaps_start.elapsed().as_secs_f64() * 1000.0;

        let total_ms = start.elapsed().as_secs_f64() * 1000.0;

        // 4. Verificación
        let ss_b_encoded = ss_b.encode();
        let ss_a_encoded = ss_a.encode();
        let match_ok = ss_b_encoded == ss_a_encoded;

        // Build result
        let mut dict = Dictionary::new();
        dict.set("algorithm", algorithm.to_variant());
        dict.set("match", match_ok.to_variant());
        dict.set("shared_secret_a", PackedByteArray::from_iter(ss_a_encoded.clone()).to_variant());
        dict.set("shared_secret_b", PackedByteArray::from_iter(ss_b_encoded.clone()).to_variant());
        dict.set("private_key_size", (sk_a.encode().len() as i64).to_variant());
        dict.set("public_key_size", (received_pk.len() as i64).to_variant());
        dict.set("ciphertext_size", (received_ct.len() as i64).to_variant());
        dict.set("shared_secret_size", (ss_a_encoded.len() as i64).to_variant());
        dict.set("keygen_ms", keygen_ms.to_variant());
        dict.set("encaps_ms", encaps_ms.to_variant());
        dict.set("decaps_ms", decaps_ms.to_variant());
        dict.set("total_ms", total_ms.to_variant());

        if match_ok {
            godot_print!("✅ KEM {} round-trip OK ({:.2}ms)", alg_str, total_ms);
        } else {
            godot_error!("❌ KEM {} round-trip FAILED — shared secrets mismatch!", alg_str);
        }

        dict.to_variant()
    }

    /// Convierte bytes a string hexadecimal
    #[func]
    pub fn bytes_to_hex(&self, data: PackedByteArray) -> GString {
        let hex_str: String = data.as_slice().iter().map(|b| format!("{:02x}", b)).collect();
        GString::from(&hex_str)
    }
}
