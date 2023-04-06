use array2::Array2;
use csc411_image::*;

pub fn read(input: Option<String>) -> Array2<imgtype::Rgb> {
    let copy = input.clone();
    let img = RgbImage::read(copy.as_deref()).unwrap();

    let mut vec: Vec<imgtype::Rgb> = vec![];
    
    //checks whether or not the pixel values are between 1 and 9
    for pixel in img.pixels {
        vec.push(pixel);
    }

    //creates an arr2 of the data taken in from the .pgm file
    let mut wdth = img.widt as usize;
    let mut hght = img.height as usize;
    let mut arr2 = Array2::<imgtype::Rgb>::from_row_major(wdth, hght, vec);


    //Trims the arr2 if needed
    let mut vec: Vec<imgtype::Rgb> = vec![];
    if (img.height % 2) != 0 {

        //iterate & delete last row (OR make new arr2 with the specs)
        for pixel in arr2.iter_row_major() - img.width() {
            vec.push(pixel);
        }
        hght -= 1;
        arr2 = Array2::<imgtype::Rgb>::from_row_major(wdth, hght, vec);

    }
    let mut vec: Vec<imgtype::Rgb> = vec![];
    if (img.width % 2) != 0 {

        //iterate & delete last row (OR make new arr2 with the specs)
        for pixel in arr2.iter_column_major() - img.width() {
            vec.push(pixel);
        }
        wdth -= 1;
        arr2 = Array2::<imgtype::Rgb>::from_column_major(wdth, hght, vec);

    }


    //integer to float
    //let mut vec: Vec<imgtype::Rgb> = vec![];
    //for pixel in arr2.iter_row_major() {
    //    let float_pixel = pixel / (255,255,255);
    //    vec.push(float_pixel);
    //}

    return arr2;

}