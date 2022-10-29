pub mod mandelbrot {
    use crate::{IMAGE_SIZE, MAX_ITERATION};

    const RADIUS: f64 = 2.;

    const X_RANGE: (f64, f64) = (-2.00, 0.47);
    //const Y_RANGE: (f64, f64) = (-1.12, 1.12);
    const Y_RANGE: (f64, f64) = (-1.12, 0.);

    const X_OFF: f64 = (X_RANGE.0 + X_RANGE.1) / 2.;
    const Y_OFF: f64 = (Y_RANGE.0 + Y_RANGE.1) / 2.;

    const X_SCALE: f64 = (IMAGE_SIZE.0 as f64) / (-(X_RANGE.0 - X_RANGE.1));
    const Y_SCALE: f64 = (IMAGE_SIZE.1 as f64) / (-(Y_RANGE.0 - Y_RANGE.1));

    ///Source: https://en.wikipedia.org/wiki/Mandelbrot_set
    pub fn get_pixel(px: f64, py: f64) -> u64 {
        let x0 = px - (IMAGE_SIZE.0 / 2) as f64;
        let y0 = py - (IMAGE_SIZE.1 / 2) as f64;

        let x0 = (x0 / X_SCALE) + X_OFF;
        let y0 = (y0 / Y_SCALE) + Y_OFF;

        let mut x = 0.0;
        let mut y = 0.0;
        let mut iteration = 0_u64;

        //println!("{x} : {y}");
        while ((x * x + y * y) <= RADIUS * RADIUS) && (iteration < MAX_ITERATION) {
            let xtemp = x * x - y * y + x0;
            y = 2. * x * y + y0;
            x = xtemp;
            iteration += 1;
        }

        //color := palette[iteration]
        //plot(Px, Py, color)

        iteration
    }
}

///2 Dimensions
pub struct Dim<T> {
    x: T,
    y: T,
}
impl<T> Dim<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
pub struct Mandelbrot {
    pub image_size: Dim<usize>,
    x_range: (f64, f64),
    y_range: (f64, f64),
    offset: Dim<f64>,
    scale: Dim<f64>,
    radius: f64,
    max_iterations: u64,
}
#[allow(dead_code)]

impl Mandelbrot {
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
}
