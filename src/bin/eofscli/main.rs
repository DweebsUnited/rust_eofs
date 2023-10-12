use eofs::*;
use eofs::bd::*;

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

fn main() {

    let cli = Cli::parse();

    let fpath: &str = cli.fsfile.as_ref();

    match &cli.command {
        Commands::New { segments } => {
            println!("Open...");
            let mut bd = eofs::bd::FileBD::new(fpath).unwrap();
            bd.open().unwrap();

            let mut dh: FSHeader = DEFAULT_FS_HEADER.clone();
            dh.segments = *segments;

            let max_fs_size: usize = dh.page_size as usize * ( dh.reserved_pages as usize +
                dh.segments as usize * dh.fat_pages as usize * 2 +
                dh.segments as usize * dh.fat_pages as usize * dh.page_size as usize / dh.address_bytes as usize );
            dh.max_fs_size = max_fs_size as u64;

            new_fs(fpath, Some(&dh)).unwrap();

            bd.close().unwrap();
        }
        Commands::Dump {} => {
            let mut f = open_fs(fpath).unwrap();
            dump_fs(&mut f);
        }
    }

}