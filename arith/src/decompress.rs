use arith::compute::*;
use Array2::Array2;
use csc411_image::*;
use csc411_rpegio;
use bitpack::bitpack::*;

pub fn decompress_read (input: Option::<String>) {

    //read in the input
    let (compressed_data, width, height) = csc411_rpegio::input_rpeg_data(input).unwrap();

    let mut vec2: Vec<video_pixel> = vec![];

    //32bit chunk iterator
    for word in compressed_data {
        let Y_vec = bitunpacking(word);
        //convert to 4 vid
        for i in 0..(Y_vec.len() - 2) {
            let pixel = video_pixel{y: Y_vec[i], pB: Y_vec[4], pR: Y_vec[5]};
            vec2.push(pixel);
        }
    }

    //make array2 for output
    let mut arr2_vid = Array2::<video_pixel>::single_val(width, height, video_pixel{y: 0.0, pB: 0.0, pR: 0.0});
    //get next chunk in arr2 & set pixels
    let mut count = 0;
    for _chunk in arr2_vid.get_chunks() {
        for i in 0..height/2{
            for j in 0..width/2{
                arr2_vid.set_square(j*2, i*2, vec2[count..count+3].to_vec());
                count += 4;
            }
        }
    }

    //vid 2 rgb float
    let arr2_rgbi = to_rgb(arr2_vid);

    //output arr2 rgb
    write(arr2_rgbi);
}

fn write(rot: Array2<imgtype::Rgb>) {
    let mut vec: Vec<imgtype::Rgb> = vec![];
    for pixel in rot.iter_row_major(){
        vec.push(pixel.2.clone());
    }

    let ppm = RgbImage{pixels: vec, width: rot.get_width() as u32, height: rot.get_height() as u32, denominator: 255};
    ppm.write(None).unwrap();
}

fn bitunpacking (word: [u8; 4]) -> Vec<f32> {

    //pull in opposite order
    let new_word = u32::from_be_bytes(word) as u64;

    let pR = getu(new_word, 4, 0) as usize;
    let pB = getu(new_word, 4, 4)as usize;
    let d = gets(new_word, 5, 8) as f32;
    let c = gets(new_word, 5, 13) as f32;
    let b = gets(new_word, 5, 18) as f32;
    let a = gets(new_word, 9, 23) as f32;

    csc411_arith::chroma_of_index(pB);
    csc411_arith::chroma_of_index(pR);

    let mut Ys = GetYs((a/511.0, b/103.33333, c/103.33333, d/103.33333));
    Ys.push(pB as f32);
    Ys.push(pR as f32);

    return Ys;
}