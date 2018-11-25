use super::{Camera, ColorFormat, DepthFormat, Events, Mesh, Resources, Pipe};
use gfx::Device;
use gfx_device_gl::Factory;
use glutin::dpi::LogicalSize;
use glutin::{GlContext, GlWindow};

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub struct Renderer {
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
    pub fn new(events: &mut Events) -> Self {
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

        let events_loop = events.get_events_loop();
        let (window, device, mut factory, mut render_target, mut depth_stencil) =
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
        gfx_window_glutin::update_views(&window, &mut render_target, &mut depth_stencil);

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
