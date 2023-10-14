use std::{
    fs::File,
    io::{BufReader, Read, Seek},
    path::{Path, PathBuf},
};

use anyhow::Context;
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
        let this_entry = archive
            .get_file(index)
            .unwrap()
            .with_context(|| format!("Unable to read archive {name} file index index {index}"))?;
        let entry_name = this_entry
            .path()
            .to_str()
            .with_context(|| format!("Database {name} file entry {index} had non-string name"))?;
        if entry_name == format!("data/database/{}.dat", name) {
            entry = Some(index);
            break;
        }
    }
    let real_entry = archive
        .get_file(entry.ok_or_else(|| anyhow::anyhow!("DB Entry {} not found", name))?)
        .unwrap()?;

    let db_records = makaikit_databases_serde::decode_database::<_, T>(real_entry)
        .with_context(|| format!("Unable to decode data/database/{name}.dat"))?;

    for record in db_records.iter() {
        let out_dir = path.join(name);
        std::fs::create_dir_all(&out_dir)
            .with_context(|| format!("Unable to create directories up to {}", out_dir.display()))?;
        let out_file_path = out_dir.join(format!(
            "{}_{}.json",
            record.database_id(),
            record.database_enum_name()
        ));
        serde_json::to_writer_pretty(File::create(&out_file_path)?, record).with_context(|| {
            format!(
                "Unable to serialize and write DB {} record {} ({})",
                name,
                record.database_id(),
                record.database_enum_name()
            )
        })?;
    }

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let mut archive = Archive::open(BufReader::new(File::open(&args.path)?))?;

    let dest = args.out_dir.unwrap_or(PathBuf::from(""));

    unpack_db::<_, ActData>(&mut archive, "act", &dest)?;
    unpack_db::<_, ActEffectData>(&mut archive, "acteffect", &dest)?;
    unpack_db::<_, ActFeatureData>(&mut archive, "actfeature", &dest)?;
    unpack_db::<_, ActLearnData>(&mut archive, "actlearn", &dest)?;
    unpack_db::<_, ActMapData>(&mut archive, "actmap", &dest)?;
    unpack_db::<_, AiData>(&mut archive, "ai", &dest)?;
    unpack_db::<_, AiPartsData>(&mut archive, "aiparts", &dest)?;
    unpack_db::<_, AnimeData>(&mut archive, "anime", &dest)?;
    unpack_db::<_, AnimeBankData>(&mut archive, "animebank", &dest)?;
    unpack_db::<_, ArchiveData>(&mut archive, "archive", &dest)?;
    unpack_db::<_, AreaData>(&mut archive, "area", &dest)?;
    unpack_db::<_, BattleFlagData>(&mut archive, "battleflag", &dest)?;
    unpack_db::<_, BgmData>(&mut archive, "bgm", &dest)?;
    unpack_db::<_, BuData>(&mut archive, "bu", &dest)?;
    unpack_db::<_, CharaClassData>(&mut archive, "characlass", &dest)?;
    unpack_db::<_, CharaData>(&mut archive, "character", &dest)?;
    unpack_db::<_, CharaFeatureData>(&mut archive, "charafeature", &dest)?;
    unpack_db::<_, CharaZukanData>(&mut archive, "charazukan", &dest)?;
    unpack_db::<_, CheatSettingData>(&mut archive, "cheatsetting", &dest)?;
    unpack_db::<_, DopingData>(&mut archive, "doping", &dest)?;
    unpack_db::<_, DrinkData>(&mut archive, "drink", &dest)?;
    unpack_db::<_, DungeonData>(&mut archive, "dungeon", &dest)?;
    unpack_db::<_, EvilityData>(&mut archive, "evility", &dest)?;
    unpack_db::<_, ItemStrengthenData>(&mut archive, "itemstrengthen", &dest)?;
    unpack_db::<_, JobData>(&mut archive, "job", &dest)?;
    unpack_db::<_, StringData>(&mut archive, "string", &dest)?;
    unpack_db::<_, WishData>(&mut archive, "wish", &dest)?;

    Ok(())
}
