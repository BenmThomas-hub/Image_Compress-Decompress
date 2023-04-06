mod compute;
use crate::compute::*;
use arith::compute::to_video;
use crate::readFile::RgbFloat;
use Array2::Array2;
use csc411_image::*;

pub fn compress (arr2: Array2::<RgbFloat>) {

    //rgb2vid
    let vid_arr2 = to_video(arr2);

    //pack 2x2 into word

        //get chunk

        //compute word & get coeff

    //bitpack

    //set index


}