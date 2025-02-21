use std::fmt::Debug;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use crate:: window::window_handler::WindowHolder;

#[derive(Debug,Default)]
pub struct AppEventBuffer<UserEvent: 'static+Debug+Clone+Default>{
    commands:Vec<AppEvent<UserEvent>>,
}
impl<UserEvent: 'static+Debug+Clone+Default+Eq> AppEventBuffer<UserEvent>{
    pub fn new() ->AppEventBuffer<UserEvent>{
        AppEventBuffer{
            commands:Vec::new(),
        }
    }
    pub fn clear(&mut self){
        self.commands.clear()
    }
    pub(super) fn register_event(&mut self,app_event:AppEvent<UserEvent>){
        self.commands.push(app_event);
    }
    pub(crate) fn handle(&self,active_event_loop: & ActiveEventLoop, event_loop_proxy: & EventLoopProxy<UserEvent>, window: & WindowHolder) {
        for cmd in self.commands.iter(){
            match cmd{
                AppEvent::Exit => {
                    active_event_loop.exit();
                }
                AppEvent::ResizeWindow(size) => {
                    window.set_fullscreen(false);
                    window.resize(size.clone());
                }
                AppEvent::FullScreen => {
                    window.set_fullscreen(true);
                }AppEvent::Windowed => {
                    window.set_fullscreen(false);
                }
                AppEvent::SendEvent(event) => {
                    event_loop_proxy.send_event(event.clone()).unwrap();
                }
                AppEvent::Center => {
                    if let Some(monitor) = active_event_loop.available_monitors().next() {
                        let monitor_size = monitor.size();
                        let window_size = window.inner_size();

                        let x = (monitor_size.width.saturating_sub(window_size.width)) / 2;
                        let y = (monitor_size.height.saturating_sub(window_size.height)) / 2;

                        window.set_position(PhysicalPosition::new(x, y));
                    }
                }
            }
        }
    }
}
impl<UserEvent: 'static+Debug+Clone+Default+Eq> Iterator for AppEventBuffer<UserEvent>{
    type Item = AppEvent<UserEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        self.commands.pop()
    }
}
#[derive(Debug)]
pub enum AppEvent<UserEvent>{
    Exit,
    ResizeWindow(PhysicalSize<u32>),
    FullScreen,
    Windowed,
    Center,
    SendEvent(UserEvent),
}