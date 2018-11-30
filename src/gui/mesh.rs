use super::super::gfx;
use super::super::gfx::traits::FactoryExt;
use super::super::{ColorFormat, Mesh, Pipeline, Renderer, Resources};
use super::Rect;
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
        let vs_code = include_bytes!("../shader/ui_150_core.glslv").to_vec();
        let fs_code = include_bytes!("../shader/ui_150_core.glslf").to_vec();

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
fn pixel_to_homogeneous_coordinate(x: f32, y: f32, screen_size: LogicalSize) -> (f32, f32) {
    (
        x * 2.0 / screen_size.width as f32 - 1.0,
        2.0 - y * 2.0 / screen_size.height as f32 - 1.0,
    )
}

impl UIMesh {
    pub fn new(renderer: &mut Renderer, rect: &Rect, color: [f32; 3]) -> Self {
        let screen_size = renderer.window.get_outer_size().unwrap();
        let position_left_top =
            pixel_to_homogeneous_coordinate(rect.position.0, rect.position.1, screen_size);
        let position_right_bottom = pixel_to_homogeneous_coordinate(
            rect.position.0 + rect.size.0,
            rect.position.1 + rect.size.1,
            screen_size,
        );

        let top_left = Vertex {
            pos: [position_left_top.0, position_left_top.1],
            color,
        };
        let top_right = Vertex {
            pos: [position_right_bottom.0, position_left_top.1],
            color,
        };
        let bottom_right = Vertex {
            pos: [position_right_bottom.0, position_right_bottom.1],
            color,
        };
        let bottom_left = Vertex {
            pos: [position_left_top.0, position_right_bottom.1],
            color,
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
