mod readFile;
use crate::readFile::read;
use clap::Parser;
use Array2::Array2;
use csc411_image::*;
//use csc411_arith::*;

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
    let fname = args.input;

    let arr2 = read(fname);

}