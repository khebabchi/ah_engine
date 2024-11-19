use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::Fullscreen;
use crate::{AHEvent, AHSize};
use crate::window::Window;

pub struct AHAppCmdHandler<'ael,'win> {
    active_event_loop:&'ael ActiveEventLoop,
    window:&'win Window
}
impl<'ael,'win>  AHAppCmdHandler<'ael,'win> {
    pub(crate) fn new(active_event_loop: &'ael ActiveEventLoop, window: &'win Window) -> AHAppCmdHandler<'ael,'win> {
        AHAppCmdHandler{
            active_event_loop,window
        }
    }
    pub(crate) fn handle(self,cmd_buffer:AHAppCmdBuffer){
        for cmd in cmd_buffer{
            match cmd{
                AHAppCmd::Exit => {
                    self.active_event_loop.exit();
                }
                AHAppCmd::ResizeWindow(size) => {
                    self.window.resize(size);
                }
                AHAppCmd::FullScreen => {
                    self.window.set_fullscreen(true);
                }AHAppCmd::Windowed => {
                    self.window.set_fullscreen(false);
                }
            }
        }
    }
}
pub struct AHAppCmdBuffer{
    commands:Vec<AHAppCmd>,
}
impl AHAppCmdBuffer{
    pub fn new() ->AHAppCmdBuffer{
        AHAppCmdBuffer{
            commands:Vec::new(),
        }
    }
    pub fn register_cmd(&mut self,cmd:AHAppCmd){
        self.commands.push(cmd);
    }
}
impl Iterator for AHAppCmdBuffer{
    type Item = AHAppCmd;

    fn next(&mut self) -> Option<Self::Item> {
        self.commands.pop()
    }
}
pub enum AHAppCmd{
    Exit,
    ResizeWindow(AHSize),
    FullScreen,
    Windowed
}