use rayon::prelude::*;
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

use crate::color;
pub struct Mandelbrot {
    pub image_size: Dim<usize>,
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
    offset: Dim<f64>,
    scale: Dim<f64>,
    pub radius: f64,
    pub max_iterations: u64,
}
#[allow(dead_code)]

impl Mandelbrot {
    pub fn default(width: usize, height: usize) -> Self {
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
        };
        inst.calculate_offset();
        inst.calculate_scale();

        inst
    }

    pub fn radius(mut self, r: f64) -> Self {
        self.radius = r;
        self
    }

    pub fn max_iterations(mut self, n: u64) -> Self {
        self.max_iterations = n;
        self
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

    ///Get a 2D Vector of colors for every single pixel on the screen
    pub fn get_color_map(&self) -> Vec<Vec<[u8; 3]>> {
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
}
