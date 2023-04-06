mod readFile;
use readFile::readFile;
use clap::Parser;
use array2::array2;
use csc411_image::*;
use csc411_arith;

mod readFile;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]


struct Args {
    input: Option<String>,

    #[clap(long = "d")]
    d: bool,
    
    #[clap(long = "c")]
    c: bool,

}

fn main() {


    /*
        command example (for now) : cargo run rotation90 row-major f_original.ppm rot_fin.ppm **
    */
    let args = Args::parse();
    let decompress = args.d;
    let compress = args.c;

    let arr2 = read(fname);

}