use wgpu::*;
use ulid::Ulid;
use crate::items::buffer::{AHIndexBuffer, AHTypeVertex, AHVertexBuffer};
use crate::items::texture::AHTexture;

// AHPipeline


// AHGroupLayout
pub struct AHGroupLayout {
    entries: Vec<LayoutEntry>,
}

// AHGroup
pub struct AHGroup {
    id: ulid::Ulid,
    slots: Vec<AHSlot>,
    index: u32,
    label: String,
}

// AHSlot Enum
#[derive(Debug, Clone, Copy)]
pub enum AHSlot {
    Texture,
    Uniform,
    Sampler,
    Storage,
}
pub struct AHUniform {
    buffer: Buffer,
    allocated_size: u32,
    is_mutable: bool,
    label: String,
    attribute: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum LayoutEntry {
    Texture,
    Sampler,
    Uniform,
    Storage,
}

// AHVertexGroup Struct




pub struct MeshLayout {
    vertex_attribs: Vec<AHTypeVertex>,
    texture_size: Option<Extent3d>, // Assuming PhysicalSize is defined elsewhere
}

// Mesh Struct
pub struct Mesh {
    id: Ulid,
    vertex_positions: AHVertexBuffer,
    vertex_attribs: AHVertexBuffer,
    index_buffer: AHIndexBuffer,
    texture: Option<AHTexture>,
}
