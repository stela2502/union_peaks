
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use union_peaks::ifile::Ifiles;
use union_peaks::ofile::Ofiles;

use clap::Parser;

/// If you have multiple samples of 10x atac data you are screqed
/// I have not found any option to 'aligne' the peaks over multiple samples.
/// This leads to the data not being comparable between samples.
/// To counter this problem this package creates unions of overlapping peaks over multiple 10 atac datasets.
/// All files are meant to be sorted in the same way.

#[derive(Parser)]
#[clap(version = "1.0.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the paths to the featuretsv.gz files (comma separated string)
    #[clap(default_value= "testData/A,testData/B",short, long)]
    paths: String,
    /// the outpath
    #[clap(default_value=  "testData/Output",short, long)]
    outpath: String,
}


fn main() {
    
    
}


