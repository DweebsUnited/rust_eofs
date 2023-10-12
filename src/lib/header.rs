use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FSHeader {
    // 0xBEEFFEED
    #[serde(with = "postcard::fixint::be")]
    pub magic: u32,
    // Page size in bytes
    #[serde(with = "postcard::fixint::be")]
    pub page_size: u16,
    // Reserved pages before FATs (including this header page)
    pub reserved_pages: u8,
    // Number of FAT pages per segment
    pub fat_pages: u8,
    // Number of segments in FS
    pub segments: u8,
    // Address size
    pub address_bytes: u8,
    #[serde(with = "postcard::fixint::be")]
    pub pad2: u16,
    #[serde(with = "postcard::fixint::be")]
    pub pad3: u32,
    // Maximum allowed FS size -- may not match FAT addressable or real file size
    // Safety for embedded systems
    #[serde(with = "postcard::fixint::be")]
    pub max_fs_size: u64,
}

pub const DEFAULT_FS_HEADER: FSHeader = FSHeader{
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