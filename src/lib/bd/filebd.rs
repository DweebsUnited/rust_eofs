use super::*;

use std::fs;
use std::io;
use std::io::{Read, Write, Seek, SeekFrom};

pub struct FileBD {
    fpath: String,
    f: Option<fs::File>,
    fbs: u32,
}

impl BD for FileBD {
    fn open(&mut self) -> Result<(), BDError> {
        if self.f.is_some() {
            return Err(BDError::InvalidOperation(INVOP_OPEN));
        }

        let f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.fpath)?;
        let m = f.metadata()?;

        self.fbs = m.len() as u32 / super::BLOCK_LEN;
        self.f = Some(f);

        Ok(())
    }

    fn close(&mut self) -> Result<(), BDError> {
        if self.f.is_none() {
            return Err(BDError::InvalidOperation(super::INVOP_CLOSE));
        }

        let f = self.f.as_mut().unwrap();
        f.flush()?;

        self.fbs = 0;
        self.f = None;

        Ok(())
    }

    fn read(&mut self, bdx: u32, buf: &mut Block) -> Result<(), BDError> {
        if self.f.is_none() {
            return Err(BDError::InvalidOperation(INVOP_NOTOPEN));
        }

        // If reading past end of file, just return 0's
        // Extend on write
        if bdx >= self.fbs {
            buf.fill(0);
            return Ok(());
        }

        // Now safe
        let f = self.f.as_mut().unwrap();

        f.seek(SeekFrom::Start((bdx * BLOCK_LEN) as u64))?;
        f.read_exact(buf)?;

        Ok(())
    }

    fn write(&mut self, bdx: u32, buf: &Block) -> Result<(), BDError> {
        if self.f.is_none() {
            return Err(BDError::InvalidOperation(INVOP_NOTOPEN));
        }

        // Now safe
        let f = self.f.as_mut().unwrap();

        // If writing past end of file, extend
        if bdx >= self.fbs {
            let empty = [0 as u8; BLOCK_LEN as usize];
            f.seek(SeekFrom::End(0))?;
            while bdx >= self.fbs {
                f.write_all(&empty)?;
                self.fbs += 1;
            }
        }

        f.seek(SeekFrom::Start((bdx * BLOCK_LEN) as u64))?;
        f.write_all(buf)?;

        Ok(())
    }
}

impl FileBD {
    pub fn new(fpath: &str) -> Result<Self, BDError> {
        return Ok(FileBD {
            fpath: fpath.to_string(),
            f: None,
            fbs: 0
        })
    }
}