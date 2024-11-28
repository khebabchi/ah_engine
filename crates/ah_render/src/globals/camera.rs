use bevy_ecs::prelude::EntityCommands;
use bevy_ecs::world::World;
use bevy_ecs_macros::Component;
use bytemuck::Pod;
use crate::items::RawMat4;
use glam::{Mat4, Vec3, Vec4};
use wgpu::{Buffer, Device, SurfaceConfiguration};
use wgpu::util::DeviceExt;
use crate::Handle;
#[derive(Component)]
pub struct CameraLayout;
#[derive(Component)]
pub struct TestCamera;
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}
#[derive(Component)]
pub struct Camera {
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    aspect: f32,
    fov_y: f32,
    z_near: f32,
    z_far: f32,
    handle:Option<Buffer>
}
pub struct CameraDescriptor {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
}
impl Camera {
    fn new(desc:CameraDescriptor,device: Device,config: &SurfaceConfiguration) -> Self {
        let mut camera=Self {
            eye:desc.eye,
            target:desc.target,
            up:desc.up,
            aspect:config.width as f32 / config.height as f32,
            fov_y: desc.fov_y,
            z_near: desc.z_near,
            z_far: desc.z_far,
            handle: None,
        };
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera.to_uniform()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        camera.handle=Some(camera_buffer);
        return camera;
    }

    /// be carefull, this will insert a new test camera,
    pub(crate) fn insert_new_test_camera(device: Device,config: &SurfaceConfiguration,entity_commands: &mut EntityCommands)->Camera {

        let mut test_camera=Camera {
            // position the camera 1 unit up and 2 units back
            // +z is out of the screen
            eye: Vec3::new(0.0, 1.0, 2.0),
            // have it look at the origin
            target: Vec3::new(0.0, 0.0, 0.0),
            // which way is "up"
            up: Vec3::Y,
            aspect: config.width as f32 / config.height as f32,
            fov_y: 45.0,
            z_near: 0.1,
            z_far: 100.0,
            handle:None
        };
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[test_camera.to_uniform()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        test_camera.handle=Some(camera_buffer);
        return test_camera;
    }
    fn to_uniform(&self)->CameraUniform{
        CameraUniform{
            view_proj: self.projection_matrix().into(),
        }
    }
    pub fn projection_matrix(&self) -> Mat4 {
        // 1.
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = Mat4::perspective_rh(
            self.fov_y.to_radians(), // Convert degrees to radians
            self.aspect,
            self.z_near,
            self.z_far,
        );

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols(
    Vec4::new(1.0, 0.0, 0.0, 0.0),
    Vec4::new(0.0, 1.0, 0.0, 0.0),
    Vec4::new(0.0, 0.0, 0.5, 0.0),
    Vec4::new(0.0, 0.0, 0.5, 1.0),
);
