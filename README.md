[![Rust](https://github.com/stela2502/union_peaks/actions/workflows/rust.yml/badge.svg)](https://github.com/stela2502/union_peaks/actions/workflows/rust.yml)

# union_peaks

This small tool 'normalizes' the peak information from multiple 10x atac seq runs.


The problem is that iof Cellranger is run separately for two or more eperiments the peaks will show small bp changes. This makes the dowstream analysis using e.g. episcanpy impossible as the same peak will have two different 'names' in the analysis.

This tool tries to change that by converting the peak positions to the union peak for overlapping peaks. This will give all smaller peaks that fall into one bigger peak in the other sample the same id. Therefore later in the analysis all ids with the same name should be summed up.

## Install

You need Rust to compile the program. You can find the necessary info here: https://www.rust-lang.org/tools/install.

After that you can clone this package and copmpile the program like that:

```
git clone git@github.com:stela2502/union_peaks.git
cd union_peaks
cargo build -r
cp target/release/merge_bed_to_jaspar_bed /usr/bin
cp target/release/union_peaks /usr/bin

```

The finished program will be there: ``target/release/union_peaks``. Copy it to your path to use it.

## Usage

The tool will change the peak information in the CellRanger feature.tsv.gz files.

```
.\target\release\union_peaks -h
union_peaks 1.0.0
Stefan L. <stefan.lang@med.lu.se>
If you have multiple samples of 10x atac data you might end up with uncomparable results. I have not
found any option to 'aligne' the peaks over multiple samples. This leads to the data not being
comparable between samples. To counter this problem this program creates unions of overlapping peaks
over multiple 10x atac datasets

USAGE:
    union_peaks.exe [OPTIONS]

OPTIONS:
    -h, --help                 Print help information
    -o, --outpath <OUTPATH>    the outpath (Peak positions will be changed) [default:
                               testData/Output]
    -p, --paths <PATHS>        the paths to the feature.tsv.gz files (one comma separated string)
                               [default: testData/B.tsv.gz,testData/C.tsv.gz]
    -V, --version              Print version information
```


## Ideas of how to publish this?

    
Would it be possible to compare GSE209878 to GSM6176366? This would be kind of interesting.
Especially as GSM6176366 also provides a csv file that I could convert using my RustySparseMMX program.

