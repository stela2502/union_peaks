
use std::fs;
mod ifile;
mod ofile;
mod feature;
use crate::ifile::Ifile;
use crate::ofile::Ofile;
use crate::feature::Feature;

use std::time::SystemTime;

use std::path::Path;

use clap::Parser;
use std::io::Write;

/// If you have multiple samples of 10x atac data you are screqed
/// I have not found any option to 'aligne' the peaks over multiple samples.
/// This leads to the data not being comparable between samples.
/// To counter this problem this package creates unions of overlapping peaks over multiple 10 atac datasets.
/// All files are meant to be sorted in the same way.

#[derive(Parser)]
#[clap(version = "1.0.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the paths to the featuretsv.gz files (comma separated string)
    #[clap(default_value= "testData/B.tsv.gz,testData/C.tsv.gz",short, long)]
    paths: String,
    /// the outpath
    #[clap(default_value=  "testData/Output",short, long)]
    outpath: String,
}


fn main() {

    let now = SystemTime::now();
    let opts: Opts = Opts::parse();

    fs::create_dir_all(&opts.outpath).expect("AlreadyExists");

    let source_files:Vec::<&str> = opts.paths.split(',').collect();
    let files_n = source_files.len();
    let mut ifiles = Vec::<Ifile>::with_capacity( files_n );
    let mut ofiles = Vec::<Ofile>::with_capacity( files_n );

    for path in source_files{
        println!("This should be a features.tsv.gz file: {}", path );
        ifiles.push( Ifile::new( path ));
        let filen = Path::new(path).file_name().unwrap().to_str().expect("Invalid UTF-8 in file name");
        ofiles.push( Ofile::new (filen, &opts.outpath ));
    }

    let mut still_data = true;
    let mut with_data: Vec<bool>= vec![ true; files_n ];

    let mut features: Vec<Feature> = vec![Feature::init(); files_n ];
    let mut write = Vec::<bool>::with_capacity( files_n );

    while still_data {
        for i in 0..files_n{
            // parse the string into a feature
            if features[i].empty && with_data[i] {

                if let Ok(text) = &ifiles[i].get_line(){
                    print!("{}", text);
                    features[i] = Feature::parse( text  );

                }else{
                    with_data[i] = false;
                }
            }
            write[i] = false;
        }
        // check something - for later
        for i in 0..files_n{
            if features[i].ty != "Peaks"{
                write[i] = true;
            }
            else {
                // the feature does not overlap with any other feature and is 'before' all other features

                // but what if it is overlapping? Don't I need a look ahead? There might actually be a whole bunch of 'small' peaks
                // cluttered together in a 'large' peak of another sample. To make that work in the episcanpy the small peaks all need to get the
                // positions of the long peak. Then they can be summed up later on.

            }
        }

        for i in 0..files_n{
            // write everything that should be written
            if write[i] {
                match writeln!( ofiles[i].buff1, "{}", features[i] ){
                    Ok(_) => features[i].empty = true,
                    Err(err) => panic!( "I could not write the data to outfile {i}:\n{err}" ),
                };
            }

            still_data = with_data.iter().any(|&x| x);
            
        }

    }

    match now.elapsed() {
        Ok(elapsed) => {
            let mut milli = elapsed.as_millis();

            let mil = milli % 1000;
            milli= (milli - mil) /1000;

            let sec = milli % 60;
            milli= (milli -sec) /60;

            let min = milli % 60;
            milli= (milli -min) /60;

            println!("union_peaks finished in {milli}h {min}min {sec} sec {mil}milli sec\n" );},
       Err(e) => {println!("Error: {e:?}");}
    }

    
}


