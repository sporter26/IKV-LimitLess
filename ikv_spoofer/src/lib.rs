#![allow(non_snake_case)]
#![allow(unused_imports)]

mod captcha;

use std::ffi::{c_void, CStr, CString};
use std::ptr;
use std::os::windows::process::CommandExt;
use std::sync::Mutex;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, PULONG, TRUE, ULONG};
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use winapi::um::consoleapi::AllocConsole;
use winapi::shared::winerror::{ERROR_SUCCESS, ERROR_BUFFER_OVERFLOW};
use winapi::um::iphlpapi::GetAdaptersInfo;
use winapi::um::iptypes::PIP_ADAPTER_INFO;
use rand::Rng;
use minhook::{MinHook, MH_STATUS};
use lazy_static::lazy_static;

use winapi::um::objbase::CoInitialize;
use winapi::um::winuser::{FindWindowA, FindWindowExA, RegisterWindowMessageA, SendMessageTimeoutA, SMTO_ABORTIFHUNG, GetDesktopWindow, EnumChildWindows, GetClassNameA, GetWindowThreadProcessId};
use winapi::um::processthreadsapi::GetCurrentProcessId;
use winapi::um::oaidl::{IDispatch, DISPPARAMS, VARIANT};
use winapi::um::oleauto::{SysAllocString, SysFreeString, VariantInit};
use winapi::shared::wtypes::{VT_DISPATCH, VT_BSTR};
use winapi::shared::winerror::S_OK;
use winapi::shared::guiddef::{REFIID, IID_NULL};



lazy_static! {
    static ref FAKE_MAC: [u8; 6] = generate_random_mac();
}

fn generate_random_mac() -> [u8; 6] {
    let mut rng = rand::thread_rng();
    let mut mac = [0u8; 6];
    rng.fill(&mut mac);
    // Ensure locally administered and unicast
    mac[0] = (mac[0] | 0x02) & 0xFE;
    mac
}

type FnGetAdaptersInfo = extern "system" fn(PIP_ADAPTER_INFO, PULONG) -> DWORD;
static mut ORIGINAL_GET_ADAPTERS_INFO: Option<FnGetAdaptersInfo> = None;

extern "system" fn hooked_get_adapters_info(pAdapterInfo: PIP_ADAPTER_INFO, pOutBufLen: PULONG) -> DWORD {
    // Log the call
    

    // Call the original function
    let original_fn = unsafe { ORIGINAL_GET_ADAPTERS_INFO.unwrap() };
    let result = original_fn(pAdapterInfo, pOutBufLen);
    
    // If successful and buffer was provided, modify the MAC addresses
    if result == ERROR_SUCCESS && !pAdapterInfo.is_null() {
        
        unsafe {
            let mut curr = pAdapterInfo;
            while !curr.is_null() {
                // Check if AddressLength is at least 6
                if (*curr).AddressLength >= 6 {
                    // Copy our fake MAC address
                    for i in 0..6 {
                        (*curr).Address[i] = FAKE_MAC[i];
                    }
                }
                curr = (*curr).Next;
            }
        }
    }
    
    result
}

type FnRegQueryValueExA = extern "system" fn(winapi::shared::minwindef::HKEY, winapi::um::winnt::LPCSTR, winapi::shared::minwindef::LPDWORD, winapi::shared::minwindef::LPDWORD, winapi::shared::minwindef::LPBYTE, winapi::shared::minwindef::LPDWORD) -> i32;
static mut ORIGINAL_REG_QUERY_VALUE_EX_A: Option<FnRegQueryValueExA> = None;

extern "system" fn hooked_reg_query_value_ex_a(hKey: winapi::shared::minwindef::HKEY, lpValueName: winapi::um::winnt::LPCSTR, lpReserved: winapi::shared::minwindef::LPDWORD, lpType: winapi::shared::minwindef::LPDWORD, lpData: winapi::shared::minwindef::LPBYTE, lpcbData: winapi::shared::minwindef::LPDWORD) -> i32 {
    let original_fn = unsafe { ORIGINAL_REG_QUERY_VALUE_EX_A.unwrap() };
    let result = original_fn(hKey, lpValueName, lpReserved, lpType, lpData, lpcbData);
    
    if result == ERROR_SUCCESS as i32 && !lpValueName.is_null() && !lpData.is_null() {
        unsafe {
            let value_name = std::ffi::CStr::from_ptr(lpValueName).to_string_lossy();
            if value_name.eq_ignore_ascii_case("MachineGuid") {
                
                let fake_guid = format!("fake-{:02X}{:02X}", FAKE_MAC[0], FAKE_MAC[1]);
                let fake_bytes = fake_guid.as_bytes();
                let len_to_copy = std::cmp::min(fake_bytes.len(), *lpcbData as usize);
                std::ptr::copy_nonoverlapping(fake_bytes.as_ptr(), lpData, len_to_copy);
                *lpcbData = len_to_copy as u32;
            }
        }
    }
    result
}

type FnRegQueryValueExW = extern "system" fn(winapi::shared::minwindef::HKEY, winapi::um::winnt::LPCWSTR, winapi::shared::minwindef::LPDWORD, winapi::shared::minwindef::LPDWORD, winapi::shared::minwindef::LPBYTE, winapi::shared::minwindef::LPDWORD) -> i32;
static mut ORIGINAL_REG_QUERY_VALUE_EX_W: Option<FnRegQueryValueExW> = None;

