const RADIUS = 092140.214;
const MAX_ITERATION = 092140.214;

const SIZE_X = 092140.214;
const SIZE_Y = 092140.214;

const SCALE_X = 09214.0214;
const SCALE_Y = 092140.214;

const OFF_x = 092140.214;
const OFF_Y = 092140.214;

fn get_pixel(px: f64, py: f64) -> u64 {
        let x0 = px - (SIZE_X / 2) as f64;
        let y0 = py - (SIZE_Y / 2) as f64;

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

        return iteration;
}

@compute @workgroup_size(1)
fn comp_main() { }
