use winapi::shared::minwindef::{BOOL, LPARAM, TRUE, FALSE};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId, SetWindowPos, SWP_NOZORDER, SWP_NOMOVE, SWP_FRAMECHANGED};
use winapi::um::winuser::{GetWindowLongA, SetWindowLongA, GWL_STYLE, WS_CAPTION, WS_THICKFRAME, WS_MINIMIZEBOX, WS_MAXIMIZEBOX, WS_SYSMENU};

struct EnumData {
    target_pid: u32,
    hwnds: Vec<HWND>,
}

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let mut pid: u32 = 0;
    GetWindowThreadProcessId(hwnd, &mut pid);
    let data = &mut *(lparam as *mut EnumData);
    
    if pid == data.target_pid {
        use winapi::um::winuser::IsWindowVisible;
        if IsWindowVisible(hwnd) != 0 {
            // Also check if window has a title so we don't grab hidden helper windows
            let mut buf = [0u8; 256];
            let len = winapi::um::winuser::GetWindowTextA(hwnd, buf.as_mut_ptr() as *mut i8, buf.len() as i32);
            if len > 0 {
                data.hwnds.push(hwnd);
            }
        }
    }
    TRUE
}

pub fn find_windows_by_pid(pid: u32) -> Vec<HWND> {
    let mut data = EnumData {
        target_pid: pid,
        hwnds: Vec::new(),
    };
    
    unsafe {
        EnumWindows(Some(enum_windows_callback), &mut data as *mut _ as LPARAM);
    }
    
    data.hwnds
}

pub fn resize_window(hwnd: HWND, width: i32, height: i32) {
    unsafe {
        // We keep the borders as requested (implied by default). Just resize.
        SetWindowPos(hwnd, std::ptr::null_mut(), 0, 0, width, height, SWP_NOZORDER | SWP_NOMOVE | SWP_FRAMECHANGED);
    }
}
