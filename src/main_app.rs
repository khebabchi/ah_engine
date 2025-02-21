use std::fmt::Debug;
use std::thread;
use std::time::Duration;
use bevy_ecs::prelude::World;
use winit::dpi::PhysicalSize;
use winit::window::WindowAttributes;
use  app::{App, AppManager};
use  app::window::app_events::AppEvent;
use  app::window::events_manager::EventManager;
use crate::load_icon;
use  renderer::Renderer;
use crate::audio_manager::AudioPlayer;

pub struct MainApp{
    world:World, 
    audio_manager:AudioPlayer 
}
impl MainApp {
    pub(crate) fn new() -> Self {
        MainApp{world:World::new(), audio_manager: AudioPlayer::new().unwrap() }
    }

}
impl<UE: 'static+Debug+Clone+Default+Eq> AppManager<UE> for MainApp {
    fn on_start(&mut self, event_manager: &mut EventManager<UE>){
        tracing::info!("MainApp on-start");
        event_manager.trigger_window_event(AppEvent::Center);
    }
    fn update(&mut self, event_manager: &mut EventManager<UE>, delta_time:f32) {
        tracing::info!("MainApp update, delta_time={}", delta_time);
        //wait 1 sec
        if event_manager.close_requested()
        {
            event_manager.trigger_window_event(AppEvent::Exit)
        }

    }
    fn render(&mut self,renderer:&mut Renderer) {
    }
    fn render_ui(&mut self,renderer:&mut Renderer) {
        egui::Window::new("winit + egui + wgpu says hello!")
            .resizable(true)
            .vscroll(true)
            .default_open(true)
            .show(renderer.egui_renderer.context(), |ui| {
                ui.label("Label!");

                if ui.button("Button!").clicked() {
                    println!("boom!")
                }

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Pixels per point: {}",
                        renderer.egui_renderer.context().pixels_per_point()
                    ));
                    if ui.button("-").clicked() {
                        renderer.scale_factor = (renderer.scale_factor - 0.1).max(0.3);
                    }
                    if ui.button("+").clicked() {
                        renderer.scale_factor = (renderer.scale_factor + 0.1).min(3.0);
                    }
                });
            });
    }
}

