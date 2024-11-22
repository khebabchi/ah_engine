use std::fmt::Debug;
use winit::dpi::PhysicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use crate::window::Window;

pub struct AHAppCmdHandler<'ael,'win,UserEvent: 'static+Debug+Clone+Default> {
    active_event_loop:&'ael ActiveEventLoop,
    window:&'win Window,
    event_loop_proxy:&'win EventLoopProxy<UserEvent>,
}
impl<'ael,'win,UserEvent: 'static+Debug+Clone+Default+Eq>  AHAppCmdHandler<'ael,'win,UserEvent> {
    pub(crate) fn new(active_event_loop: &'ael ActiveEventLoop,event_loop_proxy: &'win EventLoopProxy<UserEvent>, window: &'win Window) -> AHAppCmdHandler<'ael,'win,UserEvent> {
        AHAppCmdHandler{
            active_event_loop,window,event_loop_proxy
        }
    }
    pub(crate) fn handle(self,cmd_buffer:AHAppCmdBuffer<UserEvent>){
        for cmd in cmd_buffer{
            match cmd{
                AHAppCmd::Exit => {
                    self.active_event_loop.exit();
                }
                AHAppCmd::ResizeWindow(size) => {
                    self.window.set_fullscreen(false);
                    self.window.resize(size);
                }
                AHAppCmd::FullScreen => {
                    self.window.set_fullscreen(true);
                }AHAppCmd::Windowed => {
                    self.window.set_fullscreen(false);
                }
                AHAppCmd::SendEvent(event) => {
                    self.event_loop_proxy.send_event(event).unwrap();
                }
            }
        }
    }
}
pub struct AHAppCmdBuffer<UserEvent: 'static+Debug+Clone+Default>{
    commands:Vec<AHAppCmd<UserEvent>>,
}
impl<UserEvent: 'static+Debug+Clone+Default+Eq> AHAppCmdBuffer<UserEvent>{
    pub fn new() ->AHAppCmdBuffer<UserEvent>{
        AHAppCmdBuffer{
            commands:Vec::new(),
        }
    }
    pub fn register_cmd(&mut self,cmd:AHAppCmd<UserEvent>){
        self.commands.push(cmd);
    }
}
impl<UserEvent: 'static+Debug+Clone+Default+Eq> Iterator for AHAppCmdBuffer<UserEvent>{
    type Item = AHAppCmd<UserEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        self.commands.pop()
    }
}
pub enum AHAppCmd<UserEvent>{
    Exit,
    ResizeWindow(PhysicalSize<u32>),
    FullScreen,
    Windowed,
    SendEvent(UserEvent),
}