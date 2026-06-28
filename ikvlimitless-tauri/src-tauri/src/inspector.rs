use std::fs;
use std::io::Write;

fn main() {
    let path = std::env::temp_dir().join("ikv_sys_cache").join("launcher_bypass.exe");
    let bytes = match fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            println!("Error reading: {}", e);
            return;
        }
    };
    
    let mut out = fs::File::create("C:\\Users\\zaman\\Desktop\\IKVpro\\bypass_strings.txt").unwrap();
    let mut current_string = String::new();
    
    // AutoIt strings are often UTF-16LE. Let's dump both ASCII and UTF-16.
    // Simple ASCII extraction:
    for b in &bytes {
        if *b >= 32 && *b <= 126 {
            current_string.push(*b as char);
        } else {
            if current_string.len() >= 6 {
                writeln!(out, "{}", current_string).unwrap();
            }
            current_string.clear();
        }
    }
    
    // Simple UTF-16LE extraction:
    let mut current_utf16 = String::new();
    for i in 0..(bytes.len() / 2) {
        let c = u16::from_le_bytes([bytes[i*2], bytes[i*2+1]]);
        if c >= 32 && c <= 126 {
            current_utf16.push(c as u8 as char);
        } else {
            if current_utf16.len() >= 6 {
                writeln!(out, "U: {}", current_utf16).unwrap();
            }
            current_utf16.clear();
        }
    }
}
