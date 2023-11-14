use error_iter::ErrorIter as _;
use glam::{swizzles::*, Mat4, Vec2, Vec3};
use log::error;
use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use std::error::Error;
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};
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

    fn try_set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.set_pixel(x, y, color);
    }

    fn draw_triangle(&mut self, v0: Vec2, v1: Vec2, v2: Vec2, color: Color) {
        let min = v0.min(v1.min(v2)).as_uvec2();
        let max = v0.max(v1.max(v2)).as_uvec2();
        for py in min.y..max.y {
            for px in min.x..max.x {
                if (v1.x - v0.x) * (py as f32 - v0.y) - (v1.y - v0.y) * (px as f32 - v0.x) > 0.0
                    && (v2.x - v1.x) * (py as f32 - v1.y) - (v2.y - v1.y) * (px as f32 - v1.x) > 0.0
                    && (v0.x - v2.x) * (py as f32 - v2.y) - (v0.y - v2.y) * (px as f32 - v2.x) > 0.0
                {
                    self.try_set_pixel(px, py, color);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();

    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 300;
    const SCALE: f32 = 2.0;

    let window = {
        let size = LogicalSize::new(WIDTH as f32, HEIGHT as f32);
        let scaled_size = LogicalSize::new(WIDTH as f32 * SCALE, HEIGHT as f32 * SCALE);
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

    let mut time = 0.0f32;

    event_loop.run(move |event, elwt| {
        if input.update(&event) {
            if input.close_requested() {
                elwt.exit();
                return;
            }

            let mut frame = Frame {
                width: WIDTH,
                height: HEIGHT,
                data: pixels.frame_mut(),
            };

            frame.data.fill(0);

            if let Some((x, y)) = input.cursor() {
                let sx = x / SCALE;
                let sy = y / SCALE;

                let mut verts = [
                    Vec2 { x: 150.0, y: 150.0 },
                    Vec2 { x: sx, y: sy },
                    Vec2 { x: 200.0, y: 100.0 },
                ];

                let mat = Mat4::from_rotation_z((time / 4.0).sin() / 10.0);

                verts = verts.map(|v| Vec2 {
                    ..mat.transform_point3(Vec3::new(v.x, v.y, 0.0)).xy()
                });

                frame.draw_triangle(
                    verts[0],
                    verts[1],
                    verts[2],
                    Color {
                        r: 0.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    },
                );
            }

            frame.set_pixel(
                WIDTH - 1,
                HEIGHT - 1,
                Color {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                },
            );

            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                elwt.exit();
                return;
            }

            time = time + 1.0;

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
