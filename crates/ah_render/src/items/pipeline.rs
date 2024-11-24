use crate::items::shader::{AHShaderContext, AHShaderLayout};

pub struct AHPipeline {
    label: String,
    shader: AHShaderLayout,
    shader_context: Vec<AHShaderContext>,
}