use pbr::ProgressBar;
use std::sync::{Arc, Mutex};
use std::time::Instant;

mod color;
mod data;
mod sets;

pub const IMAGE_SIZE: (usize, usize) = (4000, 2000);
pub const MAX_ITERATION: u64 = 1_000;

pub type Color = [u8; 4];

fn main() {
    let _pb = Arc::new(Mutex::new(ProgressBar::new((IMAGE_SIZE.0) as u64)));
    let now = Instant::now();

    let mandel = sets::Mandelbrot::default(IMAGE_SIZE.0, IMAGE_SIZE.1);
    let xy_map = mandel.get_color_map();

    let elapsed = now.elapsed();
    println!("Calculation took      {:.2?}", elapsed);
    let now = Instant::now();

    let transposed = data::transpose::xy_map(&xy_map);

    let elapsed = now.elapsed();
    println!("Transpose took        {:.2?}", elapsed);
    let now = Instant::now();

    //let pixmap = data::skia::draw_pixmap(&xy_map);
    let bin = data::png_crate::to_binary(&transposed);
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
