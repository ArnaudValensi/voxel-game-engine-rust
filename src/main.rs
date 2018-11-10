#![cfg_attr(target_os = "emscripten", allow(unused_mut))]

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;
use glutin::{Event, GlContext, KeyboardInput, VirtualKeyCode, WindowEvent};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex {
        pos: [-0.5, -0.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        pos: [0.5, -0.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        pos: [0.0, 0.5],
        color: [0.0, 0.0, 1.0],
    },
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_config = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions((1024, 768).into());

    let (api, version, vs_code, fs_code) = if cfg!(target_os = "emscripten") {
        (
            glutin::Api::WebGl,
            (2, 0),
            include_bytes!("shader/triangle_300_es.glslv").to_vec(),
            include_bytes!("shader/triangle_300_es.glslf").to_vec(),
        )
    } else {
        (
            glutin::Api::OpenGl,
            (3, 2),
            include_bytes!("shader/triangle_150_core.glslv").to_vec(),
            include_bytes!("shader/triangle_150_core.glslf").to_vec(),
        )
    };

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(api, version))
        .with_vsync(true);
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_config, context, &events_loop);
    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

    let pso = factory
        .create_pipeline_simple(&vs_code, &fs_code, pipe::new())
        .unwrap();
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };

    // FIXME: On Mac 10.14 (Mojave) we need to resize the window after creation.
    // This is related to this issue https://github.com/tomaka/glutin/issues/1069
    events_loop.poll_events(|_| {});
    let logical_size = window.get_outer_size().expect("Window no longer exists");
    let physical_size = logical_size.to_physical(window.get_hidpi_factor());
    window.resize(physical_size);

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
                        window.resize(size.to_physical(window.get_hidpi_factor()));
                        gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
                    }
                    _ => (),
                }
            }
        });

        // draw a frame
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
