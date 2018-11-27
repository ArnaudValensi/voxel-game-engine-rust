#![cfg_attr(target_os = "emscripten", allow(unused_mut))]

extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate terrain_generation;

use cgmath::prelude::*;
use cgmath::{Point3, Vector3};
use terrain_generation::{
    cube_mesh_builder, Camera, Events, Input, Lifecycle, LifecycleEvent, Rect, Renderer, UIMesh,
    UIMeshPipe, VoxelMeshPipe,
};

pub fn main() {
    let mut input = Input::new();
    let mut events = Events::new();
    let mut lifecycle = Lifecycle::new();
    let mut renderer = Renderer::new(&mut events);

    let pipe = VoxelMeshPipe::new(&mut renderer);
    let ui_pipe = UIMeshPipe::new(&mut renderer);

    let camera = Camera::new(
        &renderer,
        Point3::new(0.0, 2.0, 5.0),
        (Point3::new(0.0, 0.0, 0.0) - Point3::new(0.0, 2.0, 5.0)).normalize(),
    );

    let mut mesh1 = cube_mesh_builder(&mut renderer, Vector3::new(0.0, 0.0, 0.0), [1.0, 0.2, 0.3]);
    let mut mesh2 = cube_mesh_builder(&mut renderer, Vector3::new(0.0, 0.0, -5.0), [0.2, 1.0, 0.3]);

    let rect = Rect::new((0.0, 0.0), (200.0, 100.0));
    let mut ui_mesh1 = UIMesh::new(&mut renderer, &rect);

    while let Some(event) = lifecycle.next() {
        match event {
            LifecycleEvent::FixedUpdate(_fixed_delta_time) => {}
            LifecycleEvent::Update(_delta_time) => {
                events.update(&mut renderer, &mut input);

                mesh1.update_locals(&mut renderer, &camera.get_view(), camera.get_projection());
                mesh2.update_locals(&mut renderer, &camera.get_view(), camera.get_projection());

                renderer.clear();
                renderer.draw(&mut mesh1, &pipe);
                renderer.draw(&mut mesh2, &pipe);
                renderer.draw(&mut ui_mesh1, &ui_pipe);
                renderer.flush();

                if !events.is_running() {
                    return;
                }
            }
        }
    }
}
