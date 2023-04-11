use array2::Array2;
use csc411_image::*;
#[derive(Clone)]

// Struct of RGB values converted from integers to float values
pub struct RgbFloat{
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

#[derive(Clone)]

// Struct of Pixel with values for component video
pub struct VideoPixel{
    pub y: f32,
    pub p_b: f32,
    pub p_r: f32
}

// Converts an Array2 of Video Pixels to an Array2 of the same dimensions of RGB pixels
pub fn to_rgb(data: Array2::<VideoPixel>) -> Array2<imgtype::Rgb>{
    let mut rgb_arr: Vec<imgtype::Rgb> = vec![];
    for pixel in data.clone().iter_row_major(){
        rgb_arr.push(videoto_rgb(pixel.2.clone()));
    }
    return Array2::from_row_major(data.get_width(), data.get_height(), rgb_arr)
}

// Converts an Array2 of RGB floats to an Array2 of the same dimesions of Video Pixels
pub fn to_video(data: Array2::<RgbFloat>) -> Array2<VideoPixel>{
    let mut vid_arr: Vec<VideoPixel> = vec![];
    for pixel in data.clone().iter_row_major(){
        vid_arr.push(rgbto_video(pixel.2.clone()));
    }
    return Array2::from_row_major(data.get_width(), data.get_height(), vid_arr)
}

// Converts a single pixel from it's RGB float values to the y, pB, and pR values of a Video pixel and returns it as a struct
fn rgbto_video(pixel: RgbFloat) -> VideoPixel{
    let y: f32 = pixel.red*0.299 + 0.587*pixel.green + 0.114*pixel.blue;
    let p_b: f32 = pixel.red*-0.168736 + -0.331264*pixel.green + 0.5*pixel.blue;
    let p_r: f32 = pixel.red*0.5 + -0.418688*pixel.green + 0.081312*pixel.blue;
    
    return VideoPixel{y, p_b, p_r};
}

// Converts a single pixel from it's video component values to it's RGB values then returns it as a RGB struct
fn videoto_rgb(pixel: VideoPixel) -> imgtype::Rgb{
    let r: u16 = ((pixel.y + 1.402*pixel.p_r)*255.0) as u16;
    let g: u16 = ((pixel.y + -0.344136*pixel.p_b + -0.714136*pixel.p_r) * 255.0) as u16;
    let b: u16 = ((pixel.y + 1.772*pixel.p_b) * 255.0) as u16;

    return imgtype::Rgb{red:r, green:g, blue:b};
}

// Uses the Y values of four neighboring pixels to calculate the a, b, c, and d coefficients as well as the average pB and pR values
pub fn get_coeff(pixels: Vec<VideoPixel>) -> (f32, f32, f32, f32, usize, usize){
    let a: f32 = ((pixels[3].y + pixels[2].y + pixels[1].y + pixels[0].y)/4.0_f32).clamp(0.0, 1.0);
    let b: f32 = ((pixels[3].y + pixels[2].y - pixels[1].y - pixels[0].y)/4.0_f32).clamp(-0.3, 0.3);
    let c: f32 = ((pixels[3].y - pixels[2].y + pixels[1].y - pixels[0].y)/4.0_f32).clamp(-0.3, 0.3);
    let d: f32 = ((pixels[3].y - pixels[2].y - pixels[1].y + pixels[0].y)/4.0_f32).clamp(-0.3, 0.3);
    let pb_avg: f32 = (pixels[0].p_b + pixels[1].p_b + pixels[2].p_b + pixels[3].p_b)/4.0;
    let pr_avg: f32 = (pixels[0].p_r + pixels[1].p_r + pixels[2].p_r + pixels[3].p_r)/4.0;

    return (a,b,c,d, csc411_arith::index_of_chroma(pb_avg), csc411_arith::index_of_chroma(pr_avg));
}

// Uses the a, b, c, and d coefficients to calculate the Y values for each of the 4 video pixels
pub fn get_ys(coefficients: (f32, f32, f32, f32)) -> Vec<f32>{

    let y1: f32 = coefficients.0 - coefficients.1 - coefficients.2 + coefficients.3;
    let y2: f32 = coefficients.0 - coefficients.1 + coefficients.2 - coefficients.3;
    let y3: f32 = coefficients.0 + coefficients.1 - coefficients.2 - coefficients.3;
    let y4: f32 = coefficients.0 + coefficients.1 + coefficients.2 + coefficients.3;

    return vec![y1, y2, y3, y4];
}