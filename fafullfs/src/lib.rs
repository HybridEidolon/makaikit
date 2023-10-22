use std::{
    ffi::CStr,
    io::{self, Read, Seek},
};

pub struct Archive<R: Read + Seek> {
    inner: R,
    paths: Vec<u8>,
    files: Vec<ArchiveFile>,
}

struct ArchiveFile {
    path_off: u64,
    checksum: u64,
    unk: u64,
    size: u64,
    offset: u64,
    timestamp: u64,
}

pub struct ArchiveFileAccess<'a, R: Read + Seek> {
    path: &'a CStr,
    checksum: u64,
    _unk: u64,
    offset: u64,
    size: u64,
    timestamp: u64,

    inner: &'a mut R,
    reader_offset: u64,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid magic identifier (FAFULLFS)")]
    InvalidMagic,

    #[error("Something is too large to fit in memory")]
    TooLarge,

    #[error("The name of a file in this archive is invalid")]
    InvalidName,

    #[error("IO error")]
    Io(#[from] io::Error),
}

impl<R: Read + Seek> Archive<R> {
    pub fn open(mut inner: R) -> Result<Self, Error> {
        let mut header_buf = [0u8; 40];
        inner.read_exact(&mut header_buf[..])?;

        if &header_buf[0..8] != b"FAFULLFS" {
            return Err(Error::InvalidMagic);
        }

        let files_count =
            u32::from_le_bytes(unsafe { *(&header_buf[8..12] as *const [u8] as *const [u8; 4]) });
        let _unk =
            u32::from_le_bytes(unsafe { *(&header_buf[12..16] as *const [u8] as *const [u8; 4]) });
        let paths_off =
            u64::from_le_bytes(unsafe { *(&header_buf[16..24] as *const [u8] as *const [u8; 8]) });
        let paths_len =
            u64::from_le_bytes(unsafe { *(&header_buf[24..32] as *const [u8] as *const [u8; 8]) });
        let info_off =
            u64::from_le_bytes(unsafe { *(&header_buf[32..40] as *const [u8] as *const [u8; 8]) });

        if paths_len > usize::MAX as u64 || files_count > usize::MAX as u32 {
            return Err(Error::TooLarge);
        }

        let mut paths_vec = Vec::with_capacity(paths_len as usize);
        paths_vec.resize(paths_len as usize, 0);
        inner.seek(io::SeekFrom::Start(paths_off))?;
        inner.read_exact(&mut paths_vec[..paths_len as usize])?;

        let mut files = Vec::with_capacity(files_count as usize);
        for i in 0..files_count {
            let mut file_header_buf = [0u8; 48];

            inner.seek(io::SeekFrom::Start(info_off + (i as u64) * 48))?;
            inner.read_exact(&mut file_header_buf[..48])?;

            let checksum = u64::from_le_bytes(unsafe {
                *(&file_header_buf[0..8] as *const [u8] as *const [u8; 8])
            });
            let path_off = u64::from_le_bytes(unsafe {
                *(&file_header_buf[8..16] as *const [u8] as *const [u8; 8])
            });
            let unk = u64::from_le_bytes(unsafe {
                *(&file_header_buf[16..24] as *const [u8] as *const [u8; 8])
            });
            let size = u64::from_le_bytes(unsafe {
                *(&file_header_buf[24..32] as *const [u8] as *const [u8; 8])
            });
            let offset = u64::from_le_bytes(unsafe {
                *(&file_header_buf[32..40] as *const [u8] as *const [u8; 8])
            });
            let timestamp = u64::from_le_bytes(unsafe {
                *(&file_header_buf[40..48] as *const [u8] as *const [u8; 8])
            });

            files.push(ArchiveFile {
                checksum,
                path_off,
                unk,
                size,
                offset,
                timestamp,
            });
        }

        Ok(Archive {
            inner,
            paths: paths_vec,
            files,
        })
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn get_file<'a>(
        &'a mut self,
        index: usize,
    ) -> Option<Result<ArchiveFileAccess<'a, R>, Error>> {
        self.files.get(index).map(|file| {
            let mut path_slice = match self.paths.get(file.path_off as usize..) {
                Some(slice) => slice,
                None => return Err(Error::InvalidName),
            };
            if path_slice.is_empty() {
                return Err(Error::InvalidName);
            }
            let first_null = match path_slice.iter().position(|&x| x == 0) {
                Some(i) => i,
                None => return Err(Error::InvalidName),
            };
            if first_null == 0 {
                return Err(Error::InvalidName);
            }
            path_slice = path_slice.split_at(first_null + 1).0;

            let path_cstr = match CStr::from_bytes_with_nul(path_slice) {
                Ok(cstr) => cstr,
                Err(_) => return Err(Error::InvalidName),
            };

            self.inner.seek(io::SeekFrom::Start(file.offset))?;

            Ok(ArchiveFileAccess {
                path: path_cstr,
                checksum: file.checksum,
                size: file.size,
                offset: file.offset,
                timestamp: file.timestamp,
                _unk: file.unk,
                inner: &mut self.inner,
                reader_offset: 0,
            })
        })
    }
}

impl<'a, R: Read + Seek> ArchiveFileAccess<'a, R> {
    pub fn path(&self) -> &CStr {
        self.path
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn checksum(&self) -> u64 {
        self.checksum
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

impl<'a, R: Read + Seek> Read for ArchiveFileAccess<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let buf_len = buf.len() as u64;
        if buf_len == 0 {
            return Ok(0);
        }
        let read_dest = self.offset + self.reader_offset + buf_len;
        let max_read_dest = self.offset + self.size;
        let slice_end = max_read_dest.min(read_dest) - self.offset - self.reader_offset;
        if slice_end == 0 {
            return Ok(0);
        }
        let bytes_read = self.inner.read(&mut buf[..slice_end as usize])?;
        self.reader_offset += bytes_read as u64;

        Ok(bytes_read)
    }
}
