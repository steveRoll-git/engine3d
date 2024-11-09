use glam::{vec2, Mat4, Quat, Vec2, Vec3, Vec3Swizzles};

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
}

fn is_ccw(v0: Vec2, v1: Vec2, v2: Vec2) -> bool {
    (v2.y - v0.y) * (v1.x - v0.x) > (v1.y - v0.y) * (v2.x - v0.x)
}

pub struct Frame<'a> {
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub buffer: &'a mut [u32],
}

impl<'a> Frame<'a> {
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        assert!(
            x < self.width && y < self.height,
            "pixel position out of range"
        );
        for sx in (x * self.scale)..(x * self.scale + self.scale) {
            for sy in (y * self.scale)..(y * self.scale + self.scale) {
                let index = (sx + sy * self.width * self.scale) as usize;
                self.buffer[index] = u32::from_be_bytes([0, color.r, color.g, color.b]);
            }
        }
    }

    pub fn try_set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.set_pixel(x, y, color);
    }

    pub fn draw_triangle(&mut self, v0: Vec2, v1: Vec2, v2: Vec2, color: Color) {
        if !is_ccw(v0, v1, v2) {
            return;
        }
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

pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub color: Color,
}

pub struct Camera {
    pub projection: Mat4,
    pub position: Vec3,
    pub rotation: Quat,
}

impl Camera {
    fn get_view_mat(&self) -> Mat4 {
        return Mat4::from_rotation_translation(self.rotation, self.position);
    }
}

pub struct Scene {
    pub camera: Camera,
    pub triangles: Vec<Triangle>,
}

impl Scene {
    pub fn render(&self, frame: &mut Frame) {
        let mat = self.camera.projection * self.camera.get_view_mat().inverse();
        for t in &self.triangles {
            let [v0, v1, v2] = [t.v0, t.v1, t.v2].map(|v| {
                mat.project_point3(v).xy() * vec2(frame.width as f32, -(frame.height as f32))
                    + vec2(frame.width as f32 / 2.0, frame.height as f32 / 2.0)
            });
            frame.draw_triangle(v0, v1, v2, t.color);
        }
    }
}
