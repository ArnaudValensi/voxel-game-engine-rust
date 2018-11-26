use super::gfx;
use super::Resources;

pub trait Pipeline<PD: gfx::pso::PipelineData<Resources>> {
    fn get_pso(&self) -> &gfx::PipelineState<Resources, PD::Meta>;
}
