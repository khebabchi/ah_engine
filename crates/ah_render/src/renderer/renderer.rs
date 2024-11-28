
use std::sync::Arc;
use bevy_ecs::prelude::{Res, World};
use bevy_ecs::system::EntityCommands;
use wgpu::{Adapter, Device, Instance, Surface};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::Handle;

pub struct Renderer;

impl Renderer{
    pub async fn init(world:&mut World,window:Arc<Window>){
        let size=window.inner_size();
        let instance=Renderer::create_instance();
        let (surface,instance) = Renderer::create_surface(instance,window.clone());
        let adapter = Renderer::get_adapter(&instance,&surface).await;
        let (device, queue) = Renderer::get_device_and_queue(&adapter).await;
        let config = Renderer::create_surface_config(&surface,&adapter,size);
        surface.configure(&device, &config);

      /*  let ah_element_buffer = AHElementBuffer::new(&device,AHElementType::PosTex,TEX_VERTICES,TEX_INDICES,0);

        let texture=AHTexture::new(&device,Extent3d{
            width: 512,
            height: 512,
            depth_or_array_layers: 1,
        });
        texture.update_data(&queue,create_dyn_img_from_file("./assets/tree.png"));
        let ah_bind_group=BindGroupBuilder::new(Some("texture_buffer"),0).add_texture(&queue,texture).build(&device).unwrap();


        //-------------------------------------------
        let ah_shader=AHShader::new(&device, include_str!("../shaders/shader.wgsl").into(), vec![ah_element_buffer], vec![ah_bind_group], &surface_format);*/

        world.insert_resource(Handle::new(device));
        world.insert_resource(Handle::new(adapter));
        world.insert_resource(Handle::new(instance));
        world.insert_resource(Handle::new(surface));
        world.insert_resource(Handle::new(config));
        world.insert_resource(Handle::new(queue));

    }

    fn create_instance()->Instance{
        wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::SECONDARY,
            ..Default::default()
        })
    }
    async fn get_adapter(instance: &Instance,surface:&Surface<'_>)->wgpu::Adapter{
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap()
    }
    fn create_surface(instance: Instance, window: Arc<Window>) -> (Surface<'static>,Instance) {
        (instance.create_surface(window).unwrap(),instance)
    }
    async fn get_device_and_queue(adapter: &Adapter)->(wgpu::Device,wgpu::Queue){
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap()
    }
    fn create_surface_config(surface: &Surface,adapter: &Adapter,size:PhysicalSize<u32>)->wgpu::SurfaceConfiguration{
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format=surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }
    /// insert layout into the world
    /// current bind groups :
    /// - bg0 :
    ///     - 0:camera uniform
    /// TODO
    pub(crate) fn init_layout(device: &Device,  entity_commands: &mut EntityCommands) {
        let bg0 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                // camera uniform buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });
        entity_commands.insert(Handle::new(vec![bg0]));

    }
}

