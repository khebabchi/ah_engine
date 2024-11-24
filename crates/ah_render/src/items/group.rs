use bevy_ecs::prelude::Entity;
use wgpu::*;
use wgpu::core::id::SamplerId;
use crate::items::buffer::{AHIndexBuffer, AHTypeVertex, AHVertexBuffer};
use crate::items::ids::{StorageBufferId, TextureId, UniformBufferId};
use crate::items::texture::AHTexture;

// AHPipeline


// AHGroupLayout
pub struct AHGroupLayout {
    entries: Vec<LayoutEntry>,
}

// AHGroup
pub struct AHGroup {
    slots: Vec<AHSlot>,
    index: u32,
    label: String,
}

// AHSlot Enum
#[derive(Debug, Clone, Copy)]
pub enum AHSlot {
    Texture(TextureId),
    Uniform(UniformBufferId),
    Sampler(SamplerId),
    Storage(StorageBufferId),
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


