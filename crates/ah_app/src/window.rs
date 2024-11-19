use std::sync::Arc;
use winit::window::Icon;
use crate::AHSize;

#[derive(Default)]
pub struct Window{
    window_handle:Option<Arc<winit::window::Window>>
}
impl Window{
    pub(super) fn is_created(&self) -> bool{
        self.window_handle.is_some()
    }
    pub(super) fn set_handle(&mut self, handle:winit::window::Window){
        self.window_handle=Some(Arc::new(handle));
    }
    pub(super) fn set_icon(&mut self, icon:Icon){

        if let Some(handle)= self.window_handle.clone(){
            handle.set_window_icon(Some(icon));
        }
    }
    pub(super) fn set_title(&mut self, title: &str){

        if let Some(handle)= self.window_handle.clone(){
            handle.set_title(title);
        }
    }
    pub(super) fn resize(&mut self, size: AHSize){

        if let Some(handle)= self.window_handle.clone(){
            handle.request_inner_size(size.to_physical_size()).unwrap();
        }
    }
    pub(super) fn redraw(&mut self){

        if let Some(handle)= self.window_handle.clone(){
            handle.request_redraw();
        }
    }
    pub(super) fn inner_size(&self) -> AHSize{
        let size=self.window_handle.as_ref().unwrap().inner_size();
        AHSize::from_physical_size(size)
    }

}

