
use std::fs;
mod ifile;
mod ofile;
mod feature;
mod matchgroup;
use crate::ifile::Ifile;
use crate::ofile::Ofile;
use crate::feature::Feature;
use crate::matchgroup::MatchGroup;
use std::time::SystemTime;

use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

use std::path::Path;

use clap::Parser;
use std::io::Write;

/// If you have multiple samples of 10x atac data you might end up with uncomparable results. 
/// I have not found any option to 'aligne' the peaks over multiple samples. 
/// This leads to the data not being comparable between samples. 
/// To counter this problem this program creates unions of overlapping peaks over multiple 10x atac datasets.

#[derive(Parser)]
#[clap(version = "1.0.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the paths to the feature.tsv.gz files (one comma separated string)
    #[clap(default_value= "testData/B.tsv.gz,testData/C.tsv.gz",short, long)]
    paths: String,
    /// the outpath (Peak positions will be changed)
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
        //println!("This should be a features.tsv.gz file: {}", path );
        ifiles.push( Ifile::new( path ));
        let filen = Path::new(path).file_name().unwrap().to_str().expect("Invalid UTF-8 in file name");
        ofiles.push( Ofile::new (filen, &opts.outpath ));
    }

    let mut still_data = true;
    let mut with_data: Vec<bool>= vec![ true; files_n ];

    let mut match_groups= Vec::<Feature>::with_capacity(100000);
    let mut handled:bool;

    // the features need to be registered with the outfiles
    let mut ofiles_pos: Vec<Vec<usize>> = Vec::new();
    for id in 0..files_n{
        let mut file = Vec::<usize>::with_capacity(100000);
        ofiles_pos.push( file );
    }


    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new(1000));
    pb.set_style(spinner_style);

    while still_data {
        pb.inc(1);
        for i in 0..files_n{

            if with_data[i]{
                // parse the string into a feature

                match &ifiles[i].get_line(){
                    Ok(text) => {
                        let feat = Feature::parse( text  );
                        if feat.ty != "Peaks"{
                            // if we already have match_groups we need to write them all to file!
                            for i in 0..files_n{
                                for id in &ofiles_pos[i]{
                                    //println!("printing to file {id}: {}", match_groups[id]);
                                    match writeln!( ofiles[i].buff1, "{}", match_groups[*id] ){
                                        Ok(_) => (),
                                        Err(err) => panic!( "I could not write the data to outfile {i}:\n{err}" ),
                                    };
                                }
                                ofiles_pos[i].clear();
                            }
                            match_groups.clear();

                            match writeln!( ofiles[i].buff1, "{}", feat ){
                                Ok(_) => (),
                                Err(err) => panic!( "I could not write the data to outfile {i}:\n{err}" ),
                            };
                        }else {
                            handled = false;
                            for id in (0..match_groups.len()).rev() {
                            //for  match_group in &match_groups{
                                if match_groups[id].overlapps_adjust ( &feat ){
                                    //println!("MatchGroup \n{} matched feature \n{}", match_groups[id], feat );
                                    ofiles_pos[i].push(id);
                                    //match_groups[id].register_write_to( i );
                                    handled = true;
                                    break;
                                }
                            }
                            if ! handled{
                                //match_groups.push( MatchGroup::new( &feat , i) );
                                match_groups.push(feat);
                                ofiles_pos[i].push( match_groups.len()-1);
                            }
                        }
                        
                    },
                    Err(err) => {
                        eprintln!("file {i}: {err}");
                        with_data[i] = false;
                    },
                }
            }
        }
        still_data = with_data.iter().any(|&x| x);
    }

    for i in 0..files_n{
        for id in &ofiles_pos[i]{
            println!("printing to file {i}: {}", match_groups[*id]);
            match writeln!( ofiles[i].buff1, "{}", match_groups[*id] ){
                Ok(_) => (),
                Err(err) => panic!( "I could not write the data to outfile {i}:\n{err}" ),
            };
        }
        ofiles_pos[i].clear();
    }
    match_groups.clear();

    // for match_group in &match_groups{
    //     for id in &match_group.targets{
    //         println!("printing to file {id}: {}", match_group);
    //         match writeln!( ofiles[*id].buff1, "{}", match_group ){
    //             Ok(_) => (),
    //             Err(err) => panic!( "I could not write the data to outfile {id}:\n{err}" ),
    //         };
    //     }
    // }
    //match_groups.clear();

    pb.finish_with_message( "Finished" );

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


