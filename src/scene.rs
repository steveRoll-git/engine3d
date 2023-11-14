use glam::{vec2, Mat4, Quat, Vec3, Vec3Swizzles};
use pixels::wgpu::Color;

use crate::Frame;

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
                mat.project_point3(v).xy() * vec2(frame.width as f32, frame.height as f32)
                    + vec2(frame.width as f32 / 2.0, frame.height as f32 / 2.0)
            });
            frame.draw_triangle(v0, v1, v2, t.color);
        }
    }
}
