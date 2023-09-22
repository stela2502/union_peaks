
use std::io::BufWriter;

use std::fs;
use std::fs::File;
use std::path::PathBuf;

pub struct OfileSimple {
    pub count: u32,
    //pub file1: GzEncoder<File>,
    //pub file2: GzEncoder<File>,
    pub buff1: BufWriter<File>
}

/// Ofiles encapsulates two BufWriter<GzEncoder<File>> objects to make handling of 20 of these more convenient.
impl OfileSimple{
    pub fn new_file( file:&str )->Self {

        let fp1 = PathBuf::from(file);
        let opath = fp1.parent().unwrap();
        fs::create_dir_all(opath).expect("AlreadyExists");

        // need better error handling here too
        // println!( "why does this file break? {}", file1_path.display() );
        let f1 = match File::create(fp1){
            Ok(file) => file,
            Err(err) => panic!("The file cound not be created: {err}"  )
        };

        let buff1 = BufWriter::new( f1 );

        let count:u32 = 0;
        Self{
            count,
            //file1,
            //file2,
            buff1
        }
    }
    pub fn new( file:&str, outpath:&str )->Self {

        let mut fp1 = PathBuf::from(outpath);
        fp1.push( file );

        // need better error handling here too
        // println!( "why does this file break? {}", file1_path.display() );
        let f1 = match File::create(fp1){
            Ok(file) => file,
            Err(err) => panic!("The file cound not be created: {err}"  )
        };
        
        let buff1 = BufWriter::new( f1 );

        let count:u32 = 0;
        Self{
            count,
            //file1,
            //file2,
            buff1
        }
    }

}
