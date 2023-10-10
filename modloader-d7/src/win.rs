use std::{
    borrow::Borrow,
    ffi::{CStr, CString},
    path::PathBuf,
    sync::RwLock,
};

use detour::RawDetour;
use lazy_static::lazy_static;
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use winapi::{
    shared::{
        minwindef::{DWORD, HMODULE, LPVOID},
        ntdef::{BOOLEAN, LPCSTR},
    },
    um::{
        libloaderapi::{DisableThreadLibraryCalls, GetModuleHandleA, GetProcAddress, LoadLibraryA},
        winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
};

lazy_static! {
    static ref MOD_LOAD_ORDER: RwLock<Vec<PathBuf>> = RwLock::default();
}

static mut NMPL_FILE_CFILEMGR_OPENCMN_TRAMPOLINE: *const () = std::ptr::null();
type PfnNmplFileCFileMgrOpenCmn = extern "fastcall" fn(
    this: LPVOID,
    a2: LPVOID,
    path: LPCSTR,
    e_device_type: u32,
    e_access_type: u32,
    e_file_wait_mode: u32,
    e_cache_mode: u32,
) -> LPVOID;

extern "fastcall" fn hook_nmpl_file_cfilemgr_opencmn(
    this: LPVOID,
    a2: LPVOID,
    path: LPCSTR,
    e_device_type: u32,
    mut e_access_type: u32,
    e_file_wait_mode: u32,
    e_cache_mode: u32,
) -> LPVOID {
    loop {
        if path.is_null() {
            log::debug!("reading null file");
            break;
        }
        unsafe {
            let path_str = CStr::from_ptr(path as *const i8);
            let safe_path_str = path_str.to_string_lossy();

            log::debug!(
                "getting path={} device_type={} access={}",
                safe_path_str,
                e_device_type,
                e_access_type
            );

            // check if the file exists
            // if it does, override the load mode to use loose files
            // (interesting that this codepath still exists in the release build!)
            // Only device type 5 should be overridden (?)
            // 2 is used for user data (.systemsave, save.lst, etc)
            if e_device_type == 5 {
                match std::fs::metadata(PathBuf::from("mods").join(safe_path_str.as_ref())) {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            e_access_type = 0;
                        }
                    }
                    _ => {}
                }
            }
        }
        break;
    }
    unsafe {
        let path_str = CStr::from_ptr(path as *const i8);
        let safe_path_str = path_str.to_string_lossy();

        // _generated replacements loading
        {
            let dest_path = PathBuf::from("mods/_generated").join(safe_path_str.as_ref());
            match std::fs::metadata(&dest_path) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        let new_path = CString::new(dest_path.to_string_lossy().as_ref()).unwrap();
                        log::debug!("Using generated {:?}", new_path);
                        return (std::mem::transmute::<_, PfnNmplFileCFileMgrOpenCmn>(
                            NMPL_FILE_CFILEMGR_OPENCMN_TRAMPOLINE,
                        ))(
                            this,
                            a2,
                            new_path.as_ptr(),
                            e_device_type,
                            0,
                            e_file_wait_mode,
                            e_cache_mode,
                        );
                    }
                }
                _ => {}
            }
        }

        // Mod loading
        let mod_load_order = MOD_LOAD_ORDER.read().unwrap();
        for mod_path in mod_load_order.iter() {
            let dest_path = mod_path.join("files").join(safe_path_str.as_ref());
            match std::fs::metadata(&dest_path) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        let new_path = CString::new(dest_path.to_string_lossy().as_ref()).unwrap();
                        return (std::mem::transmute::<_, PfnNmplFileCFileMgrOpenCmn>(
                            NMPL_FILE_CFILEMGR_OPENCMN_TRAMPOLINE,
                        ))(
                            this,
                            a2,
                            new_path.as_ptr(),
                            e_device_type,
                            0,
                            e_file_wait_mode,
                            e_cache_mode,
                        );
                    }
                }
                _ => {}
            }
        }
        return (std::mem::transmute::<_, PfnNmplFileCFileMgrOpenCmn>(
            NMPL_FILE_CFILEMGR_OPENCMN_TRAMPOLINE,
        ))(
            this,
            a2,
            path,
            e_device_type,
            e_access_type,
            e_file_wait_mode,
            e_cache_mode,
        );
    }
}

fn log_init() {
    let file = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("modloader-d7.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(Root::builder().appender("file").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn init_mod_load_order() {
    // Init mod load order
    let mut mod_load_order = MOD_LOAD_ORDER.write().unwrap();
    let read_dir = match std::fs::read_dir("mods") {
        Ok(r) => r,
        Err(_) => {
            log::warn!("mods directory not found or accessible; mod load order will be empty");
            return;
        }
    };
    for entry in read_dir {
        match entry {
            Ok(entry) => {
                if entry.metadata().unwrap().is_dir() && entry.file_name() != "_generated" {
                    mod_load_order.push(entry.path());
                }
            }
            Err(_) => continue,
        }
    }
    mod_load_order.reverse();
    log::info!("Mod load order: {:?}", mod_load_order.borrow());
}

fn init() {
    log_init();

    init_mod_load_order();
    log::info!("Mod load order initialized");
    // Disgaea 7 no longer has a special script archive!

    unsafe {
        LoadLibraryA(b"NmplDLL.dll\0".as_ptr() as *const i8);
        let engine_handle = GetModuleHandleA(b"NmplDLL.dll\0".as_ptr() as *const i8);
        // let base_address = engine_handle as LPVOID;

        let opencmn_detour = match RawDetour::new(
            GetProcAddress(engine_handle, b"?openCmn@CFileMgr@File@Nmpl@@IEAA?AV?$intrusive_ptr@VCFile@File@Nmpl@@@Core@3@PEBDW4EDeviceType@23@W4EAccessType@23@W4EFileWaitMode@23@W4ECacheMode@23@@Z\0".as_ptr() as LPCSTR) as *const (),
            hook_nmpl_file_cfilemgr_opencmn as *const (),
        ) {
            Err(e) => {
                log::error!("unable to find Nmpl::File::CFileMgr::openCmn: {}", e);
                return;
            }
            Ok(o) => o,
        };
        NMPL_FILE_CFILEMGR_OPENCMN_TRAMPOLINE = opencmn_detour.trampoline();
        match opencmn_detour.enable() {
            Err(e) => {
                log::error!(
                    "unable to install Nmpl::File::CFileMgr::openCmn hook: {}",
                    e
                );
            }
            _ => {}
        }
        log::info!("Nmpl::File::CFileMgr::openCmn hooked");
    }

    log::info!("Fully loaded!");
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "stdcall" fn DllMain(module: HMODULE, reason: DWORD, _reserved: LPVOID) -> BOOLEAN {
    // https://msdn.microsoft.com/en-us/library/windows/desktop/ms682583(v=vs.85).aspx
    match reason {
        DLL_PROCESS_ATTACH => unsafe {
            DisableThreadLibraryCalls(module);
            init();
        },
        DLL_PROCESS_DETACH => {}
        _ => (),
    }
    1
}
