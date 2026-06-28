use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::{GetWindowRect, PrintWindow, GetDC, ReleaseDC, PW_CLIENTONLY};
use winapi::um::wingdi::{CreateCompatibleDC, CreateCompatibleBitmap, SelectObject, DeleteObject, DeleteDC, GetDIBits, BITMAPINFOHEADER, BITMAPINFO, BI_RGB, DIB_RGB_COLORS};
use std::ptr;

pub fn capture_captcha(hwnd: HWND, x: i32, y: i32, w: i32, h: i32, out_path: &str) -> Result<(), String> {
    unsafe {
        let mut rect: RECT = std::mem::zeroed();
        GetWindowRect(hwnd, &mut rect);
        let win_w = rect.right - rect.left;
        let win_h = rect.bottom - rect.top;

        if win_w <= 0 || win_h <= 0 {
            return Err("Invalid window size".into());
        }

        let hdc_screen = GetDC(hwnd);
        let hdc_mem = CreateCompatibleDC(hdc_screen);
        let hbm = CreateCompatibleBitmap(hdc_screen, win_w, win_h);
        
        let hbm_old = SelectObject(hdc_mem, hbm as *mut _);

        // PW_CLIENTONLY = 1
        let pw_res = PrintWindow(hwnd, hdc_mem, 1);
        if pw_res == 0 {
            SelectObject(hdc_mem, hbm_old);
            DeleteObject(hbm as *mut _);
            DeleteDC(hdc_mem);
            ReleaseDC(hwnd, hdc_screen);
            return Err("PrintWindow failed".into());
        }

        let mut bmi: BITMAPINFO = std::mem::zeroed();
        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = win_w;
        bmi.bmiHeader.biHeight = -win_h; // top-down
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB;

        let buf_size = (win_w * win_h * 4) as usize;
        let mut pixels: Vec<u8> = vec![0; buf_size];

        let get_res = GetDIBits(
            hdc_mem,
            hbm,
            0,
            win_h as u32,
            pixels.as_mut_ptr() as *mut _,
            &mut bmi,
            DIB_RGB_COLORS,
        );

        SelectObject(hdc_mem, hbm_old);
        DeleteObject(hbm as *mut _);
        DeleteDC(hdc_mem);
        ReleaseDC(hwnd, hdc_screen);

        if get_res == 0 {
            return Err("GetDIBits failed".into());
        }

        // Crop image and save using bmp crate
        let mut img = bmp::Image::new(w as u32, h as u32);
        for cy in 0..h {
            for cx in 0..w {
                let sx = x + cx;
                let sy = y + cy;
                if sx >= 0 && sx < win_w && sy >= 0 && sy < win_h {
                    let idx = ((sy * win_w + sx) * 4) as usize;
                    let b = pixels[idx];
                    let g = pixels[idx + 1];
                    let r = pixels[idx + 2];
                    img.set_pixel(cx as u32, cy as u32, bmp::Pixel { r, g, b });
                }
            }
        }

        if let Err(_) = img.save(out_path) {
            return Err("Failed to save bmp".into());
        }

        Ok(())
    }
}

pub fn capture_full_window(hwnd: HWND, out_path: &str) -> Result<(), String> {
    unsafe {
        let mut rect: RECT = std::mem::zeroed();
        GetWindowRect(hwnd, &mut rect);
        let win_w = rect.right - rect.left;
        let win_h = rect.bottom - rect.top;

        if win_w <= 0 || win_h <= 0 {
            return Err("Invalid window size".into());
        }

        let hdc_screen = GetDC(hwnd);
        let hdc_mem = CreateCompatibleDC(hdc_screen);
        let hbm = CreateCompatibleBitmap(hdc_screen, win_w, win_h);
        
        let hbm_old = SelectObject(hdc_mem, hbm as *mut _);

        let pw_res = PrintWindow(hwnd, hdc_mem, PW_CLIENTONLY);
        if pw_res == 0 {
            SelectObject(hdc_mem, hbm_old);
            DeleteObject(hbm as *mut _);
            DeleteDC(hdc_mem);
            ReleaseDC(hwnd, hdc_screen);
            return Err("PrintWindow failed".into());
        }

        let mut bmi: BITMAPINFO = std::mem::zeroed();
        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = win_w;
        bmi.bmiHeader.biHeight = -win_h; // top-down
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB;

        let buf_size = (win_w * win_h * 4) as usize;
        let mut pixels: Vec<u8> = vec![0; buf_size];

        let get_res = GetDIBits(
            hdc_mem,
            hbm,
            0,
            win_h as u32,
            pixels.as_mut_ptr() as *mut _,
            &mut bmi,
            DIB_RGB_COLORS,
        );

        SelectObject(hdc_mem, hbm_old);
        DeleteObject(hbm as *mut _);
        DeleteDC(hdc_mem);
        ReleaseDC(hwnd, hdc_screen);

        if get_res == 0 {
            return Err("GetDIBits failed".into());
        }

        // Save full window as bmp
        let mut img = bmp::Image::new(win_w as u32, win_h as u32);
        for cy in 0..win_h {
            for cx in 0..win_w {
                let idx = ((cy * win_w + cx) * 4) as usize;
                let b = pixels[idx];
                let g = pixels[idx + 1];
                let r = pixels[idx + 2];
                img.set_pixel(cx as u32, cy as u32, bmp::Pixel { r, g, b });
            }
        }

        if let Err(_) = img.save(out_path) {
            return Err("Failed to save bmp".into());
        }

        Ok(())
    }
}
