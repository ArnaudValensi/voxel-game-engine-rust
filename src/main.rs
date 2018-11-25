// TODO:
//   - Use a dynamic number of vertices
#![cfg_attr(target_os = "emscripten", allow(unused_mut))]

extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate terrain_generation;

use cgmath::prelude::*;
use cgmath::{Point3, Vector3};
use terrain_generation::{
    Camera, Events, Input, Lifecycle, LifecycleEvent, Mesh, Pipe, Renderer, Transform, Vertex,
};

fn cube_mesh_builder(renderer: &mut Renderer, position: Vector3<f32>, color: [f32; 3]) -> Mesh {
    let vertices: Vec<Vertex> = vec![
        // Top (0, 0, 1)
        Vertex::new([-1, -1, 1], color),
        Vertex::new([1, -1, 1], color),
        Vertex::new([1, 1, 1], color),
        Vertex::new([-1, 1, 1], color),
        // Bottom (0, 0, -1)
        Vertex::new([-1, 1, -1], color),
        Vertex::new([1, 1, -1], color),
        Vertex::new([1, -1, -1], color),
        Vertex::new([-1, -1, -1], color),
        // Right (1, 0, 0)
        Vertex::new([1, -1, -1], color),
        Vertex::new([1, 1, -1], color),
        Vertex::new([1, 1, 1], color),
        Vertex::new([1, -1, 1], color),
        // Left (-1, 0, 0)
        Vertex::new([-1, -1, 1], color),
        Vertex::new([-1, 1, 1], color),
        Vertex::new([-1, 1, -1], color),
        Vertex::new([-1, -1, -1], color),
        // Front (0, 1, 0)
        Vertex::new([1, 1, -1], color),
        Vertex::new([-1, 1, -1], color),
        Vertex::new([-1, 1, 1], color),
        Vertex::new([1, 1, 1], color),
        // Back (0, -1, 0)
        Vertex::new([1, -1, 1], color),
        Vertex::new([-1, -1, 1], color),
        Vertex::new([-1, -1, -1], color),
        Vertex::new([1, -1, -1], color),
    ];

    let indices: Vec<u16> = vec![
        0, 1, 2, 2, 3, 0, // Top
        4, 5, 6, 6, 7, 4, // Bottom
        8, 9, 10, 10, 11, 8, // Right
        12, 13, 14, 14, 15, 12, // Left
        16, 17, 18, 18, 19, 16, // Front
        20, 21, 22, 22, 23, 20, // Back
    ];

    let up = Vector3::unit_y();
    let forward = Vector3::unit_z();
    let model = Transform::new(position, up, forward).get_transform();

    Mesh::new(renderer, &vertices, &indices, model)
}

pub fn main() {
    let mut input = Input::new();
    let mut events = Events::new();
    // let mut events_loop = glutin::EventsLoop::new();
    let mut lifecycle = Lifecycle::new();
    let mut renderer = Renderer::new(&mut events);

    let pipe = Pipe::new(&mut renderer);

    let camera = Camera::new(
        &renderer,
        Point3::new(0.0, 2.0, 5.0),
        (Point3::new(0.0, 0.0, 0.0) - Point3::new(0.0, 2.0, 5.0)).normalize(),
    );

    let mut mesh1 = cube_mesh_builder(&mut renderer, Vector3::new(0.0, 0.0, 0.0), [1.0, 0.2, 0.3]);
    let mut mesh2 = cube_mesh_builder(&mut renderer, Vector3::new(0.0, 0.0, -5.0), [0.2, 1.0, 0.3]);

    while let Some(event) = lifecycle.next() {
        match event {
            LifecycleEvent::FixedUpdate(_fixed_delta_time) => {}
            LifecycleEvent::Update(_delta_time) => {
                events.update(&mut renderer, &input);

                renderer.clear();
                renderer.draw(&mut mesh1, &camera, &pipe);
                renderer.draw(&mut mesh2, &camera, &pipe);
                renderer.flush();

                if !events.is_running() {
                    return;
                }
            }
        }
    }
}
