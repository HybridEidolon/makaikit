use std::{
    ffi::CStr,
    io::{self, Read, Seek},
};

#[derive(Debug)]
pub struct Archive<R: Read + Seek> {
    inner: R,
    files: Vec<ArchiveFile>,
}

#[derive(Debug)]
struct ArchiveFile {
    name_buf: [u8; 0x74],
    size: u32,
    offset: u32,
}

pub struct ArchiveFileAccess<'a, R: Read + Seek> {
    name: &'a CStr,
    size: u32,
    offset: u32,

    inner: &'a mut R,
    reader_offset: u64,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ReadError {
    #[error("Invalid magic identifier (DSARC FL)")]
    InvalidMagic,

    #[error("Something is too large to fit in memory")]
    TooLarge,

    #[error("The name of a file in this archive is invalid")]
    InvalidName,

    #[error("IO error")]
    Io(#[from] io::Error),
}

impl<R: Read + Seek> Archive<R> {
    pub fn open(mut inner: R) -> Result<Self, ReadError> {
        let mut header_buf = [0u8; 16];
        inner.read_exact(&mut header_buf[..])?;

        if &header_buf[0..8] != b"DSARC FL" {
            return Err(ReadError::InvalidMagic);
        }

        let files_count =
            u32::from_le_bytes(unsafe { *(&header_buf[8..12] as *const [u8] as *const [u8; 4]) });

        if files_count > usize::MAX as u32 {
            return Err(ReadError::TooLarge);
        }

        let mut files = Vec::with_capacity(files_count as usize);

        for _ in 0..files_count {
            let mut file_hdr = [0u8; 0x80];
            inner.read_exact(&mut file_hdr[..0x80])?;
            let mut name_buf = [0u8; 0x74];
            (&mut name_buf[..]).copy_from_slice(&file_hdr[..0x74]);
            let size = u32::from_le_bytes(unsafe {
                *(&file_hdr[0x74..0x78] as *const [u8] as *const [u8; 4])
            });
            let offset = u32::from_le_bytes(unsafe {
                *(&file_hdr[0x78..0x7A] as *const [u8] as *const [u8; 4])
            });

            files.push(ArchiveFile {
                name_buf,
                size,
                offset,
            });
        }

        Ok(Archive { inner, files })
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn get_file<'a>(
        &'a mut self,
        index: usize,
    ) -> Option<Result<ArchiveFileAccess<'a, R>, ReadError>> {
        self.files.get(index).map(|file| {
            let first_null = match file.name_buf.iter().position(|&x| x == 0) {
                Some(i) => i,
                None => return Err(ReadError::InvalidName),
            };
            if first_null == 0 {
                return Err(ReadError::InvalidName);
            }
            let name_slice = &file.name_buf[..first_null + 1];
            let name_cstr = match CStr::from_bytes_with_nul(name_slice) {
                Ok(cstr) => cstr,
                Err(_) => return Err(ReadError::InvalidName),
            };
            self.inner.seek(io::SeekFrom::Start(file.offset as u64))?;

            Ok(ArchiveFileAccess {
                name: name_cstr,
                size: file.size,
                offset: file.offset,
                inner: &mut self.inner,
                reader_offset: 0,
            })
        })
    }
}

impl<'a, R: Read + Seek> ArchiveFileAccess<'a, R> {
    pub fn name(&self) -> &CStr {
        self.name
    }

    pub fn size(&self) -> u64 {
        self.size as u64
    }
}

impl<'a, R: Read + Seek> Read for ArchiveFileAccess<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let buf_len = buf.len() as u64;
        if buf_len == 0 {
            return Ok(0);
        }
        let read_dest = self.offset as u64 + self.reader_offset + buf_len;
        let max_read_dest = self.offset as u64 + self.size as u64;
        let slice_end = max_read_dest.min(read_dest) - self.offset as u64 - self.reader_offset;
        if slice_end == 0 {
            return Ok(0);
        }
        let bytes_read = self.inner.read(&mut buf[..slice_end as usize])?;
        self.reader_offset += bytes_read as u64;

        Ok(bytes_read)
    }
}
