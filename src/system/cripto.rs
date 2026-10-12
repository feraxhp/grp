use argon2::Argon2;
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use grp_core::{Error, empty_notes};
use zeroize::Zeroizing;

use crate::errors::{cripto::CriptoError, general::GeneralError};

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;


#[allow(unused)]
pub struct Cripto;


#[allow(unused)]
impl Cripto {
    pub fn decript(text: &str, password: &str) -> Result<Zeroizing<String>, Error> {
        let payload = match B64.decode(text) {
            Ok(pp) => pp,
            Err(e) => {
                return Err(Error::new(
                "grp::base64", 
                "base64 decoding failed", 
                e, 
                vec![], empty_notes!()
                ))
            },
        };
        
        if payload.len() < SALT_LEN + NONCE_LEN {
            return Err(CriptoError::incomplete_payload(payload.len(), SALT_LEN + NONCE_LEN));
        }
    
        let salt = &payload[..SALT_LEN];
        let nonce_bytes = &payload[SALT_LEN..SALT_LEN + NONCE_LEN];
        let ciphertext = &payload[SALT_LEN + NONCE_LEN..];
    
        let key_bytes = Self::derive_key(password, salt)?;
        let key: Key = Key::from(*key_bytes);
        
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = Nonce::try_from(nonce_bytes)
            .map_err(|e| CriptoError::nonce_error(e))?;
    
        let decrypted = Zeroizing::new(
            cipher
                .decrypt(&nonce, ciphertext)
                .map_err(|_| CriptoError::dencrypt())?
        );
        
        let text_str = std::str::from_utf8(&decrypted)
            .map_err(|e| GeneralError::invalid_utf8(e))?;
        
        Ok(Zeroizing::new(text_str.to_string()))
    }

    pub fn encript(text: &str, password: &str) -> Result<String, Error> {
        let mut salt = [0u8; SALT_LEN];
        let mut nonce_bytes = [0u8; NONCE_LEN];
        rand::fill(&mut salt);
        rand::fill(&mut nonce_bytes);
        
        let key_bytes = Self::derive_key(password, &salt)?;
        let key: Key = Key::from(*key_bytes);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = Nonce::try_from(nonce_bytes.as_slice())
            .map_err(|e| CriptoError::nonce_error(e))?;
            
        let ciphertext = cipher
            .encrypt(&nonce, text.as_bytes())
            .map_err(|e| CriptoError::encrypt(e))?;
        
        let mut payload = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
        payload.extend_from_slice(&salt);
        payload.extend_from_slice(&nonce_bytes);
        payload.extend_from_slice(&ciphertext);
        
        Ok(B64.encode(payload))
    }
    
    fn derive_key(password: &str, salt: &[u8]) -> Result<Zeroizing<[u8; 32]>, Error> {
        let mut key = Zeroizing::new([0u8; 32]);
        Argon2::default()
            .hash_password_into(password.as_bytes(), salt, &mut *key)
            .map_err(|e| CriptoError::from_argon2(e))?;
        
        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usettings::structs::Pconf;

    #[test]
    fn test_encrypt_decrypt_cycle() {
        let secret = "ghp_1234567890abcdefghijklmnopqrstuvwxyz";
        let password = "super_secure_password";
        
        let encrypted = Cripto::encript(secret, password).expect("Encryption failed");
        assert_ne!(secret, encrypted);
        
        let decrypted = Cripto::decript(&encrypted, password).expect("Decryption failed");
        assert_eq!(secret, &*decrypted);
    }

    #[test]
    fn test_decrypt_with_wrong_password_fails() {
        let secret = "secret_token_data";
        let password = "correct_password";
        let wrong_password = "wrong_password";
        
        let encrypted = Cripto::encript(secret, password).expect("Encryption failed");
        let result = Cripto::decript(&encrypted, wrong_password);
        assert!(result.is_err());
    }

    #[test]
    fn test_pconf_serde_with_zeroizing() {
        let pconf = Pconf {
            name: "test_pconf".to_string(),
            owner: "test_owner".to_string(),
            token: Zeroizing::new("my_sensitive_token".to_string()),
            r#type: "github".to_string(),
            endpoint: "https://api.github.com".to_string(),
            encripted: false,
        };

        let json = serde_json::to_string(&pconf).expect("Serialization failed");
        assert!(json.contains("my_sensitive_token"));

        let deserialized: Pconf = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(&*deserialized.token, "my_sensitive_token");
    }
}
