use rayon::prelude::*;
use tiny_skia::*;

mod color;
mod data;
mod sets;

const IMAGE_SIZE: (u32, u32) = (2000, 1000);
const MAX_ITERATION: u64 = 1_000;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let y_range = 0..=IMAGE_SIZE.1;
    let yx_map = y_range
        .into_par_iter()
        .map(|y| {
            let x_range = 0..=IMAGE_SIZE.0;

            x_range
                .into_par_iter()
                .map(move |x| {
                    //Get iteration count
                    let iter = sets::mandelbrot::get_pixel(x as f64, y as f64);

                    color::from_iterations(iter)
                })
                .collect::<Vec<Color>>()
        })
        .collect::<Vec<Vec<Color>>>();

    let elapsed = now.elapsed();
    println!("Calculation took      {:.2?}", elapsed);

    let now = Instant::now();

    //let pixmap = data::skia::draw_pixmap(&map);
    let bin = data::png_crate::to_binary(&yx_map);
    //let raster = data::png_pong_crate::to_raster(&map);

    let elapsed = now.elapsed();
    println!("Drawing took          {:.2?}", elapsed);

    let now = Instant::now();

    //data::skia::save_file(&pixmap);
    data::png_crate::save_file(&bin[..]);
    //data::png_pong_crate::save_file(raster);

    let elapsed = now.elapsed();
    println!("Writing took          {:.2?}", elapsed);
}
