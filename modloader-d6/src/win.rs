use std::ffi::CStr;

use detour::RawDetour;
use log::LevelFilter;
use log4rs::{append::file::FileAppender, encode::pattern::PatternEncoder, Config, config::{Appender, Root}};
use winapi::{
    shared::{
        minwindef::{DWORD, HMODULE, LPVOID},
        ntdef::BOOLEAN,
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

static AOB_LOAD_FROM_ARCHIVE_FUNC: &'static [u8] = &[
    0x40, 0x55, 0x56, 0x57, 0x41, 0x56, 0x41, 0x57, 0x48, 0x81,
    0xEC, 0xD0, 0x00, 0x00, 0x00, 0x48, 0xC7, 0x44, 0x24, 0x20,
    0xFE, 0xFF, 0xFF, 0xFF, 0x48, 0x89, 0x9C, 0x24, 0x18, 0x01,
    0x00, 0x00, 0x48, 0x8B, 0x05, 0xE9, 0xA9, 0xB3, 0x00, 0x48,
    0x33, 0xC4, 0x48, 0x89, 0x84, 0x24, 0xC0, 0x00, 0x00, 0x00,
    0x45, 0x0F, 0xB6, 0xF1, 0x49, 0x8B, 0xE8, 0x48, 0x8B, 0xF2,
    0x48, 0x8B, 0xD9, 0x48, 0x89, 0x4C, 0x24, 0x28, 0x4C, 0x89,
    0x44, 0x24, 0x30
];

static mut TRAMPOLINE: *const () = std::ptr::null();
type PfnLoadFromArchive = extern "fastcall" fn(a1: LPVOID, a2: LPVOID, a3: LPVOID, a4: i8) -> LPVOID;

extern "fastcall" fn do_thing(a1: LPVOID, a2: LPVOID, a3: LPVOID, a4: i8) -> LPVOID {
    loop {
        if a2.is_null() {
            log::debug!("reading null file");
            break;
        }
        unsafe {
            let path_str = CStr::from_ptr(a2 as *const i8);
            let safe_path_str = path_str.to_string_lossy();
            log::info!("loading: {}", safe_path_str);
        }
        break;
    }
    unsafe { (std::mem::transmute::<_, PfnLoadFromArchive>(TRAMPOLINE))(a1, a2, a3, a4) }
}

fn log_init() {
    let file = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("modloader-d6.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(Root::builder().appender("file").build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn init() {
    log_init();

    unsafe {
        let base_handle = GetModuleHandleA(std::ptr::null());
        let base_address = base_handle as LPVOID;
        match aob_search(AOB_LOAD_FROM_ARCHIVE_FUNC, base_address, 1024 * 1024 * 12) {
            Some(addr) => {
                log::debug!("Address of load file func: {:p}", addr);

                let detour = RawDetour::new(addr as *const (), do_thing as *const ()).unwrap();
                TRAMPOLINE = detour.trampoline();
                match detour.enable() {
                    Err(e) => {
                        log::error!("unable to install hook: {}", e);
                    }
                    _ => {}
                }
            }
            None => {
                log::debug!("Address of load file func not found!");
            }
        }
    }
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
