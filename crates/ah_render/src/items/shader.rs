use bevy_ecs::prelude::Entity;
use wgpu::ShaderModule;
use crate::items::buffer::AHVertexGroup;
use crate::items::group::{AHGroup, AHGroupLayout, AHIndexBuffer, AHTypeVertex};

pub struct AHShaderLayout {
    id: Entity,
    handle: ShaderModule,
    groups: Vec<AHGroupLayout>,
    vertex_layouts: Vec<AHTypeVertex>,
}
pub struct AHShaderContext {
    groups: Vec<AHGroup>,
    vertex_group: AHVertexGroup,
    index_buffer: AHIndexBuffer,
}