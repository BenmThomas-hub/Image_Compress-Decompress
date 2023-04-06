use Array2::Array2;
use csc411_image::*;

pub fn read(input: Option<String>) -> Array2<imgtype::Rgb> {
    let copy = input.clone();
    let img = RgbImage::read(copy.as_deref()).unwrap();

    let mut vec: Vec<imgtype::Rgb> = vec![];
    
    //checks whether or not the pixel values are between 1 and 9
    for pixel in img.pixels {
        vec.push(pixel);
    }

    let mut wdth = img.width as usize;
    let mut hght = img.height as usize;
    let mut arr2 = Array2::<imgtype::Rgb>::from_row_major(wdth, hght, vec);


    //Trims the arr2 if needed
    let mut count = 0;
    let mut end = wdth*hght;
    let mut vec2: Vec<imgtype::Rgb> = vec![];

    //trim height
    if (img.height % 2) != 0 {
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
        arr2 = Array2::<imgtype::Rgb>::from_row_major(wdth, hght, vec2);
    }

    let mut vec2: Vec<imgtype::Rgb> = vec![];
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
        arr2 = Array2::<imgtype::Rgb>::from_row_major(wdth, hght, vec2);
    }


    //integer to float
    /*
    let mut vec: Vec<imgtype::Rgb> = vec![];
    for pixel in arr2.iter_row_major() {
        let tup: [f64; 3] = pixel.2.into();
        vec.push(pixel.2.clone());
    }
    
    let arr2f = Array2::<imgtype::Rgb>::from_row_major(arr2.get_width(), arr2.get_height(), vec);
    */
    


    return arr2;

}