use cgmath::prelude::*;
use cgmath::{Matrix4, Quaternion, Vector3};

pub struct Transform {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
    transform: Matrix4<f32>,
}

impl Transform {
    pub fn new(position: Vector3<f32>, up: Vector3<f32>, forward: Vector3<f32>) -> Self {
        let rotation = Quaternion::look_at(forward, up);
        let mut new_transform = Self {
            position,
            rotation,
            transform: Matrix4::identity(),
        };

        new_transform.update_transform();
        new_transform
    }

    pub fn get_transform(&self) -> Matrix4<f32> {
        self.transform
    }

    pub fn set_forward(&mut self, new_forward: Vector3<f32>) {
        let up = Vector3::unit_y();

        self.rotation = Quaternion::look_at(new_forward, up);
        self.update_transform();
    }

    fn update_transform(&mut self) {
        let rotation_matrix = Matrix4::from(self.rotation);
        let translation = Matrix4::from_translation(self.position);

        self.transform = translation * rotation_matrix;
    }
}
