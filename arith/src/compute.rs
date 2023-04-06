use Array2::Array2;
use csc411_image::*;
use arith::readFile::RgbFloat;

pub struct video_pixel {
    pub y: f32,
    pub pB: f32,
    pub pR: f32
}

pub fn to_rgb(data: Array2::<video_pixel>) -> Array2<RgbFloat>{
    let RgbArr: Vec<RgbFloat> = vec![];
    for pixel in data{
        RgbArr.push(VideotoRGB(pixel));
    }
    return Array2::from_row_major(data.get_width, data.get_height, RgbArr)
}

pub fn to_video(data: Array2::<RgbFloat>) -> Array2<video_pixel>{
    let VidArr: Vec<video_pixel> = vec![];
    for pixel in data{
        VidArr.push(RGBtoVideo(pixel));
    }
    return Array2::from_row_major(data.get_width, data.get_height, VidArr)
}
fn RGBtoVideo(pixel: RgbFloat) -> video_pixel{
    let y: f32 = pixel.red*0.299 + 0.587*pixel.green + 0.114*pixel.blue;
    let pB: f32 = pixel.red*-0.168736 + -0.331264*pixel.green + 0.5*pixel.blue;
    let pR: f32 = pixel.red*0.5 + -0.418688*pixel.green + 0.081312*pixel.blue;
    return video_pixel{y:y, pB:pB, pR:pR};
}

fn VideotoRGB(pixel: video_pixel) -> RgbFloat{
    let r: f32 = pixel.y + 1.402*pixel.pR;
    let g: f32 = pixel.y + -0.344136*pixel.pB + -0.714136*pixel.pR;
    let b: f32 = pixel.y + 1.772*pixel.pB;
    return RgbFloat{red:r, green:g, blue:b};
}

pub fn GetCoeff(iterateor: Iterator<Vec<Item = video_pixel>>) -> {
    let pixels = iterator.collect();
}