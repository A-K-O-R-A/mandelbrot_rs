use super::Mandelbrot;
use egui::{widgets::*, *};

impl Mandelbrot {
    pub fn ui(&mut self, ui: &mut Ui) {
        //ui.ctx().request_repaint();

        self.handle_frame(ui);

        ui.horizontal(|ui| {
            ui.vertical(|ui| self.options_ui(ui));
            ui.add_space(30.);
            ui.vertical(|ui| self.stats_ui(ui));
        });
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        ui.heading("Options");
        ui.add(Slider::new(&mut self.radius, 1.0..=10.0).text("Radius"));
        ui.add(Slider::new(&mut self.max_iterations, 1..=40_000).text("Max iterations"));
        if ui.button("Reset zoom").clicked() {
            self.change_range((-2.00, 0.47), (-1.12, 0.));
            self.rerender();
        }

        if ui.button("Force rerender").clicked() {
            self.rerender();
        }
    }

    fn stats_ui(&mut self, ui: &mut Ui) {
        ui.heading("Stats");
        ui.label(format!(
            "Image size {}x{}",
            self.image_size.x, self.image_size.y
        ));
        ui.label(format!(
            "X Range    {:.6} to {:.6}",
            self.x_range.0, self.x_range.1
        ));

        ui.label(format!(
            "Y Range    {:.6} to {:.6}",
            self.y_range.0, self.y_range.1
        ));
    }

    fn handle_frame(&mut self, ui: &mut Ui) {
        let clip_rect = ui.available_rect_before_wrap();

        let old_w = self.image_size.x;
        let old_h = self.image_size.y;

        let new_w = clip_rect.width() as usize;
        let new_h = (clip_rect.width() / 2.) as usize;

        //Adjust rendering size
        self.image_size(new_w, new_h);

        //Did the user zoom?
        let zoom_delta = (-ui.input().scroll_delta.y / 200.) + 1.;
        let mouse_pos = ui.input().pointer.hover_pos();
        if zoom_delta != 1.0 && mouse_pos.is_some() {
            self.zoom(zoom_delta, mouse_pos.unwrap());
            println!("Changed zoom...recaching");

            return self.rerender();
        }

        //Did the window change size?
        if !(new_w == old_w && new_h == old_h) {
            println!(
                "Window resized...recaching (from {}x{} to {}x{})",
                old_w, old_h, new_w, new_h
            );
            return self.rerender();
        }

        //Do we have a cached image?
        if let Some(image) = &self.image {
            //Cached image
            image.show_size(
                ui,
                Vec2::new(self.image_size.x as f32, self.image_size.y as f32),
            );
            return;
        }

        //Need to rerender

        self.rerender();
    }
}