#[cfg(test)]
mod tests {

    use crate::ifile::Ifile;
    use crate::ofile::Ofile;
    use crate::feature::Feature;
    use crate::matchgroup::MatchGroup;

    #[test]
    fn check_parse() {

        let test_data1 = vec![
            "chr1:10-20\tchr1:10-20\tPeaks\tchr1\t10\t20".to_string(),
            "chr1:30-35\tchr1:30-35\tPeaks\tchr1\t30\t35".to_string(),
            "chr1:55-60\tchr1:55-60\tPeaks\tchr1\t55\t60".to_string(),
            "chr1:65-70\tchr1:65-70\tPeaks\tchr1\t65\t70".to_string(),
            "chr1:99-110\tchr1:99-110\tPeaks\tchr1\t99\t110".to_string(),
            "chr1:600-620\tchr1:600-620\tPeaks\tchr1\t600\t620".to_string(),
            "chr1:2000-2010\tchr1:2000-2010\tPeaks\tchr1\t2000\t2010".to_string()
        ];
        let test_data2 = vec![
            "chr1:9-19\tchr1:9-19\tPeaks\tchr1\t9\t19".to_string(),
            "chr1:52-70\tchr1:52-70\tPeaks\tchr1\t52\t70".to_string(),
            "chr1:100-120\tchr1:100-120\tPeaks\tchr1\t100\t120".to_string(),
            "chr1:450-500\tchr1:450-500\tPeaks\tchr1\t450\t500".to_string(),
            "chr1:600-660\tchr1:600-660\tPeaks\tchr1\t600\t660".to_string(),
            "chr1:1000-1010\tchr1:1000-1010\tPeaks\tchr1\t1000\t1010".to_string()
        ];

        let mut ifiles= [ Ifile{ data: test_data1 }, Ifile{ data: test_data2 }];

        let files_n = 2;
        let mut still_data = true;
        let mut with_data: Vec<bool>= vec![ true; files_n ];
        let mut match_groups= Vec::<Feature>::with_capacity(100);
        let mut handled:bool;


        // the features need to be registered with the outfiles
        let mut ofiles_pos: Vec<Vec<usize>> = Vec::new();
        for id in 0..files_n{
            let mut file = Vec::<usize>::with_capacity(100000);
            ofiles_pos.push( file );
        }

        while still_data {
            for i in 0..files_n{

                if with_data[i]{
                    // parse the string into a feature

                    match &ifiles[i].get_line(){
                        Ok(text) => {
                            let feat = Feature::parse( text  );

                            handled = false;
                            for id in (0..match_groups.len()).rev() {
                            //for  match_group in &match_groups{
                                if match_groups[id].overlapps_adjust ( &feat ){
                                    ofiles_pos[i].push(id);
                                    //match_groups[id].register_write_to( i );
                                    handled = true;
                                    break;
                                }
                            }
                            if ! handled{
                                //match_groups.push( MatchGroup::new( &feat , i) );
                                match_groups.push(feat);
                                ofiles_pos[i].push( match_groups.len()-1);
                            }
                            
                            
                        },
                        Err(err) => {
                            eprintln!("file {i}: {err}");
                            with_data[i] = false;
                        },
                    }
                }
            }
            still_data = with_data.iter().any(|&x| x);
        }
        
        let mut ofiles: Vec<Vec<String>> = Vec::new();
        let mut file1 = Vec::<String>::with_capacity(20);
        let mut file2 = Vec::<String>::with_capacity(20);
        ofiles.push( file1 );
        ofiles.push( file2 );


        for i in 0..files_n{
            for id in &ofiles_pos[i]{
                ofiles[i].push( format!("{}", match_groups[*id]));
            }
            ofiles_pos[i].clear();
        }
        match_groups.clear();

        // for match_group in &match_groups{
        //     for id in &match_group.targets{
        //         ofiles[*id].push( format!("{}", match_group));
        //     }
        // }
        // match_groups.clear();

        // now lets  check what we got:

        let mut ex1 = Vec::<String>::with_capacity(20);
        let mut ex2 = Vec::<String>::with_capacity(20);
        ex1.push( "chr1:9-20\tchr1:9-20\tPeaks\tchr1\t9\t20".to_string() );
        ex1.push( "chr1:30-35\tchr1:30-35\tPeaks\tchr1\t30\t35".to_string() );
        ex1.push( "chr1:52-70\tchr1:52-70\tPeaks\tchr1\t52\t70".to_string() );
        ex1.push( "chr1:52-70\tchr1:52-70\tPeaks\tchr1\t52\t70".to_string() );
        ex1.push( "chr1:99-120\tchr1:99-120\tPeaks\tchr1\t99\t120".to_string() );
        ex1.push( "chr1:600-660\tchr1:600-660\tPeaks\tchr1\t600\t660".to_string() );
        ex1.push( "chr1:2000-2010\tchr1:2000-2010\tPeaks\tchr1\t2000\t2010".to_string() );

        ex2.push( "chr1:9-20\tchr1:9-20\tPeaks\tchr1\t9\t20".to_string() );
        ex2.push( "chr1:52-70\tchr1:52-70\tPeaks\tchr1\t52\t70".to_string() );
        ex2.push( "chr1:99-120\tchr1:99-120\tPeaks\tchr1\t99\t120".to_string() );
        ex2.push( "chr1:450-500\tchr1:450-500\tPeaks\tchr1\t450\t500".to_string() );
        ex2.push( "chr1:600-660\tchr1:600-660\tPeaks\tchr1\t600\t660".to_string() );
        ex2.push( "chr1:1000-1010\tchr1:1000-1010\tPeaks\tchr1\t1000\t1010".to_string() );

        for id in 0..ofiles[0].len(){
            assert_eq!(ex1[id], ofiles[0][id]);
        }
        for id in 0..ofiles[1].len(){
            assert_eq!(ex2[id], ofiles[1][id]);
        }
        assert_eq!(ex1, ofiles[0]);
        assert_eq!(ex2, ofiles[1]);

    }

