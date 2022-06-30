use std::io::{self, Read, Seek};

pub struct Archive<R: Read + Seek> {
    inner: R,
    files: Vec<ArchiveFile>,
}

struct ArchiveFile {
    path: String,
    checksum: u64,
    unk: u64,
    size: u64,
    offset: u64,
    timestamp: u64,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OpenError {
    #[error("Invalid magic identifier (FAFULLFS)")]
    InvalidMagic,

    #[error("IO error")]
    Io(#[from] io::Error),
}

impl<R: Read + Seek> Archive<R> {
    pub fn open(mut inner: R) -> Result<Self, OpenError> {
        todo!()
    }
}
