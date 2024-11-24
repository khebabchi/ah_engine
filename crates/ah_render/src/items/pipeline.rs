use crate::items::shader::{AHShaderContext, AHShaderLayout};

pub struct AHPipeline {
    shader: AHShaderLayout,
    shader_context: Vec<AHShaderContext>,
    label: String,
}