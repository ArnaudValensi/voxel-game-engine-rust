// TODO:
//   - Use a dynamic number of vertices
#![cfg_attr(target_os = "emscripten", allow(unused_mut))]

#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate terrain_generation;

use cgmath::prelude::*;
use cgmath::{Deg, Matrix4, Point3, Vector3};
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_device_gl::Factory;
use glutin::dpi::LogicalSize;
use glutin::{Event, GlContext, GlWindow, KeyboardInput, VirtualKeyCode, WindowEvent};
use terrain_generation::{Input, Lifecycle, LifecycleEvent, Transform};

pub type Resources = gfx_device_gl::Resources;
pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Locals {
        model: [[f32; 4]; 4] = "u_Model",
        view: [[f32; 4]; 4] = "u_View",
        proj: [[f32; 4]; 4] = "u_Proj",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<gfx::format::DepthStencil> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    fn new(p: [i8; 3], c: [f32; 3]) -> Vertex {
        Vertex {
            pos: [f32::from(p[0]), f32::from(p[1]), f32::from(p[2])],
            color: [c[0], c[1], c[2]],
        }
    }
}

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

struct Renderer {
    pub window: GlWindow,
    pub factory: Factory,
    pub device: gfx_device_gl::Device,
    pub encoder: gfx::Encoder<Resources, gfx_device_gl::CommandBuffer>,
    pub render_target:
        gfx::handle::RenderTargetView<Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>,
    pub depth_stencil:
        gfx::handle::DepthStencilView<Resources, (gfx::format::D24_S8, gfx::format::Unorm)>,
}

impl Renderer {
    pub fn new(events_loop: &mut glutin::EventsLoop) -> Self {
        let window_config = glutin::WindowBuilder::new()
            .with_title("Triangle example".to_string())
            .with_dimensions((1024, 768).into());

        let (api, version) = if cfg!(target_os = "emscripten") {
            (glutin::Api::WebGl, (2, 0))
        } else {
            (glutin::Api::OpenGl, (3, 2))
        };

        let context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(api, version))
            .with_vsync(true);

        let (window, device, mut factory, render_target, depth_stencil) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(
                window_config,
                context,
                events_loop,
            );
        let encoder = gfx::Encoder::from(factory.create_command_buffer());

        // FIXME: On Mac 10.14 (Mojave) we need to resize the window after creation.
        // This is related to this issue https://github.com/tomaka/glutin/issues/1069
        events_loop.poll_events(|_| {});
        let logical_size = window.get_outer_size().expect("Window no longer exists");
        let physical_size = logical_size.to_physical(window.get_hidpi_factor());
        window.resize(physical_size);
        // gfx_window_glutin::update_views(&window, &mut render_target, &mut depth_stencil);

        Self {
            window,
            factory,
            device,
            encoder,
            render_target,
            depth_stencil,
        }
    }

    pub fn clear(&mut self) {
        self.encoder.clear(&self.render_target, CLEAR_COLOR);
        self.encoder.clear_depth(&self.depth_stencil, 1.0);
        self.encoder.clear_stencil(&self.depth_stencil, 0);
    }

    pub fn draw(&mut self, mesh: &mut Mesh, camera: &Camera, pipe: &Pipe) {
        mesh.update_locals(self, &camera.get_view(), camera.get_projection());
        self.encoder.draw(&mesh.slice, &pipe.pso, &mesh.data);
    }

    pub fn flush(&mut self) {
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().unwrap();
        self.device.cleanup();
    }

    pub fn resize(&mut self, size: LogicalSize) {
        self.window
            .resize(size.to_physical(self.window.get_hidpi_factor()));
        gfx_window_glutin::update_views(
            &self.window,
            &mut self.render_target,
            &mut self.depth_stencil,
        );
    }
}

struct Pipe {
    pso: gfx::PipelineState<Resources, pipe::Meta>,
}

impl Pipe {
    pub fn new(renderer: &mut Renderer) -> Self {
        let (vs_code, fs_code) = if cfg!(target_os = "emscripten") {
            (
                include_bytes!("shader/triangle_300_es.glslv").to_vec(),
                include_bytes!("shader/triangle_300_es.glslf").to_vec(),
            )
        } else {
            (
                include_bytes!("shader/triangle_150_core.glslv").to_vec(),
                include_bytes!("shader/triangle_150_core.glslf").to_vec(),
            )
        };

        let pso = renderer
            .factory
            .create_pipeline_simple(&vs_code, &fs_code, pipe::new())
            .unwrap();

        Self { pso }
    }
}

#[derive(Clone, Debug)]
struct Mesh {
    slice: gfx::Slice<Resources>,
    data: pipe::Data<Resources>,
    transform: Matrix4<f32>,
}

impl Mesh {
    pub fn new(
        renderer: &mut Renderer,
        vertices: &[Vertex],
        indices: &[u16],
        transform: Matrix4<f32>,
    ) -> Self {
        let (vbuf, slice) = renderer
            .factory
            .create_vertex_buffer_with_slice(vertices, indices);

        let locals_buffer = renderer.factory.create_constant_buffer(1);

        let data = pipe::Data {
            vbuf,
            locals: locals_buffer,
            out: renderer.render_target.clone(),
            out_depth: renderer.depth_stencil.clone(),
        };

        Self {
            data,
            slice,
            transform,
        }
    }

    pub fn update_locals(
        &mut self,
        renderer: &mut Renderer,
        view: &Matrix4<f32>,
        proj: &Matrix4<f32>,
    ) {
        let locals = Locals {
            model: self.transform.into(),
            view: (*view).into(),
            proj: (*proj).into(),
        };

        renderer
            .encoder
            .update_buffer(&self.data.locals, &[locals], 0)
            .unwrap();
    }
}

struct Camera {
    position: Point3<f32>,
    forward: Vector3<f32>,
    projection: Matrix4<f32>,
}

impl Camera {
    fn new(renderer: &Renderer, position: Point3<f32>, forward: Vector3<f32>) -> Self {
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
    let mut events_loop = glutin::EventsLoop::new();
    let mut lifecycle = Lifecycle::new();
    let mut renderer = Renderer::new(&mut events_loop);

    let pipe = Pipe::new(&mut renderer);

    let camera = Camera::new(
        &renderer,
        Point3::new(0.0, 2.0, 5.0),
        (Point3::new(0.0, 0.0, 0.0) - Point3::new(0.0, 2.0, 5.0)).normalize(),
    );

    let mut mesh1 = cube_mesh_builder(&mut renderer, Vector3::new(0.0, 0.0, 0.0), [1.0, 0.2, 0.3]);
    let mut mesh2 = cube_mesh_builder(&mut renderer, Vector3::new(0.0, 0.0, -5.0), [0.2, 1.0, 0.3]);

    let mut is_running = true;
    while let Some(event) = lifecycle.next() {
        match event {
            LifecycleEvent::FixedUpdate(_fixed_delta_time) => {}
            LifecycleEvent::Update(_delta_time) => {
                events_loop.poll_events(|event| {
                    if let Event::WindowEvent { event, .. } = event {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => is_running = false,
                            WindowEvent::Resized(size) => {
                                renderer.resize(size);
                            }
                            _ => (),
                        }
                    }
                });

                renderer.clear();
                renderer.draw(&mut mesh1, &camera, &pipe);
                renderer.draw(&mut mesh2, &camera, &pipe);
                renderer.flush();

                if !is_running {
                    return;
                }
            }
        }
    }
}
