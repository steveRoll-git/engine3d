use std::error::Error;

use error_iter::ErrorIter as _;
use log::error;
use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

struct Frame<'a> {
    width: u32,
    height: u32,
    data: &'a mut [u8],
}

impl<'a> Frame<'a> {
    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        assert!(
            x < self.width && y < self.height,
            "pixel position out of range"
        );
        let index = ((x + y * self.width) * 4) as usize;
        self.data[index..index + 4].copy_from_slice(&[
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        ]);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();

    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 300;

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 2.0, HEIGHT as f64 * 2.0);
        WindowBuilder::new()
            .with_title("3dgame")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::RedrawRequested => {
                    if let Err(err) = pixels.render() {
                        log_error("pixels.render", err);
                        elwt.exit();
                        return;
                    }
                }
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                _ => (),
            },
            _ => (),
        }

        let mut frame = Frame {
            width: WIDTH,
            height: HEIGHT,
            data: pixels.frame_mut(),
        };

        for i in 0..50 as u32 {
            frame.set_pixel(
                i,
                i,
                Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            );
        }

        frame.set_pixel(WIDTH - 1, HEIGHT - 1, Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 });

        if input.update(&event) {
            window.request_redraw();
        }
    })?;

    Ok(())
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
