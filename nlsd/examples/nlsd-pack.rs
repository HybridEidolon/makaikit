use std::{
    fs::File,
    io::Read,
    io::Write,
    io::{Seek, SeekFrom},
    path::PathBuf,
};

use byteorder::{WriteBytesExt, LE};
use clap::Parser;

#[derive(Parser)]
struct Args {
    out_file: PathBuf,
    in_file_0: PathBuf,
    in_file_1: Option<PathBuf>,
    in_file_2: Option<PathBuf>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let mut out = File::create(&args.out_file)?;

    let ext = args
        .in_file_0
        .extension()
        .ok_or_else(|| anyhow::anyhow!("No input file extension"))?
        .to_str()
        .unwrap();
    let format = match ext {
        "wav" => 5,
        "ogg" => 7,
        _ => anyhow::bail!("unsupported file extension {}", ext),
    };
    let stereo = if format == 7 { 1 } else { 0 };

    out.write_u32::<LE>(format as u32)?;

    let mut f0 = File::open(&args.in_file_0)?;
    let f0s = f0.seek(SeekFrom::End(0))?;
    f0.seek(SeekFrom::Start(0))?;

    if let Some(ref fp1) = args.in_file_2.as_ref() {
        let mut f1 = File::open(fp1)?;
        let f1s = f1.seek(SeekFrom::End(0))?;
        f1.seek(SeekFrom::Start(0))?;
        if let Some(ref fp2) = args.in_file_1.as_ref() {
            let mut f2 = File::open(fp2)?;
            let f2s = f2.seek(SeekFrom::End(0))?;
            f2.seek(SeekFrom::Start(0))?;

            out.write_u32::<LE>(f2s as u32 + f1s as u32 + f0s as u32)?;
            out.write_u16::<LE>(44100)?;
            out.write_u8(stereo)?;
            out.write_u8(0)?;
            out.write_u32::<LE>(0)?;
            out.write_u32::<LE>(f0s as u32)?;
            out.write_u32::<LE>(f1s as u32)?;
            std::io::copy(&mut f0, &mut out)?;
            std::io::copy(&mut f1, &mut out)?;
            std::io::copy(&mut f2, &mut out)?;
        } else {
            out.write_u32::<LE>(f1s as u32 + f0s as u32)?;
            out.write_u16::<LE>(44100)?;
            out.write_u8(stereo)?;
            out.write_u8(0)?;
            out.write_u32::<LE>(0)?;
            out.write_u32::<LE>(0 as u32)?;
            out.write_u32::<LE>(f0s as u32)?;
            std::io::copy(&mut f0, &mut out)?;
            std::io::copy(&mut f1, &mut out)?;
        }
    } else {
        out.write_u32::<LE>(f0s as u32)?;
        out.write_u16::<LE>(44100)?;
        out.write_u8(stereo)?;
        out.write_u8(0)?;
        out.write_u32::<LE>(0)?;
        out.write_u32::<LE>(0 as u32)?;
        out.write_u32::<LE>(f0s as u32)?;
        std::io::copy(&mut f0, &mut out)?;
    }
    Ok(())
}
