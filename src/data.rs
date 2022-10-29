// For reading and opening files
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::color::*;
use crate::IMAGE_SIZE;
use tiny_skia::*;

const DATA_SIZE: usize = (IMAGE_SIZE.0 * IMAGE_SIZE.1 * 4) as usize;

pub fn transpose_map(xy_map: &Vec<(u32, Vec<(u32, Color)>)>) -> Vec<Vec<Color>> {
    let mut yx_map: Vec<Vec<Color>> = Vec::with_capacity(IMAGE_SIZE.1 as usize);

    let mut y = 0;
    while y < IMAGE_SIZE.1 as usize {
        let mut x = 0;
        let mut vec = Vec::with_capacity(IMAGE_SIZE.0 as usize);
        while x < IMAGE_SIZE.0 as usize {
            vec.push(xy_map[x].1[y].1);
            x += 1;
        }
        yx_map.push(vec);
        y += 1;
    }
    yx_map
}

#[allow(dead_code)]
pub mod skia {
    use super::*;

    pub fn save_file(pixmap: &Pixmap) {
        let _ = &pixmap.save_png("skia.png").unwrap();
    }

    pub fn draw_pixmap(map: &Vec<(u32, Vec<(u32, Color)>)>) -> Pixmap {
        let mut paint = Paint::default();
        let mut pixmap = Pixmap::new(IMAGE_SIZE.0, IMAGE_SIZE.1).unwrap();
        //let pixmap = Pixmap::from_vec(&mut data).unwrap();

        for (x, y_vec) in map {
            for (y, color) in y_vec {
                //Create single pixel as rect
                let rect =
                    Rect::from_xywh(*x as f32, *y as f32, 1., 1.).expect("Couldn't create rect");

                //Change color
                paint.shader = Shader::SolidColor(*color);

                //paint pixel
                pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            }
        }

        pixmap
    }
}

#[allow(dead_code)]
pub mod png_pong_crate {
    use png_pong::PngRaster;

    use super::*;

    pub fn save_file(raster: PngRaster) {
        let mut out_data = Vec::new();
        let mut encoder = png_pong::Encoder::new(&mut out_data).into_step_enc();
        let step = png_pong::Step { raster, delay: 0 };
        encoder.encode(&step).expect("Failed to add frame");
        std::fs::write("png_pong.png", out_data).expect("Failed to save image");
    }

    pub fn to_raster(xy_map: &Vec<(u32, Vec<(u32, Color)>)>) -> PngRaster {
        let mut pixels = Vec::with_capacity(DATA_SIZE / 4);
        let yx_map = transpose_map(xy_map);

        for x_vec in yx_map {
            for color in x_vec {
                //Get bytes
                let bytes = color.to_bytes();
                let srgba = pix::rgb::SRgba8::new(bytes[0], bytes[1], bytes[2], bytes[3]);

                pixels.push(srgba);
            }
        }

        let raster = png_pong::PngRaster::Rgba8(pix::Raster::with_pixels(
            IMAGE_SIZE.0,
            IMAGE_SIZE.1,
            &pixels[0..(DATA_SIZE / 4)],
        ));

        raster
    }
}

#[allow(dead_code)]
pub mod png_crate {
    use super::*;
    pub fn save_file(data: &[u8]) {
        let path = Path::new(r"./png_crate.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, IMAGE_SIZE.0, IMAGE_SIZE.1); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        //let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
        //let data = [255, 0, 0, 255, 0, 0, 0, 255];
        writer.write_image_data(data).unwrap(); // Save
    }

    #[allow(dead_code)]
    pub fn to_binary(xy_map: &Vec<(u32, Vec<(u32, Color)>)>) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::with_capacity(DATA_SIZE);

        let yx_map = transpose_map(xy_map);
        for x_vec in yx_map {
            for color in x_vec {
                //Get bytes
                let mut bytes = color.to_vec();

                data.append(&mut bytes);
            }
        }

        data
    }
}
