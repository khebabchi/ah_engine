use std::ops::Range;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Query, With};
use bevy_ecs_macros::Component;
use wgpu::{Buffer, Device, Extent3d, IndexFormat, RenderPass};
use wgpu::util::RenderEncoder;
use crate::items::{AHTypeVertex, IBufferInfo, VBufferInfo};
use crate::items::ids_and_types::{IndBufferId, IndBufferQuery, InstBufferId, TextureId, TextureQuery, UVBufferId, VBufferId, VBufferQuery};

pub struct MeshLayout {
    vertex_attribs: Vec<AHTypeVertex>,
    texture_size: Option<Extent3d>, // Assuming PhysicalSize is defined elsewhere
}

#[derive(Component)]
pub struct Mesh {
    id: Entity,
    vertex_positions: VBufferId, // entity of vertex containing positions
    vertex_attribs: Vec<VBufferId>, // entity of vertex buffers
    instance_buffer:InstBufferId,
    index_buffer: IndBufferId, // entity of index buffer
    num_indices: u32,
    textures: Vec<(TextureId,UVBufferId)>,  // vertex buffer of uv coordinates
}
impl Mesh {
    pub(crate) fn bind(&self,render_pass: &mut RenderPass,vbs:&VBufferQuery,ibs:&IndBufferQuery,textures:&TextureQuery) {

        // bind positions vertex
        let positions=vbs.get(self.vertex_positions).unwrap();

        // bind vertex attribs
        render_pass.set_vertex_buffer(0,positions.slice_all());
        for (ind,vb_id) in self.vertex_attribs.iter().enumerate(){
            let vertex_buffer=vbs.get(vb_id.clone()).unwrap();
            render_pass.set_vertex_buffer(ind,vertex_buffer.slice(vertex_buffer.slice(..)));
        }

        // bind index buffer
        let (ib,ib_info)=ibs.get(self.index_buffer).unwrap();
        render_pass.set_index_buffer(ib.slice(..self.num_indices), ib_info.format);

        // bind textures
        todo!()
    }
}