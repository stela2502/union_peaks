
use std::io::BufReader;
use std::io::BufRead;

use std::fs::File;
use std::path::PathBuf;

pub struct IfileSimple {
    pub data: Vec<String>, // BufReader<GzDecoder<File>>,
}

/// Ofiles encapsulates two BufWriter<GzEncoder<File>> objects to make handling of 20 of these more convenient.
impl IfileSimple{
    pub fn new( file:&str )->Self {

        println!("I process the file {}", file );
        let fp1 = PathBuf::from(file);
        //let file1_path = PathBuf::from(outpath).join(format!("{}.{}.{}", mode, id, fp1.file_name().unwrap().to_str().unwrap() ));
        
        // need better error handling here too
        // println!( "why does this file break? {}", file1_path.display() );
        let f1 = match File::open(fp1){
            Ok(file) => file,
            Err(err) => panic!("An error occured opening the file: {}", err )
        };

        let mut data = Vec::<String>::with_capacity(200*1000);

        //println!("{:?}", file1.header() ); // None

        let cursor = BufReader::new( f1 );

        //let mut buf = String::new();
        //let mut ok = true;

        for line in cursor.lines(){
            match line{
                Ok(line) => {
                    //println!("This line worked well: '{}'", &line.clone());
                    data.push(line);
                    },
                Err(err) => eprintln!("I could not read a line from the gz file! {err}"),
            }
        }

        Self{
            data
        }
    }


    pub fn get_line(&mut self) -> Result< String, &str> {
        //println!("{} lines in the file", self.data.len() );
        if ! self.data.is_empty(){
            Ok(self.data.remove(0))
        }else {
            Err::< String, &str >( "no more data" )
        }
        
    }
}
