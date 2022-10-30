use core::color;
use egui::{containers::*, widgets::*, *};
use egui_extras::RetainedImage;
use rayon::prelude::*;
use std::time::Instant;

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
///2 Dimensions
pub struct Dim<T> {
    x: T,
    y: T,
}
impl<T> Dim<T> {
    #[allow(dead_code)]
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Mandelbrot {
    pub image_size: Dim<usize>,
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
    ///Range offset
    offset: Dim<f64>,
    scale: Dim<f64>,
    pub radius: f64,
    pub max_iterations: u64,
    cache: Option<Vec<Vec<[u8; 4]>>>,
    image: Option<RetainedImage>,
}

impl Default for Mandelbrot {
    fn default() -> Self {
        let mut inst = Mandelbrot {
            image_size: Dim { x: 200, y: 200 },
            x_range: (-2.00, 0.47),
            y_range: (-1.12, 0.),
            offset: Dim { x: 0., y: 0. },
            scale: Dim { x: 0., y: 0. },
            radius: 2.,
            max_iterations: 1_000,
            cache: None,
            image: None,
        };
        inst.calculate_offset();
        inst.calculate_scale();

        inst
    }
}

#[allow(dead_code)]
impl Mandelbrot {
    pub fn from_size(width: usize, height: usize) -> Self {
        let mut inst = Mandelbrot {
            image_size: Dim {
                x: width,
                y: height,
            },
            x_range: (-2.00, 0.47),
            y_range: (-1.12, 0.),
            offset: Dim { x: 0., y: 0. },
            scale: Dim { x: 0., y: 0. },
            radius: 2.,
            max_iterations: 1_000,
            cache: None,
            image: None,
        };
        inst.calculate_offset();
        inst.calculate_scale();

        inst
    }
    pub fn from_range(image_size: Dim<usize>, x_range: (f64, f64), y_range: (f64, f64)) -> Self {
        let mut inst = Mandelbrot {
            image_size,
            x_range,
            y_range,
            offset: Dim { x: 0., y: 0. },
            scale: Dim { x: 0., y: 0. },
            radius: 2.,
            max_iterations: 1_000,
            cache: None,
            image: None,
        };
        inst.calculate_offset();
        inst.calculate_scale();

        inst
    }

    pub fn radius(&mut self, r: f64) {
        self.radius = r;
    }
    pub fn max_iterations(&mut self, n: u64) {
        self.max_iterations = n;
    }
    pub fn image_size(&mut self, w: usize, h: usize) {
        self.image_size = Dim { x: w, y: h };
        self.calculate_offset();
        self.calculate_scale();
    }

    fn calculate_offset(&mut self) {
        let x_offset = (self.x_range.0 + self.x_range.1) / 2.;
        let y_offset = (self.y_range.0 + self.y_range.1) / 2.;

        self.offset = Dim {
            x: x_offset,
            y: y_offset,
        }
    }
    fn calculate_scale(&mut self) {
        let x_scale = (self.image_size.x as f64) / (-(self.x_range.0 - self.x_range.1));
        let y_scale = (self.image_size.y as f64) / (-(self.y_range.0 - self.y_range.1));

        self.scale = Dim {
            x: x_scale,
            y: y_scale,
        }
    }
    ///Recalculate offset and scale
    pub fn change_range(&mut self, x_range: (f64, f64), y_range: (f64, f64)) {
        self.x_range = x_range;
        self.y_range = y_range;

        self.calculate_offset();
        self.calculate_scale();
    }

    ///Get value of the mandelbrot set according to a pixel on the screen
    pub fn get_pixel(&self, px: f64, py: f64) -> u64 {
        let x0 = px - (self.image_size.x / 2) as f64;
        let y0 = py - (self.image_size.y / 2) as f64;

        let x0 = (x0 / self.scale.x) + self.offset.x;
        let y0 = (y0 / self.scale.y) + self.offset.y;

        let mut x = 0.0;
        let mut y = 0.0;
        let mut iteration = 0_u64;

        while ((x * x + y * y) <= self.radius * self.radius) && (iteration < self.max_iterations) {
            let xtemp = x * x - y * y + x0;
            y = 2. * x * y + y0;
            x = xtemp;
            iteration += 1;
        }

        iteration
    }

    ///Get a 2D Vector of colors for every single pixel on the screen Row<Column<[u8;4]>>
    pub fn get_color_map(&self) -> Vec<Vec<[u8; 4]>> {
        let x_range = 0..self.image_size.x;
        x_range
            .into_par_iter()
            .map(|x| {
                let y_range = 0..self.image_size.y;
                y_range
                    .into_par_iter()
                    .map(|y| {
                        //Get iteration count
                        let iter = self.get_pixel(x as f64, y as f64);

                        color::from_iterations(iter, color::scale::exponential)
                    })
                    .collect()
            })
            .collect()
    }

