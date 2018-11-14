// TODO:
//   - Use a dynamic number of vertices
//   - Display a mesh based on a world position
//   - Display multiple meshes
#![cfg_attr(target_os = "emscripten", allow(unused_mut))]

#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;

use cgmath::{Deg, Matrix4, Point3, SquareMatrix, Vector3, Vector4};
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_device_gl::Factory;
use glutin::dpi::LogicalSize;
use glutin::{Event, GlContext, GlWindow, KeyboardInput, VirtualKeyCode, WindowEvent};

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
        // Global buffers are added for compatibility when constant buffers are not supported.
        model: gfx::Global<[[f32; 4]; 4]> = "u_Model",
        view: gfx::Global<[[f32; 4]; 4]> = "u_View",
        proj: gfx::Global<[[f32; 4]; 4]> = "u_Proj",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

struct Renderer {
    pub window: GlWindow,
    pub factory: Factory,
    pub device: gfx_device_gl::Device,
    pub encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    pub render_target: gfx::handle::RenderTargetView<
        gfx_device_gl::Resources,
        (gfx::format::R8_G8_B8_A8, gfx::format::Unorm),
    >,
    pub depth_stencil: gfx::handle::DepthStencilView<
        gfx_device_gl::Resources,
        (gfx::format::D24_S8, gfx::format::Unorm),
    >,
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
                &events_loop,
            );
        let encoder = gfx::Encoder::from(factory.create_command_buffer());

        // FIXME: On Mac 10.14 (Mojave) we need to resize the window after creation.
        // This is related to this issue https://github.com/tomaka/glutin/issues/1069
        events_loop.poll_events(|_| {});
        let logical_size = window.get_outer_size().expect("Window no longer exists");
        let physical_size = logical_size.to_physical(window.get_hidpi_factor());
        window.resize(physical_size);

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
    }

    pub fn draw(&mut self, mesh: &Mesh, material: &Material) {
        self.encoder.draw(&mesh.slice, &material.pso, &mesh.data);
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

struct Material {
    pso: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
}

impl Material {
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

struct Mesh {
    slice: gfx::Slice<gfx_device_gl::Resources>,
    data: pipe::Data<gfx_device_gl::Resources>,
}

impl Mesh {
    pub fn new(renderer: &mut Renderer) -> Self {
        let trangles = vec![
            Vertex {
                pos: [-0.5, -0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                pos: [0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                pos: [0.0, 0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        let (vertex_buffer, slice) = renderer
            .factory
            .create_vertex_buffer_with_slice(&trangles, ());

        let logical_size = renderer.window.get_inner_size().unwrap();
        let aspect_ratio = logical_size.width as f32 / logical_size.height as f32;

        let model = Matrix4::identity();
        let view = Matrix4::look_at(
            Point3::new(-2.0, 0.0, 1.0),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::unit_y(),
        );
        let proj = cgmath::perspective(Deg(60.0f32), aspect_ratio, 0.1, 1000.0);
        let mvp = proj * view * model;

        println!(
            "mvp * Vector4: {:#?}",
            mvp * Vector4::<f32>::new(-0.5, -0.5, 0.0, 1.0),
        );

        // let data = pipe::Data {
        //     vbuf: vertex_buffer,
        //     locals: renderer.factory.create_constant_buffer(1),
        //     model: Matrix4::identity().into(),
        //     view: view.into(),
        //     proj: cgmath::perspective(Deg(60.0f32), aspect_ratio, 0.1, 1000.0).into(),
        //     out_color: renderer.render_target.clone(),
        //     out_depth: renderer.depth_stencil.clone(),
        // };

        let data = pipe::Data {
            vbuf: vertex_buffer,
            locals: renderer.factory.create_constant_buffer(1),
            model: model.into(),
            view: view.into(),
            proj: proj.into(),
            out: renderer.render_target.clone(),
        };

        let locals = Locals {
            model: model.into(),
            view: view.into(),
            proj: proj.into(),
        };

        renderer.encoder
            .update_buffer(&data.locals, &[locals], 0)
            .unwrap();

        Self { slice, data }
    }
}

pub fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let mut renderer = Renderer::new(&mut events_loop);
    let material = Material::new(&mut renderer);
    let mesh = Mesh::new(&mut renderer);

    let mut running = true;
    while running {
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
                    } => running = false,
                    WindowEvent::Resized(size) => {
                        renderer.resize(size);
                    }
                    _ => (),
                }
            }
        });

        // draw a frame
        renderer.clear();
        renderer.draw(&mesh, &material);
        renderer.flush();
    }
}
