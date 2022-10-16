use image::GenericImageView;
use std::io::stdout;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
};

fn main() {
    // Read the image and process dimensions
    let img = image::open("/home/talison/projects/rust/images/examples/cat.jpeg").unwrap();
    let (width, height) = img.dimensions();

    for i in 0..height {
        for j in 0..width {
            let pixel = img.get_pixel(i, j);
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
