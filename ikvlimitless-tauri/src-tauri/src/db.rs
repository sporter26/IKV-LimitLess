use crate::models::Account;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::process::Command;
use std::os::windows::process::CommandExt;
use rusqlite::{Connection, params};
use crate::crypto;

pub struct Database {
    db_path: PathBuf,
    game_path: Arc<Mutex<String>>,
}

impl Database {
    pub fn new(db_path: &Path) -> Result<Self, String> {
        let conn = Connection::open(db_path)
            .map_err(|e| format!("Veritabanı açılamadı: {}", e))?;

        conn.execute_batch("
            CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                server TEXT NOT NULL,
                server_port INTEGER NOT NULL,
                is_running INTEGER NOT NULL DEFAULT 0,
                farm_mode INTEGER NOT NULL DEFAULT 0,
                boss_mode INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                pid INTEGER
            );
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
        ").map_err(|e| format!("Tablo oluşturulamadı: {}", e))?;

        // Attempt to add PID column if it doesn't exist (for existing databases)
        let _ = conn.execute("ALTER TABLE accounts ADD COLUMN pid INTEGER", []);

        // Mevcut game_path'i yükle
        let game_path_val: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'game_path'",
                [],
                |row| row.get(0),
            )
            .unwrap_or_default();

        Ok(Database {
            db_path: db_path.to_path_buf(),
            game_path: Arc::new(Mutex::new(game_path_val)),
        })
    }

    fn open_conn(&self) -> Result<Connection, String> {
        Connection::open(&self.db_path)
            .map_err(|e| format!("DB bağlantısı kurulamadı: {}", e))
    }

    pub fn get_db_path(&self) -> std::path::PathBuf {
        self.db_path.clone()
    }

    pub async fn set_game_path(&self, path: String) -> Result<(), String> {
        let conn = self.open_conn()?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('game_path', ?1)",
            params![path],
        ).map_err(|e| format!("Yol kaydedilemedi: {}", e))?;

        let mut gp = self.game_path.lock().unwrap();
        *gp = path;
        Ok(())
    }

    pub async fn get_game_path(&self) -> Result<String, String> {
        let gp = self.game_path.lock().unwrap();
        Ok(gp.clone())
    }

    pub async fn set_setting(&self, key: String, value: String) -> Result<(), String> {
        let conn = self.open_conn()?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        ).map_err(|e| format!("Ayar kaydedilemedi: {}", e))?;
        Ok(())
    }

    pub async fn get_setting(&self, key: String) -> Result<String, String> {
        let conn = self.open_conn()?;
        let val: String = conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |row| row.get(0),
        ).unwrap_or_default();
        Ok(val)
    }

