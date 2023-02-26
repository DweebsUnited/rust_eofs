pub mod fsbuffer;

use clap::__macro_refs::once_cell::race::OnceNonZeroUsize;
use fsbuffer::FSBuffer;

fn div_ceil( lhs: u32, rhs: u32 ) -> u32 {
    return ( lhs + ( rhs - 1 ) ) / rhs
}

pub const BLOCKLEN: usize = 512;

pub type Block = [u8; BLOCKLEN];

pub struct Header {

    magic: u32,
    block_size: u32,
    blocks_sector: u32,
    reserved_sectors: u32, // == 1 + numFATs * lenFAT(blocks), rounded to next sector boundary @ n * blockSize * blocksPerSector
    num_fats: u32,
    fat_blocks: u32,
    data_sectors: u32, // + reservedSectors gives total filesystem size

}

#[derive(Debug, Clone)]
pub struct FileFAT {

    entries: Vec<u32>,

}

pub struct FileSystem {
    header: Header,
    fats: Vec<FileFAT>,
    data_sectors: Vec<Block>,
}

pub struct Dir {

}

pub struct File {

}

pub struct DirIter {

}

impl FileSystem {
    pub fn new( block_size: u32, blocks_sector: u32, num_fats: u32, total_fs_size: u32 ) -> FileSystem {
        // TODO: Lot of laziness needed..
        //   Don't preallocate the entire file here
        //   Will require custom buffered RW-er

        let sector_size: u32 = block_size * blocks_sector;

        // Some calculations...
        // Figure out how big a FAT we need to cover the whole FS
        // Add blocks to the FAT size until the amount of addressable sectors is greater than data space
        // TODO: Tradeoff check, adding one more block to each FAT eats more than is wasted by not addressing entire space? -- Most likely will occur right around reserved sector boundary..
        // TODO: Calculate directly without looping?

        let mut fat_blocks: u32 = 1;
        let mut reserved_sectors: u32 = div_ceil( block_size + num_fats * fat_blocks * block_size, sector_size );
        let mut data_sectors: u32 = total_fs_size / ( block_size * blocks_sector ) - reserved_sectors;

        while fat_blocks * block_size / 4 < data_sectors {
            fat_blocks += 1;
            reserved_sectors = div_ceil( block_size + num_fats * fat_blocks * block_size, sector_size );
            data_sectors = total_fs_size / ( block_size * blocks_sector ) - reserved_sectors;
        }

        return FileSystem {
            header: Header {
                magic: 0xBEEFBEEF,
                block_size: block_size as u32,
                blocks_sector: blocks_sector as u32,
                reserved_sectors: reserved_sectors as u32,
                num_fats: num_fats as u32,
                fat_blocks: fat_blocks as u32,
                data_sectors: data_sectors as u32,
            },
            fats: vec![ FileFAT{ entries: vec![0; ( block_size / 4 ) as usize] }; num_fats as usize ],
            data_sectors: vec![ [0 as u8; BLOCKLEN]; data_sectors as usize ],
        }

    }

    pub fn printstats( &self ) {
        println!( "Magic number == 0xBEEFBEEF? {}", self.header.magic == 0xBEEFBEEF as u32 );
        println!( "Block size: ...... {}", self.header.block_size );
        println!( "Blocks per sector: {}", self.header.blocks_sector );
        println!( "Sector size: ..... {}", self.header.block_size * self.header.blocks_sector );
        println!( "" );
        println!( "Reserved sectors:  {}", self.header.reserved_sectors );
        println!( "\tCopies of FAT: ....... {}", self.header.num_fats );
        println!( "\tBlocks per FAT: ...... {}", self.header.fat_blocks );
        println!( "Data sectors: .... {}", self.header.data_sectors );
        println!( "" );
        println!( "Entries per FAT block: {}", self.header.block_size / 4 );
        println!( "Addressable blocks: .. {}", self.header.block_size / 4 * self.header.blocks_sector );
        println!( "Addressable: ......... {}", self.header.block_size / 4 * self.header.blocks_sector * self.header.block_size );
        println!( "" );
        println!( "Total FS size: . {}", ( self.header.data_sectors + self.header.reserved_sectors ) * self.header.block_size * self.header.blocks_sector );
        println!( "Total FS blocks: {}", ( self.header.data_sectors + self.header.reserved_sectors ) * self.header.blocks_sector );
        println!( "Reserved size: .. {}", self.header.reserved_sectors * self.header.block_size * self.header.blocks_sector );
        println!( "Reserved blocks:  {}", self.header.reserved_sectors * self.header.blocks_sector );
        println!( "Reserved sectors: {}", self.header.reserved_sectors );
        println!( "Data size: .. {}", self.header.data_sectors * self.header.block_size * self.header.blocks_sector );
        println!( "Data blocks:  {}", self.header.data_sectors * self.header.blocks_sector );
        println!( "Data sectors: {}", self.header.data_sectors );
    }

    pub fn save( &self, filename: &str ) {

        // TODO

        FSBuffer::new( filename );

    }

    pub fn open( filename: &str ) -> Option<FileSystem> {

        // TODO

        return None;

    }

    pub fn root( &self ) -> &Dir {

        // TODO

        return &Dir{ };

    }

}

impl Dir {
    pub fn iter( &self ) -> DirIter {
        return DirIter{ };
    }
}

impl Iterator for DirIter {
    // TODO: Return copy of some identifying
    type Item = File;

    fn next( &mut self ) -> Option<Self::Item> {
        // TODO
        return None
    }
}