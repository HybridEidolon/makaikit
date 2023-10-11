use std::{
    fs::File,
    io::{BufReader, Read, Seek},
    path::{Path, PathBuf},
};

use clap::Parser;
use makaikit_databases_d7::{charazukan::CharaZukanData, *};
use makaikit_databases_serde::DatabaseRecord;
use makaikit_fafullfs::Archive;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Parser)]
struct Args {
    path: PathBuf,
    out_dir: Option<PathBuf>,
}

fn unpack_db<R, T>(archive: &mut Archive<R>, name: &str, path: &Path) -> Result<(), anyhow::Error>
where
    R: Read + Seek,
    T: DeserializeOwned + Serialize + DatabaseRecord,
{
    let archive_len = archive.len();

    let mut entry = None;
    for index in 0..archive_len {
        let this_entry = archive.get_file(index).unwrap().unwrap();
        let entry_name = this_entry.path().to_str().unwrap();
        if entry_name == format!("data/database/{}.dat", name) {
            entry = Some(index);
            break;
        }
    }
    let real_entry = archive
        .get_file(entry.ok_or_else(|| anyhow::anyhow!("DB Entry {} not found", name))?)
        .unwrap()?;

    let db_records = makaikit_databases_serde::decode_database::<_, T>(real_entry)?;

    for record in db_records.iter() {
        std::fs::create_dir_all(path.join(name))?;
        let out_file_path = path.join(name).join(format!(
            "{}_{}.json",
            record.database_id(),
            record.database_enum_name()
        ));
        serde_json::to_writer_pretty(File::create(&out_file_path)?, record)?;
    }

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let mut archive = Archive::open(BufReader::new(File::open(&args.path)?))?;

    let dest = args.out_dir.unwrap_or(PathBuf::from(""));

    unpack_db::<_, BattleFlagData>(&mut archive, "battleflag", &dest)?;
    unpack_db::<_, BgmData>(&mut archive, "bgm", &dest)?;
    unpack_db::<_, CharaClassData>(&mut archive, "characlass", &dest)?;
    unpack_db::<_, CharaData>(&mut archive, "character", &dest)?;
    unpack_db::<_, CharaFeatureData>(&mut archive, "charafeature", &dest)?;
    unpack_db::<_, CharaZukanData>(&mut archive, "charazukan", &dest)?;
    unpack_db::<_, CheatSettingData>(&mut archive, "cheatsetting", &dest)?;
    unpack_db::<_, JobData>(&mut archive, "job", &dest)?;
    unpack_db::<_, StringData>(&mut archive, "string", &dest)?;

    Ok(())
}
