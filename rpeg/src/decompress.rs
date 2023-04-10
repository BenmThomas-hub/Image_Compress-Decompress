use rpeg::compute::*;
use array2::Array2;
use csc411_image::*;
use csc411_rpegio;
use bitpack::bitpack::*;

pub fn decompress_read (input: Option::<String>) {

    //read in the input
    let (comp_ressed_data, width, height) = csc411_rpegio::input_rpeg_data(input).unwrap();

    let mut vec2: Vec<VideoPixel> = vec![];

    //32bit chunk iterator
    for word in comp_ressed_data {
        let y_vec = bitunpacking(word);
        //convert to 4 vid
        for i in 0..(y_vec.len() - 2) {
            let pixel = VideoPixel{y: y_vec[i], p_b: y_vec[4], p_r: y_vec[5]};
            vec2.push(pixel);
        }
    }

    //make Array2 for output
    let mut arr2_vid = Array2::<VideoPixel>::single_val(width, height, VideoPixel{y: 0.0, p_b: 0.0, p_r: 0.0});
    //get next chunk in arr2 & set pixels
    let mut count = 0;
    for i in 0..height/2{
        for j in 0..width/2{
            arr2_vid.set_square(j*2, i*2, vec2[count..count+3].to_vec());
            count += 4;
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

fn bitunpacking(word: [u8; 4]) -> Vec<f32> {
    let mut update_word: [u8; 4] = [0, 0, 0, 0];
    let mut count = 0;
    for i in word{
        update_word[count] = i.reverse_bits();
        count += 1;
    }
    //pull in opposite order
    let new_word = u32::from_be_bytes(update_word) as u64;

    let p_r = getu(new_word, 4, 0) as usize;
    let p_b = getu(new_word, 4, 4)as usize;
    let d = gets(new_word, 5, 8) as f32;
    let c = gets(new_word, 5, 13) as f32;
    let b = gets(new_word, 5, 18) as f32;
    let a = gets(new_word, 9, 23) as f32;

    csc411_arith::chroma_of_index(p_b);
    csc411_arith::chroma_of_index(p_r);

    let mut ys = get_ys((a/511.0, b/103.33333, c/103.33333, d/103.33333));
    ys.push(p_b as f32);
    ys.push(p_r as f32);

    return ys;
}