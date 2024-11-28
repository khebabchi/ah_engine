use bevy_ecs::prelude::Entity;
use wgpu::*;
use wgpu::core::id::SamplerId;
use crate::items::ids_and_types::{StorageBufferId, TextureId, UBufferId};

pub struct GroupInfo {
    group_layout: Vec<Vec<LayoutEntry>>,
}


#[derive(Debug, Clone, Copy)]
pub enum LayoutEntry {
    Texture,
    Sampler,
    Uniform,
    Storage,
}


