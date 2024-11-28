use std::fmt::{Debug, Display};
use std::hash::Hash;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop};
use winit::window::{Window, WindowId};
use winit::event::*;
use crate::window_instance::AHWindow;

impl<UserEvent:'static+Clone+Debug+Default+Eq> ApplicationHandler<UserEvent> for AHWindow<UserEvent> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.window_handle.is_created() {
            let window = event_loop
                .create_window(Window::default_attributes().with_window_icon(self.icon.clone()).with_title(self.title.clone()))
                .unwrap();
            self.window_handle.set_handle(window);
        }
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        self.event_state.dispatch_user_event(event);
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if let WindowEvent::RedrawRequested = event {
            self.window_handle.redraw();
            self.handle_events(event_loop);
            self.event_state.clear();
        }
        self.event_state.dispatch_window_event(event);

    }
}
