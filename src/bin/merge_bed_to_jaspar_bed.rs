
use union_peaks::ifile::Ifile;
use union_peaks::ofile::Ofile;
use union_peaks::feature::Feature;
//use union_peaks::matchgroup::MatchGroup;
use std::time::SystemTime;

use indicatif::ProgressStyle;
use indicatif::MultiProgress;
use indicatif::ProgressBar;

use clap::Parser;
use std::io::Write;


/// This script is used to annotated 10x atac data using the jaspar database.
/// The jaspar database is simply a bed file in this case. BibBed files are not supported up to now.
/// The program expects the bed files to be sorted - otherwise it will fail!


#[derive(Parser)]
#[clap(version = "1.0.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the path to the bed file to be matched to 
    #[clap(default_value= "testData/A.bed",short, long)]
    a: String,
    /// the path to the bed file to be matched to 
    #[clap(default_value= "testData/B.bed",short, long)]
    b: String,
    /// the outfile of the merge
    #[clap(default_value=  "testData/Output_bed/res.bed.gz",short, long)]
    o: String,
    /// the cutoff for the JASPR TF binding patterns (default 500) 
    #[clap(default_value_t= 500,short, long)]
    cutoff: usize,
}


fn main() {

    let now = SystemTime::now();
    let opts: Opts = Opts::parse();

    //fs::create_dir_all(&opts.outpath).expect("AlreadyExists");

    let mut ifiles = Vec::<Ifile>::with_capacity( 2 );
    let mut ofiles = Vec::<Ofile>::with_capacity( 1 );
    ifiles.push( Ifile::new( &opts.a ));
	ifiles.push( Ifile::new( &opts.b ));

	ofiles.push( Ofile::new_file( &opts.o ));

    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new(1000));
    pb.set_style(spinner_style);

    let mut id =0;

    let mut still_data = true;

    let mut other:Feature = match &ifiles[1].get_line() {
    	Ok(text) => {
			Feature::parse_bed( text  )
		},
		Err(err) => {
    		eprintln!("b 2: {err}");
			still_data = false;
			Feature::blank()
		}
	};
    while match &ifiles[0].get_line() {
    	Ok(text) => {
	        id += 1;
	        if id % 1000 ==0{
	            pb.inc(1);
	        }
            let mut feat = Feature::parse_bed( text  );

            // get rid of all other elements that lie before our entry
           	while still_data && other.before( &feat ) {
            	other = match &ifiles[1].get_line() {
	            	Ok(text) => {
            			Feature::parse_bed( text  )
            		},
            		Err(err) => {
	            		eprintln!("file b 2: {err}");
            			still_data = false;
            			Feature::blank()
        			}
        		};
        	}
        	// collect the matching ones
        	while still_data && other.overlaps( &feat ) {
        		// here we collect the info that should be added to this feature.
        		if &other.var > &opts.cutoff{
        			feat.push( format!("{}/{}/{}", other.name, other.name2, other.var ) );
        		}

            	other = match &ifiles[1].get_line() {
	            	Ok(text) => {
            			Feature::parse_bed( text  )
            		},
            		Err(err) => {
	            		eprintln!("file b 2: {err}");
            			still_data = false;
            			Feature::blank()
        			}
        		};
        	}

        	// now we collected all matching entried for this feature - print it
        	match writeln!( ofiles[0].buff1, "{}", feat.to_bed() ){
                Ok(_) => (),
                Err(err) => panic!( "I could not write the data to outfile:\n{err}" ),
            };
        	true
        },
        Err(err) => {
           	eprintln!("file a: {err}");
           	false
        }
    }{} // there is nothing else to do here.

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

            println!("merge_bed_to_jasper_bed finished in {milli}h {min}min {sec} sec {mil}milli sec\n" );},
       Err(e) => {println!("Error: {e:?}");}
    }
}



