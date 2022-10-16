use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
};
use image::GenericImageView;
use std::io::stdout;
use std::path::Path;

fn main() -> Result<(), ()> {
    let path = Path::new("./examples/cat.jpeg");

    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();

    for i in 0..height {
        for j in 0..width {
            let pixel = img.get_pixel(j, i);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let c = Color::from((r, g, b));
            execute!(
                stdout(),
                SetBackgroundColor(c),
                Print(" ".to_string()),
                ResetColor
            )
            .unwrap();
        }
        print!(" \n");
    }

    Ok(())
}
