use std::{
    collections::VecDeque,
    io::{self, Cursor, Read, Write},
};

pub struct Decoder<R: Read> {
    inner: R,
    _decomp_len: u64,
    copy_buf: VecDeque<u8>,
    eof: bool,
}

enum Cmd {
    Literal { len: u64 },
    Pointer { len: u64, off: u64 },
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
        let decomp_len =
            u32::from_le_bytes(unsafe { *(&r[8..12] as *const [u8] as *const [u8; 4]) }) as u64;

        Ok(Decoder {
            inner,
            _decomp_len: decomp_len,
            copy_buf: VecDeque::with_capacity(4096),
            eof: false,
        })
    }

    fn next_cmd(&mut self) -> io::Result<Option<Cmd>> {
        let mut r = [0u8; 3];

        match self.inner.read_exact(&mut r[..1]) {
            Ok(_) => {}
            Err(e) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    // end of encoded text
                    return Ok(None);
                } else {
                    return Err(e);
                }
            }
        }
        let cmd = r[0] as u32;

        if cmd < 0x80 {
            // Literal
            return Ok(Some(Cmd::Literal { len: cmd as u64 }));
        } else if cmd < 0xC0 {
            // Short ptr
            return Ok(Some(Cmd::Pointer {
                len: cmd.wrapping_shr(4).wrapping_sub(7) as u64,
                off: (cmd & 0x0F).wrapping_add(1) as u64,
            }));
        } else if cmd < 0xE0 {
            // Long ptr
            self.inner.read_exact(&mut r[..1])?;
            return Ok(Some(Cmd::Pointer {
                len: cmd.wrapping_sub(0xBE) as u64,
                off: (r[0] as u32).wrapping_add(1) as u64,
            }));
        } else {
            // Longer ptr
            self.inner.read_exact(&mut r[..2])?;
            return Ok(Some(Cmd::Pointer {
                len: cmd
                    .wrapping_shl(4)
                    .wrapping_add((r[0] as u32).wrapping_shr(4))
                    .wrapping_sub(0xE00 - 3) as u64,
                off: (r[0] as u32 & 0x0F)
                    .wrapping_shl(8)
                    .wrapping_add(r[1] as u32)
                    .wrapping_add(1) as u64,
            }));
        }
    }
}

impl<R> Read for Decoder<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        while self.copy_buf.len() < 4095 + buf.len() && !self.eof {
            match self.next_cmd()? {
                None => {
                    self.eof = true;
                    break;
                }
                Some(Cmd::Literal { len }) => {
                    if len >= 0x80 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "invalid literal >= 0x80",
                        ));
                    }

                    let take = (&mut self.inner).take(len as u64).bytes();
                    let mut written = 0;

                    for byte in take {
                        match byte {
                            Err(e) => return Err(e),
                            Ok(byte) => {
                                written += 1;
                                self.copy_buf.push_back(byte);
                            }
                        }
                    }
                    if written != len {
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "literal copy reached EOF early",
                        ));
                    }
                }
                Some(Cmd::Pointer { len, off }) => {
                    if off > 4096 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "invalid pointer copy",
                        ));
                    }
                    if len > 514 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "invalid pointer copy",
                        ));
                    }

                    for _ in 0..len {
                        if off == 0 || self.copy_buf.len() < off as usize {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "bad pointer copy in stream",
                            ));
                        }
                        self.copy_buf
                            .push_back(self.copy_buf[self.copy_buf.len() - off as usize]);
                    }
                }
            }
        }

        // then, drain the amount of the copy buffer that is necessary to read
        let bytes_read = std::cmp::min(buf.len(), self.copy_buf.len());
        let mut cursor = Cursor::new(buf);
        self.copy_buf.drain(..bytes_read).for_each(|b| {
            cursor.write_all(&[b]).unwrap();
        });

        Ok(bytes_read)
    }
}
