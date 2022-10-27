use tiny_skia::*;

const X_RANGE: (f64, f64) = (-2.00, 0.47);
const X_OFF: f64 = (X_RANGE.0 + X_RANGE.1) / 2.;
const Y_RANGE: (f64, f64) = (-1.12, 1.12);
const Y_OFF: f64 = (Y_RANGE.0 + Y_RANGE.1) / 2.;

const IMAGE_SIZE: (u32, u32) = (4000, 4000);
const X_SCALE: f64 = (IMAGE_SIZE.0 as f64) / (-(X_RANGE.0 - X_RANGE.1));
const Y_SCALE: f64 = (IMAGE_SIZE.1 as f64) / (-(Y_RANGE.0 - Y_RANGE.1));

const MAX_ITERATION: u64 = 1000;
const RADIUS: f64 = 2.;

fn main() {
    let mut paint = Paint::default();
    let mut pixmap = Pixmap::new(IMAGE_SIZE.0, IMAGE_SIZE.1).unwrap();

    let mut x = 0.;
    while (x as u32) < IMAGE_SIZE.0 {
        let mut y = 0.;
        while (y as u32) < IMAGE_SIZE.1 {
            let rect = Rect::from_xywh(x, y, 1., 1.).expect("Couldn't create rect");

            //Get iteration count
            let iter = mandelbrot(x as f64, y as f64);

            //Change color
            paint.shader = Shader::SolidColor(iteration_to_color(iter));

            //paint pixel
            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            y += 1.;
        }
        x += 1.;
    }

    pixmap.save_png("image.png").unwrap();
}

fn iteration_to_color(iteration: u64) -> Color {
    let iter_fact = iteration as f32 / MAX_ITERATION as f32;
    Color::from_rgba8(0, 0, (iter_fact * 255.) as u8, 255)
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
