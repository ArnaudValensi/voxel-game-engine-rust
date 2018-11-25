use super::Renderer;
use cgmath::{Deg, Matrix4, Point3, Vector3};

pub struct Camera {
    position: Point3<f32>,
    forward: Vector3<f32>,
    projection: Matrix4<f32>,
}

impl Camera {
    pub fn new(renderer: &Renderer, position: Point3<f32>, forward: Vector3<f32>) -> Self {
        let logical_size = renderer.window.get_inner_size().unwrap();
        let aspect_ratio = logical_size.width as f32 / logical_size.height as f32;
        let projection = cgmath::perspective(Deg(60.0f32), aspect_ratio, 0.1, 1000.0);

        Self {
            position,
            forward,
            projection,
        }
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        Matrix4::look_at(
            self.position,
            self.position + self.forward,
            Vector3::unit_y(),
        )
    }

    pub fn get_projection(&self) -> &Matrix4<f32> {
        &self.projection
    }
}
