use super::gfx;
use super::Resources;

pub trait Mesh<PD: gfx::pso::PipelineData<Resources>> {
    fn get_data(&self) -> &PD;
    fn get_slice(&self) -> &gfx::Slice<Resources>;
}
