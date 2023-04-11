use rpeg::compute::*;
use array2::Array2;
use csc411_image::*;
use bitpack::bitpack::*;

// Decompresses the 32-bit words into a full ppm image file
pub fn decompress_read (compressed_data: Vec<[u8; 4]>, width: usize, height: usize) {

    let width = width;
    let height = height;

    let mut vec2: Vec<VideoPixel> = vec![];

    //32bit chunk iterator
    for word in compressed_data {
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
            if count != vec2.len(){
                arr2_vid.set_square(j*2, i*2, vec2[count..count+4].to_vec());
                count += 4;
            }
        }
    }

    //vid 2 rgb float
    let arr2_rgbi = to_rgb(arr2_vid);

    //output arr2 rgb
    write(arr2_rgbi);
}

// Prints the ppm file by taking in an Array2 of the RGB pixel data
fn write(out: Array2<imgtype::Rgb>) {
    let mut vec: Vec<imgtype::Rgb> = vec![];
    for pixel in out.iter_row_major(){
        vec.push(pixel.2.clone());
    }

    let ppm = RgbImage{pixels: vec, width: out.get_width() as u32, height: out.get_height() as u32, denominator: 255};

    ppm.write(None).unwrap();
}

// Converts 32-bit words into the corresponding pixels Y values for each of the four grouped pixewls as well as their pB and pR averages
fn bitunpacking(word: [u8; 4]) -> Vec<f32> {
    // Sets new word to the u64 value of the bytes stored
    let new_word = u32::from_be_bytes(word) as u64;
    // Converts each set of bytes from random set of numbers to values for all coefficients and pB and pR
    let p_r = getu(new_word, 4, 0) as usize;
    let p_b = getu(new_word, 4, 4)as usize;
    let d = gets(new_word, 5, 8) as f32;
    let c = gets(new_word, 5, 13) as f32;
    let b = gets(new_word, 5, 18) as f32;
    let a = getu(new_word, 9, 23) as f32;
    // Converts pB and pR indexes to actual values
    let p_b_index = csc411_arith::chroma_of_index(p_b);
    let p_r_index = csc411_arith::chroma_of_index(p_r);
    //Converts coefficients to y values for each pixel then pushes them to an arrau along with pB and pR
    let mut ys = get_ys((a/511.0, b/103.333, c/103.333, d/103.333));
    ys.push(p_b_index as f32);
    ys.push(p_r_index as f32);

    return ys;
}