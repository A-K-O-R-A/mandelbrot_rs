use std::{error::Error, time::Instant};

mod color;
mod data;
mod sets;

pub const SIZE: (usize, usize) = (4000, 2000);
pub const MAX_ITERATION: u64 = 1_000;

pub type Color = [u8; 3];

fn main() -> Result<(), Box<dyn Error>> {
    //let path = r"./single.png";
    //single_main(path)?;

    let path = r"./chunked.png";
    chunked_main(path)?;

    Ok(())
}

fn chunked_main(path: &str) -> Result<(), Box<dyn Error>> {
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    //About 10GB = 10 * 1024 KB = 10 * 1024 * 1024;
    const CHUNK_SIZE: usize = 1024 * 4; //* 1024 * 2 *

    let path = Path::new(path);
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, SIZE.0 as u32, SIZE.1 as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;

    /*
    |
    |
    |
    */

    let now = Instant::now();

    let yx_map = data::single::collect_color_map();

    let elapsed = now.elapsed();
    println!("Calculation took      {:.2?}", elapsed);
    let now = Instant::now();

    let bin = data::single::to_rgb_binary(&yx_map);
    //let writer = data::chunks::create_writer(r"./png_crate.png");

    let elapsed = now.elapsed();
    println!("Coversion took        {:.2?}", elapsed);
    let now = Instant::now();

    writer.write_image_data(&bin[..])?;

    let elapsed = now.elapsed();
    println!("Encoding/Writing      {:.2?}", elapsed);

    /*
    |
    |
    |
     */

    Ok(())
}

#[allow(dead_code)]
fn single_main(path: &str) -> Result<(), Box<dyn Error>> {
    let now = Instant::now();

    let yx_map = data::single::collect_color_map();

    let elapsed = now.elapsed();
    println!("Calculation took      {:.2?}", elapsed);
    let now = Instant::now();

    let bin = data::single::to_rgb_binary(&yx_map);
    //let writer = data::chunks::create_writer(r"./png_crate.png");

    let elapsed = now.elapsed();
    println!("Coversion took        {:.2?}", elapsed);
    let now = Instant::now();

    data::single::save_file(path, &bin[..])?;

    let elapsed = now.elapsed();
    println!("Encoding/Writing      {:.2?}", elapsed);

    Ok(())
}
