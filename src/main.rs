use rayon::prelude::*;
use tiny_skia::*;

use crate::color::ToBytes;

mod color;
mod sets;

const IMAGE_SIZE: (u32, u32) = (4000, 2000);
const MAX_ITERATION: u64 = 1_000;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

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

    let elapsed = now.elapsed();
    println!("Calculation took      {:.2?}", elapsed);

    let now = Instant::now();

    let mut paint = Paint::default();
    let mut pixmap = Pixmap::new(IMAGE_SIZE.0, IMAGE_SIZE.1).unwrap();
    //println!("{:?}", pixmap.data().into_iter().count());
    //println!("{:?}", (IMAGE_SIZE.0 * IMAGE_SIZE.1 * 4));

    //data = [R, G, B, A] bytes...
    //let mut data: Vec<u8> = Vec::with_capacity(cap);
    let data_size = (IMAGE_SIZE.0 * IMAGE_SIZE.1 * 4) as usize;
    let mut data: Vec<u8> = Vec::with_capacity(data_size);
    println!("Configured size  {}", data_size);

    for (_x, y_vec) in map {
        for (_y, color) in y_vec {
            //Get bytes
            let mut bytes = color.to_vec();
            //println!("{:?}", bytes);

            data.append(&mut bytes);
        }
    }

    println!("Actual data size {}", data.len());

    let mut pixmap =
        PixmapMut::from_bytes(&mut data[0..data_size], IMAGE_SIZE.0, IMAGE_SIZE.1).unwrap();
    let cap = pixmap.data_mut().into_iter().count();
    println!("Pixmap size      {}", cap);

    //let pixmap = Pixmap::from_vec(&mut data).unwrap();

    /*
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
     */

    let elapsed = now.elapsed();
    println!("Drawing took          {:.2?}", elapsed);

    let now = Instant::now();

    pixmap.to_owned().save_png("image.png").unwrap();
    let elapsed = now.elapsed();
    println!("Writing took          {:.2?}", elapsed);
}
