use palette::{rgb::Rgb, Hsv, IntoColor, Pixel};

#[allow(dead_code)]
pub mod scale {
    ///Linear scale, only works with a specific amount of iterations
    pub fn linear(iteration: u64) -> f32 {
        iteration as f32 / 1_000 as f32
    }

    ///Logarithmic scale, still needs some tuning
    pub fn logarithmic(iteration: u64) -> f32 {
        ((linear(iteration) * 100. + 1.).log(1000000.) * 299.) / 100.
    }

    ///Exponential scale, still needs some tuning
    pub fn exponential(iteration: u64) -> f32 {
        -1. / (iteration as f32).powf(0.27) + 1.
    }
}

pub fn from_iterations(iteration: u64, used_scale: fn(u64) -> f32) -> [u8; 3] {
    //Exponential scale
    let iter_fact = used_scale(iteration);

    //let hsv_c = Hsv::new(iter_fact * 360., 1., 1.);
    let hsv_c = Hsv::new(iter_fact * 300. + 20., 1., 1.);
    let rgb_c: Rgb = hsv_c.into_color();

    //println!("{hsv_c:?}");
    rgb_c.into_format().into_raw()
}