extern "system" fn hooked_reg_query_value_ex_w(hKey: winapi::shared::minwindef::HKEY, lpValueName: winapi::um::winnt::LPCWSTR, lpReserved: winapi::shared::minwindef::LPDWORD, lpType: winapi::shared::minwindef::LPDWORD, lpData: winapi::shared::minwindef::LPBYTE, lpcbData: winapi::shared::minwindef::LPDWORD) -> i32 {
    let original_fn = unsafe { ORIGINAL_REG_QUERY_VALUE_EX_W.unwrap() };
    let result = original_fn(hKey, lpValueName, lpReserved, lpType, lpData, lpcbData);
    
    if result == ERROR_SUCCESS as i32 && !lpValueName.is_null() && !lpData.is_null() {
        unsafe {
            // Check if it's MachineGuid
            let mut len = 0;
            while *lpValueName.offset(len) != 0 { len += 1; }
            let slice = std::slice::from_raw_parts(lpValueName, len as usize);
            let value_name = String::from_utf16_lossy(slice);
            if value_name.eq_ignore_ascii_case("MachineGuid") {
                
                let fake_guid = format!("fake-{:02X}{:02X}", FAKE_MAC[0], FAKE_MAC[1]);
                let fake_utf16: Vec<u16> = fake_guid.encode_utf16().chain(std::iter::once(0)).collect();
                let bytes_len = fake_utf16.len() * 2;
                let len_to_copy = std::cmp::min(bytes_len, *lpcbData as usize);
                std::ptr::copy_nonoverlapping(fake_utf16.as_ptr() as *const u8, lpData, len_to_copy);
                *lpcbData = len_to_copy as u32;
            }
        }
    }
    result
}

// --- GetVolumeInformation Hooks ---
type FnGetVolumeInformationA = extern "system" fn(winapi::um::winnt::LPCSTR, winapi::um::winnt::LPSTR, winapi::shared::minwindef::DWORD, winapi::shared::minwindef::LPDWORD, winapi::shared::minwindef::LPDWORD, winapi::shared::minwindef::LPDWORD, winapi::um::winnt::LPSTR, winapi::shared::minwindef::DWORD) -> winapi::shared::minwindef::BOOL;
static mut ORIGINAL_GET_VOLUME_INFORMATION_A: Option<FnGetVolumeInformationA> = None;

extern "system" fn hooked_get_volume_information_a(lpRootPathName: winapi::um::winnt::LPCSTR, lpVolumeNameBuffer: winapi::um::winnt::LPSTR, nVolumeNameSize: winapi::shared::minwindef::DWORD, lpVolumeSerialNumber: winapi::shared::minwindef::LPDWORD, lpMaximumComponentLength: winapi::shared::minwindef::LPDWORD, lpFileSystemFlags: winapi::shared::minwindef::LPDWORD, lpFileSystemNameBuffer: winapi::um::winnt::LPSTR, nFileSystemNameSize: winapi::shared::minwindef::DWORD) -> winapi::shared::minwindef::BOOL {
    let original_fn = unsafe { ORIGINAL_GET_VOLUME_INFORMATION_A.unwrap() };
    let result = original_fn(lpRootPathName, lpVolumeNameBuffer, nVolumeNameSize, lpVolumeSerialNumber, lpMaximumComponentLength, lpFileSystemFlags, lpFileSystemNameBuffer, nFileSystemNameSize);
    if result != 0 && !lpVolumeSerialNumber.is_null() {
        unsafe {
            
            *lpVolumeSerialNumber = *lpVolumeSerialNumber ^ (FAKE_MAC[0] as u32 | ((FAKE_MAC[1] as u32) << 8));
        }
    }
    result
}

type FnGetVolumeInformationW = extern "system" fn(winapi::um::winnt::LPCWSTR, winapi::um::winnt::LPWSTR, winapi::shared::minwindef::DWORD, winapi::shared::minwindef::LPDWORD, winapi::shared::minwindef::LPDWORD, winapi::shared::minwindef::LPDWORD, winapi::um::winnt::LPWSTR, winapi::shared::minwindef::DWORD) -> winapi::shared::minwindef::BOOL;
static mut ORIGINAL_GET_VOLUME_INFORMATION_W: Option<FnGetVolumeInformationW> = None;

extern "system" fn hooked_get_volume_information_w(lpRootPathName: winapi::um::winnt::LPCWSTR, lpVolumeNameBuffer: winapi::um::winnt::LPWSTR, nVolumeNameSize: winapi::shared::minwindef::DWORD, lpVolumeSerialNumber: winapi::shared::minwindef::LPDWORD, lpMaximumComponentLength: winapi::shared::minwindef::LPDWORD, lpFileSystemFlags: winapi::shared::minwindef::LPDWORD, lpFileSystemNameBuffer: winapi::um::winnt::LPWSTR, nFileSystemNameSize: winapi::shared::minwindef::DWORD) -> winapi::shared::minwindef::BOOL {
    let original_fn = unsafe { ORIGINAL_GET_VOLUME_INFORMATION_W.unwrap() };
    let result = original_fn(lpRootPathName, lpVolumeNameBuffer, nVolumeNameSize, lpVolumeSerialNumber, lpMaximumComponentLength, lpFileSystemFlags, lpFileSystemNameBuffer, nFileSystemNameSize);
    if result != 0 && !lpVolumeSerialNumber.is_null() {
        unsafe {
            
            *lpVolumeSerialNumber = *lpVolumeSerialNumber ^ (FAKE_MAC[0] as u32 | ((FAKE_MAC[1] as u32) << 8));
        }
    }
    result
}

