use std::sync::Arc;

pub struct Material {
    color: Option<glam::Vec3>,
    texture_diffuse: Option<Arc<wgpu::Texture>>, // !!!! need to make sure its not mutable access !!!!
    texture_specualr: Option<Arc<wgpu::Texture>>, // !!!! need to make sure its not mutable access !!!!
    ambient: f32,
    shininess: f32,
}
impl Default for Material {
    fn default() -> Self {
        Self {
            color: Default::default(),
            texture_diffuse: Default::default(),
            texture_specualr: Default::default(),
            ambient: 0.05,
            shininess: 40.0,
        }
    }
}
