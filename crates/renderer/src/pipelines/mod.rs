pub use default::DefaultPileline;
mod default;
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PipelineName {
    Default,
    Light,
}
pub trait RenderPipeline {
    type Input;
    const TEXTURE_OUT_COUNT:u8;
    fn render(render_pass:wgpu::RenderPass,input :Self::Input);
}
pub trait ComputePipeline {
    type Input;
    type Output;
    fn compute(render_pass:wgpu::RenderPass,input :Self::Input)->Self::Output;
}

pub fn init_pipelines() {}
