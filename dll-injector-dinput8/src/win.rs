use std::ffi::{CStr, CString, OsStr};
use std::os::windows::prelude::OsStrExt;
use std::{mem, fs};
use std::io;
use std::path::PathBuf;

use winapi::shared::guiddef::IID;
use winapi::shared::minwindef::{DWORD, HINSTANCE, HMODULE};
use winapi::shared::ntdef::{HRESULT, LPCWSTR};
use winapi::um::libloaderapi::{DisableThreadLibraryCalls, LoadLibraryW};
use winapi::um::winnt::{BOOLEAN, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use winapi::{
    shared::{
        minwindef::{FARPROC, LPVOID, MAX_PATH},
        ntdef::{LPCSTR, LPSTR},
    },
    um::{
        libloaderapi::{GetProcAddress, LoadLibraryA},
        sysinfoapi::GetSystemDirectoryA,
        unknwnbase::LPUNKNOWN,
    },
};

fn get_system_directory() -> CString {
    unsafe {
        let mut syspath: [u8; MAX_PATH] = [0; MAX_PATH];
        let syspath_ptr: LPSTR = mem::transmute::<_, LPSTR>(&mut syspath);
        let len = GetSystemDirectoryA(syspath_ptr, MAX_PATH as u32) as usize + 1;
        match CStr::from_bytes_with_nul(&syspath[..len + 1]) {
            Ok(c) => c.to_owned(),
            Err(_) => CString::new("C:\\WINDOWS\\system32").unwrap(),
        }
    }
}

#[allow(non_snake_case)]
type PFNDirectInput8Create =
    extern "stdcall" fn(HINSTANCE, DWORD, *const IID, *mut LPVOID, LPUNKNOWN) -> HRESULT;

#[allow(non_snake_case)]
//#[no_mangle]
#[export_name = "DirectInput8Create"]
pub extern "stdcall" fn DirectInput8Create(
    inst: HINSTANCE,
    version: DWORD,
    riid: *const IID,
    out: *mut LPVOID,
    u: LPUNKNOWN,
) -> HRESULT {
    let syspath = get_system_directory().into_string().unwrap() + "\\dinput8.dll";
    unsafe {
        let hMod = LoadLibraryA(syspath.as_ptr() as LPCSTR);
        let fnName = CString::new("DirectInput8Create").unwrap();
        let procaddr = mem::transmute::<FARPROC, PFNDirectInput8Create>(GetProcAddress(
            hMod,
            fnName.as_ptr() as LPCSTR,
        ));
        let res = (procaddr)(inst, version, riid, out, u);

        res
    }
}

fn load_dlls_in_directory() -> io::Result<()> {
    let mut plugins_path = PathBuf::new();
    plugins_path.push("mkplugins");
    let read_dir = match fs::read_dir(&plugins_path) {
        Ok(d) => d,
        Err(_) => return Ok(()),
    };
    for entry in read_dir {
        let entry = entry?;
        let path = entry.path();
        let full_path = path;
        if entry.file_type()?.is_file() && PathBuf::from(entry.file_name()).extension() == Some(&std::ffi::OsString::from("dll")) {
            let full_path_os_str: &OsStr = full_path.as_ref();
            let full_path_str = full_path_os_str.to_string_lossy();
            println!("Loading dll '{}'", full_path_str);
            let mut wstr: Vec<u16> = full_path_os_str.encode_wide().collect();
            wstr.push(0);
            unsafe {
                let module = LoadLibraryW(wstr.as_ptr() as LPCWSTR);
                if module.is_null() {
                    let error = winapi::um::errhandlingapi::GetLastError();
                    println!("Failed to load library '{}', error code {}: ", full_path_str, error);
                }
            }
        }
    }
    Ok(())
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "stdcall" fn DllMain(module: HMODULE, reason: DWORD, _reserved: LPVOID) -> BOOLEAN {
    // https://msdn.microsoft.com/en-us/library/windows/desktop/ms682583(v=vs.85).aspx
    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                DisableThreadLibraryCalls(module);
            }
            load_dlls_in_directory().unwrap();
        }
        DLL_PROCESS_DETACH => {}
        _ => (),
    }
    1
}
