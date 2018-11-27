use super::gfx;
use super::{ColorFormat, Mesh, Pipeline, Renderer, Resources};
use gfx::traits::FactoryExt;
use glutin::dpi::LogicalSize;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub struct UIMeshPipe {
    pub pso: gfx::PipelineState<Resources, pipe::Meta>,
}

impl UIMeshPipe {
    pub fn new(renderer: &mut Renderer) -> Self {
        let vs_code = include_bytes!("shader/ui_150_core.glslv").to_vec();
        let fs_code = include_bytes!("shader/ui_150_core.glslf").to_vec();

        let pso = renderer
            .factory
            .create_pipeline_simple(&vs_code, &fs_code, pipe::new())
            .unwrap();

        Self { pso }
    }
}

impl Pipeline<pipe::Data<Resources>> for UIMeshPipe {
    fn get_pso(&self) -> &gfx::PipelineState<Resources, pipe::Meta> {
        &self.pso
    }
}

#[derive(Clone, Debug)]
pub struct UIMesh {
    pub slice: gfx::Slice<Resources>,
    pub data: pipe::Data<Resources>,
}

#[inline]
fn pixel_to_homogeneous_coordinate(coordinate: (f32, f32), screen_size: LogicalSize) -> (f32, f32) {
    (
        coordinate.0 * 2.0 / screen_size.width as f32 - 1.0,
        2.0 - coordinate.1 * 2.0 / screen_size.height as f32 - 1.0,
    )
}

impl UIMesh {
    pub fn new(renderer: &mut Renderer, rect: &Rect) -> Self {
        let screen_size = renderer.window.get_outer_size().unwrap();

        const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

        let homogeneous_position = pixel_to_homogeneous_coordinate(rect.position, screen_size);
        let homogeneous_size = pixel_to_homogeneous_coordinate(rect.size, screen_size);

        let top_left = Vertex {
            pos: [homogeneous_position.0, homogeneous_position.1],
            color: WHITE,
        };
        let top_right = Vertex {
            pos: [homogeneous_size.0, homogeneous_position.1],
            color: WHITE,
        };
        let bottom_right = Vertex {
            pos: [homogeneous_size.0, homogeneous_size.1],
            color: WHITE,
        };
        let bottom_left = Vertex {
            pos: [homogeneous_position.0, homogeneous_size.1],
            color: WHITE,
        };

        let suqare: &[Vertex] = &[top_left, top_right, bottom_right, bottom_left];

        const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

        let (vbuf, slice) = renderer
            .factory
            .create_vertex_buffer_with_slice(suqare, INDICES);

        let data = pipe::Data {
            vbuf,
            out: renderer.render_target.clone(),
        };

        Self { data, slice }
    }
}

impl Mesh<pipe::Data<Resources>> for UIMesh {
    fn get_data(&self) -> &pipe::Data<Resources> {
        &self.data
    }

    fn get_slice(&self) -> &gfx::Slice<Resources> {
        &self.slice
    }
}

pub struct Rect {
    pub position: (f32, f32),
    pub size: (f32, f32),
}

impl Rect {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Self {
        Self { position, size }
    }
}
