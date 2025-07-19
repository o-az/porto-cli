use crate::error::{PortoError, Result};
use rand::rngs::OsRng;
use secp256k1::{PublicKey, SecretKey, Secp256k1};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminKey {
    pub private_key: String,
    pub public_key: String,
    pub key_type: String,
    pub address: String,
}

impl AdminKey {
    pub fn new() -> Result<Self> {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        let secret_key = SecretKey::new(&mut rng);
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        let private_key = hex::encode(secret_key.secret_bytes());
        let public_key_bytes = public_key.serialize_uncompressed();
        let public_key_hex = hex::encode(&public_key_bytes);

        // Calculate Ethereum address from public key
        let address = Self::public_key_to_address(&public_key)?;

        Ok(AdminKey {
            private_key: format!("0x{}", private_key),
            public_key: format!("0x{}", public_key_hex),
            key_type: "secp256k1".to_string(),
            address,
        })
    }

    fn public_key_to_address(public_key: &PublicKey) -> Result<String> {
        let public_key_bytes = public_key.serialize_uncompressed();
        
        // Skip the first byte (0x04) and hash the remaining 64 bytes
        let mut hasher = Keccak256::new();
        hasher.update(&public_key_bytes[1..]);
        let hash = hasher.finalize();
        
        // Take the last 20 bytes as the address
        let address_bytes = &hash[12..];
        Ok(format!("0x{}", hex::encode(address_bytes)))
    }

    pub fn sign(&self, message: &[u8]) -> Result<String> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(
            &hex::decode(self.private_key.trim_start_matches("0x"))
                .map_err(|e| PortoError::CryptoError(format!("Invalid private key: {}", e)))?
        ).map_err(|e| PortoError::CryptoError(format!("Invalid secret key: {}", e)))?;

        let mut hasher = Keccak256::new();
        hasher.update(message);
        let hash = hasher.finalize();

        let message = secp256k1::Message::from_digest_slice(&hash)
            .map_err(|e| PortoError::CryptoError(format!("Invalid message: {}", e)))?;

        let sig = secp.sign_ecdsa(&message, &secret_key);
        Ok(format!("0x{}", hex::encode(sig.serialize_compact())))
    }
}