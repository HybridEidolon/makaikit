use std::{
    borrow::Borrow,
    collections::HashMap,
    ffi::{CStr, CString},
    fs::File,
    io::{BufWriter, Read, Seek},
    path::{Path, PathBuf},
    str::FromStr,
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
use makaikit_databases_d7::{
    BattleFlagData, BgmData, CharaClassData, CharaData, CheatSettingData, JobData, StringData,
};
use makaikit_databases_serde::DatabaseRecord;
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
    e_access_type: u32,
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
        }
        break;
    }
    unsafe {
        let path_str = CStr::from_ptr(path as *const i8);
        let safe_path_str = path_str.to_string_lossy();

        // _generated replacements loading
        {
            let dest_path = PathBuf::from("mods/_generated").join(safe_path_str.as_ref());
            let mut dest_path_str = dest_path.to_string_lossy().into_owned();
            dest_path_str = dest_path_str.replace("\\", "/");
            match std::fs::metadata(&dest_path) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        let new_path = CString::new(dest_path_str).unwrap();
                        log::info!("Using generated {:?}", new_path);
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

#[derive(Debug)]
enum RecordIdentifier {
    Id(i32),
    EnumName(String),
}

fn parse_file_stem(base: &str) -> Option<RecordIdentifier> {
    if base.is_empty() {
        return None;
    }

    let split = match base.split_once("_") {
        Some(v) => v,
        None => return Some(RecordIdentifier::EnumName(base.to_owned())),
    };
    match <i32 as FromStr>::from_str(split.0) {
        Ok(v) => return Some(RecordIdentifier::Id(v)),
        Err(_) => {}
    }
    Some(RecordIdentifier::EnumName(base.to_owned()))
}

fn repack_database<R: Read + Seek, T: DatabaseRecord>(
    archive: &mut makaikit_fafullfs::Archive<R>,
    name: &str,
) where
    T: serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
    let mod_load_order = MOD_LOAD_ORDER.read().unwrap();
    let mut entry = None;
    let archive_len = archive.len();

    log::info!("Archive len {archive_len}");

    for index in 0..archive_len {
        let this_entry = archive.get_file(index).unwrap().unwrap();
        let entry_name = this_entry.path().to_str().unwrap();
        if entry_name == format!("data/database/{}.dat", name) {
            entry = Some(index);
            log::info!("Found {entry_name}");
            break;
        }
    }
    let real_entry = archive.get_file(entry.unwrap()).unwrap().unwrap();
    log::info!("Opened entry");

    let db_records = match makaikit_databases_serde::decode_database::<_, T>(real_entry) {
        Err(e) => {
            log::error!("Unable to read database: {}", e);
            return;
        }
        Ok(o) => o,
    };

    for record in db_records.iter() {
        std::fs::create_dir_all(format!("mods_export/databases/{}", name)).unwrap();
        serde_json::to_writer_pretty(
            File::create(format!(
                "mods_export/databases/{}/{}_{}.json",
                name,
                record.database_id(),
                record.database_enum_name()
            ))
            .unwrap(),
            record,
        )
        .unwrap();
    }

    let mut db_map = HashMap::<i32, T>::new();
    for record in db_records {
        db_map.insert(record.database_id(), record);
    }

    for entry in mod_load_order.iter() {
        let database_root_path = entry.join("databases").join(name);
        let db_read_dir = match database_root_path.read_dir() {
            Err(e) => {
                log::info!(
                    "Unable to open {}, moving on: {}",
                    database_root_path.display(),
                    e
                );
                continue;
            }
            Ok(o) => o,
        };

        for read_dir_entry_result in db_read_dir {
            let read_dir_entry = match read_dir_entry_result {
                Err(_) => continue,
                Ok(o) => o,
            };
            let mut read_dir_file = match File::open(read_dir_entry.path()) {
                Err(_) => continue,
                Ok(o) => o,
            };
            let dir_entry_path = read_dir_entry.path();
            let file_name = dir_entry_path.file_name().unwrap().to_string_lossy();
            let file_stem = dir_entry_path.file_stem().unwrap().to_string_lossy();

            if file_name.ends_with(".patch.json") {
                log::debug!("JSON Patch {}", dir_entry_path.display());
                let name_identifier =
                    match parse_file_stem(&file_name.strip_suffix(".patch.json").unwrap()) {
                        None => {
                            log::error!(
                                "File stem {} does not identify a database record",
                                file_stem
                            );
                            continue;
                        }
                        Some(i) => i,
                    };
                let orig_record_maybe = match name_identifier {
                    RecordIdentifier::Id(id) => db_map.get(&id),
                    RecordIdentifier::EnumName(ref name) => db_map.iter().find_map(|e| {
                        if e.1.database_enum_name() == name {
                            Some(e.1)
                        } else {
                            None
                        }
                    }),
                };
                let orig_record = match orig_record_maybe {
                    None => {
                        log::error!("Applying {} patch {} for ID {:?} failed because there is no source record", name, dir_entry_path.display(), name_identifier);
                        continue;
                    }
                    Some(r) => r,
                };
                let mut record_json = serde_json::to_value(orig_record).unwrap();
                let json_patch =
                    match serde_json::from_reader::<_, json_patch::Patch>(&mut read_dir_file) {
                        Err(e) => {
                            log::error!(
                                "File {} is not a proper json: {}",
                                dir_entry_path.display(),
                                e
                            );
                            continue;
                        }
                        Ok(o) => o,
                    };
                match json_patch::patch(&mut record_json, &json_patch.0) {
                    Err(e) => {
                        log::error!(
                            "JSON patch {} application failed: {}",
                            dir_entry_path.display(),
                            e
                        );
                        continue;
                    }
                    _ => {}
                }
                let new_record = match serde_json::from_value::<T>(record_json) {
                    Err(e) => {
                        log::error!(
                            "Parsing record after applying JSON patch {} failed: {}",
                            dir_entry_path.display(),
                            e
                        );
                        continue;
                    }
                    Ok(v) => v,
                };
                db_map.insert(new_record.database_id(), new_record);
            } else if file_name.ends_with(".merge.json") {
                log::debug!("Merge patch {}", dir_entry_path.display());
                let name_identifier =
                    match parse_file_stem(&file_name.strip_suffix(".merge.json").unwrap()) {
                        None => {
                            log::error!(
                                "File stem {} does not identify a database record",
                                file_stem
                            );
                            continue;
                        }
                        Some(i) => i,
                    };
                let orig_record_maybe = match name_identifier {
                    RecordIdentifier::Id(id) => db_map.get(&id),
                    RecordIdentifier::EnumName(ref name) => db_map.iter().find_map(|e| {
                        if e.1.database_enum_name() == name {
                            Some(e.1)
                        } else {
                            None
                        }
                    }),
                };
                let orig_record = match orig_record_maybe {
                    None => {
                        log::error!("Applying {} merge patch {} for ID {:?} failed because there is no source record", name, dir_entry_path.display(), name_identifier);
                        continue;
                    }
                    Some(r) => r,
                };
                let mut record_json = serde_json::to_value(orig_record).unwrap();
                let merge_patch =
                    match serde_json::from_reader::<_, serde_json::Value>(&mut read_dir_file) {
                        Err(e) => {
                            log::error!(
                                "File {} is not a proper json: {}",
                                dir_entry_path.display(),
                                e
                            );
                            continue;
                        }
                        Ok(o) => o,
                    };
                json_patch::merge(&mut record_json, &merge_patch);
                let new_record = match serde_json::from_value::<T>(record_json) {
                    Err(e) => {
                        log::error!(
                            "Parsing record after applying merge patch {} failed: {}",
                            dir_entry_path.display(),
                            e
                        );
                        continue;
                    }
                    Ok(v) => v,
                };
                db_map.insert(new_record.database_id(), new_record);
            } else if file_name.ends_with(".json") {
                log::debug!("Record replacement {}", dir_entry_path.display());
                let record = match serde_json::from_reader::<_, T>(&mut read_dir_file) {
                    Err(e) => {
                        log::error!(
                            "File {} is not a proper {}: {}",
                            dir_entry_path.display(),
                            name,
                            e
                        );
                        continue;
                    }
                    Ok(o) => o,
                };
                db_map.insert(record.database_id(), record);
            }
        }
    }

    let mut db_records = Vec::new();
    for (_key, value) in db_map {
        db_records.push(value);
    }

    match std::fs::create_dir_all("mods/_generated/data/database") {
        Err(e) => {
            log::error!("Unable to create path mods/_generated/data/database: {}", e);
            return;
        }
        _ => {}
    }
    let generated_path =
        PathBuf::from("mods/_generated").join(format!("data/database/{}.dat", name));
    let out_file = match File::create(&generated_path) {
        Err(e) => {
            log::error!(
                "Could not write out db file {}: {}",
                generated_path.display(),
                e
            );
            return;
        }
        Ok(f) => BufWriter::new(f),
    };
    match makaikit_databases_serde::encode_database(out_file, db_records) {
        Err(e) => {
            log::error!(
                "Could not generate db {} to file {}: {}",
                name,
                generated_path.display(),
                e
            );
        }
        _ => {}
    }
}

fn repack_databases() {
    let mut archive = makaikit_fafullfs::Archive::open(File::open("data.dat").unwrap()).unwrap();
    repack_database::<_, BattleFlagData>(&mut archive, "battleflag");
    repack_database::<_, BgmData>(&mut archive, "bgm");
    repack_database::<_, CharaClassData>(&mut archive, "characlass");
    repack_database::<_, CharaData>(&mut archive, "character");
    repack_database::<_, CheatSettingData>(&mut archive, "cheatsetting");
    repack_database::<_, JobData>(&mut archive, "job");
    repack_database::<_, StringData>(&mut archive, "string");
}

fn init() {
    log_init();

    init_mod_load_order();
    log::info!("Mod load order initialized");
    // Disgaea 7 no longer has a special script archive!
    repack_databases();

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
