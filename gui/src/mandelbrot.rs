use core::color;
use egui::{containers::*, widgets::*, *};
use rayon::prelude::*;
use std::f32::consts::TAU;

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
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

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Mandelbrot {
    pub image_size: Dim<usize>,
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
    offset: Dim<f64>,
    scale: Dim<f64>,
    pub radius: f64,
    pub max_iterations: u64,
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
        };
        inst.calculate_offset();
        inst.calculate_scale();

        inst
    }
}

#[allow(dead_code)]
impl Mandelbrot {
    pub fn from_size(width: usize, height: usize) -> Self {
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
    pub fn get_color_map(&self) -> Vec<Vec<[u8; 4]>> {
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

impl Mandelbrot {
    pub fn ui(&mut self, ui: &mut Ui, seconds_since_midnight: Option<f64>) {
        if !self.paused {
            self.time = seconds_since_midnight.unwrap_or_else(|| ui.input().time);
            ui.ctx().request_repaint();
        }

        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap(),
        );
        self.paint(&painter);
        // Make sure we allocate what we used (everything)
        ui.expand_to_include_rect(painter.clip_rect());

        Frame::popup(ui.style())
            .stroke(Stroke::none())
            .show(ui, |ui| {
                ui.set_max_width(270.0);
                CollapsingHeader::new("Settings")
                    .show(ui, |ui| self.options_ui(ui, seconds_since_midnight));
            });
    }

    fn options_ui(&mut self, ui: &mut Ui, seconds_since_midnight: Option<f64>) {
        if seconds_since_midnight.is_some() {
            ui.label(format!(
                "Local time: {:02}:{:02}:{:02}.{:03}",
                (self.time % (24.0 * 60.0 * 60.0) / 3600.0).floor(),
                (self.time % (60.0 * 60.0) / 60.0).floor(),
                (self.time % 60.0).floor(),
                (self.time % 1.0 * 100.0).floor()
            ));
        } else {
            ui.label("The fractal_clock clock is not showing the correct time");
        };
        ui.label(format!("Painted line count: {}", self.line_count));

        ui.checkbox(&mut self.paused, "Paused");
        ui.add(Slider::new(&mut self.zoom, 0.0..=1.0).text("zoom"));
        ui.add(Slider::new(&mut self.start_line_width, 0.0..=5.0).text("Start line width"));
        ui.add(Slider::new(&mut self.depth, 0..=14).text("depth"));
        ui.add(Slider::new(&mut self.length_factor, 0.0..=1.0).text("length factor"));
        ui.add(Slider::new(&mut self.luminance_factor, 0.0..=1.0).text("luminance factor"));
        ui.add(Slider::new(&mut self.width_factor, 0.0..=1.0).text("width factor"));

        egui::reset_button(ui, self);

        ui.hyperlink_to(
            "Inspired by a screensaver by Rob Mayoff",
            "http://www.dqd.com/~mayoff/programs/FractalClock/",
        );
        // ui.add(egui_demo_lib::egui_github_link_file!());
    }

    fn paint(&mut self, painter: &Painter) {
        struct Hand {
            length: f32,
            angle: f32,
            vec: Vec2,
        }

        impl Hand {
            fn from_length_angle(length: f32, angle: f32) -> Self {
                Self {
                    length,
                    angle,
                    vec: length * Vec2::angled(angle),
                }
            }
        }

        let angle_from_period =
            |period| TAU * (self.time.rem_euclid(period) / period) as f32 - TAU / 4.0;

        let hands = [
            // Second hand:
            Hand::from_length_angle(self.length_factor, angle_from_period(60.0)),
            // Minute hand:
            Hand::from_length_angle(self.length_factor, angle_from_period(60.0 * 60.0)),
            // Hour hand:
            Hand::from_length_angle(0.5, angle_from_period(12.0 * 60.0 * 60.0)),
        ];

        let mut shapes: Vec<Shape> = Vec::new();

        let rect = painter.clip_rect();
        let to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
            rect,
        );

        let mut paint_line = |points: [Pos2; 2], color: Color32, width: f32| {
            let line = [to_screen * points[0], to_screen * points[1]];

            // culling
            if rect.intersects(Rect::from_two_pos(line[0], line[1])) {
                shapes.push(Shape::line_segment(line, (width, color)));
            }
        };

        let hand_rotations = [
            hands[0].angle - hands[2].angle + TAU / 2.0,
            hands[1].angle - hands[2].angle + TAU / 2.0,
        ];

        let hand_rotors = [
            hands[0].length * emath::Rot2::from_angle(hand_rotations[0]),
            hands[1].length * emath::Rot2::from_angle(hand_rotations[1]),
        ];

        #[derive(Clone, Copy)]
        struct Node {
            pos: Pos2,
            dir: Vec2,
        }

        let mut nodes = Vec::new();

        let mut width = self.start_line_width;

        for (i, hand) in hands.iter().enumerate() {
            let center = pos2(0.0, 0.0);
            let end = center + hand.vec;
            paint_line([center, end], Color32::from_additive_luminance(255), width);
            if i < 2 {
                nodes.push(Node {
                    pos: end,
                    dir: hand.vec,
                });
            }
        }

        let mut luminance = 0.7; // Start dimmer than main hands

        let mut new_nodes = Vec::new();
        for _ in 0..self.depth {
            new_nodes.clear();
            new_nodes.reserve(nodes.len() * 2);

            luminance *= self.luminance_factor;
            width *= self.width_factor;

            let luminance_u8 = (255.0 * luminance).round() as u8;
            if luminance_u8 == 0 {
                break;
            }

            for &rotor in &hand_rotors {
                for a in &nodes {
                    let new_dir = rotor * a.dir;
                    let b = Node {
                        pos: a.pos + new_dir,
                        dir: new_dir,
                    };
                    paint_line(
                        [a.pos, b.pos],
                        Color32::from_additive_luminance(luminance_u8),
                        width,
                    );
                    new_nodes.push(b);
                }
            }

            std::mem::swap(&mut nodes, &mut new_nodes);
        }
        self.line_count = shapes.len();
        painter.extend(shapes);
    }
}
