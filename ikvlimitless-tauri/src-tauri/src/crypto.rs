use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};
use sha2::{Sha256, Digest};
use std::process::Command;
use std::os::windows::process::CommandExt;
use base64::{Engine as _, engine::general_purpose};

/// Retrieves the motherboard UUID (HWID) via PowerShell.
pub fn get_hwid() -> String {
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", "(Get-CimInstance -Class Win32_ComputerSystemProduct).UUID"])
        .creation_flags(0x08000000)
        .output();
        
    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let uuid = stdout.trim().to_string();
        if !uuid.is_empty() && !uuid.contains("Error") {
            return uuid;
        }
    }
    // Fallback ID just in case
    "FALLBACK-HWID-IKV-LIMITLESS-0001".to_string()
}

/// Generates a 32-byte AES key by hashing the HWID using SHA-256.
fn get_aes_key() -> Key<Aes256Gcm> {
    let hwid = get_hwid();
    let mut hasher = Sha256::new();
    hasher.update(hwid.as_bytes());
    hasher.update(b"limitless_secret_salt_2026"); // Additional salt
    let result = hasher.finalize();
    *Key::<Aes256Gcm>::from_slice(&result)
}

/// Encrypts plaintext and returns base64 string
pub fn encrypt(plaintext: &str) -> Result<String, String> {
    if plaintext.is_empty() {
        return Ok(String::new());
    }
    let key = get_aes_key();
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes())
        .map_err(|e| format!("Encryption failed: {:?}", e))?;
    
    // Output format: Base64(nonce + ciphertext)
    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);
    Ok(general_purpose::STANDARD.encode(combined))
}

/// Decrypts base64 string and returns plaintext
pub fn decrypt(base64_str: &str) -> Result<String, String> {
    if base64_str.is_empty() {
        return Ok(String::new());
    }
    
    // If it doesn't look like base64 or isn't encrypted by us (legacy text fallback), return as is
    if !base64_str.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=') {
        return Ok(base64_str.to_string());
    }
    
    let combined = match general_purpose::STANDARD.decode(base64_str) {
        Ok(data) => data,
        Err(_) => return Ok(base64_str.to_string()), // Probably unencrypted legacy data
    };
    
    // Nonce is 12 bytes
    if combined.len() < 12 {
        return Ok(base64_str.to_string()); // Not valid ciphertext
    }
    
    let key = get_aes_key();
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(&combined[0..12]);
    let ciphertext = &combined[12..];
    
    match cipher.decrypt(nonce, ciphertext) {
        Ok(plaintext_bytes) => {
            String::from_utf8(plaintext_bytes).map_err(|e| format!("Invalid UTF-8: {:?}", e))
        },
        Err(_) => {
            // Decryption failed. This means either wrong HWID, or it was unencrypted base64 legacy data.
            // In case it's legacy data, we return the original base64_str as fallback.
            Ok(base64_str.to_string())
        }
    }
}
