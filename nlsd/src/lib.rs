use std::io::{Read, Seek, SeekFrom, Write};

use byteorder::{ReadBytesExt, LE};

pub struct NlsdRead<R> {
    format: u32,
    total_size: u32,
    sample_rate: u16,
    stereo: bool,
    _samples: u32,
    middle_ofs: u32,
    end_ofs: u32,
    read: R,
}

pub struct NlsdSectionRead<'a, R> {
    parent: &'a mut NlsdRead<R>,
    _start: u64,
    len: u64,
    pos: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("Unrecognized value for {0}: {1}")]
    UnrecognizedValue(&'static str, String),

    #[error("IO error")]
    Io(#[from] std::io::Error),
}

impl<R> NlsdRead<R>
where
    R: Read,
{
    pub fn open(mut data: R) -> Result<Self, ReadError> {
        let format = data.read_u32::<LE>()?;
        if format != 5 && format != 7 {
            return Err(ReadError::UnrecognizedValue(
                "format",
                format!("{}", format),
            ));
        }
        let total_size = data.read_u32::<LE>()?;
        let sample_rate = data.read_u16::<LE>()?;
        let stereo_byte = data.read_u8()?;
        if stereo_byte > 1 {
            return Err(ReadError::UnrecognizedValue(
                "stereo",
                format!("{}", stereo_byte),
            ));
        }
        let unused_byte = data.read_u8()?;
        if unused_byte != 0 {
            return Err(ReadError::UnrecognizedValue(
                "unused_byte",
                format!("{}", unused_byte),
            ));
        }
        let stereo = stereo_byte > 0;
        let samples = data.read_u32::<LE>()?;
        let middle_ofs = data.read_u32::<LE>()?;
        let end_ofs = data.read_u32::<LE>()?;

        Ok(NlsdRead {
            format,
            total_size,
            sample_rate,
            stereo,
            _samples: samples,
            middle_ofs,
            end_ofs,
            read: data,
        })
    }
}

impl<R> NlsdRead<R> {
    pub fn has_start(&self) -> bool {
        self.middle_ofs != 0
    }

    pub fn has_end(&self) -> bool {
        self.end_ofs + self.middle_ofs != self.total_size
    }

    pub fn format(&self) -> u32 {
        self.format
    }

    pub fn sample_rate(&self) -> u16 {
        self.sample_rate
    }

    pub fn stereo(&self) -> bool {
        self.stereo
    }
}

impl<R> NlsdRead<R>
where
    R: Read + Seek,
{
    pub fn section_begin<'a>(&'a mut self) -> Result<Option<NlsdSectionRead<'a, R>>, ReadError> {
        if !self.has_start() {
            return Ok(None);
        }
        self.read.seek(SeekFrom::Start(0x18))?;
        Ok(Some(NlsdSectionRead {
            len: self.middle_ofs as u64,
            parent: self,
            _start: 0x18,
            pos: 0,
        }))
    }

    pub fn section_middle<'a>(&'a mut self) -> Result<NlsdSectionRead<'a, R>, ReadError> {
        let start_ofs = 0x18 + self.middle_ofs as u64;
        self.read.seek(SeekFrom::Start(start_ofs))?;
        Ok(NlsdSectionRead {
            len: self.end_ofs as u64 - self.middle_ofs as u64,
            parent: self,
            _start: start_ofs,
            pos: 0,
        })
    }

    pub fn section_end<'a>(&'a mut self) -> Result<Option<NlsdSectionRead<'a, R>>, ReadError> {
        if !self.has_end() {
            return Ok(None);
        }
        let start_ofs = 0x18 + self.end_ofs as u64;
        self.read.seek(SeekFrom::Start(start_ofs))?;
        Ok(Some(NlsdSectionRead {
            len: self.total_size as u64 - self.end_ofs as u64,
            parent: self,
            _start: start_ofs,
            pos: 0,
        }))
    }
}

impl<'a, R> Read for NlsdSectionRead<'a, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let read_len = buf.len().min(self.len as usize - self.pos as usize);
        let actual_read = self.parent.read.read(&mut buf[..read_len])?;
        self.pos += actual_read as u64;

        Ok(actual_read)
    }
}
