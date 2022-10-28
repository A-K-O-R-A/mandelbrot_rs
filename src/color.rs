use tiny_skia::*;

pub trait QwQ {
    fn add(&self, b: Self) -> Self;
    fn sub(&self, b: Self) -> Self;
    fn mult(&self, p: f32) -> Self;
}
pub fn clamp(n: f32) -> f32 {
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

#[allow(dead_code)]
pub mod scale {
    pub fn linear(iteration: u64) -> f32 {
        iteration as f32 / crate::MAX_ITERATION as f32
    }

    pub fn logarithmic(iteration: u64) -> f32 {
        ((linear(iteration) * 100. + 1.).log(1000000.) * 299.) / 100.
    }

    pub fn exponential(iteration: u64) -> f32 {
        -1. / (iteration as f32).powf(0.25) + 1.
    }
}

pub fn from_iterations(iteration: u64) -> Color {
    let green = Color::from_rgba8(255, 0, 0, 255);
    let blue = Color::from_rgba8(0, 0, 0, 255);

    //Linear scale
    //let iter_fact = scale::linear(iteration);

    //Logarithmic scale
    //let iter_fact = scale::logarithmic(iteration);

    //Exponential scale
    let iter_fact = scale::exponential(iteration);

    blue.add(green.sub(blue).mult(iter_fact))
}