// --- GetComputerName Hooks ---
type FnGetComputerNameA = extern "system" fn(winapi::um::winnt::LPSTR, winapi::shared::minwindef::LPDWORD) -> winapi::shared::minwindef::BOOL;
static mut ORIGINAL_GET_COMPUTER_NAME_A: Option<FnGetComputerNameA> = None;

extern "system" fn hooked_get_computer_name_a(lpBuffer: winapi::um::winnt::LPSTR, lpnSize: winapi::shared::minwindef::LPDWORD) -> winapi::shared::minwindef::BOOL {
    let original_fn = unsafe { ORIGINAL_GET_COMPUTER_NAME_A.unwrap() };
    let result = original_fn(lpBuffer, lpnSize);
    if result != 0 && !lpBuffer.is_null() {
        unsafe {
            
            let fake_name = format!("PC-{:02X}{:02X}", FAKE_MAC[0], FAKE_MAC[1]);
            let bytes = fake_name.as_bytes();
            let len_to_copy = std::cmp::min(bytes.len(), *lpnSize as usize);
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), lpBuffer as *mut u8, len_to_copy);
            *lpBuffer.offset(len_to_copy as isize) = 0;
            *lpnSize = len_to_copy as u32;
        }
    }
    result
}

type FnGetComputerNameW = extern "system" fn(winapi::um::winnt::LPWSTR, winapi::shared::minwindef::LPDWORD) -> winapi::shared::minwindef::BOOL;
static mut ORIGINAL_GET_COMPUTER_NAME_W: Option<FnGetComputerNameW> = None;

extern "system" fn hooked_get_computer_name_w(lpBuffer: winapi::um::winnt::LPWSTR, lpnSize: winapi::shared::minwindef::LPDWORD) -> winapi::shared::minwindef::BOOL {
    let original_fn = unsafe { ORIGINAL_GET_COMPUTER_NAME_W.unwrap() };
    let result = original_fn(lpBuffer, lpnSize);
    if result != 0 && !lpBuffer.is_null() {
        unsafe {
            
            let fake_name = format!("PC-{:02X}{:02X}", FAKE_MAC[0], FAKE_MAC[1]);
            let fake_utf16: Vec<u16> = fake_name.encode_utf16().chain(std::iter::once(0)).collect();
            let len_to_copy = std::cmp::min(fake_utf16.len() - 1, *lpnSize as usize);
            std::ptr::copy_nonoverlapping(fake_utf16.as_ptr(), lpBuffer, len_to_copy);
            *lpBuffer.offset(len_to_copy as isize) = 0;
            *lpnSize = len_to_copy as u32;
        }
    }
    result
}

// --- DeviceIoControl Hook ---
type FnDeviceIoControl = extern "system" fn(winapi::shared::ntdef::HANDLE, winapi::shared::minwindef::DWORD, winapi::shared::minwindef::LPVOID, winapi::shared::minwindef::DWORD, winapi::shared::minwindef::LPVOID, winapi::shared::minwindef::DWORD, winapi::shared::minwindef::LPDWORD, winapi::um::minwinbase::LPOVERLAPPED) -> winapi::shared::minwindef::BOOL;
static mut ORIGINAL_DEVICE_IO_CONTROL: Option<FnDeviceIoControl> = None;

extern "system" fn hooked_device_io_control(hDevice: winapi::shared::ntdef::HANDLE, dwIoControlCode: winapi::shared::minwindef::DWORD, lpInBuffer: winapi::shared::minwindef::LPVOID, nInBufferSize: winapi::shared::minwindef::DWORD, lpOutBuffer: winapi::shared::minwindef::LPVOID, nOutBufferSize: winapi::shared::minwindef::DWORD, lpBytesReturned: winapi::shared::minwindef::LPDWORD, lpOverlapped: winapi::um::minwinbase::LPOVERLAPPED) -> winapi::shared::minwindef::BOOL {
    // IOCTL_STORAGE_QUERY_PROPERTY = 0x002D1400, SMART_GET_VERSION = 0x00074080, DFP_RECEIVE_DRIVE_DATA = 0x0007C088
    if dwIoControlCode == 0x002D1400 || dwIoControlCode == 0x00074080 || dwIoControlCode == 0x0007C088 {
        
        unsafe { if !lpBytesReturned.is_null() { *lpBytesReturned = 0; } }
        // Fake failure to prevent getting hardware serials
        unsafe { winapi::um::errhandlingapi::SetLastError(winapi::shared::winerror::ERROR_ACCESS_DENIED); }
        return 0;
    }
    
    let original_fn = unsafe { ORIGINAL_DEVICE_IO_CONTROL.unwrap() };
    original_fn(hDevice, dwIoControlCode, lpInBuffer, nInBufferSize, lpOutBuffer, nOutBufferSize, lpBytesReturned, lpOverlapped)
}

// --- CoCreateInstance Hook ---
type FnCoCreateInstance = extern "system" fn(*const winapi::shared::guiddef::IID, *mut winapi::ctypes::c_void, winapi::shared::minwindef::DWORD, *const winapi::shared::guiddef::IID, *mut *mut winapi::ctypes::c_void) -> winapi::shared::winerror::HRESULT;
static mut ORIGINAL_CO_CREATE_INSTANCE: Option<FnCoCreateInstance> = None;

