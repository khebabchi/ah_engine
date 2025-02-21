use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop};
use winit::window::{Window, WindowId };
use winit::event::*;
use crate::App;
use crate::time::Time;
use crate::app_manager::AppManager;
use renderer::Renderer;

impl<UserEvent:'static+Clone+Debug+Default+Eq,AM> ApplicationHandler<UserEvent> for App<UserEvent,AM>
where
    UserEvent:'static+Clone+Debug+Default+Eq,
    AM:AppManager<UserEvent>
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.window_holder.is_created() {

            let window = Arc::new(event_loop
                .create_window(self.window_holder.config.clone())
                .unwrap());

            self.window_holder.set_handle(window.clone());
            self.renderer= Some(Renderer::new(window.clone()));

            // execute on_start function
            self.main_loop_manager.on_start(&mut self.event_manager);
            // handle the received window commands
            self.event_manager.app_event_buffer.handle(event_loop,&self.event_proxy.as_ref().unwrap(),&self.window_holder);

            self.time.init();
        }
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        self.event_manager.dispatch_user_event(event);
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        self.renderer.as_mut().unwrap().egui_renderer.handle_input(&self.window_holder.window_handle(),&event);
        if let WindowEvent::RedrawRequested = event {
            self.window_holder.redraw();

            // execute fixed_update function each fixed_dt
            if self.time.fixed_dt_elapsed() { self.main_loop_manager.fixed_update(&mut self.event_manager,Time::FIXED_DELTA_TIME)}

            self.main_loop_manager.update(&mut self.event_manager,self.time.delta_time);

            self.renderer.as_mut().unwrap()
                .handle_redraw(self.window_holder.handle.clone().unwrap(),
                               |renderer| {
                                   self.main_loop_manager.render(renderer);
                                   self.main_loop_manager.render_ui(renderer);
                               });



            // handles window events
            self.event_manager.app_event_buffer.handle(event_loop,&self.event_proxy.as_ref().unwrap(),&self.window_holder);

            // reset events
            self.event_manager.clear();

            self.time.update_dt();
        }
        self.event_manager.dispatch_window_event(event);

    }
}
