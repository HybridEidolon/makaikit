use std::{
    ffi::CStr,
    io::{self, Write},
};

pub struct ArchiveWriter {
    files: Vec<ArchiveFile>,
    buf: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum WriteError {
    #[error("File name is too long")]
    NameTooLong,

    #[error("IO error")]
    Io(#[from] io::Error),
}

pub struct ArchiveFileWriter<'a> {
    archive: &'a mut ArchiveWriter,
    name: [u8; 0x74],
    start: usize,
    len: usize,
}

pub struct ArchiveFile {
    name: [u8; 0x74],
    off: usize,
    len: usize,
}

impl ArchiveWriter {
    pub fn new() -> Self {
        ArchiveWriter {
            buf: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ArchiveWriter {
            buf: Vec::with_capacity(capacity),
            files: Vec::new(),
        }
    }

    pub fn file<'a>(&'a mut self, name: &CStr) -> Result<ArchiveFileWriter<'a>, WriteError> {
        if name.to_bytes().len() >= 0x74 {
            return Err(WriteError::NameTooLong);
        }
        let mut file_name = [0; 0x74];
        file_name.copy_from_slice(name.to_bytes());

        let start = self.buf.len();

        Ok(ArchiveFileWriter {
            archive: self,
            name: file_name,
            len: 0,
            start,
        })
    }

    pub fn finish(self) -> Result<Vec<u8>, WriteError> {
        let mut out = Vec::with_capacity(self.buf.len());
        out.write_all(b"DSARC FL")?;
        let count_bytes = (self.files.len() as u32).to_le_bytes();
        out.write_all(&count_bytes[..])?;
        out.write_all(&0u32.to_le_bytes()[..])?;
        for f in self.files.iter() {
            out.write_all(&f.name[..])?;
            let size_bytes = (f.len as u32).to_le_bytes();
            out.write_all(&size_bytes[..])?;
            let off_bytes = (f.off as u32).to_le_bytes();
            out.write_all(&off_bytes[..])?;
        }
        out.write_all(&self.buf[..])?;
        Ok(out)
    }
}

impl<'a> Write for ArchiveFileWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = self.archive.buf.write(buf)?;
        self.len += written;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        // no-op because vec's flush is also no-op
        Ok(())
    }
}

impl<'a> Drop for ArchiveFileWriter<'a> {
    fn drop(&mut self) {
        let buf_len = self.archive.buf.len();
        let pad_len = match buf_len % 16 {
            0 => buf_len,
            r => buf_len + (16 - r),
        };
        self.archive.buf.resize(pad_len, 0);
        self.archive.files.push(ArchiveFile {
            name: self.name,
            off: self.start,
            len: self.len,
        });
    }
}
