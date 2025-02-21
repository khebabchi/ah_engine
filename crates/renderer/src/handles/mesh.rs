use super::material::Material;

pub struct Mesh
{
    material: Material,
    vertex_count:u32,
	index_count:u32,
	vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer
}