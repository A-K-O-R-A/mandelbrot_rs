use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::ops::Range;

use pbr::ProgressBar;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use crate::color;
use crate::sets;
use crate::{Color, ROWS_PER_CHUNK, SIZE};

#[allow(dead_code)]
pub mod chunked {
    use super::*;

    pub const DATA_SIZE_RGB: usize = SIZE.0 * SIZE.1 * 3;
    //About 10GB = 10 * 1024 KB = 10 * 1024 * 1024;
    //Each chunk should contain a natural number of rows
    pub const CHUNK_SIZE_RGB: usize = SIZE.0 * ROWS_PER_CHUNK * 3;
    pub const CHUNK_COUNT: usize = DATA_SIZE_RGB / CHUNK_SIZE_RGB;

    pub fn check_size() {
        if CHUNK_COUNT * CHUNK_SIZE_RGB != DATA_SIZE_RGB {
            eprintln!(
                "{} chunks with {} bytes each don' fit into {} bytes",
                CHUNK_COUNT, CHUNK_SIZE_RGB, DATA_SIZE_RGB
            );
            assert_eq!(CHUNK_COUNT * CHUNK_SIZE_RGB, DATA_SIZE_RGB);
        }

        if (CHUNK_SIZE_RGB as f32 / SIZE.0 as f32) % 1. != 0. {
            eprintln!(
                "{} rows don't perfectly fit in all {} bytes of a chunk",
                CHUNK_SIZE_RGB / SIZE.0,
                CHUNK_SIZE_RGB,
            );
            assert_eq!(CHUNK_COUNT * CHUNK_SIZE_RGB, DATA_SIZE_RGB);
        }
    }

    /*
    let output = Arc::new([0_u8; CHUNK_SIZE]);

    for t in 0..4 {
        let cloned = output.clone();


        let p = (cloned.as_ptr() as *mut u32).offset(t as isize)
    }
    */
    ///Generate chunk withs a specifies row range (max is 0..SIZE.1)
    pub fn generate_chunk(row_range: Range<usize>) -> Arc<Box<[u8; CHUNK_SIZE_RGB]>> {
        let pb = Arc::new(Mutex::new(ProgressBar::new(
            (row_range.end - row_range.start) as u64,
        )));
        pb.lock().unwrap().message("Row ");

        let data = Arc::new(Box::new([0_u8; CHUNK_SIZE_RGB]));
        let index_range = (row_range.start * SIZE.0)..(row_range.end * SIZE.0);
        let first_index = index_range.start;

        let cloned = data.clone();

        row_range.into_par_iter().for_each(|y| {
            (0..SIZE.0).into_par_iter().for_each(|x| {
                //let x = pixel_i % SIZE.0;
                //let y = pixel_i / SIZE.1;

                //Get iteration count
                let iteration_count = sets::mandelbrot::get_pixel(x as f64, y as f64);
                let c = color::from_iterations(iteration_count);

                //let cloned = data.clone();
                let data_index = (x + (y * SIZE.0)) - first_index;

                unsafe {
                    let p1 = (cloned.as_ptr() as *mut u8).offset((data_index * 3) as isize);
                    let p2 = p1.offset(1 as isize);
                    let p3 = p2.offset(1 as isize);
                    *p1 = c[0];
                    *p2 = c[1];
                    *p3 = c[2];
                }
            });

            //Reduces speed a bit
            pb.lock().unwrap().inc();
        });

        data
    }

