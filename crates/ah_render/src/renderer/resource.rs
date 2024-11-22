use std::ops::Deref;
use bevy_ecs::change_detection::Res;
use bevy_ecs_macros::Resource;

#[derive(Resource)]
pub struct AHResource<T>{
    inner:T
}
impl<T> AHResource<T>{
    pub fn new(inner:T)->Self{
        Self{inner}
    }
}
impl<T> Deref for AHResource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
pub type AHHandle<'w,T> = Res<'w,AHResource<T>>;