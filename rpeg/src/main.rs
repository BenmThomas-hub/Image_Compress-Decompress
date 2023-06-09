mod compress;
use crate::compress::*;
use clap::Parser;
mod decompress;
use crate::decompress::*;
use csc411_rpegio;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]


struct Args {
    input: Option<String>,

    #[clap(short, long)]
    decompress: bool,
    
    #[clap(short, long)]
    compress: bool,

}

fn main() {

    let args = Args::parse();
    let decompress = args.decompress;
    let compress = args.compress;
    let fname = args.input;

    //begins either the compress or decompress operations
    if  decompress == true {
        let (compressed_data, width, height) = csc411_rpegio::input_rpeg_data(fname).unwrap();
        decompress_read(compressed_data, width, height);
    }
    else if  compress == true {
        let (output, width, height) = compress_read(fname);
        csc411_rpegio::output_rpeg_data(&output, width, height).unwrap();
    }
}