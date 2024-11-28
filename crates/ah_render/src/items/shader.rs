use bevy_ecs::prelude::Entity;
use wgpu::ShaderModule;
use crate::items::{ AHTypeVertex};
use crate::items::group::{Group, GroupLayout};
use crate::items::scene::Scene;

pub struct ShaderInfo {
    groups: Vec<GroupLayout>,
    vb_layouts: Vec<AHTypeVertex>,
}


pub struct AHShaderContext {
    groups: Vec<GroupInfoId>,
    scene: Scene,
}

impl AHShaderContext {
    pub(crate) fn execute(&self){

    }
    pub fn reload(){
        todo!()
    }
}