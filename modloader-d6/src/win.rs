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
        libloaderapi::{DisableThreadLibraryCalls, GetModuleHandleA},
        memoryapi::VirtualQuery,
        winnt::{
            DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, MEMORY_BASIC_INFORMATION, MEM_COMMIT,
            PAGE_NOACCESS,
        },
    },
};

lazy_static! {
    static ref MOD_LOAD_ORDER: RwLock<Vec<PathBuf>> = RwLock::default();
}

fn aob_search(buf: &[u8], start: LPVOID, dist: usize) -> Option<LPVOID> {
    if buf.is_empty() {
        return None;
    }

    unsafe {
        let mut current: LPVOID = start;
        let end = (start as usize + dist) as LPVOID;

        loop {
            let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();

            if current >= end {
                break;
            }

            if VirtualQuery(
                current,
                &mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            ) <= 0
                || mbi.State != MEM_COMMIT
                || mbi.Protect == PAGE_NOACCESS
            {
                current = (current as usize + 1) as LPVOID;
                continue;
            }

            let safe_slice: &'static mut [u8] =
                std::slice::from_raw_parts_mut(current as *mut u8, mbi.RegionSize.min(dist));
            match safe_slice.windows(buf.len()).find(|e| *e == buf) {
                Some(s) => {
                    return Some(s.as_ptr() as LPVOID);
                }
                None => {
                    current = (current as usize + mbi.RegionSize) as LPVOID;
                    continue;
                }
            }
        }
    }

    None
}

// static AOB_LOAD_FROM_ARCHIVE_FUNC: &'static [u8] = &[
//     0x40, 0x55, 0x56, 0x57, 0x41, 0x56, 0x41, 0x57, 0x48, 0x81, 0xEC, 0xD0, 0x00, 0x00, 0x00, 0x48,
//     0xC7, 0x44, 0x24, 0x20, 0xFE, 0xFF, 0xFF, 0xFF, 0x48, 0x89, 0x9C, 0x24, 0x18, 0x01, 0x00, 0x00,
//     0x48, 0x8B, 0x05, 0xE9, 0xA9, 0xB3, 0x00, 0x48, 0x33, 0xC4, 0x48, 0x89, 0x84, 0x24, 0xC0, 0x00,
//     0x00, 0x00, 0x45, 0x0F, 0xB6, 0xF1, 0x49, 0x8B, 0xE8, 0x48, 0x8B, 0xF2, 0x48, 0x8B, 0xD9, 0x48,
//     0x89, 0x4C, 0x24, 0x28, 0x4C, 0x89, 0x44, 0x24, 0x30,
// ];

// static AOB_GET_FILE_FROM_ARCHIVE_FUNC: &'static [u8] = &[
//     0x40, 0x53, 0x55, 0x56, 0x57, 0x41, 0x56, 0x41, 0x57, 0x48, 0x83, 0xEC, 0x78, 0x48, 0xC7, 0x44,
//     0x24, 0x50, 0xFE, 0xFF, 0xFF, 0xFF, 0x48, 0x8B, 0xF2, 0x4C, 0x8B, 0xF1, 0x48, 0x89, 0x54, 0x24,
//     0x48, 0x45, 0x33, 0xFF, 0x44, 0x89, 0x7C, 0x24, 0x40, 0x8B, 0x84, 0x24, 0xE0, 0x00, 0x00, 0x00,
//     0x89, 0x44, 0x24, 0x30, 0x8B, 0x84, 0x24, 0xD0, 0x00, 0x00, 0x00, 0x89, 0x44, 0x24, 0x20,
// ];

static AOB_GET_FILE_FROM_ARCHIVE_2_FUNC: &'static [u8] = &[
    0x40, 0x53, 0x55, 0x56, 0x57, 0x41, 0x54, 0x41, 0x55, 0x41, 0x56, 0x41, 0x57, 0x48, 0x81, 0xEC,
    0x88, 0x01, 0x00, 0x00, 0x48, 0xC7, 0x44, 0x24, 0x28, 0xFE, 0xFF, 0xFF,
    0xFF,
    // 0x48, 0x8B, 0x05,
    // 0x6C, 0x8C, 0x44, 0x00, 0x48, 0x33, 0xC4, 0x48, 0x89, 0x84, 0x24, 0x70, 0x01, 0x00, 0x00, 0x41,
    // 0x8B, 0xF1, 0x49, 0x8B, 0xF8, 0x4C, 0x8B, 0xF2, 0x4C, 0x8B, 0xF9, 0x48, 0x89, 0x54, 0x24, 0x20,
    // 0x41, 0x83, 0xF9, 0x05,
];

