use std::fmt::Debug;
use std::hash::Hash;
use bevy_ecs::prelude::World;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::Icon;
use crate::{AHAppCmdBuffer};
use crate::command::AHAppCmdHandler;
use crate::events_state::AHEvents;
use crate::window::Window;
/// T is the worlds_hash_map value (the ket is bevy_ecs::World)

pub struct AHApp<UserEvent: 'static+Debug+Clone+Default>{
    pub(crate) window: Window,
    pub(crate) event_state:AHEvents<UserEvent>,
    pub(crate) event_proxy:Option<EventLoopProxy<UserEvent>>,
    worlds: Vec<World>,
    pub(crate) title:String,
    pub(crate) icon:Option<Icon>,
    pub event_handler: crate::events_state::AHEventHandler<UserEvent>
}
#[derive(Clone,Hash,Default,Debug, Eq,PartialEq)]
pub enum Worlds{
    #[default]
    Render,
    Game,
}
impl<UserEvent : 'static+Clone+Debug+Default> AHApp<UserEvent>
{
    pub fn new<F>(title:String,icon:Option<Icon>,event_handler:F) -> Self where F:Fn(AHEvents<UserEvent>)->AHAppCmdBuffer + 'static{
        AHApp{
            window: Default::default(),
            event_state: Default::default(),
            event_proxy:None,
            worlds: vec![],
            title,
            icon,
            event_handler:Box::new(event_handler),
        }
    }

    /// Create the **Event Loop** and **Run The App**
    /// - Some information :
    ///   - **ControlFlow::wait** --> **on event rerender**
    ///   - **ControlFlow::pull** --> **every frame rerender**


    pub(crate) fn handle_events(&mut self, active_event_loop: &ActiveEventLoop) {
        let cmd_handler= AHAppCmdHandler::new(active_event_loop,&self.window);
        let event_state=self.event_state.clone();
        let cmd_buffer=(self.event_handler)(event_state);
        cmd_handler.handle(cmd_buffer);

    }

    pub fn run(&mut self) {
        tracing_subscriber::fmt::init();
        let event_loop = EventLoop::with_user_event().build().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        self.event_proxy=Some(event_loop.create_proxy());
        event_loop.run_app(self).unwrap();
    }
}