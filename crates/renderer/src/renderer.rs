use std::collections::HashMap;
use std::sync::Arc;
use bevy_ecs::prelude::{Res, World};
use bevy_ecs::system::EntityCommands;
use egui_wgpu::ScreenDescriptor;
use pollster::FutureExt;
use wgpu::{Adapter, BufferUsages, Device, Extent3d, Instance, Surface, SurfaceError};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::egui_tools::EguiRenderer;
use crate::pipelines::{DefaultPileline, PipelineName};


pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipelines: HashMap<PipelineName, wgpu::RenderPipeline>,
    pub egui_renderer : EguiRenderer,
    pub scale_factor:f32
}

impl<'a>  Renderer<'a> {
    pub fn new(window:Arc<Window>)->Self{
        let size=window.inner_size();
        let instance=Renderer::create_instance();
        let (surface,instance) = Renderer::create_surface(instance,window.clone());
        let adapter = Renderer::get_adapter(&instance,&surface).block_on();
        let (device, queue) = Renderer::get_device_and_queue(&adapter).block_on();
        let config = Renderer::create_surface_config(&surface,&adapter,size);
        surface.configure(&device, &config);

        let mut render_pipelines= HashMap::new();
        render_pipelines.insert(PipelineName::Default,DefaultPileline::new(&device,&config));

        let egui_renderer =EguiRenderer::new(&device, config.format, None, 1, window);

        Self{
            surface,
            device,
            queue,
            config,
            render_pipelines,
            egui_renderer,
            scale_factor:1.0
        }
    }

    fn create_instance()->Instance{
        wgpu::Instance::new(&wgpu::InstanceDescriptor {
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

    pub fn handle_redraw<F>(&mut self,window:Arc<Window>,mut render_callback:F)
    where
        F: FnMut(&mut Renderer),
    {
        // Attempt to handle minimizing window
        if let Some(min) = window.is_minimized() {
            if min {
                println!("Window is minimized");
                return;
            }
        }



        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: window.scale_factor() as f32
                * self.scale_factor,
        };

        let surface_texture = self.surface.get_current_texture();

        match surface_texture {
            Err(SurfaceError::Outdated) => {
                // Ignoring outdated to allow resizing and minimization
                println!("wgpu surface outdated");
                return;
            }
            Err(_) => {
                surface_texture.expect("Failed to acquire next swap chain texture");
                return;
            }
            Ok(_) => {}
        };

        let surface_texture = surface_texture.unwrap();

        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });



        {
            self.egui_renderer.begin_frame(&window);

            render_callback(self);

            self.egui_renderer.end_frame_and_draw(
                &self.device,
                &self.queue,
                &mut encoder,
                &window,
                &surface_view,
                screen_descriptor,
            );
        }

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}

