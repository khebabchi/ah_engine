use bevy_ecs_macros::Component;
use glam::{Vec3, Quat, Mat4};
#[derive(Component,Copy, Clone,PartialEq, Debug)]
pub struct Transform {
    pub translation: Vec3,    // Translation
    pub rotation: Quat,    // Rotation (Quaternion for 3D)
    pub scale: Vec3,       // Scaling
}

impl Transform {
    /// Creates a default transform
    pub fn new() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// Computes the transformation matrix
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }
    /// Applies a translation
    pub fn translate(&mut self, delta: Vec3) {
        self.translation += delta;
    }

    /// Rotates the transform
    pub fn rotate(&mut self, delta: Quat) {
        self.rotation = delta * self.rotation;
    }

    /// Scales the transform
    pub fn scale(&mut self, factor: Vec3) {
        self.scale *= factor;
    }


}