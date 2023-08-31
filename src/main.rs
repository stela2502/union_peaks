
use std::fs;
mod ifile;
mod ofile;
mod feature;
mod matchgroup;
mod traits;
use crate::ifile::Ifile;
use crate::ofile::Ofile;
use crate::feature::Feature;
use crate::matchgroup::MatchGroup;
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

    let mut match_groups= Vec::<MatchGroup>::with_capacity(100);
    let mut handled:bool;

    while still_data {
        for i in 0..files_n{
            // parse the string into a feature
            match &ifiles[i].get_line(){
                Ok(text) => {
                    let feat = Feature::parse( text  );
                    if feat.ty != "Peaks"{
                        // if we already have match_groups we need to write them all to file!
                        for match_group in &match_groups{
                            for id in &match_group.targets{
                                match writeln!( ofiles[*id].buff1, "{}", match_group ){
                                    Ok(_) => (),
                                    Err(err) => panic!( "I could not write the data to outfile {id}:\n{err}" ),
                                }
                            }
                        }
                       
                        match writeln!( ofiles[i].buff1, "{}", feat ){
                            Ok(_) => (),
                            Err(err) => panic!( "I could not write the data to outfile {i}:\n{err}" ),
                        };
                    }else {
                        handled = false;
                        for id in 0..match_groups.len(){
                        //for  match_group in &match_groups{
                            if match_groups[id].overlapps_adjust ( &feat ){
                                match_groups[id].register_write_to( i );
                                handled = true;
                            }
                        }
                        if ! handled{
                            match_groups.push( MatchGroup::new( &feat , i) );
                        }
                    }
                    
                },
                Err(err) => {
                    eprintln!("I got an error from the gzipped file {i}: {err} - assume finished");
                    with_data[i] = false;
                },
            }
        }
        still_data = with_data.iter().any(|&x| x);
    }

    for match_group in &match_groups{
        for id in &match_group.targets{
            match writeln!( ofiles[*id].buff1, "{}", match_group ){
                Ok(_) => (),
                Err(err) => panic!( "I could not write the data to outfile {id}:\n{err}" ),
            };
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


