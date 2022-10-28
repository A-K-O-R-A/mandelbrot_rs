use rayon::prelude::*;
use tiny_skia::*;

mod color;
mod sets;

const IMAGE_SIZE: (u32, u32) = (4000, 2000);
const MAX_ITERATION: u64 = 1_000;

fn main() {
    let mut paint = Paint::default();
    let mut pixmap = Pixmap::new(IMAGE_SIZE.0, IMAGE_SIZE.1).unwrap();

    let x_range = 0..=IMAGE_SIZE.0;
    let map = x_range
        .into_par_iter()
        .map(|x| {
            let y_range = 0..=IMAGE_SIZE.1;
            (
                x,
                y_range
                    .into_par_iter()
                    .map(move |y| {
                        //Get iteration count
                        let iter = sets::mandelbrot::get_pixel(x as f64, y as f64);

                        (y, color::from_iterations(iter))
                    })
                    .collect::<Vec<(u32, Color)>>(),
            )
        })
        .collect::<Vec<(u32, Vec<(u32, Color)>)>>();

    for (x, y_vec) in map {
        for (y, color) in y_vec {
            //Create single pixel as rect
            let rect = Rect::from_xywh(x as f32, y as f32, 1., 1.).expect("Couldn't create rect");

            //Change color
            paint.shader = Shader::SolidColor(color);

            //paint pixel
            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
        }
    }

    pixmap.save_png("image.png").unwrap();
}
