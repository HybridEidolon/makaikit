use std::{fs::File, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use makaikit_nlsd::NlsdRead;

#[derive(Parser)]
struct Args {
    input: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    // let file_name = args
    //     .input
    //     .file_name()
    //     .ok_or_else(|| anyhow::anyhow!("Path {} is not valid", args.input.display()))?;
    let file_stem = args
        .input
        .file_stem()
        .ok_or_else(|| anyhow::anyhow!("Path {} has no file stem", args.input.display()))?
        .to_string_lossy();
    let parent_dir = args
        .input
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Path {} is not valid", args.input.display()))?;

    let mut nlsd = NlsdRead::open(File::open(&args.input)?)?;
    let extension = match nlsd.format() {
        5 => "wav",
        7 => "ogg",
        _ => unreachable!(),
    };
    match nlsd.section_begin() {
        Ok(Some(mut section)) => {
            std::io::copy(
                &mut section,
                &mut File::create(&format!("{}_0.{}", file_stem, extension))?,
            )?;
        }
        Err(e) => {
            return Err(e).with_context(|| format!("Unable to read beginning of NLSD"));
        }
        _ => {}
    }
    std::io::copy(
        &mut nlsd.section_middle()?,
        &mut File::create(&format!("{}_1.{}", file_stem, extension))?,
    )?;
    match nlsd.section_end() {
        Ok(Some(mut section)) => {
            std::io::copy(
                &mut section,
                &mut File::create(&format!("{}_2.{}", file_stem, extension))?,
            )?;
        }
        Err(e) => {
            return Err(e).with_context(|| format!("Unable to read beginning of NLSD"));
        }
        _ => {}
    }
    Ok(())
}
