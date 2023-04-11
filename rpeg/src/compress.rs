use rpeg::compute::*;
use array2::Array2;
use csc411_image::*;
//use csc411_rpegio;
use bitpack::bitpack::*;
//use std::fs::File;
//use std::fs::write;

pub fn compress_read (input: Option<String>) -> (Vec<[u8; 4]>, usize, usize) {
    //read
    let img = read(input);
    //i2f and trim
    let arr2 = rgb_to_rgbf(img);

    //rgb2vid
    let vid_arr2 = to_video(arr2);

    //pack 2x2 into word
    let mut compressed_data: Vec<[u8; 4]> = Vec::new();
    for chunk in vid_arr2.get_chunks() {
        let coeff = get_coeff(chunk); //compute word & get coeff
        let bit = bitpacking(coeff);
        //println!("{:?}, {:?}", coeff, bit);
        compressed_data.push(bit);
    }
    
    //to keep width and height
    let width = vid_arr2.get_width();
    let height = vid_arr2.get_height();

    return (compressed_data, width, height);
}

fn read(input: Option<String>) -> csc411_image::RgbImage {
    let copy = input.clone();
    let img = RgbImage::read(copy.as_deref()).unwrap();

    return img;
}

fn rgb_to_rgbf(img: RgbImage) -> Array2<RgbFloat>{

    let mut rgb_f32_vec: Vec<RgbFloat> = vec![];
    
    //converts RGB integers to RGB floats
    for pixel in img.pixels {
        //println!("{}, {} , {}", pixel.red, pixel.green, pixel.blue);
        let new_pix: RgbFloat = RgbFloat{red:(pixel.red as f32 / 255.0), green:(pixel.green as f32 / 255.0), blue:(pixel.blue as f32 / 255.0)};
        rgb_f32_vec.push(new_pix);
    }

    let mut wdth = img.width as usize;
    let mut hght = img.height as usize;
    let mut arr2 = Array2::<RgbFloat>::from_row_major(wdth, hght, rgb_f32_vec);


    //Trims the arr2 if needed
    let mut count = 0;
    let mut end = wdth*hght;
    let mut vec2: Vec<RgbFloat> = vec![];

    //trim height
    if (img.height % 2) != 0{
        end -= wdth;

        //iterate & delete last row (OR make new arr2 with the specs)
        for pixel in arr2.iter_row_major() {
            if count == end {
                break;
            }
            else {
                vec2.push(pixel.2.clone());
            }
            count += 1;
        }
        hght -= 1;
        arr2 = Array2::<RgbFloat>::from_row_major(wdth, hght, vec2);
    }

    let mut vec2: Vec<RgbFloat> = vec![];
    let mut end = wdth*hght;
    count = 0;

    //trim width
    if (img.width % 2) != 0 {
        end -= hght;

        //iterate & delete last row (OR make new arr2 with the specs)
        for pixel in arr2.iter_row_major() {
            if count == end {
                break;
            }
            else {
                vec2.push(pixel.2.clone());
            }
            count += 1;
        }
        wdth -= 1;
        arr2 = Array2::<RgbFloat>::from_row_major(wdth, hght, vec2);
    }

    return arr2;
}

fn bitpacking (coeff: (f32, f32, f32, f32, usize, usize)) -> [u8; 4] {

    let mut word = 0 as u64;
    //println!("{}, {}, {}, {}, {}, {}", coeff.0, coeff.1, coeff.2, coeff.3, coeff.4, coeff.5);
    //println!("{},{}", coeff.4, coeff.5);
    //println!("{}, {}, {}, {}, {}, {}", coeff.0*511.0, coeff.1*50.0, coeff.2*50.0, coeff.3*50.0, coeff.4, coeff.5);
    let p_r = newu(word, 4, 0, coeff.5 as u64).unwrap();
    let p_b = newu(word, 4, 4, coeff.4 as u64).unwrap();
    let d = news(word, 5, 8, (coeff.3 * 50.0) as i64).unwrap();
    let c = news(word, 5, 13, (coeff.2 * 50.0) as i64).unwrap();
    let b = news(word, 5, 18, (coeff.1 * 50.0) as i64).unwrap();
    let a = newu(word, 9, 23, (coeff.0 * 511.0) as u64).unwrap();
    //println!("{:?}, {:?}, {:?}, {:?}, {:?}, {:?}", a, b, c, d, p_b, p_r);
    word = p_r | p_b | d | c | b | a;
    let bit: [u8; 4] = (word as u32).to_be_bytes();
    //println!("{}, {:?}", word, bit);
    return bit;
}