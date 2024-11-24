use bevy_ecs::prelude::Entity;
use image::DynamicImage;
use wgpu::{AddressMode, FilterMode, CompareFunction, SamplerBorderColor, Sampler, Texture, Extent3d};

pub struct AHSampler {
    handle: Sampler,
    address_mode_u: AddressMode,
    address_mode_v: AddressMode,
    address_mode_w: AddressMode,
    mag_filter: FilterMode,
    min_filter: FilterMode,
    mipmap_filter: FilterMode,
    lod_min_clamp: f32,
    lod_max_clamp: f32,
    compare: Option<CompareFunction>,
    anisotropy_clamp: u16,
    border_color: Option<SamplerBorderColor>,
}


// AHTexture Struct
pub struct AHTexture {
    allocated: bool,
    handle: Texture,
    last_use: u8,
    size: Extent3d, // Assuming PhysicalSize is from a crate like `winit` or defined elsewhere
    slot: u32,
}