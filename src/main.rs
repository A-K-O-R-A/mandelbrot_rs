use num_complex::Complex;
use tiny_skia::*;

const X_RANGE: (f64, f64) = (-2.00, 0.47);
const Y_RANGE: (f64, f64) = (-1.12, 1.12);

const IMAGE_SIZE: (u32, u32) = (500, 500);
const X_SCALE: f64 = (IMAGE_SIZE.0 as f64) / (-(X_RANGE.0 - X_RANGE.1));
const Y_SCALE: f64 = (IMAGE_SIZE.1 as f64) / (-(Y_RANGE.0 - Y_RANGE.1));

const MAX_ITERATION: u64 = 1000;
const RADIUS: f64 = 2.;

fn main() {
    let mut paint = Paint::default();
    paint.set_color_rgba8(0, 127, 0, 200);
    paint.anti_alias = true;

    let path = {
        let mut pb = PathBuilder::new();
        const RADIUS: f32 = 250.0;
        const CENTER: f32 = 250.0;
        pb.move_to(CENTER + RADIUS, CENTER);
        for i in 1..8 {
            let a = 2.6927937 * i as f32;
            pb.line_to(CENTER + RADIUS * a.cos(), CENTER + RADIUS * a.sin());
        }
        pb.finish().unwrap()
    };

    let mut stroke = Stroke::default();
    stroke.width = 6.0;
    stroke.line_cap = LineCap::Round;
    stroke.dash = StrokeDash::new(vec![20.0, 40.0], 0.0);

    let mut pixmap = Pixmap::new(IMAGE_SIZE.0, IMAGE_SIZE.1).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    pixmap.save_png("image.png").unwrap();
}

fn mandelbrot(px: f64, py: f64) -> u64 {
    let x0 = px - (IMAGE_SIZE.0 / 2) as f64;
    let y0 = py - (IMAGE_SIZE.1 / 2) as f64;

    let x0 = x0 / X_SCALE;
    let y0 = y0 / Y_SCALE;

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
