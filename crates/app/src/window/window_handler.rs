use std::sync::Arc;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::{Fullscreen, Icon, Window, WindowAttributes};

#[derive(Default)]
pub struct WindowHolder {
    pub(crate) handle: Option<Arc<winit::window::Window>>,
    pub(crate) config:WindowAttributes
}
impl WindowHolder {
    pub(crate) fn is_created(&self) -> bool {
        self.handle.is_some()
    }
    pub fn window_handle(&self)->Arc<Window>{
        self.handle.clone().unwrap()
    }

    pub(crate) fn set_handle(&mut self, handle: Arc<winit::window::Window>) {
        self.handle = Some(handle.clone());
    }
    pub(crate) fn resize(&self, size: PhysicalSize<u32>) {
        if let Some(handle) = self.handle.clone() {
            handle.request_inner_size(size);
        }
    }
    pub(crate) fn set_position(&self, position: PhysicalPosition<u32>) {
        if let Some(handle) = self.handle.clone() {
            handle.set_outer_position(position);
        }
    }
    pub fn set_fullscreen(&self, state: bool) -> bool {
        if let Some(handle) = self.handle.clone() {
            if state {
                handle.set_fullscreen(Some(Fullscreen::Borderless(None)));
            } else {
                handle.set_fullscreen(None);
            }

            return true;
        }
        false
    }
    pub(crate) fn redraw(&self) {
        if let Some(handle) = self.handle.clone() {
            handle.request_redraw();
        }
    }
    pub(crate) fn inner_size(&self) -> PhysicalSize<u32> {
        self.handle.as_ref().unwrap().inner_size()

    }
}
