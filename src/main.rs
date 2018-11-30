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

struct Element {
    rect: Rect,
    color: [f32; 3],
}

struct ElementBuilder {
    rect: Rect,
    color: Option<[f32; 3]>,
}

impl ElementBuilder {
    pub fn new(rect: Rect) -> Self {
        Self { rect, color: None }
    }

    pub fn color(mut self, color: [f32; 3]) -> Self {
        self.color = Some(color);
        self
    }

    pub fn build(self) -> Element {
        let color = match self.color {
            Some(color) => color,
            None => [1.0, 1.0, 1.0],
        };

        Element {
            rect: self.rect,
            color,
        }
    }
}

struct Gui {}

impl Gui {
    pub fn render(mut renderer: &mut Renderer, pipe: &UIMeshPipe, element: &Element) {
        let mut mesh = UIMesh::new(&mut renderer, &element.rect, element.color);

        renderer.draw(&mut mesh, pipe);
    }

    fn create_element(position: (f32, f32), size: (f32, f32)) -> ElementBuilder {
        let rect = Rect::new(position, size);

        ElementBuilder::new(rect)
    }
}

fn hello() -> Element {
    Gui::create_element((20.0, 20.0), (400.0, 400.0))
        .color([1.0, 1.0, 0.0])
        .build()
    // .child(|| {
    //     Gui::create_element((20.0, 20.0), (200.0, 100.0))
    // })
}

pub fn main() {
    let mut input = Input::new();
    let mut events = Events::new();
    let mut lifecycle = Lifecycle::new();
    let mut renderer = Renderer::new(&mut events);
    // let gui = Gui::new(&renderer);

    let pipe = VoxelMeshPipe::new(&mut renderer);
    let ui_pipe = UIMeshPipe::new(&mut renderer);

    let camera = Camera::new(
        &renderer,
        Point3::new(0.0, 2.0, 5.0),
        (Point3::new(0.0, 0.0, 0.0) - Point3::new(0.0, 2.0, 5.0)).normalize(),
    );

    let mut mesh1 = cube_mesh_builder(&mut renderer, Vector3::new(0.0, 0.0, 0.0), [1.0, 0.2, 0.3]);
    let mut mesh2 = cube_mesh_builder(&mut renderer, Vector3::new(0.0, 0.0, -5.0), [0.2, 1.0, 0.3]);

    // let rect = Rect::new((0.0, 0.0), (200.0, 100.0));
    // let mut ui_mesh1 = UIMesh::new(&mut renderer, &rect);

    let hello = hello();

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
                // renderer.draw(&mut ui_mesh1, &ui_pipe);
                Gui::render(&mut renderer, &ui_pipe, &hello);
                renderer.flush();

                if !events.is_running() {
                    return;
                }
            }
        }
    }
}