static AOB_GET_LOOSE_FILE_PATH_AFTER_ROOT_DIRECTORIES: &'static [u8] = &[
    0xBA, 0x00, 0x03, 0x00, 0x00, 0x4C, 0x8B, 0x0C, 0xF9, 0x48, 0x8D, 0x44, 0x24, 0x30, 0x48, 0x8B,
    0xCB, 0x48, 0x89, 0x44, 0x24, 0x20,
];

// "load file" is actually an async load
// static mut LOAD_FILE_FROM_ARCHIVE_TRAMPOLINE: *const () = std::ptr::null();
// static mut GET_FILE_FROM_ARCHIVE_TRAMPOLINE: *const () = std::ptr::null();
static mut GET_FILE_FROM_ARCHIVE_2_TRAMPOLINE: *const () = std::ptr::null();
// type PfnLoadFromArchive =
//     extern "fastcall" fn(a1: LPVOID, a2: LPVOID, a3: LPVOID, a4: i8) -> LPVOID;
// type PfnGetFileFromArchive = extern "fastcall" fn(
//     this: LPVOID,
//     unk: LPVOID,
//     path: LPCSTR,
//     unk2: LPVOID,
//     unk3: DWORD,
//     unk4: LPVOID,
//     unk5: DWORD,
// ) -> LPCSTR;
type PfnGetFileFromArchive2 = extern "fastcall" fn(
    a1: LPVOID,
    a2: LPVOID,
    a3: LPCSTR,
    a4: u64,
    load_from_archive: DWORD,
    a6: LPVOID,
    a7: DWORD,
) -> LPVOID;

// extern "fastcall" fn do_thing(a1: LPVOID, a2: LPVOID, a3: LPVOID, a4: i8) -> LPVOID {
//     loop {
//         if a2.is_null() {
//             log::debug!("reading null file");
//             break;
//         }
//         unsafe {
//             let path_str = CStr::from_ptr(a2 as *const i8);
//             let safe_path_str = path_str.to_string_lossy();
//             log::info!("loading: {}", safe_path_str);
//         }
//         break;
//     }
//     unsafe {
//         (std::mem::transmute::<_, PfnLoadFromArchive>(LOAD_FILE_FROM_ARCHIVE_TRAMPOLINE))(
//             a1, a2, a3, a4,
//         )
//     }
// }

// extern "fastcall" fn hook_get_file_from_archive(
//     this: LPVOID,
//     unk: LPVOID,
//     path: LPCSTR,
//     unk2: LPVOID,
//     unk3: DWORD,
//     unk4: LPVOID,
//     unk5: DWORD,
// ) -> LPCSTR {
//     loop {
//         if path.is_null() {
//             log::debug!("reading null file");
//             break;
//         }
//         unsafe {
//             let path_str = CStr::from_ptr(path as *const i8);
//             let safe_path_str = path_str.to_string_lossy();
//             log::info!("getting: {}", safe_path_str);
//         }
//         break;
//     }
//     unsafe {
//         let ret = (std::mem::transmute::<_, PfnGetFileFromArchive>(
//             GET_FILE_FROM_ARCHIVE_TRAMPOLINE,
//         ))(this, unk, path, unk2, unk3, unk4, unk5);

//         ret
//     }
// }

static mut ROOT_DIR_PATHS: *mut *mut i8 = std::ptr::null_mut();

extern "fastcall" fn hook_get_file_from_archive_2(
    a1: LPVOID,
    a2: LPVOID,
    a3: LPCSTR,
    a4: u64,
    mut load_from_archive: DWORD,
    a6: LPVOID,
    a7: DWORD,
) -> LPVOID {
    loop {
        if a3.is_null() {
            log::debug!("reading null file (func 2)");
            break;
        }
        unsafe {
            let path_str = CStr::from_ptr(a3 as *const i8);
            let safe_path_str = path_str.to_string_lossy();

            // if !ROOT_DIR_PATHS.is_null() && !ROOT_DIR_PATHS.offset(a4 as isize).is_null() {
            //     let root_path_str = CStr::from_ptr(*ROOT_DIR_PATHS.offset(a4 as isize));
            //     let safe_root_path_str = root_path_str.to_string_lossy();
            //     log::debug!(
            //         "getting (root): {} {}{}",
            //         a4,
            //         safe_root_path_str,
            //         safe_path_str
            //     );
            // } else {
            log::debug!("getting: {} {}", a4, safe_path_str);
            // }

            // check if the file exists
            // if it does, override the load mode to use loose files
            // (interesting that this codepath still exists in the release build!)
            // Only path mount 5 should be overridden
            // 2 is used for user data (.systemsave, save.lst, etc)
            if a4 == 5 {
                match std::fs::metadata(PathBuf::from("mods").join(safe_path_str.as_ref())) {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            load_from_archive = 0;
                        }
                    }
                    _ => {}
                }
            }
        }
        break;
    }
    unsafe {
        let path_str = CStr::from_ptr(a3 as *const i8);
        let safe_path_str = path_str.to_string_lossy();
        let mod_load_order = MOD_LOAD_ORDER.read().unwrap();
        for mod_path in mod_load_order.iter() {
            let dest_path = mod_path.join("files").join(safe_path_str.as_ref());
            match std::fs::metadata(&dest_path) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        let new_path = CString::new(dest_path.to_string_lossy().as_ref()).unwrap();
                        return (std::mem::transmute::<_, PfnGetFileFromArchive2>(
                            GET_FILE_FROM_ARCHIVE_2_TRAMPOLINE,
                        ))(a1, a2, new_path.as_ptr(), a4, 0, a6, a7);
                    }
                }
                _ => {}
            }
        }
        return (std::mem::transmute::<_, PfnGetFileFromArchive2>(
            GET_FILE_FROM_ARCHIVE_2_TRAMPOLINE,
        ))(a1, a2, a3, a4, load_from_archive, a6, a7);
    }
}

