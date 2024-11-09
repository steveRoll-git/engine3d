mod scene;
use scene::*;

use error_iter::ErrorIter as _;
use glam::{Mat4, Quat, Vec3};
use log::error;
use softbuffer::{Context, Surface};
use std::{error::Error, num::NonZeroU32, rc::Rc};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowButtons, WindowId},
};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;
const SCALE: u32 = 1;

struct Canvas {
    window: Rc<Window>,
    surface: Surface<Rc<Window>, Rc<Window>>,
}

struct App {
    scene: Scene,
    time: f32,
    canvas: Option<Canvas>,
}

impl App {
    fn new() -> App {
        App {
            canvas: None,
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
                            r: 255,
                            g: 204,
                            b: 180,
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
        let scaled_size = LogicalSize::new(WIDTH * SCALE, HEIGHT * SCALE);
        let window = Rc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("3dgame")
                        .with_inner_size(scaled_size)
                        .with_min_inner_size(size)
                        .with_resizable(false)
                        .with_enabled_buttons(WindowButtons::MINIMIZE | WindowButtons::CLOSE),
                )
                .unwrap(),
        );
        let context = Context::new(window.clone()).unwrap();
        let mut surface = Surface::new(&context, window.clone()).unwrap();
        surface
            .resize(
                NonZeroU32::new(WIDTH).unwrap(),
                NonZeroU32::new(HEIGHT).unwrap(),
            )
            .unwrap();
        self.canvas = Some(Canvas { window, surface });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(size) => {
                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    self.canvas
                        .as_mut()
                        .unwrap()
                        .surface
                        .resize(width, height)
                        .unwrap();
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let surface = self.canvas.as_mut().unwrap();
                let mut buffer = surface.surface.buffer_mut().unwrap();

                let mut frame = Frame {
                    width: WIDTH,
                    height: HEIGHT,
                    scale: SCALE,
                    buffer: &mut buffer,
                };

                frame.buffer.fill(0);

                self.scene.camera.rotation = Quat::from_euler(
                    glam::EulerRot::XYZ,
                    0.0,
                    (self.time / 48.0).sin() / 3.0,
                    0.0,
                );

                self.scene.render(&mut frame);

                if let Err(err) = buffer.present() {
                    log_error("buffer.present", err);
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
