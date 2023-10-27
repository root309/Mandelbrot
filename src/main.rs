extern crate piston_window;
extern crate rayon;

use piston_window::*;
use rayon::prelude::*;

fn mandelbrot(cx: f64, cy: f64, max_iter: usize) -> usize {
    let mut x = 0.0;
    let mut y = 0.0;
    for i in 0..max_iter {
        if x * x + y * y > 4.0 {
            return i;
        }
        let xtemp = x * x - y * y + cx;
        y = 2.0 * x * y + cy;
        x = xtemp;
    }
    max_iter
}

fn compute_row(y: u32, width: u32, height: u32, scale: f64, offset_x: f64, offset_y: f64) -> Vec<(u32, u32, [f32; 4])> {
    (0..width).into_par_iter().map(|x| {
        let mx = (x as f64 / width as f64) * 4.0 - 2.0;
        let my = (y as f64 / height as f64) * 4.0 - 2.0;

        let mx = mx / scale + offset_x;
        let my = my / scale + offset_y;

        let value = mandelbrot(mx, my, 1000);
        let color = if value < 1000 {
            [0.0, (value % 256) as f32 / 256.0, 0.0, 1.0]
        } else {
            [0.0, 0.0, 0.0, 1.0]
        };
        (x, y, color)
    }).collect()
}

fn main() {
    let (width, height) = (800, 600);
    let mut window: PistonWindow = WindowSettings::new("Mandelbrot", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut scale = 1.0;
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            let buffer: Vec<(u32, u32, [f32; 4])> = (0..height).into_par_iter().flat_map(|y| {
                compute_row(y, width, height, scale, offset_x, offset_y)
            }).collect();

            window.draw_2d(&event, |c, g, _| {
                clear([1.0; 4], g);
                for &(x, y, color) in &buffer {
                    rectangle(color, [x as f64, y as f64, 1.0, 1.0], c.transform, g);
                }
            });
        }

        if let Some(Button::Keyboard(Key::Up)) = event.press_args() {
            offset_y -= 0.1 / scale;
        }
        if let Some(Button::Keyboard(Key::Down)) = event.press_args() {
            offset_y += 0.1 / scale;
        }
        if let Some(Button::Keyboard(Key::Left)) = event.press_args() {
            offset_x -= 0.1 / scale;
        }
        if let Some(Button::Keyboard(Key::Right)) = event.press_args() {
            offset_x += 0.1 / scale;
        }
        if let Some(Button::Keyboard(Key::Z)) = event.press_args() {
            scale *= 1.1;
        }
        if let Some(Button::Keyboard(Key::X)) = event.press_args() {
            scale /= 1.1;
        }

    }
}