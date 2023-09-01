use std::path;
use std::{fs, io, option};
use std::io::{Read, Write, Seek, SeekFrom};
use std::mem::size_of;

use serde::{Serialize, Deserialize};
use postcard::{from_bytes, to_vec};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author = "Eric Osburn", version = "0.1", about = "EOFS cli interface", long_about = None)]
struct Cli {
    #[arg(short = 'f', long = "fsfile", value_name = "FSFILE")]
    fsfile: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[arg(short = 's', long = "segments", value_name = "SEGMENTS")]
        segments: u8,
    },
    Dump {},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FSHeader {
    // 0xBEEFFEED
    #[serde(with = "postcard::fixint::be")]
    magic: u32,
    // Page size in bytes
    #[serde(with = "postcard::fixint::be")]
    page_size: u16,
    // Reserved pages before FATs (including this header page)
    reserved_pages: u8,
    // Number of FAT pages per segment
    fat_pages: u8,
    // Number of segments in FS
    segments: u8,
    // Address size
    address_bytes: u8,
    #[serde(with = "postcard::fixint::be")]
    pad2: u16,
    #[serde(with = "postcard::fixint::be")]
    pad3: u32,
    // Maximum allowed FS size -- may not match FAT addressable or real file size
    // Safety for embedded systems
    #[serde(with = "postcard::fixint::be")]
    max_fs_size: u64,
}

const DEFAULT_FS_HEADER: FSHeader = FSHeader{
    magic: 0xBEEFFEED,
    page_size: 512,
    reserved_pages: 1,
    fat_pages: 1,
    segments: 0,
    address_bytes: 4,
    pad2: 0,
    pad3: 0,
    max_fs_size: 512,
};

fn new_fs<P: AsRef<path::Path>>(fpath: P, header: option::Option<&FSHeader>) -> Result<fs::File, io::Error> {

    let dh: FSHeader = DEFAULT_FS_HEADER.clone();

    let h: &FSHeader = match header {
        option::Option::None => {
            &dh
        }
        option::Option::Some(h) => h
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

fn open_fs<P: AsRef<path::Path>>(fpath: P) -> Result<fs::File, io::Error> {
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(fpath)
}

fn dump_fs(f: &mut fs::File) {

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

fn main() {

    let cli = Cli::parse();

    let fpath: &str = cli.fsfile.as_ref();

    match &cli.command {
        Commands::New { segments } => {
            let mut dh: FSHeader = DEFAULT_FS_HEADER.clone();
            dh.segments = *segments;

            let max_fs_size: usize = dh.page_size as usize * ( dh.reserved_pages as usize +
                dh.segments as usize * dh.fat_pages as usize * 2 +
                dh.segments as usize * dh.fat_pages as usize * dh.page_size as usize / dh.address_bytes as usize );
            dh.max_fs_size = max_fs_size as u64;

            new_fs(fpath, option::Option::Some(&dh)).unwrap();
        }
        Commands::Dump {} => {
            let mut f = open_fs(fpath).unwrap();
            dump_fs(&mut f);
        }
    }

}