use crate::readFile::RgbFloat;
use Array2::Array2;
use csc411_image::*;
use crate::compute;

pub fn decompress (input: Option::<String>) {

    //read in the input
    let (compressed_data, width, height) = csc411_rpegio::input_rpeg_data(input).unwrap();

    //32bit chunk iterator

        //bit unpack
        //convert to 4 vid pixels

        //vec2.push(each pixel)

    //make array2 for output

    //get next chunk in arr2 & set pixels
        //vid 2 rgb float

}