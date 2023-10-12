use std::{fs, io, path};
use std::mem::size_of;
use std::io::{Read, Write, Seek, SeekFrom};

use postcard::{from_bytes, to_vec};

pub const BLOCK_LEN: u32 = 512;

pub type Block = [u8; BLOCK_LEN as usize];

pub mod bd;
pub use bd::*;

pub mod header;
pub use header::*;

pub fn new_fs<P: AsRef<path::Path>>(fpath: P, header: Option<&FSHeader>) -> Result<fs::File, io::Error> {

    let dh: FSHeader = DEFAULT_FS_HEADER.clone();

    let h: &FSHeader = match header {
        Option::None => {
            &dh
        }
        Option::Some(h) => h
    };

    let mut f: fs::File = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(fpath)?;

    let mut dat: heapless::Vec<u8, 512> = to_vec(h).unwrap();

    dat.resize(512, 0).unwrap();

    f.write_all(&dat)?;

    dat.fill(0);

    let addressable_pages = h.fat_pages as usize * h.page_size as usize / h.address_bytes as usize;

    for _sdx in 0..(h.segments) {
        for _fdx in 0..(h.fat_pages) {
            for _pdx in 0..(addressable_pages) {
                f.write_all(&dat)?;
            }
        }
    }

    Ok(f)

}

pub fn open_fs<P: AsRef<path::Path>>(fpath: P) -> Result<fs::File, io::Error> {
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(fpath)
}

pub fn dump_fs(f: &mut fs::File) {

    let size = f.seek(SeekFrom::End(0)).unwrap();

    dbg!(size);

    f.seek(io::SeekFrom::Start(0)).unwrap();

    let mut header_raw: [u8; size_of::<FSHeader>()] = [0; size_of::<FSHeader>()];
    dbg!(header_raw.len());
    f.read_exact(&mut header_raw).unwrap();

    let header: FSHeader = from_bytes(&header_raw).unwrap();

    dbg!(&header);

    // Calculate some derived values
    // Current FS size
    let curr_fs_size_pages: usize = header.reserved_pages as usize +
        header.segments as usize * header.fat_pages as usize * 2 +
        header.segments as usize * header.fat_pages as usize * header.page_size as usize / header.address_bytes as usize;
    dbg!(curr_fs_size_pages);
    let curr_fs_size: usize = curr_fs_size_pages * header.page_size as usize;
    dbg!(curr_fs_size);

}