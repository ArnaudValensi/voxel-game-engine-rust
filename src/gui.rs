use super::gfx;
use super::{ColorFormat, Mesh, Pipeline, Renderer, Resources};
use gfx::traits::FactoryExt;

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

impl UIMesh {
    pub fn new(renderer: &mut Renderer) -> Self {
        const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

        const SQUARE: &[Vertex] = &[
            Vertex {
                pos: [0.5, -0.5],
                color: WHITE,
            },
            Vertex {
                pos: [-0.5, -0.5],
                color: WHITE,
            },
            Vertex {
                pos: [-0.5, 0.5],
                color: WHITE,
            },
            Vertex {
                pos: [0.5, 0.5],
                color: WHITE,
            },
        ];

        const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

        let (vbuf, slice) = renderer
            .factory
            .create_vertex_buffer_with_slice(SQUARE, INDICES);

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
