use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumChildWindows, GetDesktopWindow};
use winapi::shared::minwindef::LPARAM;
use std::ptr;
use std::process::Command;
use std::os::windows::process::CommandExt;
use crate::{log_debug, idispatch_get_property, idispatch_invoke_method, FnObjectFromLresult};
use winapi::um::oaidl::IDispatch;
use winapi::shared::winerror::S_OK;
use std::os::raw::c_void;
use winapi::um::objbase::CoInitialize;
use winapi::um::winuser::{RegisterWindowMessageA, SendMessageTimeoutA, SMTO_ABORTIFHUNG, GetAncestor, GetWindowLongA, SetWindowLongA, SetLayeredWindowAttributes, GA_ROOT, GWL_EXSTYLE, WS_EX_LAYERED, LWA_ALPHA};

pub fn spawn_ocr_macro_thread(user: String, pass: String) {
    std::thread::spawn(move || {
        log_debug("Starting OCR Macro Thread...");
        unsafe { CoInitialize(ptr::null_mut()); }

        // Find the Internet Explorer server window (the actual web content area)
        let mut hwnd_ie: HWND = ptr::null_mut();
        for attempt in 0..100 {
            std::thread::sleep(std::time::Duration::from_millis(100)); // Increased from 5ms to prevent 'Not Responding' freeze
            unsafe {
                EnumChildWindows(
                    GetDesktopWindow(),
                    Some(crate::enum_child_proc),
                    &mut hwnd_ie as *mut HWND as LPARAM,
                );
            }
            if !hwnd_ie.is_null() {
                log_debug(&format!("OCR: Found IE Server HWND at iteration {}", attempt));
                break;
            }
        }

        if !hwnd_ie.is_null() {
            log_debug("OCR: IE Server HWND found. Starting injection...");
        }

        if hwnd_ie.is_null() {
            log_debug("OCR: Failed to find IE Server HWND!");
            return;
        }

        let hwnd_main = unsafe { winapi::um::winuser::GetAncestor(hwnd_ie, winapi::um::winuser::GA_ROOT) };
        if !hwnd_main.is_null() {
            unsafe {
                winapi::um::winuser::SetWindowPos(hwnd_main, ptr::null_mut(), -32000, -32000, 0, 0, winapi::um::winuser::SWP_NOSIZE | winapi::um::winuser::SWP_NOZORDER);
            }
        }

        log_debug("OCR: IE Server HWND found and main window moved off-screen. Starting injection...");

        // Step 2: Main Captcha and Injection Loop
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        let mut submit_count = 0;
        let hex_pass: String = pass.bytes().map(|b| format!("{:02x}", b)).collect();
        
        for _captcha_attempt in 0..30 { // 30 * 300ms = 9 seconds max wait
            std::thread::sleep(std::time::Duration::from_millis(300)); // Give game UI thread breathing room
            
            unsafe {
                let msg = RegisterWindowMessageA(b"WM_HTML_GETOBJECT\0".as_ptr() as *const i8);
                let mut lresult: usize = 0;
                SendMessageTimeoutA(hwnd_ie, msg, 0, 0, SMTO_ABORTIFHUNG, 50, &mut lresult);
                
                if lresult == 0 { crate::log_debug("COM: lresult == 0"); continue; }

                let oleacc = winapi::um::libloaderapi::LoadLibraryA(b"oleacc.dll\0".as_ptr() as *const i8);
                if oleacc.is_null() { crate::log_debug("COM: oleacc is null"); continue; }
                
                let func = winapi::um::libloaderapi::GetProcAddress(oleacc, b"ObjectFromLresult\0".as_ptr() as *const i8);
                if func.is_null() { crate::log_debug("COM: func is null"); continue; }
                
                let object_from_lresult: FnObjectFromLresult = std::mem::transmute(func);
                let iid_idispatch = winapi::shared::guiddef::GUID {
                    Data1: 0x00020400, Data2: 0x0000, Data3: 0x0000,
                    Data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
                };
                
                let mut disp_ptr: *mut c_void = ptr::null_mut();
                let hr = object_from_lresult(lresult as isize, &iid_idispatch, 0, &mut disp_ptr);
                
                if hr != S_OK || disp_ptr.is_null() { 
                    crate::log_debug(&format!("COM: hr != S_OK or disp_ptr is null. hr: {}", hr));
                    continue; 
                }

                let doc_disp = disp_ptr as *mut IDispatch;
                if let Some(win_disp) = crate::idispatch_get_property(doc_disp, "parentWindow") {
                    crate::log_debug("COM: Got parentWindow. Injecting JS...");
                    let hex_user: String = user.bytes().map(|b| format!("{:02x}", b)).collect();
                    
                    // --- Inject Username and Password ---
                    let js_user_pass = format!("
                        (function() {{
                            if (window._ikv_injected) return;
                            function h2a(h) {{
                                var s = '';
                                for (var i = 0; i < h.length; i += 2) s += String.fromCharCode(parseInt(h.substr(i, 2), 16));
                                return decodeURIComponent(escape(s));
                            }}
                            var u = document.getElementById('txtUserName') || document.getElementsByName('userName')[0];
                            var p = document.getElementById('txtPassword') || document.getElementById('txtPasword') || document.getElementById('txtPass') || document.getElementsByName('password')[0];
                            
                            if (!u || !p) {{
                                var inputs = document.getElementsByTagName('input');
                                for (var i = 0; i < inputs.length; i++) {{
                                    if (!u && inputs[i].type === 'text' && inputs[i].name !== 'txtSecImage') u = inputs[i];
                                    if (!p && inputs[i].type === 'password') p = inputs[i];
                                }}
                            }}
                            
                            if (u && p) {{
                                u.value = h2a('{}');
                                p.value = h2a('{}');
                                try {{ p.focus(); p.blur(); }} catch(e) {{}}
                                window._ikv_injected = true;
                            }}
                            window.ikv_html_dump = document.documentElement.outerHTML;
                        }})();
                    ", hex_user, hex_pass);
                    crate::idispatch_invoke_method(win_disp, "execScript", &js_user_pass, "javascript");
                    
                    if let Some(html_dump) = crate::idispatch_get_string_property(win_disp, "ikv_html_dump") {
                        let _ = std::fs::write("C:\\Users\\Public\\ikv_dom.txt", html_dump);
                        crate::log_debug("COM: Dumped DOM HTML to ikv_dom.txt");
                    }
                    
                    // Use JS to save the exact captcha coordinates to a global window variable
                    let js_coords = "
                        (function() {
                            var c_img = document.getElementById('imgOnay');
                            if (!c_img) {
                                var imgs = document.getElementsByTagName('img');
                                for (var i = 0; i < imgs.length; i++) {
                                    var src = (imgs[i].src || '').toLowerCase();
                                    var id = (imgs[i].id || '').toLowerCase();
                                    if (src.indexOf('captcha') > -1 || src.indexOf('sec') > -1 || src.indexOf('kod') > -1 || src.indexOf('onay') > -1 || id.indexOf('captcha') > -1 || id.indexOf('sec') > -1 || id.indexOf('kod') > -1 || id.indexOf('onay') > -1) {
                                        c_img = imgs[i]; break;
                                    }
                                }
                            }
                            if (c_img && !c_img._ikv_captured) {
                                var rect = c_img.getBoundingClientRect();
                                var w = rect.width !== undefined ? rect.width : (rect.right - rect.left);
                                var h = rect.height !== undefined ? rect.height : (rect.bottom - rect.top);
                                window.ikv_captcha_coords = Math.round(rect.left) + ',' + Math.round(rect.top) + ',' + Math.round(w) + ',' + Math.round(h);
                                c_img._ikv_captured = true;
                            } else {
                                window.ikv_captcha_coords = '';
                            }
                        })();
                    ".to_string();
                    crate::idispatch_invoke_method(win_disp, "execScript", &js_coords, "javascript");
                    
                    std::thread::sleep(std::time::Duration::from_millis(50)); // Restored to 50ms
                    
                    // Read the variable directly from memory! No file or title needed!
                    if let Some(coord_str) = crate::idispatch_get_string_property(win_disp, "ikv_captcha_coords") {
                        crate::log_debug(&format!("COM: Got coord_str: {}", coord_str));
                        let coord_str = coord_str.trim();
                        
                        let parts: Vec<&str> = coord_str.split(',').collect();
                        if parts.len() == 4 {
                            if let (Ok(x), Ok(y), Ok(w), Ok(h)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>(), parts[2].parse::<i32>(), parts[3].parse::<i32>()) {
                                // Add small padding to capture area just in case
                                let padding = 2;
                                let cap_x = std::cmp::max(0, x - padding);
                                let cap_y = std::cmp::max(0, y - padding);
                                let cap_w = w + (padding * 2);
                                let cap_h = h + (padding * 2);
                                
                                let process_id = winapi::um::processthreadsapi::GetCurrentProcessId();
                                let mut img_path_buf = std::env::temp_dir();
                                img_path_buf.push(format!("ikv_captcha_{}.bmp", process_id));
                                let img_path = img_path_buf.to_string_lossy().into_owned();
                                
                                crate::log_debug(&format!("OCR: Attempting to capture captcha to: {}", img_path));
                                match crate::captcha::capture_captcha(hwnd_ie, cap_x, cap_y, cap_w, cap_h, &img_path) {
                                    Ok(_) => {
                                        crate::log_debug("OCR: Capture successful. Binding UDP socket...");
                                        match std::net::UdpSocket::bind("127.0.0.1:0") {
                                            Ok(socket) => {
                                                socket.set_read_timeout(Some(std::time::Duration::from_millis(3000))).unwrap();
                                                crate::log_debug(&format!("OCR: Sending path via UDP to 50055: {}", img_path));
                                                match socket.send_to(img_path.as_bytes(), "127.0.0.1:50055") {
                                                    Ok(_) => {
                                                        crate::log_debug("OCR: UDP send successful. Waiting for response...");
                                                        let mut buf = [0; 128];
                                                        match socket.recv_from(&mut buf) {
                                                            Ok((amt, _)) => {
                                                                let _ = std::fs::remove_file(&img_path); // DELETE THE BMP FILE
                                                                let solved_text = String::from_utf8_lossy(&buf[..amt]).trim().to_string();
                                                                crate::log_debug(&format!("OCR: Received UDP response: {}", solved_text));
                                                                
                                                                // Only proceed if we got a valid alphanumeric string of expected length
                                                                if !solved_text.is_empty() && !solved_text.starts_with("ERROR") && solved_text.len() >= 4 && solved_text.len() <= 8 {
                                                                    crate::log_debug(&format!("OCR: Captcha solved via UDP = '{}'", solved_text));
                                                    
                                                    // Fill captcha and click login
                                                    let js_submit = format!("
                                                        (function() {{
                                                            var c = document.getElementById('txtSecImage');
                                                            if (c) c.value = '{}';
                                                            
                                                            setTimeout(function() {{
                                                                var btn = document.getElementById('btnEnter') || document.getElementById('btnLogin') || document.getElementById('btnSubmit');
                                                                if (!btn) {{
                                                                    var inputs = document.getElementsByTagName('input');
                                                                    for (var i = 0; i < inputs.length; i++) {{
                                                                        if (inputs[i].name === 'btnEnter' || inputs[i].type === 'submit' || inputs[i].value === 'OYUNA BAŞLA') {{
                                                                            btn = inputs[i];
                                                                            break;
                                                                        }}
                                                                    }}
                                                                }}
                                                                if (btn) {{
                                                                    btn.click();
                                                                }} else if (document.forms.length > 0) {{
                                                                    document.forms[0].submit();
                                                                }}
                                                            }}, 50);
                                                        }})();
                                                    ", solved_text);
                                                    
                                                    crate::idispatch_invoke_method(win_disp, "execScript", &js_submit, "javascript");
                                                    crate::log_debug("OCR: Submit clicked!");
                                                    
                                                    submit_count += 1;
                                                    
                                                    (*win_disp).Release();
                                                    (*doc_disp).Release();
                                                    
                                                    crate::log_debug("OCR: Login submitted. Waiting 2.5 seconds to see if page reloads due to wrong captcha...");
                                                    std::thread::sleep(std::time::Duration::from_millis(2500));
                                                    continue;
                                                } else {
                                                    crate::log_debug(&format!("OCR: Ignored invalid read: '{}'", solved_text));
                                                }
                                            }
                                            Err(e) => crate::log_debug(&format!("OCR: UDP recv_from error: {:?}", e)),
                                        }
                                    }
                                    Err(e) => crate::log_debug(&format!("OCR: UDP send_to error: {:?}", e)),
                                }
                            }
                            Err(e) => crate::log_debug(&format!("OCR: UDP bind error: {:?}", e)),
                        }
                    }
                    Err(e) => crate::log_debug(&format!("OCR: capture_captcha error: {:?}", e)),
                }
                            }
                        }
                    } else {
                        crate::log_debug("COM: ikv_captcha_coords property not found or empty.");
                    }
                    (*win_disp).Release();
                } else {
                    crate::log_debug("COM: Failed to get parentWindow property.");
                }
                (*doc_disp).Release();
            }
        }
        
        crate::log_debug("OCR: Captcha phase complete.");
        unsafe { winapi::um::combaseapi::CoUninitialize();        }

        // Restore the window position when the thread exits (meaning login is done or failed completely)
        if !hwnd_main.is_null() {
            unsafe {
                let screen_w = winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN);
                let screen_h = winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN);
                let win_w = 800; // approximate game window width
                let win_h = 600; // approximate game window height
                let center_x = (screen_w - win_w) / 2;
                let center_y = (screen_h - win_h) / 2;
                winapi::um::winuser::SetWindowPos(hwnd_main, ptr::null_mut(), center_x, center_y, 0, 0, winapi::um::winuser::SWP_NOSIZE | winapi::um::winuser::SWP_NOZORDER);
            }
        }
        
        log_debug("OCR Macro Thread Exiting.");
    });
}