    pub fn transpose_xy_map(xy_map: &Vec<Vec<[u8; 4]>>) -> Vec<Vec<[u8; 4]>> {
        let mut yx_map: Vec<Vec<[u8; 4]>> = Vec::new();

        let max_x = xy_map.len();
        let max_y = xy_map[0].len();

        let mut y = 0;
        while y < max_y {
            let mut x = 0;
            let mut row = Vec::with_capacity(max_x);
            while x < max_x {
                row.push(xy_map[x][y]);
                x += 1;
            }
            yx_map.push(row);
            y += 1;
        }
        yx_map
    }
    pub fn to_raster(xy_map: &Vec<Vec<[u8; 4]>>) -> png_pong::PngRaster {
        let mut pixels = Vec::new();
        let yx_map = Mandelbrot::transpose_xy_map(&xy_map);

        let max_x = yx_map.len();
        let max_y = yx_map[0].len();

        for x_vec in yx_map {
            for bytes in x_vec {
                let srgba = pix::rgb::SRgba8::new(bytes[0], bytes[1], bytes[2], bytes[3]);

                pixels.push(srgba);
            }
        }

        let raster = png_pong::PngRaster::Rgba8(pix::Raster::with_pixels(
            max_x as u32,
            max_y as u32,
            &pixels[0..(max_x * max_y)],
        ));

        raster
    }
    pub fn to_binary(yx_map: &Vec<Vec<[u8; 4]>>) -> Vec<u8> {
        //Without multithreading
        let mut data: Vec<u8> = Vec::new();
        //let yx_map = transpose::yx_map(yx_map);

        for x_vec in yx_map {
            for color in x_vec {
                //Get bytes
                let mut bytes = color.to_vec();

                data.append(&mut bytes);
            }
        }

        data
    }
    pub fn write_cache_to_image(&mut self) {
        let raster = Mandelbrot::to_raster(self.cache.as_ref().unwrap());
        let mut out_data = Vec::new();

        let now = Instant::now();

        let mut encoder = png_pong::Encoder::new(&mut out_data).into_step_enc();
        let step = png_pong::Step { raster, delay: 0 };
        encoder.encode(&step).expect("Failed to add frame");

        let elapsed = now.elapsed();
        println!("Encoding took         {:.2?}", elapsed);

        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [self.image_size.x, self.image_size.y],
            &Mandelbrot::to_binary(self.cache.as_ref().unwrap())[..],
        );

        self.image = Some(RetainedImage::from_color_image("uwu", color_image));
        //self.image =
        //    Some(RetainedImage::from_color_image("uwu", &out_data).expect("Couldn't read image"));
    }
}

impl Mandelbrot {
    pub fn ui(&mut self, ui: &mut Ui) {
        //ui.ctx().request_repaint();

        let clip_rect = ui.available_rect_before_wrap();
        
        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            clip_rect,
        );

        if let Some(image) = &self.image {
            image.show_size(
                ui,
                Vec2::new(self.image_size.x as f32, self.image_size.y as f32),
            );
        } else {
            self.paint(&painter);
        }
        // Make sure we allocate what we used (everything)
        ui.expand_to_include_rect(painter.clip_rect());

        Frame::popup(ui.style())
            .stroke(Stroke::none())
            .show(ui, |ui| {
                ui.set_max_width(270.0);
                CollapsingHeader::new("Settings").show(ui, |ui| self.options_ui(ui));
            });
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        ui.add(Slider::new(&mut self.radius, 1.0..=10.0).text("Radius"));
        ui.add(Slider::new(&mut self.max_iterations, 1..=40_000).text("Max iterations"));

        //egui::reset_button(ui, self);
    }

    fn paint(&mut self, painter: &Painter) {
        let mut shapes: Vec<Shape> = Vec::new();

        let clip_rect = painter.clip_rect();

        let width = clip_rect.width() as usize;
        let height = (clip_rect.width() / 2.) as usize;

        if let Some(cache) = &self.cache {
            let old_width = cache.len();
            let old_height = cache[0].len();

            if width == old_width && height == old_height {
                println!("using cache");

                for (x, column) in cache.iter().enumerate() {
                    for (y, color) in column.iter().enumerate() {
                        let x = x as f32;
                        let y = y as f32;

                        let rect = Rect::from_two_pos(Pos2::new(x, y), Pos2::new(x + 1., y + 1.));
                        let fill_color =
                            Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
                        let shape = Shape::rect_filled(rect, Rounding::none(), fill_color);
                        shapes.push(shape);
                    }
                }
                painter.extend(shapes);
                return;
            }
        }

        //Adjust rendering size
        self.image_size(width as usize, height as usize);

        let now = Instant::now();

        let pixels = self.get_color_map();

        let elapsed = now.elapsed();
        println!("Calculation took      {:.2?}", elapsed);

        for (x, column) in pixels.iter().enumerate() {
            for (y, color) in column.iter().enumerate() {
                let x = x as f32;
                let y = y as f32;

                let rect = Rect::from_two_pos(Pos2::new(x, y), Pos2::new(x + 1., y + 1.));
                let fill_color =
                    Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
                let shape = Shape::rect_filled(rect, Rounding::none(), fill_color);
                shapes.push(shape);
            }
        }

        self.cache = Some(pixels);
        self.write_cache_to_image();

        painter.extend(shapes);
    }
}
