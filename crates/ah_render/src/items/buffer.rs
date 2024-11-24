use ulid::Ulid;
use wgpu::Buffer;

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
    id: Ulid,
    vertex_buffers: Vec<AHVertexBuffer>, // Assuming AHVertexBuffer is defined elsewhere
    index_buffer: Vec<AHIndexBuffer>,    // Assuming AHIndexBuffer is defined elsewhere
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