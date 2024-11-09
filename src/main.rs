mod scene;
use scene::*;

use error_iter::ErrorIter as _;
use glam::{Mat4, Quat, Vec3};
use log::error;
use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use std::error::Error;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowButtons, WindowId},
};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;
const SCALE: f32 = 2.0;

struct PixelsSurface {
    window: Window,
    pixels: Pixels,
}

struct App {
    scene: Scene,
    time: f32,
    surface: Option<PixelsSurface>,
}

impl App {
    fn new() -> App {
        App {
            surface: None,
            time: 0.0,
            scene: Scene {
                camera: Camera {
                    projection: Mat4::perspective_infinite_rh(
                        (90f32).to_radians(),
                        WIDTH as f32 / HEIGHT as f32,
                        0.1,
                    ),
                    position: Vec3::new(0.0, 0.0, 1.5),
                    rotation: Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.4, 0.0),
                },
                triangles: vec![
                    Triangle {
                        v0: Vec3::new(-0.5, -0.5, 0.0),
                        v1: Vec3::new(-0.5, 0.5, 0.0),
                        v2: Vec3::new(0.5, -0.5, 0.0),
                        color: Color::RED,
                    },
                    Triangle {
                        v0: Vec3::new(0.5, 0.5, 0.0),
                        v1: Vec3::new(-0.2, 0.2, 0.0),
                        v2: Vec3::new(-0.5, -0.5, 0.0),
                        color: Color {
                            r: 1.0,
                            g: 0.8,
                            b: 0.6,
                            a: 1.0,
                        },
                    },
                ],
            },
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(WIDTH as f32, HEIGHT as f32);
        let scaled_size = LogicalSize::new(WIDTH as f32 * SCALE, HEIGHT as f32 * SCALE);
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("3dgame")
                    .with_inner_size(scaled_size)
                    .with_min_inner_size(size)
                    .with_resizable(false)
                    .with_enabled_buttons(WindowButtons::MINIMIZE | WindowButtons::CLOSE),
            )
            .unwrap();
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
        };
        self.surface = Some(PixelsSurface { window, pixels });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let surface = self.surface.as_mut().unwrap();

                let mut frame = Frame {
                    width: WIDTH,
                    height: HEIGHT,
                    data: surface.pixels.frame_mut(),
                };

                frame.data.fill(0);

                self.scene.camera.rotation = Quat::from_euler(
                    glam::EulerRot::XYZ,
                    0.0,
                    (self.time / 48.0).sin() / 3.0,
                    0.0,
                );

                self.scene.render(&mut frame);

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

                if let Err(err) = surface.pixels.render() {
                    log_error("pixels.render", err);
                    event_loop.exit();
                    return;
                }

                self.time += 1.0;

                surface.window.request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();

    event_loop.run_app(&mut app)?;

    Ok(())
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