extern "system" fn hooked_co_create_instance(rclsid: *const winapi::shared::guiddef::IID, pUnkOuter: *mut winapi::ctypes::c_void, dwClsContext: winapi::shared::minwindef::DWORD, riid: *const winapi::shared::guiddef::IID, ppv: *mut *mut winapi::ctypes::c_void) -> winapi::shared::winerror::HRESULT {
    unsafe {
        if !rclsid.is_null() {
            // WbemLocator CLSID: 4590f811-1d3a-11d0-891f-00aa004b2e24
            if (*rclsid).Data1 == 0x4590f811 {
                
                return winapi::shared::winerror::E_ACCESSDENIED;
            }
        }
    }
    let original_fn = unsafe { ORIGINAL_CO_CREATE_INSTANCE.unwrap() };
    original_fn(rclsid, pUnkOuter, dwClsContext, riid, ppv)
}

type FnObjectFromLresult = extern "system" fn(winapi::shared::minwindef::LRESULT, REFIID, winapi::shared::minwindef::WPARAM, *mut *mut c_void) -> winapi::shared::winerror::HRESULT;

pub(crate) unsafe fn idispatch_get_property(disp: *mut IDispatch, name: &str) -> Option<*mut IDispatch> {
    let mut name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut name_ptr = name_wide.as_mut_ptr();
    let mut dispid = 0;
    
    if (*disp).GetIDsOfNames(&IID_NULL, &mut name_ptr, 1, 0x0800, &mut dispid) != S_OK {
        return None;
    }
    
    let mut result: VARIANT = std::mem::zeroed();
    VariantInit(&mut result);
    let mut params: DISPPARAMS = std::mem::zeroed();
    
    if (*disp).Invoke(dispid, &IID_NULL, 0x0800, 2 /* DISPATCH_PROPERTYGET */, &mut params, &mut result, ptr::null_mut(), ptr::null_mut()) == S_OK {
        if result.n1.n2_mut().vt == VT_DISPATCH as u16 {
            let pdisp = *result.n1.n2_mut().n3.pdispVal();
            if !pdisp.is_null() {
                return Some(pdisp);
            }
        }
    }
    None
}

pub(crate) unsafe fn idispatch_get_string_property(disp: *mut IDispatch, name: &str) -> Option<String> {
    let mut name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut name_ptr = name_wide.as_mut_ptr();
    let mut dispid = 0;
    
    if (*disp).GetIDsOfNames(&IID_NULL, &mut name_ptr, 1, 0x0800, &mut dispid) != S_OK {
        return None;
    }
    
    let mut result: VARIANT = std::mem::zeroed();
    VariantInit(&mut result);
    let mut params: DISPPARAMS = std::mem::zeroed();
    
    if (*disp).Invoke(dispid, &IID_NULL, 0x0800, 2 /* DISPATCH_PROPERTYGET */, &mut params, &mut result, ptr::null_mut(), ptr::null_mut()) == S_OK {
        if result.n1.n2_mut().vt == VT_BSTR as u16 {
            let bstr = *result.n1.n2_mut().n3.bstrVal();
            if !bstr.is_null() {
                let len = winapi::um::oleauto::SysStringLen(bstr) as usize;
                let slice = std::slice::from_raw_parts(bstr, len);
                let string = String::from_utf16_lossy(slice);
                return Some(string);
            }
        }
    }
    None
}

pub(crate) unsafe fn idispatch_invoke_method(disp: *mut IDispatch, name: &str, arg1: &str, arg2: &str) {
    let mut name_wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut name_ptr = name_wide.as_mut_ptr();
    let mut dispid = 0;
    
    if (*disp).GetIDsOfNames(&IID_NULL, &mut name_ptr, 1, 0x0800, &mut dispid) != S_OK {
        return;
    }
    
    let bstr1 = SysAllocString(arg1.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>().as_ptr());
    let bstr2 = SysAllocString(arg2.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>().as_ptr());
    
    let mut var1: VARIANT = std::mem::zeroed();
    VariantInit(&mut var1);
    var1.n1.n2_mut().vt = VT_BSTR as u16;
    *var1.n1.n2_mut().n3.bstrVal_mut() = bstr1;
    
    let mut var2: VARIANT = std::mem::zeroed();
    VariantInit(&mut var2);
    var2.n1.n2_mut().vt = VT_BSTR as u16;
    *var2.n1.n2_mut().n3.bstrVal_mut() = bstr2;
    
    // args must be in reverse order
    let mut args = vec![var2, var1];
    
    let mut params = DISPPARAMS {
        rgvarg: args.as_mut_ptr(),
        rgdispidNamedArgs: ptr::null_mut(),
        cArgs: 2,
        cNamedArgs: 0,
    };
    
    let mut result: VARIANT = std::mem::zeroed();
    VariantInit(&mut result);
    
    (*disp).Invoke(dispid, &IID_NULL, 0x0800, 1 /* DISPATCH_METHOD */, &mut params, &mut result, ptr::null_mut(), ptr::null_mut());
    
    SysFreeString(bstr1);
    SysFreeString(bstr2);
}

pub(crate) fn log_debug(msg: &str) {
    let log_path = "C:\\Users\\Public\\ikv_debug.log";
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
        use std::io::Write;
        let _ = writeln!(file, "{}", msg);
    }
}

