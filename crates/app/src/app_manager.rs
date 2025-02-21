use std::fmt::Debug;
use bevy_ecs::prelude::World;
use crate:: App;
use crate:: window::app_events::AppEventBuffer;
use crate:: window::events_manager::EventManager;
use  renderer::Renderer;

pub trait AppManager<UserEvent: 'static+Debug+Clone+Default+Eq>{
    fn on_start(&mut self,event_manager: &mut EventManager<UserEvent>);
    fn update(&mut self,event_manager: &mut EventManager<UserEvent>,delta_time:f32);
    fn fixed_update(&mut self,event_manager: &mut EventManager<UserEvent>,fixed_delta_time:f32){}
    fn render(&mut self,renderer:&mut Renderer);
    fn render_ui(&mut self,renderer:&mut Renderer);
}


