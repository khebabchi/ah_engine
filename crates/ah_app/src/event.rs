use std::fmt::Debug;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use crate::{AHAppCmdBuffer, AHSize};
use crate::command::AHAppCmdHandler;
use crate::window::Window;

#[derive(Clone,Debug)]
pub enum AHEvent <UserEvent>{
    App(UserEvent),
    Window(WindowEvent)
}

pub struct AHEventQueue<UserEvent:'static+Clone+Debug> {
    pub queue: Vec<AHEvent<UserEvent>>,
    pub handle_function:AHEventHandler<UserEvent>
}
type AHEventHandler<UserEvent>=Box<dyn FnMut(AHEvents<UserEvent>)->AHAppCmdBuffer>;
pub struct Actions<'ael,'win>{
    event_loop:&'ael ActiveEventLoop,
    window:&'win mut Window
}
#[macro_export]
macro_rules! is_event_requested {
    ($self:expr, $event:pat) => {
        // Return the result of iterating over the queue and matching the event
        if $self.queue.iter().any(|e| matches!(e, AHEvent::Window($event))){
            tracing::info!("requested event checked : {:?}",stringify!($event));

        }
    };
}
pub type AHEvents<UserEvent>=Vec<AHEvent<UserEvent>>;
impl<UserEvent:'static+Clone+Debug> AHEventQueue<UserEvent> {
    pub(crate) fn new<F>(handle_function:F) -> AHEventQueue<UserEvent> where F:FnMut(AHEvents<UserEvent>)->AHAppCmdBuffer + 'static{
        AHEventQueue{ queue: vec![], handle_function:Box::new(handle_function) }
    }

    pub(crate) fn push_window_event(&mut self, event: WindowEvent) {
        self.queue.push(AHEvent::Window(event));
    }
    pub fn is_empty(&self)->bool{
        self.queue.is_empty()
    }



    /// for debug only
    ///
    pub(crate) fn handle(&mut self, active_event_loop: &ActiveEventLoop,window:&Window) {
        let cmd_handler= AHAppCmdHandler::new(active_event_loop,window);
        let cmd_buffer=(self.handle_function)(self.queue.clone());
        cmd_handler.handle(cmd_buffer);
        self.queue.clear();

    }
}