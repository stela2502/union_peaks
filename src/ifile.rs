
use std::io::BufReader;
use std::io::BufRead;

use flate2::write::GzDecoder;

use std::fs::File;
use std::path::PathBuf;
use std::time::SystemTime;

pub struct Ifile {
    pub data: Vec<String>, // BufReader<GzDecoder<File>>,
}

/// Ofiles encapsulates two BufWriter<GzEncoder<File>> objects to make handling of 20 of these more convenient.
impl Ifile{
    pub fn new( file:&str )->Self {

        println!("I process the file {}", file.to_string() );
        let fp1 = PathBuf::from(file);
        //let file1_path = PathBuf::from(outpath).join(format!("{}.{}.{}", mode, id, fp1.file_name().unwrap().to_str().unwrap() ));
        
        // need better error handling here too
        // println!( "why does this file break? {}", file1_path.display() );
        let f1 = match File::open(fp1){
            Ok(file) => file,
            Err(err) => panic!("An error occured opening the file: {}", err )
        };

        let mut data = Vec::<String>::with_capacity(200*1000);

        let file1 = GzDecoder::new(f1);

        let mut cursor = BufReader::new( file1 );

        let mut buf = String::new();
        let mut ok = true;
        let bytes = 0;

        while ok{
            match cursor.read_line(&mut buf) {
                Ok(bytes) if bytes > 0 => {
                    println!("I got something!");
                    data.push(buf.clone());
                    buf.clear();
                },
                Ok(_) => ok = false,
                Err(err) => eprintln!("I could not read a line from the gz file! {err}"),
                // match line{
                //     Ok(line) => {
                //         println!("This line worked well: '{}'", &line.clone());
                //         data.push(line);
                //         },
                //     Err(err) => eprintln!("I could not read a line from the gz file! {err}"),
                // }
            }
        }

        Self{
            data
        }
    }


    pub fn get_line(&mut self) -> Result< String, &str> {
        //println!("{} lines in the file", self.data.len() );
        if ! self.data.is_empty(){
            return Ok(self.data.remove(0));
        }else {
            Err::< String, &str >( "no more data" )
        }
        
    }
}
