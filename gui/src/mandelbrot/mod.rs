use egui_extras::RetainedImage;

mod logic;
mod ui;

use crate::util::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Mandelbrot {
    pub image_size: Dim<usize>,
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
    ///Range offset
    pub offset: Dim<f64>,
    pub scale: Dim<f64>,
    pub radius: f64,
    pub max_iterations: u64,
    pub cache: Option<Vec<Vec<[u8; 3]>>>,
    pub image: Option<RetainedImage>,
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