unsafe extern "system" fn enum_child_proc(hwnd: winapi::shared::windef::HWND, lparam: winapi::shared::minwindef::LPARAM) -> winapi::shared::minwindef::BOOL {
    let mut pid: u32 = 0;
    GetWindowThreadProcessId(hwnd, &mut pid);
    if pid == GetCurrentProcessId() {
        let mut class_name = [0u8; 256];
        GetClassNameA(hwnd, class_name.as_mut_ptr() as *mut i8, 256);
        let class_str = std::ffi::CStr::from_ptr(class_name.as_ptr() as *const i8).to_string_lossy();
        if class_str == "Internet Explorer_Server" {
            *(lparam as *mut winapi::shared::windef::HWND) = hwnd;
            return 0; // Stop enumerating
        }
    }
    1 // Continue enumerating
}

fn spawn_macro_thread(user: String, pass: String) {
    std::thread::spawn(move || {
        log_debug("Macro thread started!");
        unsafe { CoInitialize(ptr::null_mut()); }
        
        let mut hwnd_ie: winapi::shared::windef::HWND = ptr::null_mut();
        for i in 0..300 {
            unsafe {
                EnumChildWindows(GetDesktopWindow(), Some(enum_child_proc), &mut hwnd_ie as *mut _ as winapi::shared::minwindef::LPARAM);
            }
            if !hwnd_ie.is_null() { 
                log_debug(&format!("Found IE Server HWND at iteration {}", i));
                break; 
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        if hwnd_ie.is_null() { 
            log_debug("Failed to find IE Server HWND!");
            return; 
        }
        
        // We removed the 1000ms sleep. We will now retry injection in a loop!
        log_debug("IE Server HWND found. Starting DOM injection loop...");

        for inject_attempt in 0..15 {
            // Sleep 200ms between attempts
            std::thread::sleep(std::time::Duration::from_millis(200));
            
            unsafe {
                let msg = RegisterWindowMessageA(b"WM_HTML_GETOBJECT\0".as_ptr() as *const i8);
                let mut lresult: usize = 0;
                SendMessageTimeoutA(hwnd_ie, msg, 0, 0, SMTO_ABORTIFHUNG, 1000, &mut lresult);
                
                if lresult != 0 {
                    let oleacc = winapi::um::libloaderapi::LoadLibraryA(b"oleacc.dll\0".as_ptr() as *const i8);
                    if !oleacc.is_null() {
                        let func = winapi::um::libloaderapi::GetProcAddress(oleacc, b"ObjectFromLresult\0".as_ptr() as *const i8);
                        if !func.is_null() {
                            let object_from_lresult: FnObjectFromLresult = std::mem::transmute(func);
                            
                            let mut iid_idispatch = winapi::shared::guiddef::GUID {
                                Data1: 0x00020400,
                                Data2: 0x0000,
                                Data3: 0x0000,
                                Data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
                            };
                            
                            let mut disp_ptr: *mut c_void = ptr::null_mut();
                            let hr = object_from_lresult(lresult as isize, &iid_idispatch, 0, &mut disp_ptr);
                            
                            if hr == S_OK && !disp_ptr.is_null() {
                                let doc_disp = disp_ptr as *mut IDispatch;
                                if let Some(win_disp) = idispatch_get_property(doc_disp, "parentWindow") {
                                    let hex_user: String = user.bytes().map(|b| format!("{:02x}", b)).collect();
                                    let hex_pass: String = pass.bytes().map(|b| format!("{:02x}", b)).collect();
                                    
                                    let js = format!("
                                        (function() {{
                                            if (window._ikv_injected) return;
                                            function h2a(h) {{
                                                var s = '';
                                                for (var i = 0; i < h.length; i += 2) s += String.fromCharCode(parseInt(h.substr(i, 2), 16));
                                                return decodeURIComponent(escape(s));
                                            }}
                                            var u = document.getElementById('txtUserName') || document.getElementsByName('userName')[0] || document.querySelector('input[type=\"text\"]');
                                            var p = document.getElementById('txtPassword') || document.getElementById('txtPasword') || document.getElementsByName('password')[0] || document.querySelector('input[type=\"password\"]');
                                            
                                            if(u && p) {{
                                                u.value = h2a('{}');
                                                p.value = h2a('{}');
                                                
                                                var c = null;
                                                var inputs = document.getElementsByTagName('input');
                                                for (var i = 0; i < inputs.length; i++) {{
                                                    var inp = inputs[i];
                                                    if (inp.type === 'text' && inp !== u) {{
                                                        var id = (inp.id || '').toLowerCase();
                                                        var name = (inp.name || '').toLowerCase();
                                                        if (id.indexOf('sec') > -1 || name.indexOf('sec') > -1 || id.indexOf('cap') > -1 || name.indexOf('cap') > -1 || id.indexOf('kod') > -1 || name.indexOf('kod') > -1) {{
                                                            c = inp; break;
                                                        }}
                                                    }}
                                                }}
                                                if (!c) {{
                                                    for (var i = 0; i < inputs.length; i++) {{
                                                        if (inputs[i].type === 'text' && inputs[i] !== u) {{
                                                            c = inputs[i]; break;
                                                        }}
                                                    }}
                                                }}
                                                if (c) {{ c.focus(); }}
                                                
                                                window._ikv_injected = true;
                                            }}
                                        }})();
                                    ", hex_user, hex_pass);
                                    
                                    idispatch_invoke_method(win_disp, "execScript", &js, "javascript");
                                    
                                    (*win_disp).Release();
                                }
                                (*doc_disp).Release();
                            }
                        }
                    }
                }
            }
        }
    });
}

fn click_coordinates(x: i32, y: i32) {
    unsafe {
        let process_id = winapi::um::processthreadsapi::GetCurrentProcessId();
        let mut target_hwnd: winapi::shared::windef::HWND = std::ptr::null_mut();
        
        // Find the main window of our process
        winapi::um::winuser::EnumWindows(Some(enum_windows_proc), &mut target_hwnd as *mut _ as winapi::shared::minwindef::LPARAM);
        
        if !target_hwnd.is_null() {
            log_debug(&format!("Found Game HWND. Clicking X: {}, Y: {}", x, y));
            
            // Convert client coordinates to screen coordinates
            let mut pt = winapi::shared::windef::POINT { x, y };
            winapi::um::winuser::ClientToScreen(target_hwnd, &mut pt);
            
            // Move mouse
            winapi::um::winuser::SetCursorPos(pt.x, pt.y);
            std::thread::sleep(std::time::Duration::from_millis(10));
            
            // Send actual hardware mouse click
            let mut input: winapi::um::winuser::INPUT = std::mem::zeroed();
            input.type_ = winapi::um::winuser::INPUT_MOUSE;
            input.u.mi_mut().dwFlags = winapi::um::winuser::MOUSEEVENTF_LEFTDOWN;
            winapi::um::winuser::SendInput(1, &mut input, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32);
            
            std::thread::sleep(std::time::Duration::from_millis(50));
            
            input.u.mi_mut().dwFlags = winapi::um::winuser::MOUSEEVENTF_LEFTUP;
            winapi::um::winuser::SendInput(1, &mut input, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32);
            
        } else {
            log_debug("Could not find Game HWND for clicking!");
        }
    }
}

unsafe extern "system" fn enum_windows_proc(hwnd: winapi::shared::windef::HWND, lparam: winapi::shared::minwindef::LPARAM) -> winapi::shared::minwindef::BOOL {
    let mut pid: u32 = 0;
    winapi::um::winuser::GetWindowThreadProcessId(hwnd, &mut pid);
    if pid == winapi::um::processthreadsapi::GetCurrentProcessId() {
        // Ensure it's a visible window
        if winapi::um::winuser::IsWindowVisible(hwnd) != 0 {
            *(lparam as *mut winapi::shared::windef::HWND) = hwnd;
            return 0; // Stop enumeration
        }
    }
    1 // Continue
}

fn scan_memory_for_string() {
    std::thread::spawn(move || {
        let log_file = "C:\\Users\\Public\\ikv_scanner_log.txt";
        let _ = std::fs::write(log_file, "Starting FAST offset-based memory scan for Tungsten Madeni...\n");
        let process_handle = unsafe { winapi::um::processthreadsapi::GetCurrentProcess() };
        let search_pattern: [u8; 10] = [0x54, 0x00, 0x6F, 0x00, 0x70, 0x00, 0x6C, 0x00, 0x61, 0x00]; // "Topla" in UTF-16LE

        loop {
            let mut address: usize = 0;
            let mut mem_info: winapi::um::winnt::MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
            let mut found_count = 0;

            loop {
                let result = unsafe {
                    winapi::um::memoryapi::VirtualQuery(
                        address as *const winapi::ctypes::c_void,
                        &mut mem_info,
                        std::mem::size_of::<winapi::um::winnt::MEMORY_BASIC_INFORMATION>(),
                    )
                };

                if result == 0 {
                    break; // Reached end of memory
                }

                let protect = mem_info.Protect & 0xFF;
                if mem_info.State == winapi::um::winnt::MEM_COMMIT
                    && (protect == 0x02 || protect == 0x04 || protect == 0x20 || protect == 0x40)
                {
                    let mut buffer = vec![0u8; mem_info.RegionSize];
                    let mut bytes_read: usize = 0;

                    unsafe {
                        winapi::um::memoryapi::ReadProcessMemory(
                            process_handle,
                            mem_info.BaseAddress,
                            buffer.as_mut_ptr() as *mut winapi::ctypes::c_void,
                            mem_info.RegionSize,
                            &mut bytes_read,
                        );
                    }

                    if bytes_read > 0 && buffer.len() >= search_pattern.len() {
                        // Search for the pattern in the buffer
                        for i in (0..=(buffer.len() - search_pattern.len())).step_by(2) {
                            if buffer[i..i + search_pattern.len()] == search_pattern {
                                let found_addr = mem_info.BaseAddress as usize + i;
                                let _ = std::fs::OpenOptions::new().create(true).append(true).open(log_file).map(|mut f| {
                                    use std::io::Write;
                                    let _ = writeln!(f, "FOUND STRING AT DYNAMIC ADDRESS: {:#x}", found_addr);
                                });
                                found_count += 1;
                                
                                // FAST OFFSET CLICK!
                                let _ = std::fs::OpenOptions::new().create(true).append(true).open(log_file).map(|mut f| {
                                    use std::io::Write;
                                    let _ = writeln!(f, "String found! Sending instant offset click...");
                                });
                                click_cursor_with_offset(20, 20); // 20 pixels down and right from current cursor
                                
                                std::thread::sleep(std::time::Duration::from_millis(5000)); // Wait 5 seconds after a successful click
                            }
                        }
                    }
                }

                address += mem_info.RegionSize;
            }

            if found_count > 0 {
                let _ = std::fs::OpenOptions::new().create(true).append(true).open(log_file).map(|mut f| {
                    use std::io::Write;
                    let _ = writeln!(f, "Memory scan complete. Found {} instances.", found_count);
                });
            }
            std::thread::sleep(std::time::Duration::from_millis(100)); // Scan every 100ms for FAST response
        }
    });
}

fn click_cursor_with_offset(offset_x: i32, offset_y: i32) {
    unsafe {
        let mut pt = winapi::shared::windef::POINT { x: 0, y: 0 };
        winapi::um::winuser::GetCursorPos(&mut pt);
        
        let virtual_width = winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXVIRTUALSCREEN);
        let virtual_height = winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYVIRTUALSCREEN);
        let virtual_left = winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_XVIRTUALSCREEN);
        let virtual_top = winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_YVIRTUALSCREEN);
        
        if virtual_width > 0 && virtual_height > 0 {
            let target_x = pt.x + offset_x - virtual_left;
            let target_y = pt.y + offset_y - virtual_top;
            
            let abs_x = (target_x * 65535) / virtual_width;
            let abs_y = (target_y * 65535) / virtual_height;

            let mut move_input: winapi::um::winuser::INPUT = std::mem::zeroed();
            move_input.type_ = winapi::um::winuser::INPUT_MOUSE;
            move_input.u.mi_mut().dx = abs_x;
            move_input.u.mi_mut().dy = abs_y;
            move_input.u.mi_mut().dwFlags = winapi::um::winuser::MOUSEEVENTF_MOVE | winapi::um::winuser::MOUSEEVENTF_ABSOLUTE | winapi::um::winuser::MOUSEEVENTF_VIRTUALDESK;
            winapi::um::winuser::SendInput(1, &mut move_input, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32);
        }
        
        std::thread::sleep(std::time::Duration::from_millis(50));
        
        let mut input_down: winapi::um::winuser::INPUT = std::mem::zeroed();
        input_down.type_ = winapi::um::winuser::INPUT_MOUSE;
        input_down.u.mi_mut().dwFlags = winapi::um::winuser::MOUSEEVENTF_LEFTDOWN;
        winapi::um::winuser::SendInput(1, &mut input_down, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32);
        
        std::thread::sleep(std::time::Duration::from_millis(50));
        
        let mut input_up: winapi::um::winuser::INPUT = std::mem::zeroed();
        input_up.type_ = winapi::um::winuser::INPUT_MOUSE;
        input_up.u.mi_mut().dwFlags = winapi::um::winuser::MOUSEEVENTF_LEFTUP;
        winapi::um::winuser::SendInput(1, &mut input_up, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32);
    }
}

