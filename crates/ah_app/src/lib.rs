use ::winit::dpi::PhysicalSize;

mod winit;
pub mod window;
pub mod app_instance;
mod event;


#[derive(Clone)]
pub struct AHSize{
    pub width:u32,
    pub height:u32
}
impl AHSize{
    pub fn new(width:u32,height:u32)->AHSize{
        AHSize{
            width,height
        }
    }
    pub fn to_physical_size(&self)->PhysicalSize<u32>{
        PhysicalSize::new(self.width as u32,self.height as u32)
    }
    pub fn from_physical_size(size:PhysicalSize<u32>)->AHSize{
        AHSize{
            width:size.width,
            height:size.height
        }
    }
}
pub use event::*;