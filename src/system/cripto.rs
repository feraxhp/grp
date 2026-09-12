use argon2::Argon2;
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use grp_core::Error;

use crate::errors::{cripto::CriptoError, general::GeneralError};

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;


#[allow(unused)]
pub struct Cripto;


#[allow(unused)]
impl Cripto {
    pub fn decript(text: &str, password: &str) -> Result<String, Error> {
        let payload = B64.decode(text)
            .unwrap();
        
        if payload.len() < SALT_LEN + NONCE_LEN {
            return Err(CriptoError::incomplete_payload(payload.len(), SALT_LEN + NONCE_LEN));
        }
    
        let salt = &payload[..SALT_LEN];
        let nonce_bytes = &payload[SALT_LEN..SALT_LEN + NONCE_LEN];
        let ciphertext = &payload[SALT_LEN + NONCE_LEN..];
    
        let key_bytes = Self::derive_key(&password, salt)?;
        let key: Key = Key::from_iter(key_bytes);
        
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = Nonce::try_from(nonce_bytes)
            .map_err(|e| CriptoError::nonce_error(e))?;
    
        let decrypted = cipher
            .decrypt(&nonce, ciphertext)
            .map_err(|_| CriptoError::dencrypt())?;
        
        // The decripted text is assert to be valid UTF-8
        let text = String::from_utf8(decrypted)
            .map_err(|e| GeneralError::invalid_utf8(e))?;
        
        Ok(text)
    }

    pub fn encript(text: &str, password: &str) -> Result<String, Error> {
        let mut salt = [0u8; SALT_LEN];
        let mut nonce_bytes = [0u8; NONCE_LEN];
        rand::fill(&mut salt);
        rand::fill(&mut nonce_bytes);
        
        let key_bytes = Self::derive_key(&password, &salt)?;
        let key: Key = Key::from_iter(key_bytes);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = Nonce::try_from(nonce_bytes)
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
    
    fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32], Error> {
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| CriptoError::from_argon2(e))?;
        
        Ok(key)
    }
}