fn log_init() {
    let file = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("modloader-d6.log")
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
                if entry.metadata().unwrap().is_dir() {
                    mod_load_order.push(entry.path());
                }
            }
            Err(_) => continue,
        }
    }
    log::info!("Mod load order: {:?}", mod_load_order.borrow());
}

fn init() {
    log_init();

    init_mod_load_order();
    log::info!("Mod load order initialized");

    unsafe {
        let base_handle = GetModuleHandleA(std::ptr::null());
        let base_address = base_handle as LPVOID;
        // match aob_search(AOB_LOAD_FROM_ARCHIVE_FUNC, base_address, 1024 * 1024 * 12) {
        //     Some(addr) => {
        //         log::debug!("Address of load file func: {:p}", addr);

        //         let detour = RawDetour::new(addr as *const (), do_thing as *const ()).unwrap();
        //         LOAD_FILE_FROM_ARCHIVE_TRAMPOLINE = detour.trampoline();
        //         match detour.enable() {
        //             Err(e) => {
        //                 log::error!("unable to install hook: {}", e);
        //             }
        //             _ => {}
        //         }
        //     }
        //     None => {
        //         log::error!("Address of load file func not found!");
        //     }
        // }
        // match aob_search(
        //     AOB_GET_FILE_FROM_ARCHIVE_FUNC,
        //     base_address,
        //     1024 * 1024 * 12,
        // ) {
        //     Some(addr) => {
        //         log::debug!("Address of get file func: {:p}", addr);

        //         let detour =
        //             RawDetour::new(addr as *const (), hook_get_file_from_archive as *const ())
        //                 .unwrap();
        //         GET_FILE_FROM_ARCHIVE_TRAMPOLINE = detour.trampoline();
        //         match detour.enable() {
        //             Err(e) => {
        //                 log::error!("unable to install get file hook: {}", e);
        //             }
        //             _ => {}
        //         }
        //     }
        //     None => {
        //         log::error!("Address of get file func not found!");
        //     }
        // }
        match aob_search(
            AOB_GET_FILE_FROM_ARCHIVE_2_FUNC,
            base_address,
            1024 * 1024 * 12,
        ) {
            Some(addr) => {
                log::debug!("Address of get file func 2: {:p}", addr);

                let detour =
                    RawDetour::new(addr as *const (), hook_get_file_from_archive_2 as *const ())
                        .unwrap();
                GET_FILE_FROM_ARCHIVE_2_TRAMPOLINE = detour.trampoline();
                match detour.enable() {
                    Err(e) => {
                        log::error!("unable to install get file 2 hook: {}", e);
                    }
                    _ => {}
                }
            }
            None => {
                log::error!("Address of get file func 2 not found!");
            }
        }

        match aob_search(
            AOB_GET_LOOSE_FILE_PATH_AFTER_ROOT_DIRECTORIES,
            base_address,
            1024 * 1024 * 12,
        ) {
            Some(addr) => {
                // static MODS_PATH: &'static [u8; 6] = b"mods/\0";
                // We need to interpret this instruction:
                // lea    rcx,[rip+0x44b789]
                let instr_offset = (addr as *mut i8).offset(-7);
                let imm_offset = *((addr as *mut i8).offset(-4) as *mut u32) as isize;
                ROOT_DIR_PATHS = instr_offset.offset(imm_offset) as *mut *mut i8;
                log::debug!("ROOT_DIR_PATHS: {:p}", ROOT_DIR_PATHS);

                // let data_root = ROOT_DIR_PATHS.offset(5);
                // *data_root = MODS_PATH.as_ptr() as *mut i8;
            }
            None => {
                log::error!("AOB for loose file root directories not found");
            }
        }

        log::info!("Hooks installed");
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
