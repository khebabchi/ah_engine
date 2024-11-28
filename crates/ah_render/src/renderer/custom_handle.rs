use std::ops::Deref;
use bevy_ecs::change_detection::Res;
use bevy_ecs_macros::{Component, Resource};




#[derive(Resource,Component)]
pub struct Handle<T>{
    inner:T
}
impl<T> crate::Handle<T> {
    pub fn new(inner:T)->Self{
        Self{inner}
    }
}
impl<T> Deref for crate::Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

