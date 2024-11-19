use crate::app_instance::AHApp;
use crate::event::AHAppEvent;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop};
use winit::window::{Window, WindowId};
use winit::event::*
;

impl ApplicationHandler<AHAppEvent> for AHApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.window.is_created() {
            let window = event_loop
                .create_window(Window::default_attributes())
                .unwrap();
            self.window.set_handle(window);
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if let WindowEvent::RedrawRequested = event {
            self.window.redraw();
            self.event_queue.execute_event_handler(event_loop, &mut self.window);
        }
        self.event_queue.push_window_event(event);

    }
}
