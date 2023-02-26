use std::fs::File as FSFile;
use std::io::prelude::*;
use std::path::Path;

pub struct FSBuffer {

}

impl FSBuffer {

    pub fn new( filename: &str ) {
        let path = Path::new( &filename );

        let mut file = FSFile::create( path ).unwrap();
    }

}