    ///Generate rows in a specifies row range (max is 0..SIZE.1)
    pub fn generate_rows(row_range: Range<usize>) -> Vec<Vec<Color>> {
        let pb = Arc::new(Mutex::new(ProgressBar::new(
            (row_range.end - row_range.start) as u64,
        )));
        pb.lock().unwrap().message("Row ");

        row_range
            .into_par_iter()
            .map(move |y| {
                let x_range = 0..SIZE.0;

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

    pub fn chunk_to_rgb_binary(yx_map: &Vec<Vec<Color>>) -> Box<[u8; CHUNK_SIZE_RGB]> {
        //Without multithreading

        let column_count = yx_map.len();
        let row_length = yx_map[0].len();

        let mut data = Box::new([0_u8; CHUNK_SIZE_RGB]);

        for y in 0..column_count {
            for x in 0..row_length {
                //Get bytes
                let bytes = yx_map[y][x];
                let pos = (x + (y * row_length)) * 3;

                data[pos] = bytes[0];
                data[pos + 1] = bytes[1];
                data[pos + 2] = bytes[2];
                //Alpha channel is always 25
                //data[pos + 3] = 255;
            }
        }

        data
    }

    ///Generate rows in a specifies row range (max is 0..SIZE.1)
    pub fn generate_rows_bin(row_range: Range<usize>) -> Box<[u8; CHUNK_SIZE_RGB]> {
        let start_index = row_range.start * SIZE.0;
        let end_index = row_range.end * SIZE.0;
        let pb = Arc::new(Mutex::new(ProgressBar::new(
            (row_range.end - row_range.start) as u64,
        )));
        pb.lock().unwrap().message("Row ");

        let cpus = num_cpus::get();
        println!("{}", cpus);

        let mut arr = Box::new([0_u8; CHUNK_SIZE_RGB]);

        for i in row_range.into_iter() {
            println!("{i}");
        }

        let colors: Vec<Color> = (start_index..end_index)
            .into_par_iter()
            .map(|i| {
                let x = i % SIZE.0;
                let y = i / SIZE.1;

                let iter = sets::mandelbrot::get_pixel(x as f64, y as f64);

                color::from_iterations(iter)
            })
            .collect();

        for i in 0..colors.len() {
            let pos = i * 3;
            arr[pos] = colors[i][0];
            arr[pos + 1] = colors[i][1];
            arr[pos + 1] = colors[i][2];
        }

        arr
    }

    //For time and file size estimates
    const SIZE_FACTOR: f64 = 5.01171875e-08;
    const TIME_FACTOR: f64 = 1.4357734375e-07;

    pub fn size_estimate() -> byte_unit::AdjustedByte {
        byte_unit::Byte::from_unit(
            SIZE_FACTOR * (SIZE.0 * SIZE.1) as f64,
            byte_unit::ByteUnit::MiB,
        )
        .unwrap()
        .get_appropriate_unit(true)
    }

    pub fn time_estimate() -> f64 {
        TIME_FACTOR * (SIZE.0 * SIZE.1) as f64
    }
}

#[allow(dead_code)]
pub mod single {
    use super::*;

    pub const DATA_SIZE_RGB: usize = SIZE.0 * SIZE.1 * 3;
    pub const DATA_SIZE_RGBA: usize = SIZE.0 * SIZE.1 * 4;

    pub fn save_file(path: &str, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, SIZE.0 as u32, SIZE.1 as u32); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;

        writer.write_image_data(data)?; // Save

        Ok(())
    }

    pub fn to_rgb_binary(yx_map: &Vec<Vec<Color>>) -> Box<[u8; DATA_SIZE_RGB]> {
        //Without multithreading

        let column_count = yx_map.len();
        let row_length = yx_map[0].len();

        let mut data = Box::new([0_u8; DATA_SIZE_RGB]);

        for y in 0..column_count {
            for x in 0..row_length {
                //Get bytes
                let bytes = yx_map[y][x];
                let pos = (x + (y * row_length)) * 3;

                data[pos] = bytes[0];
                data[pos + 1] = bytes[1];
                data[pos + 2] = bytes[2];
                //Alpha channel is always 25
                //data[pos + 3] = 255;
            }
        }

        data
    }

    pub fn to_rgba_binary(yx_map: &Vec<Vec<Color>>) -> Box<[u8; DATA_SIZE_RGBA]> {
        //Without multithreading

        let column_count = yx_map.len();
        let row_length = yx_map[0].len();

        let mut data = Box::new([0_u8; DATA_SIZE_RGBA]);

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

    pub fn collect_color_map() -> Vec<Vec<Color>> {
        let pb = Arc::new(Mutex::new(ProgressBar::new((SIZE.1) as u64)));
        let y_range = 0..SIZE.1;

        y_range
            .into_par_iter()
            .map(move |y| {
                let x_range = 0..SIZE.0;

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
