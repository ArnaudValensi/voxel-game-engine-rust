use super::gfx;
use super::{ColorFormat, Mesh, Pipeline, Renderer, Resources};
use cgmath::Matrix4;
use gfx::traits::FactoryExt;

gfx_defines! {
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
    pub fn new(p: [i8; 3], c: [f32; 3]) -> Vertex {
        Vertex {
            pos: [f32::from(p[0]), f32::from(p[1]), f32::from(p[2])],
            color: [c[0], c[1], c[2]],
        }
    }
}

pub struct VoxelMeshPipe {
    pub pso: gfx::PipelineState<Resources, pipe::Meta>,
}

impl VoxelMeshPipe {
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

impl Pipeline<pipe::Data<Resources>> for VoxelMeshPipe {
    fn get_pso(&self) -> &gfx::PipelineState<Resources, pipe::Meta> {
        &self.pso
    }
}

#[derive(Clone, Debug)]
pub struct VoxelMesh {
    pub slice: gfx::Slice<Resources>,
    pub data: pipe::Data<Resources>,
    transform: Matrix4<f32>,
}

impl VoxelMesh {
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

impl Mesh<pipe::Data<Resources>> for VoxelMesh {
    fn get_data(&self) -> &pipe::Data<Resources> {
        &self.data
    }

    fn get_slice(&self) -> &gfx::Slice<Resources> {
        &self.slice
    }
}
