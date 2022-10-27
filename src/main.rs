use rayon::prelude::*;
use tiny_skia::*;

const X_RANGE: (f64, f64) = (-2.00, 0.47);
const X_OFF: f64 = (X_RANGE.0 + X_RANGE.1) / 2.;
const Y_RANGE: (f64, f64) = (-1.12, 1.12);
const Y_OFF: f64 = (Y_RANGE.0 + Y_RANGE.1) / 2.;

const IMAGE_SIZE: (u32, u32) = (2000, 2000);
const X_SCALE: f64 = (IMAGE_SIZE.0 as f64) / (-(X_RANGE.0 - X_RANGE.1));
const Y_SCALE: f64 = (IMAGE_SIZE.1 as f64) / (-(Y_RANGE.0 - Y_RANGE.1));

const MAX_ITERATION: u64 = 1000;
const RADIUS: f64 = 2.;

fn main() {
    let mut paint = Paint::default();
    let mut pixmap = Pixmap::new(IMAGE_SIZE.0, IMAGE_SIZE.1).unwrap();

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
                        let iter = mandelbrot(x as f64, y as f64);

                        (y, iteration_to_color(iter))
                    })
                    .collect::<Vec<(u32, Color)>>(),
            )
        })
        .collect::<Vec<(u32, Vec<(u32, Color)>)>>();

    for (x, y_vec) in map {
        for (y, color) in y_vec {
            //Create rect
            let rect = Rect::from_xywh(x as f32, y as f32, 1., 1.).expect("Couldn't create rect");

            //Change color
            paint.shader = Shader::SolidColor(color);

            //paint pixel
            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
        }
    }

    pixmap.save_png("image.png").unwrap();
}

trait QwQ {
    fn add(&self, b: Self) -> Self;
    fn sub(&self, b: Self) -> Self;
    fn mult(&self, p: f32) -> Self;
}
fn clamp(n: f32) -> f32 {
    if n > 1. {
        return 1.;
    } else if n < 0. {
        return clamp(-n);
    }
    n
}

impl QwQ for Color {
    fn add(&self, b: Self) -> Self {
        Color::from_rgba(
            clamp(self.red() + b.red()),
            clamp(self.green() + b.green()),
            clamp(self.blue() + b.blue()),
            1.,
        )
        .unwrap()
    }

    fn sub(&self, b: Self) -> Self {
        Color::from_rgba(
            clamp(self.red() - b.red()),
            clamp(self.green() - b.green()),
            clamp(self.blue() - b.blue()),
            1.,
        )
        .unwrap()
    }

    fn mult(&self, p: f32) -> Self {
        Color::from_rgba(
            clamp(self.red() * p),
            clamp(self.green() * p),
            clamp(self.blue() * p),
            1.,
        )
        .unwrap()
    }
}

fn iteration_to_color(iteration: u64) -> Color {
    let green = Color::from_rgba8(0, 255, 31, 255);
    let blue = Color::from_rgba8(0, 3, 255, 255);

    let iter_fact = iteration as f32 / MAX_ITERATION as f32;

    blue.add(green.sub(blue).mult(iter_fact))
}

///Source: https://en.wikipedia.org/wiki/Mandelbrot_set
fn mandelbrot(px: f64, py: f64) -> u64 {
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
