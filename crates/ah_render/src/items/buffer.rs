use bevy_ecs::prelude::Entity;
use wgpu::Buffer;
use crate::items::ids::{IndexBufferId, VertexBufferId};

pub struct AHVertexBuffer {
    allocated_size: u32,
    buffer: Buffer,
    is_mutable: bool,
    vertex_type: AHTypeVertex,
    label: String,
    uploaded: bool,
    attributes: Vec<AHTypeVertex>,
}
pub struct AHVertexGroup {
    id: Entity,
    vertex_buffers: Vec<VertexBufferId>, // Assuming AHVertexBuffer is defined elsewhere
    index_buffer: Vec<IndexBufferId>,    // entity of index buffer
}

pub struct AHIndexBuffer {
    allocated_size: u32,
    buffer: Buffer,
    is_mutable: bool,
    uploaded: bool,
}
#[derive(Debug, Clone, Copy)]
pub enum AHTypeVertex {
    Position,
    Color,
    Texture,
}