use crate::db::Database;
use crate::models::{Account, AddAccountRequest, UpdateAccountRequest, ApiResponse, LaunchAllResponse};
use std::process::Command;
use std::os::windows::process::CommandExt;
use tauri::State;
use tauri::{AppHandle, Manager, WebviewWindowBuilder, WebviewUrl};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};



#[tauri::command(rename_all = "snake_case")]
pub async fn get_accounts(db: State<'_, Database>) -> Result<ApiResponse<Vec<Account>>, String> {
    match db.get_accounts().await {
        Ok(accounts) => Ok(ApiResponse { success: true, data: Some(accounts), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn add_account(req: AddAccountRequest, db: State<'_, Database>) -> Result<ApiResponse<Account>, String> {
    match db.add_account(req.username, req.password, req.server, req.server_port).await {
        Ok(account) => Ok(ApiResponse { success: true, data: Some(account), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_account(req: UpdateAccountRequest, db: State<'_, Database>) -> Result<ApiResponse<()>, String> {
    match db.update_account(req.id, req.username, req.password, req.server, req.server_port).await {
        Ok(_) => Ok(ApiResponse { success: true, data: Some(()), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn remove_account(account_id: String, db: State<'_, Database>) -> Result<ApiResponse<()>, String> {
    match db.remove_account(account_id).await {
        Ok(_) => Ok(ApiResponse { success: true, data: Some(()), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn replace_local_accounts(accounts: Vec<Account>, db: State<'_, Database>) -> Result<ApiResponse<()>, String> {
    match db.replace_accounts(accounts).await {
        Ok(_) => Ok(ApiResponse { success: true, data: Some(()), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn launch_game(account_id: String, spoofer_active: bool, window_size: String, db: State<'_, Database>, app_handle: AppHandle) -> Result<ApiResponse<()>, String> {
    match internal_launch_game(&account_id, spoofer_active, &window_size, &db, &app_handle).await {
        Ok(_) => Ok(ApiResponse { success: true, data: Some(()), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn launch_all_accounts(spoofer_active: bool, window_sizes: std::collections::HashMap<String, String>, bot_token: String, db: State<'_, Database>, app_handle: AppHandle) -> Result<ApiResponse<LaunchAllResponse>, String> {
    let accounts = match db.get_accounts().await {
        Ok(a) => a,
        Err(e) => return Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    };

    let count = accounts.len();
    let mut errors: Vec<String> = Vec::new();

    for (i, account) in accounts.iter().enumerate() {
        let w_size = window_sizes.get(&account.id).cloned().unwrap_or_else(|| "small".to_string());
        match internal_launch_game(&account.id, spoofer_active, &w_size, &db, &app_handle).await {
            Ok(_) => {},
            Err(e) => errors.push(format!("{}: {}", account.username, e)),
        }
        let delay = 9000;
        tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
    }

    if errors.is_empty() {
        Ok(ApiResponse { success: true, data: Some(LaunchAllResponse { data: count }), error: None })
    } else {
        Ok(ApiResponse { success: true, data: Some(LaunchAllResponse { data: count - errors.len() }), error: Some(errors.join("; ")) })
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn toggle_farm_mode(account_id: String, db: State<'_, Database>) -> Result<ApiResponse<()>, String> {
    match db.toggle_farm_mode(account_id).await {
        Ok(_) => Ok(ApiResponse { success: true, data: Some(()), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn toggle_boss_mode(account_id: String, db: State<'_, Database>) -> Result<ApiResponse<()>, String> {
    match db.toggle_boss_mode(account_id).await {
        Ok(_) => Ok(ApiResponse { success: true, data: Some(()), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_game_path(path: String, db: State<'_, Database>) -> Result<ApiResponse<()>, String> {
    let istanbul_exe = std::path::Path::new(&path).join("istanbul.exe");
    if !istanbul_exe.exists() {
        return Ok(ApiResponse { success: false, data: None, error: Some("Seçilen klasörde istanbul.exe bulunamadı!".into()) });
    }
    match db.set_game_path(path).await {
        Ok(_) => Ok(ApiResponse { success: true, data: Some(()), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_game_path(db: State<'_, Database>) -> Result<ApiResponse<String>, String> {
    match db.get_game_path().await {
        Ok(path) => Ok(ApiResponse { success: true, data: Some(path), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}



#[tauri::command(rename_all = "snake_case")]
pub async fn set_setting(key: String, value: String, db: State<'_, Database>) -> Result<ApiResponse<()>, String> {
    match db.set_setting(key, value).await {
        Ok(_) => Ok(ApiResponse { success: true, data: Some(()), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_setting(key: String, db: State<'_, Database>) -> Result<ApiResponse<String>, String> {
    match db.get_setting(key).await {
        Ok(val) => Ok(ApiResponse { success: true, data: Some(val), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

use reqwest::Client;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CaptchaResponse {
    pub image_base64: String,
    pub viewstate: String,
    pub eventvalidation: String,
    pub viewstategenerator: String,
    pub cookies: Vec<String>,
}

fn get_random_ip_headers() -> reqwest::header::HeaderMap {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    let ip1 = (nanos % 254) + 1;
    let ip2 = ((nanos / 100) % 254) + 1;
    let ip3 = ((nanos / 10000) % 254) + 1;
    let ip4 = ((nanos / 1000000) % 254) + 1;
    let random_ip = format!("{}.{}.{}.{}", ip1, ip2, ip3, ip4);
    
    let mut headers = reqwest::header::HeaderMap::new();
    if let Ok(v) = random_ip.parse() {
        headers.insert("X-Forwarded-For", v);
    }
    if let Ok(v) = random_ip.parse() {
        headers.insert("X-Real-IP", v);
    }
    if let Ok(v) = random_ip.parse() {
        headers.insert("Client-IP", v);
    }
    if let Ok(v) = random_ip.parse() {
        headers.insert("True-Client-IP", v);
    }
    headers
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_captcha() -> Result<ApiResponse<CaptchaResponse>, String> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .cookie_store(true)
        .build()
        .map_err(|e| e.to_string())?;

    let url = "http://www.istanbuloyun.com/logingame.aspx?server=93.155.105.236:27206";
    
    let res = client.get(url).headers(get_random_ip_headers()).send().await.map_err(|e| e.to_string())?;
    
    // Extract cookies and format for Cookie header (key=value)
    let mut cookies = Vec::new();
    for cookie in res.headers().get_all(reqwest::header::SET_COOKIE) {
        if let Ok(c_str) = cookie.to_str() {
            if let Some(key_val) = c_str.split(';').next() {
                cookies.push(key_val.to_string());
            }
        }
    }
    
    let html = res.text().await.map_err(|e| e.to_string())?;
    
    // Regex ile parse
    let re_vs = regex::Regex::new(r#"id="__VIEWSTATE" value="([^"]+)""#).unwrap();
    let re_ev = regex::Regex::new(r#"id="__EVENTVALIDATION" value="([^"]+)""#).unwrap();
    let re_vg = regex::Regex::new(r#"id="__VIEWSTATEGENERATOR" value="([^"]+)""#).unwrap();
    
    let viewstate = re_vs.captures(&html).map(|c| c[1].to_string()).unwrap_or_default();
    let eventvalidation = re_ev.captures(&html).map(|c| c[1].to_string()).unwrap_or_default();
    let viewstategenerator = re_vg.captures(&html).map(|c| c[1].to_string()).unwrap_or_default();
    
    // Secimg.aspx çek (Aynı client kullanıldığı için cookie otomatik gider)
    let img_res = client.get("http://www.istanbuloyun.com/securityimage.aspx").headers(get_random_ip_headers()).send().await.map_err(|e| e.to_string())?;
    let img_bytes = img_res.bytes().await.map_err(|e| e.to_string())?;
    
    use base64::{Engine as _, engine::general_purpose};
    let image_base64 = general_purpose::STANDARD.encode(&img_bytes);
    
    Ok(ApiResponse {
        success: true,
        data: Some(CaptchaResponse {
            image_base64,
            viewstate,
            eventvalidation,
            viewstategenerator,
            cookies,
        }),
        error: None,
    })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn submit_login(
    account_id: String,
    captcha: String,
    payload: CaptchaResponse,
    db: State<'_, Database>,
    app_handle: AppHandle,
) -> Result<ApiResponse<()>, String> {
    let account = match db.get_account(&account_id).await {
        Ok(acc) => acc,
        Err(e) => return Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    };

    // İlk deneme: Kullanıcının gönderdiği captcha ile
    let result = attempt_login(&account, &captcha, &payload).await;
    
    match result {
        Ok(token) => {
            return launch_after_login(&account_id, &account, &token, &db).await;
        }
        Err(first_err) => {
            println!("İlk captcha denemesi başarısız: {}. 2. deneme yapılıyor...", first_err);
            
            // 2. deneme: Yeni captcha al ve captcha sunucusuyla çöz
            match retry_with_new_captcha(&account).await {
                Ok(token) => {
                    return launch_after_login(&account_id, &account, &token, &db).await;
                }
                Err(retry_err) => {
                    return Ok(ApiResponse { 
                        success: false, 
                        data: None, 
                        error: Some(format!("2 deneme de başarısız. 1. hata: {} | 2. hata: {}", first_err, retry_err)) 
                    });
                }
            }
        }
    }
}

/// Tek bir login denemesi yapar. Başarılıysa token döner, değilse hata.
async fn attempt_login(
    account: &Account,
    captcha: &str,
    payload: &CaptchaResponse,
) -> Result<String, String> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .cookie_store(true)
        .build()
        .map_err(|e| e.to_string())?;
        
    let url = format!("http://www.istanbuloyun.com/logingame.aspx?server={}:{}", account.server, account.server_port);
    
    let mut form = std::collections::HashMap::new();
    form.insert("__VIEWSTATE", payload.viewstate.clone());
    form.insert("__EVENTVALIDATION", payload.eventvalidation.clone());
    form.insert("__VIEWSTATEGENERATOR", payload.viewstategenerator.clone());
    form.insert("txtUserName", account.username.clone());
    form.insert("txtPasword", account.password.clone());
    form.insert("txtSecImage", captcha.to_string());
    form.insert("btnEnter", "OYUNA BAŞLA".to_string());
    
    let cookie_header = payload.cookies.join("; ");
    let mut req_headers = get_random_ip_headers();
    req_headers.insert(reqwest::header::COOKIE, cookie_header.parse().unwrap());
    
    let req = client.post(&url).form(&form).headers(req_headers);
    let res = req.send().await.map_err(|e| e.to_string())?;
    let token = res.text().await.map_err(|e| e.to_string())?;
    
    if token.contains("user=") && token.contains("pass=") {
        Ok(token)
    } else {
        Err(format!("Giriş başarısız: {}", &token[..std::cmp::min(100, token.len())]))
    }
}

/// Yeni captcha alıp captcha sunucusuyla çözerek tekrar login dener
async fn retry_with_new_captcha(account: &Account) -> Result<String, String> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .cookie_store(true)
        .build()
        .map_err(|e| e.to_string())?;

    let url = format!("http://www.istanbuloyun.com/logingame.aspx?server={}:{}", account.server, account.server_port);
    
    // 1. Yeni login sayfasını çek (yeni session)
    let res = client.get(&url).headers(get_random_ip_headers()).send().await.map_err(|e| e.to_string())?;
    
    let mut cookies = Vec::new();
    for cookie in res.headers().get_all(reqwest::header::SET_COOKIE) {
        if let Ok(c_str) = cookie.to_str() {
            if let Some(key_val) = c_str.split(';').next() {
                cookies.push(key_val.to_string());
            }
        }
    }
    
    let html = res.text().await.map_err(|e| e.to_string())?;
    
    let re_vs = regex::Regex::new(r#"id="__VIEWSTATE" value="([^"]+)""#).unwrap();
    let re_ev = regex::Regex::new(r#"id="__EVENTVALIDATION" value="([^"]+)""#).unwrap();
    let re_vg = regex::Regex::new(r#"id="__VIEWSTATEGENERATOR" value="([^"]+)""#).unwrap();
    
    let viewstate = re_vs.captures(&html).map(|c| c[1].to_string()).unwrap_or_default();
    let eventvalidation = re_ev.captures(&html).map(|c| c[1].to_string()).unwrap_or_default();
    let viewstategenerator = re_vg.captures(&html).map(|c| c[1].to_string()).unwrap_or_default();
    
    // 2. Yeni captcha resmini çek
    let img_res = client.get("http://www.istanbuloyun.com/securityimage.aspx").headers(get_random_ip_headers()).send().await.map_err(|e| e.to_string())?;
    let img_bytes = img_res.bytes().await.map_err(|e| e.to_string())?;
    
    // 3. Captcha sunucusuna gönder (UDP port 50055)
    let temp_path = std::env::temp_dir().join(format!("ikv_retry_captcha_{}.bmp", std::process::id()));
    std::fs::write(&temp_path, &img_bytes).map_err(|e| format!("Captcha dosyası yazılamadı: {}", e))?;
    
    let solved = {
        use std::net::UdpSocket;
        let socket = UdpSocket::bind("127.0.0.1:0").map_err(|e| format!("UDP bind hatası: {}", e))?;
        socket.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
        let path_str = temp_path.to_string_lossy().into_owned();
        socket.send_to(path_str.as_bytes(), "127.0.0.1:50055").map_err(|e| format!("UDP gönderim hatası: {}", e))?;
        
        let mut recv_buf = [0; 64];
        match socket.recv_from(&mut recv_buf) {
            Ok((size, _)) => {
                let result = String::from_utf8_lossy(&recv_buf[..size]).trim().to_string();
                if result == "FAIL" || result.is_empty() {
                    return Err("Captcha çözülemedi (sunucu FAIL döndü)".into());
                }
                result
            }
            Err(e) => return Err(format!("Captcha sunucusundan yanıt alınamadı: {}", e)),
        }
    };
    let _ = std::fs::remove_file(&temp_path);
    
    println!("Retry captcha çözüldü: {}", solved);
    
    // 4. Yeni captcha ile login dene
    let new_payload = CaptchaResponse {
        image_base64: String::new(),
        viewstate,
        eventvalidation,
        viewstategenerator,
        cookies,
    };
    
    attempt_login(account, &solved, &new_payload).await
}

/// Login başarılı olduktan sonra oyunu başlatan yardımcı fonksiyon
async fn launch_after_login(
    account_id: &str,
    account: &Account,
    token: &str,
    db: &Database,
) -> Result<ApiResponse<()>, String> {
    let game_path_str = db.get_game_path().await.unwrap_or_else(|_| "C:\\Sobee\\Istanbul Kiyamet Vakti".to_string());
    let game_dir = std::path::PathBuf::from(game_path_str.clone());
    
    let istanbul_exe = game_dir.join("istanbul.exe");
    let token_file = game_dir.join("base").join("login_token.txt");
    
    if istanbul_exe.exists() {
        let mut file = std::fs::File::create(&token_file).map_err(|e| e.to_string())?;
        use std::io::Write;
        file.write_all(token.as_bytes()).map_err(|e| e.to_string())?;
        
        let safe_user = account.username.replace(|c: char| !c.is_alphanumeric(), "_");
        let fixed_name = format!("istanbul-{}.exe", safe_user);
        let bypass_exe = game_dir.join(&fixed_name);
        if !bypass_exe.exists() {
            std::fs::copy(&istanbul_exe, &bypass_exe).map_err(|e| e.to_string())?;
        }
        
        let bypass_tool = std::env::temp_dir().join("ikv_sys_cache").join("launcher_bypass.exe");
        
        let mut child = Command::new(&bypass_tool)
            .creation_flags(0x08000000)
            .arg(&game_path_str)
            .arg(&account.server)
            .arg(&account.server_port.to_string())
            .arg("2")
            .arg(&account.username)
            .arg(&account.password)
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Bypass aracı ({}) çalıştırılamadı: {}", bypass_tool.display(), e))?;

        let account_id_clone = account_id.to_string();
        let db_path = db.get_db_path();
        if let Some(stdout) = child.stdout.take() {
            std::thread::spawn(move || {
                use std::io::{BufRead, BufReader};
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if line.contains("PID=") {
                            if let Some(pid_str) = line.split("PID=").nth(1) {
                                if let Ok(pid) = pid_str.trim().parse::<u32>() {
                                    if let Ok(conn) = rusqlite::Connection::open(&db_path) {
                                        let _ = conn.execute("UPDATE accounts SET pid = ?1 WHERE id = ?2", rusqlite::params![pid, account_id_clone]);
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }

        let _ = db.set_account_running(account_id, true).await;
        return Ok(ApiResponse { success: true, data: Some(()), error: None });
    } else {
        return Ok(ApiResponse { success: false, data: None, error: Some("istanbul.exe bulunamadı".into()) });
    }
}



// Yardımcı Fonksiyon: Direkt launcher_bypass ile oyunu başlat
async fn internal_launch_game(
    account_id: &str,
    spoofer_active: bool,
    window_size: &str,
    db: &Database,
    app_handle: &tauri::AppHandle,
) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    let game_path_str = db.get_game_path().await.unwrap_or_else(|_| "C:\\Sobee\\Istanbul Kiyamet Vakti".to_string());
    let game_dir = std::path::PathBuf::from(&game_path_str);
    crate::db::Database::update_pref_launch(&game_path_str, window_size);

    let account = db.get_account(account_id).await.map_err(|e| e.to_string())?;

    let istanbul_exe = game_dir.join("istanbul.exe");
    if !istanbul_exe.exists() {
        return Err("istanbul.exe bulunamadı".into());
    }

    let safe_user = account.username.replace(|c: char| !c.is_alphanumeric(), "_");
    let fixed_name = format!("istanbul-{}.exe", safe_user);
    let bypass_exe = game_dir.join(&fixed_name);
    
    // Yalnızca dosya yoksa kopyala (önceki girişten kalmış olabilir)
    if !bypass_exe.exists() {
        if let Err(e) = std::fs::copy(&istanbul_exe, &bypass_exe) {
            println!("Uyarı: {} kopyalanamadı: {}", bypass_exe.display(), e);
            // Kopyalanamazsa orijinal istanbul.exe ile devam edebilir, ancak log düşüyoruz
        }
    }

    let bypass_tool = std::env::temp_dir().join("ikv_sys_cache").join("launcher_bypass.exe");

    let flag = "2";

    let mut server_ip = account.server.trim().to_string();
    if server_ip.is_empty() || server_ip == "undefined" || server_ip == "null" {
        if account.server_port == 27206 {
            server_ip = "176.236.216.91".to_string(); // Galata default
        } else {
            server_ip = "93.155.105.236".to_string(); // Eminonu default
        }
    }
    
    println!("DEBUG: Launching with server IP: '{}', Port: '{}', GamePath: '{}'", server_ip, account.server_port, game_path_str);

    use std::process::Command;
    let mut child = Command::new(&bypass_tool)
        .creation_flags(0x08000000)
        .arg(&game_path_str)
        .arg(&server_ip)
        .arg(&account.server_port.to_string())
        .arg(flag)
        .arg(account.username.trim())
        .arg(account.password.trim())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Bypass aracı çalıştırılamadı: {}", e))?;

    let account_id_clone = account_id.to_string();
    let db_path = db.get_db_path();
    if let Some(stdout) = child.stdout.take() {
        std::thread::spawn(move || {
            use std::io::{BufRead, BufReader};
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("PID=") {
                        if let Some(pid_str) = line.split("PID=").nth(1) {
                            if let Ok(pid) = pid_str.trim().parse::<u32>() {
                                if let Ok(conn) = rusqlite::Connection::open(&db_path) {
                                    let _ = conn.execute("UPDATE accounts SET pid = ?1 WHERE id = ?2", rusqlite::params![pid, account_id_clone]);
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    // Oyun başarıyla başlatıldı, is_running'i güncelle
    let _ = db.set_account_running(account_id, true).await;

    Ok(())
}

/// Oyunu doğrudan başlatır (otomatik OCR makrosu ve Web API üzerinden)
#[tauri::command(rename_all = "snake_case")]
pub async fn launch_game_direct(
    account_id: String,
    spoofer_active: bool,
    window_size: String,
    bot_token: String,
    db: State<'_, Database>,
    app_handle: AppHandle,
) -> Result<ApiResponse<()>, String> {
    match internal_launch_game(&account_id, spoofer_active, &window_size, &db, &app_handle).await {
        Ok(_) => Ok(ApiResponse { success: true, data: Some(()), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

/// Hesabın çalışma durumunu günceller
#[tauri::command(rename_all = "snake_case")]
pub async fn set_account_running(
    account_id: String,
    running: bool,
    db: State<'_, Database>,
) -> Result<ApiResponse<()>, String> {
    match db.set_account_running(&account_id, running).await {
        Ok(_) => Ok(ApiResponse { success: true, data: Some(()), error: None }),
        Err(e) => Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

/// Belirli bir hesabın oyun sürecini durdurur
#[tauri::command(rename_all = "snake_case")]
pub async fn stop_account(
    account_id: String,
    db: State<'_, Database>,
) -> Result<ApiResponse<()>, String> {
    let accounts = match db.get_accounts().await {
        Ok(a) => a,
        Err(e) => return Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    };

    let account = accounts.iter().find(|a| a.id == account_id);
    if let Some(acc) = account {
        // Mark as not running in DB first
        let _ = db.set_account_running(&account_id, false).await;

        let mut killed = false;

        // 1. PID varsa PID ile öldür
        if let Some(pid) = acc.pid {
            let result = Command::new("taskkill")
                .args(&["/F", "/PID", &pid.to_string()])
                .creation_flags(0x08000000)
                .output();
            if result.is_ok() { killed = true; }
        }

        // 2. PID ile öldüremediyse, kullanıcı adına göre exe ismini dene
        if !killed {
            let safe_user = acc.username.replace(|c: char| !c.is_alphanumeric(), "_");
            let exe_name = format!("istanbul-{}.exe", safe_user);
            let _ = Command::new("taskkill")
                .args(&["/F", "/IM", &exe_name])
                .creation_flags(0x08000000)
                .output();
        }
    }

    Ok(ApiResponse { success: true, data: Some(()), error: None })
}

/// Uygulama başlarken tüm hesapların çalışma durumunu sıfırlar
#[tauri::command(rename_all = "snake_case")]
pub async fn reset_all_running(
    db: State<'_, Database>,
) -> Result<ApiResponse<()>, String> {
    let accounts = match db.get_accounts().await {
        Ok(a) => a,
        Err(e) => return Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    };
    for acc in &accounts {
        let _ = db.set_account_running(&acc.id, false).await;
    }
    Ok(ApiResponse { success: true, data: Some(()), error: None })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn stop_all_accounts(db: State<'_, Database>) -> Result<ApiResponse<()>, String> {
    let accounts = match db.get_accounts().await {
        Ok(a) => a,
        Err(e) => return Ok(ApiResponse { success: false, data: None, error: Some(e) }),
    };

    // 1. Her hesabı DB'de kapat ve PID/exe adı ile öldür
    for acc in &accounts {
        let _ = db.set_account_running(&acc.id, false).await;
        if let Some(pid) = acc.pid {
            let _ = Command::new("taskkill")
                .args(&["/F", "/PID", &pid.to_string()])
                .creation_flags(0x08000000)
                .output();
        }
        // Kullanıcı adına göre exe ismini de öldür
        let safe_user = acc.username.replace(|c: char| !c.is_alphanumeric(), "_");
        let exe_name = format!("istanbul-{}.exe", safe_user);
        let _ = Command::new("taskkill")
            .args(&["/F", "/IM", &exe_name])
            .creation_flags(0x08000000)
            .output();
    }

    // 2. Garantili fallback: WMIC ile istanbul- ile başlayan TÜM processleri öldür
    let _ = Command::new("cmd")
        .args(&["/C", "wmic process where \"name like 'istanbul-%'\" call terminate"])
        .creation_flags(0x08000000)
        .output();

    // 3. Ek fallback: bilinen sabit exe isimleri
    let _ = Command::new("taskkill").args(&["/F", "/IM", "istanbul.exe"]).creation_flags(0x08000000).output();

    Ok(ApiResponse { success: true, data: Some(()), error: None })
}



#[tauri::command(rename_all = "snake_case")]
pub async fn get_hwid() -> Result<ApiResponse<String>, String> {
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", "(Get-CimInstance -Class Win32_ComputerSystemProduct).UUID"])
        .creation_flags(0x08000000)
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let uuid = stdout.trim().to_string();
    
    if uuid.is_empty() || uuid.contains("Error") {
        return Ok(ApiResponse { success: false, data: None, error: Some("HWID okunamadı".to_string()) });
    }
    
    Ok(ApiResponse { success: true, data: Some(uuid), error: None })
}


