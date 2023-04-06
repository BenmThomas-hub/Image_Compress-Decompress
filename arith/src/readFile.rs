use Array2::Array2;
use csc411_image::*;

#[derive(Clone)]

pub struct RgbFloat{
    red: f32,
    green: f32,
    blue: f32
}

pub fn read(input: Option<String>) -> Array2<RgbFloat> {
    let copy = input.clone();
    let img = RgbImage::read(copy.as_deref()).unwrap();

    let mut rgb_f32_vec: Vec<RgbFloat> = vec![];
    
    //converts RGB integers to RGB floats
    for pixel in img.pixels {
        let new_pix: RgbFloat = RgbFloat {red:(pixel.red as f32/255 as f32), green:(pixel.green as f32 / 255 as f32), blue:(pixel.blue as f32/ 255 as f32)};
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