
use std::io::BufWriter;
use flate2::Compression;
use flate2::write::GzEncoder;

use std::fs::File;
use std::path::PathBuf;

pub struct Ofile {
    pub count: u32,
    //pub file1: GzEncoder<File>,
    //pub file2: GzEncoder<File>,
    pub buff1: BufWriter<GzEncoder<File>>
}

/// Ofiles encapsulates two BufWriter<GzEncoder<File>> objects to make handling of 20 of these more convenient.
impl Ofile{
    pub fn new( file:&str, outpath:&str )->Self {

        let mut fp1 = PathBuf::from(outpath);
        fp1.push( file );

        // need better error handling here too
        // println!( "why does this file break? {}", file1_path.display() );
        let f1 = match File::create(fp1){
            Ok(file) => file,
            Err(err) => panic!("The file cound not be created: {err}"  )
        };
        
        let file1 = GzEncoder::new(f1, Compression::default());

        let buff1 = BufWriter::new( file1 );

        let count:u32 = 0;
        Self{
            count,
            //file1,
            //file2,
            buff1
        }
    }

}
