use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use crate::AHSize;
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
type AHEventListener=Box<dyn FnMut(&AHEvent,Actions)>;
pub struct Actions<'ael,'win>{
    event_loop:&'ael ActiveEventLoop,
    window:&'win mut Window
}
impl<'ael,'win> Actions<'ael,'win>{
    pub fn new(event_loop:&'ael ActiveEventLoop,window:&'win mut Window) -> Actions<'ael,'win>{
        Actions{event_loop, window }
    }
    pub fn exit_app(&self){
        self.event_loop.exit();
    }
    pub fn resize(&mut self, size:AHSize){
        self.window.resize(size);
    }

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

impl AHEventQueue {
    pub(crate) fn new() -> AHEventQueue {
        AHEventQueue{ queue: vec![], handle_function: Box::new(|_,_|{}) }
    }

    pub(crate) fn push_window_event(&mut self, event: WindowEvent) {
        self.queue.push(AHEvent::Window(event));
    }
    pub fn is_empty(&self)->bool{
        self.queue.is_empty()
    }



    /// for debug only
    ///
    pub(crate) fn execute_event_handler(&mut self, active_event_loop: &ActiveEventLoop,window:&mut Window) {
        for event in self.queue.iter() {
            (self.handle_function)(event,Actions::new(active_event_loop,window));
        }
        self.queue.clear();

    }
}