mod ocr_macro;

fn initialize() {
    log_debug("DLL Injected and initialize() called!");
    
    let process_id = unsafe { winapi::um::processthreadsapi::GetCurrentProcessId() };
    
    let mut macro_path_buf = std::env::temp_dir();
    macro_path_buf.push(format!("ikv_macro_{}.txt", process_id));
    let macro_path = macro_path_buf.to_string_lossy().into_owned();
    
    let mut spoofer_active = true;

    if let Ok(content) = std::fs::read_to_string(&macro_path) {
        log_debug("Found macro file, reading...");
        let lines: Vec<&str> = content.lines().collect();
        if lines.len() >= 2 {
            log_debug("Credentials found, spawning OCR macro thread.");
            ocr_macro::spawn_ocr_macro_thread(lines[0].to_string(), lines[1].to_string());
        } else {
            log_debug("macro file does not have 2 lines.");
        }
        
        if lines.len() >= 3 && lines[2].trim().eq_ignore_ascii_case("false") {
            spoofer_active = false;
        }

        // Try to clean up the file after reading so they don't accumulate
        let _ = std::fs::remove_file(&macro_path);
    } else {
        log_debug(&format!("macro file not found: {}", macro_path));
    }

    // log_debug("Initiating FAST Memory Scanner for Topla offset...");
    // scan_memory_for_string();
    
    if !spoofer_active {
        log_debug("Spoofer is disabled via settings. Skipping hooks.");
        return;
    }
    
    log_debug("Spoofer is active. Applying MinHook hooks...");
    
    // Initialize MinHook
    unsafe {
        // minhook 0.8.0 initializes automatically or doesn't have an explicit init.
        // We just proceed to hooking.

        // Get pointer to original GetAdaptersInfo
        let module = b"iphlpapi.dll\0".as_ptr() as *const i8;
        let func = b"GetAdaptersInfo\0".as_ptr() as *const i8;
        
        let h_module = winapi::um::libloaderapi::GetModuleHandleA(module);
        if h_module.is_null() {
            winapi::um::libloaderapi::LoadLibraryA(module);
        }
        
        let target_addr = winapi::um::libloaderapi::GetProcAddress(
            winapi::um::libloaderapi::GetModuleHandleA(module),
            func
        ) as *mut c_void;

        if !target_addr.is_null() {
            if let Ok(original) = MinHook::create_hook(
                target_addr,
                hooked_get_adapters_info as *mut c_void,
            ) {
                ORIGINAL_GET_ADAPTERS_INFO = Some(std::mem::transmute(original));
            }
        }

        // Hook RegQueryValueExA
        let advapi = b"advapi32.dll\0".as_ptr() as *const i8;
        let h_advapi = winapi::um::libloaderapi::GetModuleHandleA(advapi);
        if h_advapi.is_null() {
            winapi::um::libloaderapi::LoadLibraryA(advapi);
        }

        let reg_a = winapi::um::libloaderapi::GetProcAddress(
            winapi::um::libloaderapi::GetModuleHandleA(advapi),
            b"RegQueryValueExA\0".as_ptr() as *const i8
        ) as *mut c_void;

        if !reg_a.is_null() {
            if let Ok(original) = MinHook::create_hook(
                reg_a,
                hooked_reg_query_value_ex_a as *mut c_void,
            ) {
                ORIGINAL_REG_QUERY_VALUE_EX_A = Some(std::mem::transmute(original));
            }
        }

        // Hook RegQueryValueExW
        let reg_w = winapi::um::libloaderapi::GetProcAddress(
            winapi::um::libloaderapi::GetModuleHandleA(advapi),
            b"RegQueryValueExW\0".as_ptr() as *const i8
        ) as *mut c_void;

        if !reg_w.is_null() {
            if let Ok(original) = MinHook::create_hook(
                reg_w,
                hooked_reg_query_value_ex_w as *mut c_void,
            ) {
                ORIGINAL_REG_QUERY_VALUE_EX_W = Some(std::mem::transmute(original));
            }
        }

        // Hook GetVolumeInformationA
        let kernel32 = b"kernel32.dll\0".as_ptr() as *const i8;
        let h_kernel32 = winapi::um::libloaderapi::GetModuleHandleA(kernel32);
        if h_kernel32.is_null() {
            winapi::um::libloaderapi::LoadLibraryA(kernel32);
        }

        let vol_a = winapi::um::libloaderapi::GetProcAddress(
            winapi::um::libloaderapi::GetModuleHandleA(kernel32),
            b"GetVolumeInformationA\0".as_ptr() as *const i8
        ) as *mut c_void;

        if !vol_a.is_null() {
            if let Ok(original) = MinHook::create_hook(
                vol_a,
                hooked_get_volume_information_a as *mut c_void,
            ) {
                ORIGINAL_GET_VOLUME_INFORMATION_A = Some(std::mem::transmute(original));
            }
        }

        // Hook GetVolumeInformationW
        let vol_w = winapi::um::libloaderapi::GetProcAddress(
            winapi::um::libloaderapi::GetModuleHandleA(kernel32),
            b"GetVolumeInformationW\0".as_ptr() as *const i8
        ) as *mut c_void;

        if !vol_w.is_null() {
            if let Ok(original) = MinHook::create_hook(
                vol_w,
                hooked_get_volume_information_w as *mut c_void,
            ) {
                ORIGINAL_GET_VOLUME_INFORMATION_W = Some(std::mem::transmute(original));
            }
        }

        // Hook GetComputerNameA
        let comp_a = winapi::um::libloaderapi::GetProcAddress(
            winapi::um::libloaderapi::GetModuleHandleA(kernel32),
            b"GetComputerNameA\0".as_ptr() as *const i8
        ) as *mut c_void;

        if !comp_a.is_null() {
            if let Ok(original) = MinHook::create_hook(
                comp_a,
                hooked_get_computer_name_a as *mut c_void,
            ) {
                ORIGINAL_GET_COMPUTER_NAME_A = Some(std::mem::transmute(original));
            }
        }

        // Hook GetComputerNameW
        let comp_w = winapi::um::libloaderapi::GetProcAddress(
            winapi::um::libloaderapi::GetModuleHandleA(kernel32),
            b"GetComputerNameW\0".as_ptr() as *const i8
        ) as *mut c_void;

        if !comp_w.is_null() {
            if let Ok(original) = MinHook::create_hook(
                comp_w,
                hooked_get_computer_name_w as *mut c_void,
            ) {
                ORIGINAL_GET_COMPUTER_NAME_W = Some(std::mem::transmute(original));
            }
        }

        // Hook DeviceIoControl
        let dev_io = winapi::um::libloaderapi::GetProcAddress(
            winapi::um::libloaderapi::GetModuleHandleA(kernel32),
            b"DeviceIoControl\0".as_ptr() as *const i8
        ) as *mut c_void;

        if !dev_io.is_null() {
            if let Ok(original) = MinHook::create_hook(
                dev_io,
                hooked_device_io_control as *mut c_void,
            ) {
                ORIGINAL_DEVICE_IO_CONTROL = Some(std::mem::transmute(original));
            }
        }

        // Hook CoCreateInstance
        let ole32 = b"ole32.dll\0".as_ptr() as *const i8;
        let h_ole32 = winapi::um::libloaderapi::GetModuleHandleA(ole32);
        if h_ole32.is_null() {
            winapi::um::libloaderapi::LoadLibraryA(ole32);
        }

        let co_create = winapi::um::libloaderapi::GetProcAddress(
            winapi::um::libloaderapi::GetModuleHandleA(ole32),
            b"CoCreateInstance\0".as_ptr() as *const i8
        ) as *mut c_void;

        if !co_create.is_null() {
            if let Ok(original) = MinHook::create_hook(
                co_create,
                hooked_co_create_instance as *mut c_void,
            ) {
                ORIGINAL_CO_CREATE_INSTANCE = Some(std::mem::transmute(original));
            }
        }

        let _ = MinHook::enable_all_hooks();
    }
}

fn cleanup() {
    unsafe {
        let _ = MinHook::disable_all_hooks();
        // FreeConsole();
    }
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID,
) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            initialize();
        }
        DLL_PROCESS_DETACH => {
            cleanup();
        }
        _ => {}
    }
    TRUE
}



