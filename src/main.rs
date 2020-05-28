use std::time::Instant;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod cell;
mod screen;

use screen::Screen;

const SCREEN_WIDTH: u32 = 128;
const SCREEN_HEIGHT: u32 = 128;
const TWOPI: f64 = 2.0 * std::f64::consts::PI;

// 0: Column
fn column_0(screen: &mut Screen, elapsed: f64) {
    screen.clear();
    let angle = elapsed * 0.2;
    for i in (0..4).map(|ix| ix as f64 * 0.25f64) {
        let x1 = 64.0 + 32.0 * ((angle + i) * TWOPI).cos();
        let x2 = 64.0 + 32.0 * ((angle + i + 0.25) * TWOPI).cos();
        if x1 > x2 {
            screen.rectfill(x1 as usize, 0, x2 as usize, 127, 7);
            screen.rect(x1 as usize, 0, x1 as usize, 127, 0);
            screen.rect(x2 as usize, 0, x2 as usize, 127, 0);
        }
    }
}

// 1: Twisty column
fn column_1(screen: &mut Screen, elapsed: f64) {
    let t = elapsed / 2f64;

    for y in 0..127 {
        let yy = y as f64 / 1024.0f64;
        let angle = ((0.2 * ((t * 0.1 + yy * 2.0) * TWOPI).sin()) * TWOPI).cos()
            + 0.5 * ((-0.2 * t + yy / 2.0) * TWOPI).cos();
        for i in (0..4).map(|ix| ix as f64 * 0.25f64) {
            let x1 = 64.0 + 32.0 * ((angle + i) * TWOPI).cos();
            let x2 = 64.0 + 32.0 * ((angle + i + 0.25) * TWOPI).cos();
            if x1 > x2 {
                screen.rect(x1 as usize, y, x2 as usize, y, 7);
                screen.set(x1 as usize, y, 0);
                screen.set(x2 as usize, y, 0);
            }
        }
    }
}

// 2: Colourful twisty column
fn column_2(screen: &mut Screen, elapsed: f64) {
    let t = elapsed / 2f64;
    let plt = [0, 1, 2, 8, 14, 15, 7];

    for y in 0..127 {
        let yy = y as f64 / 1024.0;
        let angle = ((0.2 * ((t * 0.1 + yy * 2.0) * TWOPI).sin()) * TWOPI).cos()
            + 0.5 * ((-0.2 * t + yy / 2.0) * TWOPI).cos();
        for i in (0..4).map(|ix| ix as f64 * 0.25) {
            let x1 = 64.0 + 32.0 * ((angle + i) * TWOPI).cos();
            let x2 = 64.0 + 32.0 * ((angle + i + 0.25) * TWOPI).cos();
            if x1 > x2 {
                let c = (x1 - x2) / (1.5 * 32.0) * plt.len() as f64;
                let ca = plt[c.floor() as usize];
                let cb = plt[((c + 0.5).floor() as usize).min(plt.len() - 1)];
                screen.rect_alt(x1 as usize, y, x2 as usize, y, ca, cb);
            }
        }
    }
}

// 3: Wild, colourful twisty column
fn column_3(screen: &mut Screen, elapsed: f64) {
    let t = elapsed / 2f64;
    let plt = [0, 1, 2, 8, 14, 15, 7];

    for y in 0..127 {
        let yy = y as f64 / 1024.0;
        let angle = ((0.2 * ((t * 0.1 + yy * 2.0) * TWOPI).sin()) * TWOPI).cos()
            + 0.5 * ((-0.2 * t + yy / 2.0) * TWOPI).cos();

        let w = 32.0
            + 4.0
                * (((-t + y as f64 / 128f64) * TWOPI).sin()
                    + 0.5 * ((0.5 * t - y as f64 / 64.0) * TWOPI).cos());
        let x = 64.0 + 16.0 * (((t * 0.1 + yy * 2.0) * TWOPI).sin() * TWOPI).cos();
        for i in (0..4).map(|ix| ix as f64 * 0.25) {
            let x1 = x + w * ((angle + i) * TWOPI).cos();
            let x2 = x + w * ((angle + i + 0.25) * TWOPI).cos();
            if x1 > x2 {
                let c = (x1 - x2) / (1.5 * 32.0) * plt.len() as f64;
                let ca = plt[(c.floor() as usize).min(plt.len() - 1)];
                let cb = plt[((c + 0.5).floor() as usize).min(plt.len() - 1)];
                screen.rect_alt(x1 as usize, y, x2 as usize, y, ca, cb);
            }
        }
    }
}

const PILLAR_COUNT: usize = 4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64))
        .with_min_inner_size(LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64))
        .with_title("pillar")
        .build(&event_loop)
        .expect("Failed to create window");
    let surface = pixels::wgpu::Surface::create(&window);
    let size = window.current_monitor().size();

    let mut screen = Screen::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);

    let surface_texture = SurfaceTexture::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface);
    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?;

    let now = Instant::now();
    let frame = 1.0f64 / 30.0f64;
    let mut last = now.elapsed().as_secs_f64();

    let mut pillar_mode = 3;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Event::RedrawRequested(_) = event {
            screen.draw(pixels.get_frame());
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(event) {
            if input.key_pressed(VirtualKeyCode::Escape)
                || input.key_pressed(VirtualKeyCode::Q)
                || input.quit()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Right) {
                pillar_mode = (pillar_mode + 1) % PILLAR_COUNT;
            }
            if input.key_pressed(VirtualKeyCode::Left) {
                pillar_mode = (pillar_mode - 1) % PILLAR_COUNT;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            let time = now.elapsed().as_secs_f64();
            if (time - last) > frame {
                screen.clear();
                match pillar_mode {
                    0 => column_0(&mut screen, time),
                    1 => column_1(&mut screen, time),
                    2 => column_2(&mut screen, time),
                    3 => column_3(&mut screen, time),
                    _ => unreachable!(),
                }
                last = time;
            }
            window.request_redraw();
        }
    });
}
