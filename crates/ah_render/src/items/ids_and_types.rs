#![feature(concat_idents)]
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Query};
use bevy_ecs::query::With;
use wgpu::{BindGroupLayout, Buffer, ShaderModule, Texture};
use crate::globals::{Camera, CameraLayout, TestCamera};
use crate::items::{GroupInfo, IBufferInfo, InstBufferInfo, ShaderInfo, TextureInfo, VBufferInfo};


macro_rules! query {
    ($name:ident : ($($var:ident),*),$filters:ty) => { // i need to get the element list,
        pub type $name<'w,'s>=Query<'w,'s,($( &'s $var,)*),$filters>;
    };

    ($name:ident : ($($var:ident),*)) => { // i need to get the element list,
        pub type $name<'w,'s>=Query<'w,'s,($( &'s $var,)*)>;
    };
    ($name:ident : $var:ident,$filters:ty) => { // i need to get the element list,
        pub type $name<'w,'s>=Query<'w,'s, &'s $var,$filters>;
    };
    ($name:ident : $var:ident) => { // i need to get the element list,
        pub type $name<'w,'s>=Query<'w,'s, &'s $var>;
    };


}

macro_rules! id {
    ($name:ident) => {
        pub type $name = Entity;
    };
}

macro_rules! square_matrix {
    ( $name:ident, $all:expr) => {
        pub type $name<'a> = &'a [[f32; $all]; $all];
    };
}


id!(VBufferId);
id!(UVBufferId);
id!(IndBufferId); // index buffer
id!(InstBufferId); // instance buffer
id!(UBufferId); // uniform buffer
id!(TextureId);
id!(MeshId);
id!(SamplerId);
id!(SBufferId); // storage buffer
id!(CameraId); // storage buffer
// query types

query!( VBufferQuery     :(Buffer,VBufferInfo));
query!( IndBufferQuery   :(Buffer,IBufferInfo));
query!( InstBufferQuery  :(Buffer,InstBufferInfo));
query!( ShaderQuery      :(ShaderModule,ShaderInfo));
query!( TextureQuery     :(Texture,TextureInfo));
query!( GroupQuery       :(BindGroupLayout,GroupInfo));
query!( CameraQuery      : BindGroupLayout ,With<CameraLayout>);
query!( TestCameraQuery      : Buffer ,With<TestCamera>);


square_matrix!(RawMat4,4);
square_matrix!(RawMat3,3);
square_matrix!(RawMat2,2);

