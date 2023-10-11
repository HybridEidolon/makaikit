use std::io::{self, Cursor, Read};

use self::nislz77::NisLz77Reader;

mod nislz77;

pub struct Decoder<R: Read> {
    inner: DecoderInner<R>,
    _decomp_len: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("Invalid magic identifier")]
    InvalidMagic,

    #[error("Unsupported encoding variant {0}")]
    UnsupportedEncoding(u32),

    #[error("IO error")]
    Io(#[from] io::Error),

    #[error("LZ4 Decompression Error")]
    Lz4DecompressError(#[from] lz4_flex::block::DecompressError),
}

enum DecoderInner<R: Read> {
    Lz77(NisLz77Reader<R>),
    Lz4(Cursor<Vec<u8>>),
}

impl<R> Decoder<R>
where
    R: Read,
{
    pub fn new(mut inner: R) -> Result<Self, DecodeError> {
        let mut r = [0u8; 12];

        inner.read_exact(&mut r[..8])?;
        if &r[..8] != b"YKCMP_V1" {
            return Err(DecodeError::InvalidMagic);
        }

        inner.read_exact(&mut r[..12])?;
        let encoding = u32::from_le_bytes(unsafe { *(&r[0..4] as *const [u8] as *const [u8; 4]) });

        let comp_len =
            u32::from_le_bytes(unsafe { *(&r[4..8] as *const [u8] as *const [u8; 4]) }) as u64;
        let decomp_len =
            u32::from_le_bytes(unsafe { *(&r[8..12] as *const [u8] as *const [u8; 4]) }) as u64;

        let decoder_inner = match encoding {
            4 => DecoderInner::Lz77(NisLz77Reader::new(inner)),
            8 | 9 => {
                let compressed = vec![0u8; comp_len as usize];
                let decompressed = lz4_flex::decompress(&compressed[..], decomp_len as usize)?;

                DecoderInner::Lz4(Cursor::new(decompressed))
            }
            e => {
                return Err(DecodeError::UnsupportedEncoding(e));
            }
        };

        Ok(Decoder {
            inner: decoder_inner,
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
            DecoderInner::Lz4(ref mut inner) => inner.read(buf),
        }
    }
}
