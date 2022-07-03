use makaikit_dsarcfl::{Archive, ArchiveWriter};
use std::{
    collections::HashSet,
    ffi::CString,
    fs::File,
    io::{self, Read, Seek},
    path::Path,
};
use walkdir::WalkDir;

#[derive(Debug, thiserror::Error)]
pub enum ScriptRepackError {
    #[error("A file in the source archive has an invalid name")]
    InvalidSourceFileName,

    #[error("IO error")]
    Io(#[from] io::Error),

    #[error("DSARC FL read error")]
    DsArcFlRead(#[from] makaikit_dsarcfl::ReadError),

    #[error("DSARC FL write error")]
    DsArcFlWrite(#[from] makaikit_dsarcfl::WriteError),
}

/// Creates a new DSARC FL buffer from the source Archive
pub fn repack_scripts<R>(
    source: &mut Archive<R>,
    replacement_paths: &[&Path],
) -> Result<Vec<u8>, ScriptRepackError>
where
    R: Read + Seek,
{
    let mut new_archive = ArchiveWriter::new();

    let mut used_names = HashSet::new();

    // First, insert replacements into the archive
    for repl_path in replacement_paths {
        log::debug!("Walking {repl_path:?} for lua scripts");
        for entry in WalkDir::new(repl_path) {
            match entry {
                Err(e) => {
                    log::warn!("Unable to access script file in mod dir: {}", e);
                }
                Ok(entry) => {
                    let ft = entry.file_type();
                    if !ft.is_file() {
                        continue;
                    }
                    if !entry
                        .path()
                        .extension()
                        .and_then(|ext| if ext == "lua" { Some(()) } else { None })
                        .is_some()
                    {
                        continue;
                    }

                    let mut name_string = entry
                        .path()
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned();
                    name_string.replace_range(name_string.rfind(".lua").unwrap().., ".lub");
                    if used_names.contains(&name_string) {
                        log::warn!("Skipping {:?} for script.dat replacement because the name {} was already inserted", entry.path(), name_string);
                        continue;
                    }

                    log::debug!("Adding script {:?}", entry.path());

                    let name_cstring = CString::new(name_string.as_bytes()).unwrap();
                    let mut file_writer = new_archive.file(&name_cstring).unwrap();
                    let mut file_reader = match File::open(entry.path()) {
                        Err(e) => {
                            log::warn!("Unable to access script file in mod dir: {}", e);
                            continue;
                        }
                        Ok(f) => f,
                    };
                    std::io::copy(&mut file_reader, &mut file_writer)?;
                    drop(file_writer);

                    used_names.insert(name_string);
                }
            }
        }
        log::debug!("Walked {repl_path:?} for lua scripts");
    }
    log::debug!("Walked mod directories");

    // Second, look for files to reinsert from the original archive that aren't already replaced
    for i in 0..source.len() {
        let mut orig_file = source.get_file(i).unwrap()?;
        let orig_name = match orig_file.name().to_str() {
            Err(_) => {
                return Err(ScriptRepackError::InvalidSourceFileName);
            }
            Ok(name) => name,
        }
        .to_owned();
        if !used_names.contains(&orig_name) {
            let name_cstring = match CString::new(orig_name.as_bytes()) {
                Err(_) => {
                    return Err(ScriptRepackError::InvalidSourceFileName);
                }
                Ok(name) => name,
            };
            let mut file_writer = new_archive.file(&name_cstring)?;
            match std::io::copy(&mut orig_file, &mut file_writer) {
                Err(e) => {
                    log::error!("Fatal: unable to copy original file {name_cstring:?}");
                    return Err(ScriptRepackError::Io(e));
                }
                _ => {}
            }
            drop(file_writer);
            log::debug!("Inserted original script {orig_name}");
        }
    }
    log::debug!("Walked source archive for unreplaced scripts");

    Ok(new_archive.finish()?)
}
