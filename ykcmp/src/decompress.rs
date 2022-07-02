use std::io::{self, Read};

use self::nislz77::NisLz77Reader;

mod nislz77;

pub struct Decoder<R: Read> {
    inner: DecoderInner<R>,
    _decomp_len: u64,
}

enum DecoderInner<R: Read> {
    Lz77(NisLz77Reader<R>),
}

impl<R> Decoder<R>
where
    R: Read,
{
    pub fn new(mut inner: R) -> io::Result<Self> {
        let mut r = [0u8; 12];

        inner.read_exact(&mut r[..8])?;
        if &r[..8] != b"YKCMP_V1" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "incorrect YKCMP_V1 magic header",
            ));
        }

        inner.read_exact(&mut r[..12])?;
        let encoding = u32::from_le_bytes(unsafe { *(&r[0..4] as *const [u8] as *const [u8; 4]) });
        if encoding != 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid encoding (only encoding 4 currently supported)",
            ));
        }

        let decomp_len =
            u32::from_le_bytes(unsafe { *(&r[8..12] as *const [u8] as *const [u8; 4]) }) as u64;

        Ok(Decoder {
            inner: DecoderInner::Lz77(NisLz77Reader::new(inner)),
            _decomp_len: decomp_len,
        })
    }
}

impl<R> Read for Decoder<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.inner {
            DecoderInner::Lz77(ref mut lz77) => lz77.read(buf),
        }
    }
}
