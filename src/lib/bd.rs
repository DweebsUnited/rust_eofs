use super::*;

use std::io;

const INVOP_OPEN: &str = "Opened already open file";
const INVOP_CLOSE: &str = "Closed already closed file";
const INVOP_NOTOPEN: &str = "File not open";

#[derive(Debug)]
pub enum BDError {
    InvalidOperation(&'static str),
    IOError(io::Error),
}

impl From<io::Error> for BDError {
    fn from(value: io::Error) -> Self {
        BDError::IOError(value)
    }
}

pub trait BD {
    fn open(&mut self) -> Result<(), BDError>;
    fn close(&mut self) -> Result<(), BDError>;

    fn read(&mut self, bdx: u32, buf: &mut Block) -> Result<(), BDError>;
    fn write(&mut self, bdx: u32, buf: &Block) -> Result<(), BDError>;
}

pub mod filebd;
pub use filebd::*;