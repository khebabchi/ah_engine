use ulid::Ulid;
use wgpu::ShaderModule;
use crate::items::buffer::AHVertexGroup;
use crate::items::group::{AHGroup, AHGroupLayout, AHIndexBuffer, AHTypeVertex};

pub struct AHShaderLayout {
    groups: Vec<AHGroupLayout>,
    id: Ulid,
    module: ShaderModule, // Assuming Module is a custom type, needs to be defined elsewhere
    vertex_layouts: Vec<AHTypeVertex>,
}
pub struct AHShaderContext {
    groups: Vec<AHGroup>,
    vertex_group: AHVertexGroup,
    index_buffer: AHIndexBuffer,
}