use std::ops::{Range, RangeBounds};
use bevy_ecs::bundle::Bundles;
use bevy_ecs::prelude::Entity;
use bevy_ecs_macros::Component;
use wgpu::{Buffer, BufferAddress, BufferSlice, IndexFormat};
use winit::dpi::Position;

#[derive(Component)]
pub struct VBufferInfo {
    pub label: String,
    pub allocated_size: u32,
    pub is_mutable: bool,
    pub attributes: Vec<AHTypeVertex>,
}
#[derive(Component)]
pub struct IBufferInfo {
    pub allocated_size: u32,
    pub is_mutable: bool,
    pub uploaded: bool,
    pub format:IndexFormat,
}
#[derive(Component)]
pub struct UBufferInfo {
    pub allocated_size: u32,
    pub is_mutable: bool,
    pub label: String,
    pub attribute: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum AHTypeVertex {
    Position,
    Color,
    UV,
}
#[derive(Component)]
pub struct InstBufferInfo{}