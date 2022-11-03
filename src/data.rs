// For reading and opening files

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::color;
use crate::sets;
use crate::{Color, IMAGE_SIZE};

#[allow(dead_code)]
pub mod single {
    const DATA_SIZE: usize = IMAGE_SIZE.0 * IMAGE_SIZE.1 * 4;

    use super::*;

    pub fn save_file(path: &str, data: &[u8]) {
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, IMAGE_SIZE.0 as u32, IMAGE_SIZE.1 as u32); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        //let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
        //let data = [255, 0, 0, 255, 0, 0, 0, 255];
        writer.write_image_data(data).unwrap(); // Save
    }

    #[allow(dead_code)]
    pub fn to_binary(yx_map: &Vec<Vec<Color>>) -> Box<[u8; DATA_SIZE]> {
        //Without multithreading

        let column_count = yx_map.len();
        let row_length = yx_map[0].len();

        let mut data = Box::new([0_u8; DATA_SIZE]);

        for y in 0..column_count {
            for x in 0..row_length {
                //Get bytes
                let bytes = yx_map[y][x];
                let pos = (x + (y * row_length)) * 4;

                data[pos] = bytes[0];
                data[pos + 1] = bytes[1];
                data[pos + 2] = bytes[2];
                //Alpha channel is always 25
                data[pos + 3] = 255;
            }
        }

        data
    }

    use pbr::ProgressBar;
    use rayon::prelude::*;
    use std::sync::{Arc, Mutex};

    pub fn collect_color_map() -> Vec<Vec<Color>> {
        let pb = Arc::new(Mutex::new(ProgressBar::new((IMAGE_SIZE.1) as u64)));
        let y_range = 0..IMAGE_SIZE.1;

        y_range
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
            .collect()
    }
}

mod chunks {}
