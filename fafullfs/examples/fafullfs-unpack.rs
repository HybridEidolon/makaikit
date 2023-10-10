use std::{ffi::OsString, fs::File, io::Write, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use makaikit_fafullfs::Archive;

#[derive(Parser, Debug)]
struct Args {
    path: PathBuf,
    out_dir: Option<PathBuf>,
    #[arg(short = 'V', long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let path_meta = args
        .path
        .metadata()
        .with_context(|| format!("Could not find input archive file {}", args.path.display()))?;
    anyhow::ensure!(path_meta.is_file(), "path must be a file");

    let out_dir_path = args.out_dir.unwrap_or(PathBuf::from("."));
    let out_dir_meta = out_dir_path
        .metadata()
        .with_context(|| format!("Could not find output dir {}", out_dir_path.display()))?;
    anyhow::ensure!(out_dir_meta.is_dir(), "out path must be a directory");

    let file = File::open(&args.path).context("unable to read input path")?;

    let mut archive = Archive::open(file).context("unable to open archive at input path")?;
    let archive_file_count = archive.len();
    for i in 0..archive_file_count {
        let mut accessor = archive
            .get_file(i)
            .unwrap()
            .with_context(|| format!("Unable to read file entry {}", i))?;
        let entry_path_osstring = accessor
            .path()
            .to_str()
            .context("Unable to parse file path")?;
        let entry_path = PathBuf::from(entry_path_osstring);
        let entry_out_path = out_dir_path.join(entry_path);
        let mut entry_out_parent_dir = entry_out_path.clone();
        entry_out_parent_dir.pop();
        std::fs::create_dir_all(&entry_out_parent_dir).with_context(|| {
            format!(
                "Unable to create directories through path {}",
                entry_out_parent_dir.display()
            )
        })?;
        let mut out_file = File::create(&entry_out_path).with_context(|| {
            format!(
                "Unable to open file {} for write to write {}",
                entry_out_path.display(),
                entry_path_osstring
            )
        })?;
        eprintln!("writing {}", entry_out_path.display());
        std::io::copy(&mut accessor, &mut out_file)
            .context("Unable to write archive file bytes to destination")?;
    }

    Ok(())
}
