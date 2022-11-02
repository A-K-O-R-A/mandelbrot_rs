use super::Mandelbrot;
use egui::{widgets::*, *};

impl Mandelbrot {
    pub fn ui(&mut self, ui: &mut Ui) {
        //ui.ctx().request_repaint();

        self.handle_interactions(ui);

        //Show retained image
        if let Some(image) = &self.image {
            //Show cached image
            if image
                .show_size(
                    ui,
                    Vec2::new(self.image_size.x as f32, self.image_size.y as f32),
                )
                .hovered()
            {
                //Image is hoverd
                let primary_down = ui.input().pointer.primary_down();
                if primary_down {
                    //User dragged mouse on image
                    let delta = ui.input().pointer.delta();
                    self.handle_drag(delta);
                    self.rerender();
                }
            }
        }

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
            println!("Forced rerender");
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
}
