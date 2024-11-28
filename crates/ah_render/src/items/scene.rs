use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Query, With};
use glam::Mat4;
use wgpu::{Buffer, RenderPass};
use crate::items::{IBufferInfo, IndBufferQuery, Mesh, VBufferInfo};
use crate::items::ids_and_types::{ MeshId, TextureQuery, VBufferQuery};

pub struct Scene {
    id: Entity,
    meshes: Vec<(MeshId,Mat4)>, // transformation matrix
    ibs: Vec<IBufferInfo>,
}

impl Scene {
    pub fn render(self,
                  render_pass: &mut RenderPass,
                  mesh_query: Query<&Mesh>,vb_query: VBufferQuery,ib_query: IndBufferQuery,texture_query: TextureQuery
    ) {
        // Retrieve the MeshHandle
        for (mesh_id,transform) in &self.meshes {
            let mesh = mesh_query.get(mesh_id.clone()).expect("MashId is invalid (no mesh found with this id)");
            mesh.bind(render_pass,&vb_query,&ib_query,&texture_query);

        }
    }

}