use byte_unit::Byte;
use pbr::MultiBar;
use std::{error::Error, io::Write, time::Instant};

mod color;
mod data;
mod sets;

use data::{chunked, single};

pub const SIZE: (usize, usize) = (20000, 10000);
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
    chunked::check_size();

    println!("Generating RGB image");
    println!(" - Dimensions {}x{}", SIZE.0, SIZE.1);
    println!(
        " - Raw Size {}",
        Byte::from_bytes(single::DATA_SIZE_RGB as u128).get_appropriate_unit(true)
    );
    println!(
        " - {} chunks with {} bytes each",
        chunked::CHUNK_COUNT,
        Byte::from_bytes(chunked::CHUNK_SIZE_RGB as u128).get_appropriate_unit(true)
    );
    println!("");

    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    let now = Instant::now();

    let path = Path::new(path);
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, SIZE.0 as u32, SIZE.1 as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    let mut stream_writer = writer.stream_writer_with_size(chunked::CHUNK_SIZE_RGB)?;

    let mb = MultiBar::new();
    let mut chunks_bar = mb.create_bar(chunked::CHUNK_COUNT as u64);
    chunks_bar.message("Generating chunk: ");

    //Start listening to bar changes
    let _ = std::thread::spawn(move || {
        mb.listen();
    });

    for i in 0..chunked::CHUNK_COUNT {
        let start_row = i * chunked::ROWS_PER_CHUNK;
        let end_row = (i + 1) * chunked::ROWS_PER_CHUNK;
        let row_range = start_row..end_row;

        chunks_bar.inc();

        let chunk = chunked::generate_rows(row_range);
        let chunk_bin = chunked::chunk_to_rgb_binary(&chunk);

        println!("Writing chunk {}...                             ", i + 1);

        let now = Instant::now();
        stream_writer.write_all(&chunk_bin[..])?;
        let elapsed = now.elapsed();

        print!("{esc}[1A{esc}[2K", esc = 27 as char);
        println!(
            "Wrote Chunk {} in {:.2?} {esc}[2A",
            i + 1,
            elapsed,
            esc = 27 as char
        );
    }
    chunks_bar.finish();

    let elapsed = now.elapsed();
    println!(
        "{esc}[2B\nWrote {} chunks in {:.2?}",
        chunked::CHUNK_COUNT,
        elapsed,
        esc = 27 as char
    );

    Ok(())
}

#[allow(dead_code)]
fn single_main(path: &str) -> Result<(), Box<dyn Error>> {
    let now = Instant::now();

    let yx_map = single::collect_color_map();

    let elapsed = now.elapsed();
    println!("Calculation took      {:.2?}", elapsed);
    let now = Instant::now();

    let bin = single::to_rgb_binary(&yx_map);
    //let writer = chunks::create_writer(r"./png_crate.png");

    let elapsed = now.elapsed();
    println!("Coversion took        {:.2?}", elapsed);
    let now = Instant::now();

    single::save_file(path, &bin[..])?;

    let elapsed = now.elapsed();
    println!("Encoding/Writing      {:.2?}", elapsed);

    Ok(())
}
