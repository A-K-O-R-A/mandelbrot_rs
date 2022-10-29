#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;

mod mandelbrot;
use mandelbrot::Mandelbrot;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {
    mandelbrot: Mandelbrot,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Try to close the window");
            self.mandelbrot.ui(ui, Some(100.));
        });
    }
}