pub fn update_pref_launch(game_path_str: &str, window_size: &str) {
    let game_path = std::path::Path::new(game_path_str);
    
    let root_dir = if game_path.is_file() || game_path_str.to_lowercase().ends_with(".exe") {
        game_path.parent().map(|p| p.to_path_buf())
    } else {
        Some(game_path.to_path_buf())
    };

    if let Some(parent) = root_dir {
        let prefs_path = parent.join("base").join("pref_launch.prefs");
        
        let (width, height) = match window_size {
            "small" => (800, 600),
            "medium" => (1024, 768),
            "large" => (1920, 1080),
            "2k_4" => (1280, 670),
            "2k_2" => (1280, 1370),
            "2k_full" => (2560, 1440),
            s if s.starts_with("custom_") => {
                let parts: Vec<&str> = s.trim_start_matches("custom_").split('x').collect();
                if parts.len() == 2 {
                    let w = parts[0].parse::<i32>().unwrap_or(800);
                    let h = parts[1].parse::<i32>().unwrap_or(600);
                    (w, h)
                } else {
                    (800, 600)
                }
            }
            _ => (800, 600)
        };

        if prefs_path.exists() {
            if let Ok(metadata) = std::fs::metadata(&prefs_path) {
                let mut perms = metadata.permissions();
                if perms.readonly() {
                    #[allow(clippy::permissions_set_readonly_false)]
                    perms.set_readonly(false);
                    if let Err(e) = std::fs::set_permissions(&prefs_path, perms) {
                        println!("Failed to remove readonly from prefs: {}", e);
                    }
                }
            }
        }

        let content = if prefs_path.exists() {
            std::fs::read_to_string(&prefs_path).unwrap_or_default()
        } else {
            String::new()
        };

        let mut new_lines = Vec::new();
        let mut found_width = false;
        let mut found_height = false;
        let mut found_fullscreen = false;
        
        for line in content.lines() {
            if line.starts_with("ScreenWidth ") {
                new_lines.push(format!("ScreenWidth {}", width));
                found_width = true;
            } else if line.starts_with("ScreenHeight ") {
                new_lines.push(format!("ScreenHeight {}", height));
                found_height = true;
            } else if line.starts_with("Fullscreen ") {
                new_lines.push("Fullscreen 0".to_string());
                found_fullscreen = true;
            } else if line.starts_with("ScreenResolution_PREF ") {
                new_lines.push("ScreenResolution_PREF 0".to_string());
            } else {
                new_lines.push(line.to_string());
            }
        }
        
        if !found_width { new_lines.push(format!("ScreenWidth {}", width)); }
        if !found_height { new_lines.push(format!("ScreenHeight {}", height)); }
        if !found_fullscreen { new_lines.push("Fullscreen 0".to_string()); }
        
        let new_content = new_lines.join("\r\n");
        if let Err(e) = std::fs::write(&prefs_path, new_content) {
            println!("Failed to write to prefs_path: {}", e);
        } else {
            println!("Successfully wrote to {:?}", prefs_path);
        }
        
        if let Ok(metadata) = std::fs::metadata(&prefs_path) {
            let mut perms = metadata.permissions();
            perms.set_readonly(true);
            if let Err(e) = std::fs::set_permissions(&prefs_path, perms) {
                println!("Failed to set readonly to prefs: {}", e);
            }
        }
    } else {
        println!("game_path.parent() is None for {}", game_path_str);
    }
}

    pub async fn get_accounts(&self) -> Result<Vec<Account>, String> {
        let conn = self.open_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, username, password, server, server_port, is_running, farm_mode, boss_mode, created_at, pid FROM accounts ORDER BY created_at ASC"
        ).map_err(|e| format!("Sorgu hazırlanamadı: {}", e))?;

        let accounts = stmt.query_map([], |row| {
            Ok(Account {
                id: row.get(0)?,
                username: crypto::decrypt(&row.get::<_, String>(1)?).unwrap_or_else(|_| row.get(1).unwrap()),
                password: crypto::decrypt(&row.get::<_, String>(2)?).unwrap_or_else(|_| row.get(2).unwrap()),
                server: row.get(3)?,
                server_port: row.get(4)?,
                is_running: row.get::<_, i32>(5)? != 0,
                farm_mode: row.get::<_, i32>(6)? != 0,
                boss_mode: row.get::<_, i32>(7)? != 0,
                created_at: row.get(8)?,
                pid: row.get(9).unwrap_or(None),
            })
        }).map_err(|e| format!("Hesaplar alınamadı: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Hesap verisi okunamadı: {}", e))?;

        Ok(accounts)
    }

    pub async fn replace_accounts(&self, accounts: Vec<Account>) -> Result<(), String> {
        let conn = self.open_conn()?;
        conn.execute("DELETE FROM accounts", []).map_err(|e| format!("Hesaplar silinemedi: {}", e))?;
        
        for acc in accounts {
            let enc_user = crypto::encrypt(&acc.username).unwrap_or(acc.username.clone());
            let enc_pass = crypto::encrypt(&acc.password).unwrap_or(acc.password.clone());
            conn.execute(
                "INSERT INTO accounts (id, username, password, server, server_port, is_running, farm_mode, boss_mode, created_at) VALUES (?1, ?2, ?3, ?4, ?5, 0, 0, 0, ?6)",
                params![acc.id, enc_user, enc_pass, acc.server, acc.server_port, acc.created_at],
            ).map_err(|e| format!("Hesap eklenemedi: {}", e))?;
        }
        Ok(())
    }

    pub async fn add_account(
        &self,
        username: String,
        password: String,
        server: String,
        server_port: i32,
    ) -> Result<Account, String> {
        let account = Account {
            id: Uuid::new_v4().to_string(),
            username: username.clone(),
            password: password.clone(),
            server: server.clone(),
            server_port,
            is_running: false,
            farm_mode: false,
            boss_mode: false,
            created_at: chrono::Local::now().to_rfc3339(),
            pid: None,
        };

        let enc_user = crypto::encrypt(&account.username).unwrap_or(account.username.clone());
        let enc_pass = crypto::encrypt(&account.password).unwrap_or(account.password.clone());

        let conn = self.open_conn()?;
        conn.execute(
            "INSERT INTO accounts (id, username, password, server, server_port, is_running, farm_mode, boss_mode, created_at) VALUES (?1, ?2, ?3, ?4, ?5, 0, 0, 0, ?6)",
            params![account.id, enc_user, enc_pass, account.server, account.server_port, account.created_at],
        ).map_err(|e| format!("Hesap eklenemedi: {}", e))?;

        Ok(account)
    }

    pub async fn update_account(
        &self,
        id: String,
        username: String,
        password: String,
        server: String,
        server_port: i32,
    ) -> Result<(), String> {
        let enc_user = crypto::encrypt(&username).unwrap_or(username);
        let enc_pass = crypto::encrypt(&password).unwrap_or(password);

        let conn = self.open_conn()?;
        conn.execute(
            "UPDATE accounts SET username = ?1, password = ?2, server = ?3, server_port = ?4 WHERE id = ?5",
            params![enc_user, enc_pass, server, server_port, id],
        ).map_err(|e| format!("Hesap güncellenemedi: {}", e))?;
        Ok(())
    }

    pub async fn remove_account(&self, account_id: String) -> Result<(), String> {
        let conn = self.open_conn()?;
        conn.execute(
            "DELETE FROM accounts WHERE id = ?1",
            params![account_id],
        ).map_err(|e| format!("Hesap silinemedi: {}", e))?;
        Ok(())
    }

    pub async fn toggle_farm_mode(&self, account_id: String) -> Result<(), String> {
        let conn = self.open_conn()?;
        conn.execute(
            "UPDATE accounts SET farm_mode = CASE WHEN farm_mode = 0 THEN 1 ELSE 0 END WHERE id = ?1",
            params![account_id],
        ).map_err(|e| format!("Farm modu güncellenemedi: {}", e))?;
        Ok(())
    }

    pub async fn toggle_boss_mode(&self, account_id: String) -> Result<(), String> {
        let conn = self.open_conn()?;
        conn.execute(
            "UPDATE accounts SET boss_mode = CASE WHEN boss_mode = 0 THEN 1 ELSE 0 END WHERE id = ?1",
            params![account_id],
        ).map_err(|e| format!("Boss modu güncellenemedi: {}", e))?;
        Ok(())
    }

    pub async fn set_account_running(&self, account_id: &str, running: bool) -> Result<(), String> {
        let conn = self.open_conn()?;
        conn.execute(
            "UPDATE accounts SET is_running = ?1 WHERE id = ?2",
            params![running as i32, account_id],
        ).map_err(|e| format!("Durum güncellenemedi: {}", e))?;
        Ok(())
    }

    pub async fn get_account(&self, account_id: &str) -> Result<Account, String> {
        let conn = self.open_conn()?;
        conn.query_row(
            "SELECT id, username, password, server, server_port, is_running, farm_mode, boss_mode, created_at, pid FROM accounts WHERE id = ?1",
            params![account_id],
            |row| Ok(Account {
                id: row.get(0)?,
                username: crypto::decrypt(&row.get::<_, String>(1)?).unwrap_or_else(|_| row.get(1).unwrap()),
                password: crypto::decrypt(&row.get::<_, String>(2)?).unwrap_or_else(|_| row.get(2).unwrap()),
                server: row.get(3)?,
                server_port: row.get(4)?,
                is_running: row.get::<_, i32>(5)? != 0,
                farm_mode: row.get::<_, i32>(6)? != 0,
                boss_mode: row.get::<_, i32>(7)? != 0,
                created_at: row.get(8)?,
                pid: row.get(9).unwrap_or(None),
            }),
        ).map_err(|e| format!("Hesap bulunamadı: {}", e))
    }

    pub async fn launch_game(&self, account_id: String, bypass_tool: PathBuf, spoofer_active: bool, window_size: String) -> Result<(), String> {
        let game_path = self.game_path.lock().unwrap().clone();

        if game_path.is_empty() {
            return Err("Lütfen önce ayarlardan oyun yolunu (istanbul.exe) seçin!".to_string());
        }

        Self::update_pref_launch(&game_path, &window_size);

        let account = self.get_account(&account_id).await?;
        let mut child = Command::new(&bypass_tool)
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .arg(game_path.clone())
            .arg(&account.server)
            .arg(&account.server_port.to_string())
            .arg("2")
            .arg(&account.username)
            .arg(&account.password)
            .arg(spoofer_active.to_string().to_lowercase())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Oyun başlatılamadı: {}", e))?;

        let account_id_clone = account_id.clone();
        let db_path = self.db_path.clone();
        if let Some(stdout) = child.stdout.take() {
            std::thread::spawn(move || {
                use std::io::{BufRead, BufReader};
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if line.contains("PID=") {
                            if let Some(pid_str) = line.split("PID=").nth(1) {
                                if let Ok(pid) = pid_str.trim().parse::<u32>() {
                                    if let Ok(conn) = Connection::open(&db_path) {
                                        let _ = conn.execute("UPDATE accounts SET pid = ?1 WHERE id = ?2", params![pid, account_id_clone]);
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }

        // is_running'i güncelle
        let conn = self.open_conn()?;
        conn.execute(
            "UPDATE accounts SET is_running = 1 WHERE id = ?1",
            params![account_id],
        ).map_err(|e| format!("Durum güncellenemedi: {}", e))?;

        Ok(())
    }
}

// Send + Sync impl (SQLite path-based connections her seferinde yeniden açılıyor)
unsafe impl Send for Database {}
unsafe impl Sync for Database {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_game_path() {
        let db_path = std::path::Path::new("C:\\Users\\zaman\\AppData\\Roaming\\com.limitless.app\\ikvlimitless.db");
        if let Ok(db) = Database::new(db_path) {
            if let Ok(path) = db.get_setting("game_path".to_string()).await {
                println!("GAME PATH FROM DB: {}", path);
                Database::update_pref_launch(&path, "medium");
            } else {
                println!("Failed to get game_path");
            }
        } else {
            println!("Failed to open DB");
        }
    }
}
