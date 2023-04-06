use Array2::Array2;
use csc411_image::*;

pub struct video_pixel {
    pub y: f32,
    pub pB: f32,
    pub pR: f32
}


pub fn to_rgb(self) -> Array2<csc411_image::Rgb>{

}

pub fn to_video(self) -> Array2<video_pixel>{

}
fn RGBtoVideo(pixel: csc411_image::Rgb) -> video_pixel{
    let y: f32 = pixel.red*0.299 + 0.587*pixel.green + 0.114*pixel.blue;
    let pB: f32 = pixel.red*-0.168736 + -0.331264*pixel.green + 0.5*pixel.blue;
    let pR: f32 = pixel.red*0.5 + -0.418688*pixel.green + 0.081312*pixel.blue;
    return video_pixel(y=y, pB=pB, pR=pR);
}

fn VideotoRGB(pixel: video_pixel) -> csc411_image::Rgb{
    let red: u16 = pixel.y + 1.402*pixel.pR;
    let green: u16 = pixel.y + -0.344136*pixel.pB + -0.714136*pixel.pR;
    let blue: u16 = pixel.y + 1.772*pixel.pB;
    return csc411_image::Rgb(red=red, green=green, blue=blue);
}