use image::*;

pub struct Image{
    pub img: image::GrayImage,
    pub threshold: u8,
}

impl Image {
    pub fn new(img: image::GrayImage, threshold: u8) -> Self { Self { img, threshold } }
}

impl lutz::Image for Image{
    fn width(&self) -> u32 {
        self.img.width()
    }

    fn height(&self) -> u32 {
        self.img.height()
    }

    fn has_pixel(&self,x:u32,y:u32) -> bool {
        self.img.get_pixel(x,y).0[0] > self.threshold
    }
}