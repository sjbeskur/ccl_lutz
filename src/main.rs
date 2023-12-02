use std::fs::File;
use std::io::Write;

use ccl_lutz::*;

type AppError = Box<dyn std::error::Error>;

//static IMG: OnceLock<Image> = OnceLock::new();

fn main() -> Result<(), AppError> {

    let mut log = File::create("data/m100-10.log")?;
    let mut img = image::open("data/m100-10.png")?;
    let luma = img.to_luma8();

    let gray = Image::new(luma,100);

    for obj_pixels in lutz::lutz::<_, Vec<_>>(gray) {
        writeln!(log, "{} {:?}", obj_pixels.len(), obj_pixels)?;

        let mut min_x = u32::max_value();
        let mut min_y = u32::max_value();
        let mut max_x = 0;
        let mut max_y = 0;

        for pixel in obj_pixels {
            min_x = min_x.min(pixel.x);
            min_y = min_y.min(pixel.y);
            max_x = max_x.max(pixel.x);
            max_y = max_y.max(pixel.y);
        }

        let rect = imageproc::rect::Rect::at(min_x as i32, min_y as i32)
            .of_size(max_x - min_x + 1, max_y - min_y + 1);
        imageproc::drawing::draw_hollow_rect_mut(&mut img, rect, image::Rgba([255, 0, 0, 255]));
    }
    img.save("data/m100-10.out.png")?;
    Ok(())
}
