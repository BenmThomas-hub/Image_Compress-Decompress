mod readFile;
mod compress;
use crate::compress::*;
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

    let args = Args::parse();
    let decompress = args.d;
    let compress = args.c;
    let fname = args.input;

    let data = read(fname);

    if  decompress == true {
        //decomp function
    }
    else if  compress == true {
        let data = compress_read(fname);
        //yuh = compress(data);
    }

}