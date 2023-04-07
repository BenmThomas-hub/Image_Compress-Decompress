use Array2::Array2;
use csc411_image::*;
use csc411_arith::*;
#[derive(Clone)]

pub struct RgbFloat{
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

#[derive(Clone)]

pub struct video_pixel{
    pub y: f32,
    pub pB: f32,
    pub pR: f32
}

pub fn to_rgb(data: Array2::<video_pixel>) -> Array2<imgtype::Rgb>{
    let mut RgbArr: Vec<imgtype::Rgb> = vec![];
    for pixel in data.clone().iter_row_major(){
        RgbArr.push(VideotoRGB(pixel.2.clone()));
    }
    return Array2::from_row_major(data.get_width(), data.get_height(), RgbArr)
}

pub fn to_video(data: Array2::<RgbFloat>) -> Array2<video_pixel>{
    let mut VidArr: Vec<video_pixel> = vec![];
    for pixel in data.clone().iter_row_major(){
        VidArr.push(RGBtoVideo(pixel.2.clone()));
    }
    return Array2::from_row_major(data.get_width(), data.get_height(), VidArr)
}

fn RGBtoVideo(pixel: RgbFloat) -> video_pixel{
    let y: f32 = pixel.red*0.299 + 0.587*pixel.green + 0.114*pixel.blue;
    let pB: f32 = pixel.red*-0.168736 + -0.331264*pixel.green + 0.5*pixel.blue;
    let pR: f32 = pixel.red*0.5 + -0.418688*pixel.green + 0.081312*pixel.blue;
    return video_pixel{y:y, pB:pB, pR:pR};
}

fn VideotoRGB(pixel: video_pixel) -> imgtype::Rgb{
    let r: u16 = (pixel.y + 1.402*pixel.pR) as u16;
    let g: u16 = (pixel.y + -0.344136*pixel.pB + -0.714136*pixel.pR) as u16;
    let b: u16 = (pixel.y + 1.772*pixel.pB) as u16;
    return imgtype::Rgb{red:r, green:g, blue:b};
}

pub fn GetCoeff(pixels: Vec<video_pixel>) -> (f32, f32, f32, f32, usize, usize){
    let a: f32 = ((pixels[3].y + pixels[2].y + pixels[1].y + pixels[0].y)/4.0).clamp(0.0, 1.0);
    let b: f32 = ((pixels[3].y + pixels[2].y - pixels[1].y - pixels[0].y)/4.0).clamp(-0.3, 0.3);
    let c: f32 = ((pixels[3].y - pixels[2].y + pixels[1].y - pixels[0].y)/4.0).clamp(-0.3, 0.3);
    let d: f32 = ((pixels[3].y - pixels[2].y - pixels[1].y + pixels[0].y)/4.0).clamp(-0.3, 0.3);
    let pBavg: f32 = ((pixels[0].pB + pixels[1].pB + pixels[2].pB + pixels[3].pB)/4.0);
    let pRavg: f32 = ((pixels[0].pR + pixels[1].pR + pixels[2].pR + pixels[3].pR)/4.0);
    return (a,b,c,d, csc411_arith::index_of_chroma(pBavg), csc411_arith::index_of_chroma(pRavg))
}

pub fn GetYs(coefficients: (f32, f32, f32, f32)) -> Vec<f32>{
    let y1: f32 = (coefficients.0 - coefficients.1 - coefficients.2 + coefficients.3);
    let y2: f32 = (coefficients.0 - coefficients.1 + coefficients.2 - coefficients.3);
    let y3: f32 = (coefficients.0 + coefficients.1 - coefficients.2 - coefficients.3);
    let y4: f32 = (coefficients.0 + coefficients.1 + coefficients.2 + coefficients.3);
    return vec![y1, y2, y3, y4];
}