        #[test]
    fn check_parse_switch_files() { // chr1:30-35 is registered after chr1:52-70 and therefore changes positions in the outfile
        let test_data1 = vec![
            "chr1:10-20\tchr1:10-20\tPeaks\tchr1\t10\t20".to_string(),
            "chr1:30-35\tchr1:30-35\tPeaks\tchr1\t30\t35".to_string(),
            "chr1:55-60\tchr1:55-60\tPeaks\tchr1\t55\t60".to_string(),
            "chr1:65-70\tchr1:65-70\tPeaks\tchr1\t65\t70".to_string(),
            "chr1:99-110\tchr1:99-110\tPeaks\tchr1\t99\t110".to_string(),
            "chr1:600-620\tchr1:600-620\tPeaks\tchr1\t600\t620".to_string(),
            "chr1:2000-2010\tchr1:2000-2010\tPeaks\tchr1\t2000\t2010".to_string()
        ];
        let test_data2 = vec![
            "chr1:9-19\tchr1:9-19\tPeaks\tchr1\t9\t19".to_string(),
            "chr1:52-70\tchr1:52-70\tPeaks\tchr1\t52\t70".to_string(),
            "chr1:100-120\tchr1:100-120\tPeaks\tchr1\t100\t120".to_string(),
            "chr1:450-500\tchr1:450-500\tPeaks\tchr1\t450\t500".to_string(),
            "chr1:600-660\tchr1:600-660\tPeaks\tchr1\t600\t660".to_string(),
            "chr1:1000-1010\tchr1:1000-1010\tPeaks\tchr1\t1000\t1010".to_string()
        ];

        let mut ifiles= [ Ifile{ data: test_data2 }, Ifile{ data: test_data1 }];

        let files_n = 2;
        let mut still_data = true;
        let mut with_data: Vec<bool>= vec![ true; files_n ];
        let mut match_groups= Vec::<Feature>::with_capacity(100);
        let mut handled:bool;


        // the features need to be registered with the outfiles
        let mut ofiles_pos: Vec<Vec<usize>> = Vec::new();
        for id in 0..files_n{
            let mut file = Vec::<usize>::with_capacity(100000);
            ofiles_pos.push( file );
        }

        while still_data {
            for i in 0..files_n{

                if with_data[i]{
                    // parse the string into a feature

                    match &ifiles[i].get_line(){
                        Ok(text) => {
                            let feat = Feature::parse( text  );

                            handled = false;
                            for id in (0..match_groups.len()).rev() {
                            //for  match_group in &match_groups{
                                if match_groups[id].overlapps_adjust ( &feat ){
                                    ofiles_pos[i].push(id);
                                    //match_groups[id].register_write_to( i );
                                    handled = true;
                                    break;
                                }
                            }
                            if ! handled{
                                //match_groups.push( MatchGroup::new( &feat , i) );
                                match_groups.push(feat);
                                ofiles_pos[i].push( match_groups.len()-1);
                            }
                            
                            
                        },
                        Err(err) => {
                            eprintln!("file {i}: {err}");
                            with_data[i] = false;
                        },
                    }
                }
            }
            still_data = with_data.iter().any(|&x| x);
        }
        
        let mut ofiles: Vec<Vec<String>> = Vec::new();
        let mut file1 = Vec::<String>::with_capacity(20);
        let mut file2 = Vec::<String>::with_capacity(20);
        ofiles.push( file1 );
        ofiles.push( file2 );


        for i in 0..files_n{
            for id in &ofiles_pos[i]{
                ofiles[i].push( format!("{}", match_groups[*id]));
            }
            ofiles_pos[i].clear();
        }
        match_groups.clear();

        // for match_group in &match_groups{
        //     for id in &match_group.targets{
        //         ofiles[*id].push( format!("{}", match_group));
        //     }
        // }
        // match_groups.clear();

        // now lets  check what we got:

        let mut ex1 = Vec::<String>::with_capacity(20);
        let mut ex2 = Vec::<String>::with_capacity(20);
        ex1.push( "chr1:9-20\tchr1:9-20\tPeaks\tchr1\t9\t20".to_string() );
        ex1.push( "chr1:30-35\tchr1:30-35\tPeaks\tchr1\t30\t35".to_string() );
        ex1.push( "chr1:52-70\tchr1:52-70\tPeaks\tchr1\t52\t70".to_string() );
        ex1.push( "chr1:52-70\tchr1:52-70\tPeaks\tchr1\t52\t70".to_string() );
        ex1.push( "chr1:99-120\tchr1:99-120\tPeaks\tchr1\t99\t120".to_string() );
        ex1.push( "chr1:600-660\tchr1:600-660\tPeaks\tchr1\t600\t660".to_string() );
        ex1.push( "chr1:2000-2010\tchr1:2000-2010\tPeaks\tchr1\t2000\t2010".to_string() );

        ex2.push( "chr1:9-20\tchr1:9-20\tPeaks\tchr1\t9\t20".to_string() );
        ex2.push( "chr1:52-70\tchr1:52-70\tPeaks\tchr1\t52\t70".to_string() );
        ex2.push( "chr1:99-120\tchr1:99-120\tPeaks\tchr1\t99\t120".to_string() );
        ex2.push( "chr1:450-500\tchr1:450-500\tPeaks\tchr1\t450\t500".to_string() );
        ex2.push( "chr1:600-660\tchr1:600-660\tPeaks\tchr1\t600\t660".to_string() );
        ex2.push( "chr1:1000-1010\tchr1:1000-1010\tPeaks\tchr1\t1000\t1010".to_string() );

        for id in 0..ofiles[0].len(){
            assert_eq!(ex2[id], ofiles[0][id]);
        }
        for id in 0..ofiles[1].len(){
            assert_eq!(ex1[id], ofiles[1][id]);
        }
        assert_eq!(ex2, ofiles[0]);
        assert_eq!(ex1, ofiles[1]);

    }

}