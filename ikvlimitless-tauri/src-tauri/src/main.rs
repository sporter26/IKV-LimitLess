#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;

mod db;
mod crypto;
mod models;
pub mod window_utils;
use tauri::{Manager, Emitter};
use std::os::windows::process::CommandExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");
            let db_path = app_dir.join("ikvlimitless.db");
            let db = db::Database::new(&db_path).expect("failed to initialize database");
            
            let db_clone = db::Database::new(&db_path).expect("failed to initialize db clone for monitor");
            let app_handle = app.handle().clone();
            
            // Clean up any ghost processes from previous sessions/crashes
            let _ = std::process::Command::new("taskkill").args(["/F", "/IM", "ikv_captcha_server.exe", "/T"])
                .creation_flags(0x08000000)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .output();

            
            // Extract embedded dependencies to %TEMP%
            let temp_dir = std::env::temp_dir();
            let deps_dir = temp_dir.join("ikv_sys_cache");
            let _ = std::fs::create_dir_all(&deps_dir);
            
            // Apply hidden attribute to the folder itself
            let _ = std::process::Command::new("attrib")
                .arg("+h")
                .arg(deps_dir.to_str().unwrap_or_default())
                .creation_flags(0x08000000)
                .spawn();
            
            // Clean up old visible files from v1.1.0/v1.1.1 if they were left behind by the updater
            if let Ok(res_dir) = app_handle.path().resource_dir() {
                let files_to_clean = ["ikv_captcha_server.exe", "ikv_spoofer.dll", "launcher_bypass.exe"];
                for f in files_to_clean {
                    let p = res_dir.join(f);
                    if p.exists() {
                        let _ = std::fs::remove_file(p);
                    }
                }
            }
            
            let exe_path = std::env::current_exe().unwrap_or_default();
            let app_root = exe_path.parent().unwrap_or(std::path::Path::new(""));
            let old_deps_dir = app_root.join("_deps");
            if old_deps_dir.exists() {
                let _ = std::fs::remove_dir_all(old_deps_dir);
            }
            
            let captcha_target = deps_dir.join("ikv_captcha_server.exe");
            let spoofer_target = deps_dir.join("ikv_spoofer.dll");
            let bypass_target = deps_dir.join("launcher_bypass.exe");
            
            let _ = std::fs::write(&captcha_target, include_bytes!("../ikv_captcha_server.exe"));
            let _ = std::fs::write(&spoofer_target, include_bytes!("../ikv_spoofer.dll"));
            let _ = std::fs::write(&bypass_target, include_bytes!("../launcher_bypass.exe"));
            
            // Hide the extracted files
            let _ = std::process::Command::new("attrib")
                .args(["+h", captcha_target.to_str().unwrap_or_default()])
                .creation_flags(0x08000000).spawn();
            let _ = std::process::Command::new("attrib")
                .args(["+h", spoofer_target.to_str().unwrap_or_default()])
                .creation_flags(0x08000000).spawn();
            let _ = std::process::Command::new("attrib")
                .args(["+h", bypass_target.to_str().unwrap_or_default()])
                .creation_flags(0x08000000).spawn();

            // Spawn captcha server and capture logs
            let log_path = deps_dir.join("captcha.log");
            if let Ok(log_file) = std::fs::OpenOptions::new().create(true).append(true).open(&log_path) {
                if let Ok(err_file) = log_file.try_clone() {
                    let _ = std::process::Command::new(&captcha_target)
                        .creation_flags(0x08000000) // CREATE_NO_WINDOW
                        .stdout(std::process::Stdio::from(log_file))
                        .stderr(std::process::Stdio::from(err_file))
                        .spawn();
                }
            } else {
                let _ = std::process::Command::new(&captcha_target)
                    .creation_flags(0x08000000) // CREATE_NO_WINDOW
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();
            }
            
            // Background PID monitor
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                    if let Ok(accounts) = db_clone.get_accounts().await {
                        let saver_enabled = db_clone.get_setting("resource_saver".to_string()).await.unwrap_or_default() == "1";
                        let mut changed = false;
                        for mut acc in accounts {
                            if acc.is_running {
                                let mut is_alive = false;
                                if let Some(pid) = acc.pid {
                                    unsafe {
                                        use winapi::um::processthreadsapi::{OpenProcess, GetExitCodeProcess};
                                        use winapi::um::handleapi::CloseHandle;
                                        use winapi::um::minwinbase::STILL_ACTIVE;
                                        use winapi::um::winnt::{PROCESS_QUERY_LIMITED_INFORMATION};
                                        
                                        let access_flags = PROCESS_QUERY_LIMITED_INFORMATION;
                                        let handle = OpenProcess(access_flags, 0, pid);
                                        if !handle.is_null() {
                                            let mut exit_code: u32 = 0;
                                            if GetExitCodeProcess(handle, &mut exit_code) != 0 {
                                                if exit_code == STILL_ACTIVE {
                                                    is_alive = true;
                                                }
                                            }
                                            CloseHandle(handle);
                                        }
                                    }
                                }
                                
                                if !is_alive {
                                    acc.is_running = false;
                                    acc.pid = None;
                                    changed = true;
                                    
                                    // Clean up the EXE safely now that we know the process is dead
                                    let game_path = db_clone.get_setting("game_path".to_string()).await.unwrap_or_default();
                                    if !game_path.is_empty() {
                                        let safe_user = acc.username.replace(|c: char| !c.is_alphanumeric(), "_");
                                        let exe_name = format!("istanbul-{}.exe", safe_user);
                                        let exe_path = std::path::Path::new(&game_path).join(&exe_name);
                                        if exe_path.exists() {
                                            let _ = std::fs::remove_file(exe_path);
                                        }
                                    }
                                    let _ = db_clone.set_account_running(&acc.id, false).await;
                                }
                            }
                        }
                        if changed {
                            let _ = app_handle.emit("accounts_changed", ());
                        }
                    }
                }
            });

            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_accounts,
            commands::add_account,
            commands::remove_account,
            commands::replace_local_accounts,
            commands::launch_game,
            commands::launch_all_accounts,
            commands::toggle_farm_mode,
            commands::toggle_boss_mode,
            commands::set_game_path,
            commands::get_game_path,
            commands::set_setting,
            commands::get_setting,
            commands::get_captcha,
            commands::get_hwid,
            commands::launch_game_direct,
            commands::set_account_running,
            commands::stop_account,
            commands::stop_all_accounts,
            commands::update_account,
            commands::reset_all_running,

        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { .. } | tauri::RunEvent::Exit => {
                let _ = std::process::Command::new("taskkill").args(["/F", "/IM", "ikv_captcha_server.exe", "/T"])
                    .creation_flags(0x08000000)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();

                let _ = std::process::Command::new("taskkill").args(["/F", "/IM", "launcher_bypass.exe", "/T"])
                    .creation_flags(0x08000000)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();
            }
            _ => {}
        });
}
// force rebuild
// clean build
