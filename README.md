# Mandelbrot GUI
This is a simple GUI written in Rust that renders the mandelbrot set interactively. This project uses the `egui` crate to create the GUI and the `rayon` crate to calculate value of the mandelbrot set for every pixel.

![Screenshot](./screenshot.png)

Currently the calculations are done on the CPU which makes the application very laggy. It is planned to offload those calculations to the GPU with the `wgpu` crate but it is not working yet.

## CLI
On the `main` branch you can find a program that just renders a part of the mandelbrot set and saves it to a file without any GUI.


## Future plans

In the future is planned to have a CLI application and a GUI application that both share a single core library that calculates the Mandelbrot set with the gpu and maybe even other sets.

## Running the GUI

If you want to try out this application for yourself you'll need to clone this repository and build it yourself.

```bash
git clone https://github.com/A-K-O-R-A/mandelbrot_rs
git switch egui
cargo build --release
```