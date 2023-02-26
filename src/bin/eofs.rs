use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
   #[command(subcommand)]
   command: Command,

    file: String,
}

#[derive(Subcommand, Debug)]
enum Command {

    Stat { },
    Disp { },
    Make {
        #[arg(default_value_t=rust_eofs::BLOCKLEN as u32,short='s',help="Number of bytes per block")]
        block_size: u32,
        #[arg(default_value_t=1,short='b',help="Number of blocks per data sector")]
        blocks_sector: u32,
        #[arg(default_value_t=1,short='f',help="Number of FAT copies to store")]
        num_fats: u32,
        #[arg(default_value_t=1<<16,short='t',help="Total max FileSystem size, will be adjusted down based on FAT settings")]
        total_fs_size: u32,
    },

}

fn main() {
    let cli = Cli::parse( );

    println!( "File we will use: {}", cli.file );

    match cli.command {

        Command::Stat{ } => {

            let f: rust_eofs::FileSystem = rust_eofs::FileSystem::open( &cli.file ).unwrap( );
            f.printstats( );

        }
        Command::Disp{ } => {

            let f: rust_eofs::FileSystem = rust_eofs::FileSystem::open( &cli.file ).unwrap( );

            let file_iter = f.root( ).iter( );

        }
        Command::Make{
            block_size,
            blocks_sector,
            num_fats,
            total_fs_size,
        } => {

            let f: rust_eofs::FileSystem = rust_eofs::FileSystem::new(
                block_size,
                blocks_sector,
                num_fats,
                total_fs_size );
            f.save( &cli.file );

        }

    }

}