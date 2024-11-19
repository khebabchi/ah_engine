use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use bevy_ecs::prelude::World;
use std::sync::Arc;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use winit::window::Icon;
use crate::{AHAppCmdBuffer, AHEvents, AHSize};
use crate::command::AHAppCmdHandler;
use crate::event::{AHAppEvent, AHEvent, AHEventQueue, Actions};
use crate::window::Window;
/// T is the worlds_hash_map value (the ket is bevy_ecs::World)

pub struct AHApp<UserEvent: 'static>{
    pub(crate) window: Window,
    pub(crate) event_queue:AHEventQueue,
    pub(crate) event_proxy:Option<EventLoopProxy<UserEvent>>,
    pub(crate) title:String,
    pub(crate) icon:Option<Icon>
}
#[derive(Clone,Hash,Default,Debug, Eq,PartialEq)]
pub enum Worlds{
    #[default]
    Render,
    Game,
}
impl<UserEvent : 'static> AHApp<UserEvent>
{
    pub fn new<F>(title:String,icon:Option<Icon>,event_handler:F) -> Self where F:FnMut(AHEvents)->AHAppCmdBuffer + 'static{
        AHApp{
            window: Default::default(),
            event_queue: AHEventQueue::new(event_handler),
            event_proxy:None,
            title,
            icon,
        }
    }

    /// Create the **Event Loop** and **Run The App**
    /// - Some information :
    ///   - **ControlFlow::wait** --> **on event rerender**
    ///   - **ControlFlow::pull** --> **every frame rerender**
    pub fn event_queue(&mut self)->&AHEventQueue{
        &self.event_queue
    }
    pub fn resize(&mut self, size:AHSize) {
        if self.window.is_created(){
            self.window.resize(size);
        }
    }

    pub fn run(&mut self) {
        tracing_subscriber::fmt::init();
        let event_loop = EventLoop::with_user_event().build().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        self.event_proxy=Some(event_loop.create_proxy());
        event_loop.run_app(self).unwrap();
    }
}