use arith::compute::*;
use Array2::Array2;
use csc411_image::*;
use csc411_rpegio;
use bitpack::bitpack::*;


pub fn compress_read (input: Option<String>) {
    //read
    let img = read(input);
    //i2f and trim
    let arr2 = rgb_to_rgbf(img);

    //rgb2vid
    let vid_arr2 = to_video(arr2);

    //pack 2x2 into word
    let mut compressed_data: Vec<[u8; 4]> = vec![];
    for chunk in vid_arr2.get_chunks() {
        let coeff = GetCoeff(chunk);//compute word & get coeff
        let bit = bitpacking(coeff);
        compressed_data.push(bit);
    }
    
    //to keep width and height
    let width = vid_arr2.get_width();
    let height = vid_arr2.get_height();

    csc411_rpegio::output_rpeg_data(&compressed_data, width, height).unwrap();
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
        let new_pix: RgbFloat = RgbFloat{red:(pixel.red as f32/255 as f32), green:(pixel.green as f32 / 255 as f32), blue:(pixel.blue as f32/ 255 as f32)};
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

    let pBin = csc411_arith::index_of_chroma(coeff.4 as f32) as f32;
    let pRin = csc411_arith::index_of_chroma(coeff.5 as f32) as f32;
    
    let pR = newu(word, 4, 0, pRin as u64);
    let pB = newu(word, 4, 4, pBin as u64);
    let d = news(word, 5, 8, (coeff.3 * 103.33333) as i64);
    let c = news(word, 5, 13, (coeff.2 * 103.33333) as i64);
    let b = news(word, 5, 18, (coeff.1 * 103.33333) as i64);
    let a = newu(word, 9, 23, (coeff.0*511.0) as u64);

    word = pR.unwrap()+pB.unwrap()+d.unwrap()+c.unwrap()+b.unwrap()+a.unwrap();

    let bit: [u8; 4] = (word as u32).to_be_bytes();

    return bit;
}