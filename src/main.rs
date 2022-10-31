use pbr::ProgressBar;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

mod color;
mod data;
mod sets;

pub const IMAGE_SIZE: (usize, usize) = (4000, 2000);
pub const MAX_ITERATION: u64 = 1_000;

pub type Color = [u8; 4];

fn main() {
    let pb = Arc::new(Mutex::new(ProgressBar::new((IMAGE_SIZE.0) as u64)));
    let now = Instant::now();

    let y_range = 0..IMAGE_SIZE.1;
    let yx_map: Vec<Vec<Color>> = y_range
        .into_par_iter()
        .map(move |y| {
            let x_range = 0..IMAGE_SIZE.0;

            let colors = x_range
                .into_par_iter()
                .map(|x| {
                    //Get iteration count
                    let iter = sets::mandelbrot::get_pixel(x as f64, y as f64);

                    color::from_iterations(iter)
                })
                .collect();

            //Reduces speed a bit
            pb.lock().unwrap().inc();

            colors
        })
        .collect();

    let elapsed = now.elapsed();
    println!("Calculation took      {:.2?}", elapsed);
    let now = Instant::now();

    //let pixmap = data::skia::draw_pixmap(&xy_map);
    let bin = data::png_crate::to_binary(&yx_map);
    //let raster = data::png_pong_crate::to_raster(&transposed);

    let elapsed = now.elapsed();
    println!("Drawing/Converting    {:.2?}", elapsed);
    let now = Instant::now();

    //data::skia::save_file(&pixmap);
    data::png_crate::save_file(&bin[..]);
    //data::png_pong_crate::save_file(raster);

    let elapsed = now.elapsed();
    println!("Encoding/Writing      {:.2?}", elapsed);
}
