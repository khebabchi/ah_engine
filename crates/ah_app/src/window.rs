use crate::AHSize;
use std::sync::Arc;
use winit::window::{Fullscreen, Icon};

#[derive(Default)]
pub struct Window {
    window_handle: Option<Arc<winit::window::Window>>,
}
impl Window {
    pub(super) fn is_created(&self) -> bool {
        self.window_handle.is_some()
    }
    pub(super) fn set_handle(&mut self, handle: winit::window::Window) {
        self.window_handle = Some(Arc::new(handle));
    }
    pub(super) fn resize(&self, size: AHSize) {
        if let Some(handle) = self.window_handle.clone() {
            handle.request_inner_size(size.to_physical_size()).unwrap();
        }
    }
    pub fn set_fullscreen(&self, state: bool) -> bool {
        if let Some(handle) = self.window_handle.clone() {
            if state {
                handle.set_fullscreen(Some(Fullscreen::Borderless(None)));
            } else {
                handle.set_fullscreen(None);
            }

            return true;
        }
        false
    }
    pub(super) fn redraw(&self) {
        if let Some(handle) = self.window_handle.clone() {
            handle.request_redraw();
        }
    }
    pub(super) fn inner_size(&self) -> AHSize {
        let size = self.window_handle.as_ref().unwrap().inner_size();
        AHSize::from_physical_size(size)
    }
}
