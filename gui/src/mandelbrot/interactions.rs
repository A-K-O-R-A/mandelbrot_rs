use egui::*;

use super::Mandelbrot;

//Interactions
impl Mandelbrot {
    pub fn handle_interactions(&mut self, ui: &mut Ui) {
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
            self.handle_zoom(zoom_delta, mouse_pos.unwrap());
            println!("User zoomed...rerendering");

            return self.rerender();
        }

        //Did the window change size?
        if !(new_w == old_w && new_h == old_h) {
            println!(
                "Window resized...rerendering (from {}x{} to {}x{})",
                old_w, old_h, new_w, new_h
            );
            return self.rerender();
        }

        //Do we have a cached image?
        if let Some(_image) = &self.image {
            //Don't rerender
            return;
        }

        //Need to rerender

        self.rerender();
    }

    pub fn handle_zoom(&mut self, delta: f32, pos: Pos2) {
        //Relative position of the curosr, from 0-1
        let rel_x = pos.x / (self.image_size.x as f32);
        let rel_y = pos.y / (self.image_size.y as f32);

        //New image are, relative values from 0-1
        let cutout = Rect::from_two_pos(
            Pos2 {
                x: rel_x - delta / 2.,
                y: rel_y - delta / 2.,
            },
            Pos2 {
                x: rel_x + delta / 2.,
                y: rel_y + delta / 2.,
            },
        );

        let (x_range, y_range) = self.cutout_to_range(cutout);
        self.change_range(x_range, y_range);
    }

    pub fn handle_drag(&mut self, delta: Vec2) {
        //Relative position of the curosr, from 0-1
        let rel_x_delta = delta.x / (self.image_size.x as f32);
        let rel_y_delta = delta.y / (self.image_size.y as f32);

        //New image are, relative values from 0-1
        let cutout = Rect::from_two_pos(
            Pos2 {
                x: 0. - rel_x_delta,
                y: 0. - rel_y_delta,
            },
            Pos2 {
                x: 1. - rel_x_delta,
                y: 1. - rel_y_delta,
            },
        );

        let (x_range, y_range) = self.cutout_to_range(cutout);
        self.change_range(x_range, y_range);
    }
}
