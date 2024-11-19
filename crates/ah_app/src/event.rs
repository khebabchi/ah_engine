use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use crate::{AHAppCmdBuffer, AHSize};
use crate::command::AHAppCmdHandler;
use crate::window::Window;

#[derive(Clone,Debug)]
pub enum AHEvent {
    App(AHAppEvent),
    Window(WindowEvent)
}
#[derive(Clone,Eq,PartialEq,Debug)]
pub enum AHAppEvent {
    HandleEvents
}

pub struct AHEventQueue {
    pub queue: Vec<AHEvent>,
    pub handle_function:AHEventListener
}
type AHEventListener=Box<dyn FnMut(AHEvents)->AHAppCmdBuffer>;
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
pub type AHEvents=Vec<AHEvent>;
impl AHEventQueue {
    pub(crate) fn new<F>(handle_function:F) -> AHEventQueue where F:FnMut(AHEvents)->AHAppCmdBuffer + 'static{
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