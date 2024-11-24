use bevy_ecs::entity::Entity;
use wgpu::Extent3d;
use crate::items::{AHIndexBuffer, AHTexture, AHTypeVertex, AHVertexBuffer};
use crate::items::ids::{IndexBufferId, VertexBufferId};

pub struct MeshLayout {
    vertex_attribs: Vec<AHTypeVertex>,
    texture_size: Option<Extent3d>, // Assuming PhysicalSize is defined elsewhere
}

// Mesh Struct
pub struct Mesh {
    id: Entity,
    vertex_position: VertexBufferId, // entity of vertex containing positions
    vertex_attribs: Vec<VertexBufferId>, // entity of vertex buffers
    index_buffer: IndexBufferId, // entity of index buffer
    texture: Option<Entity>,  // entity of texture
}
