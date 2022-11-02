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

        while ((x * x + y * y) <= RADIUS * RADIUS) && (iteration < MAX_ITERATION) {
            let xtemp = x * x - y * y + x0;
            y = 2. * x * y + y0;
            x = xtemp;
            iteration += 1;
        }

        iteration
    }
}
