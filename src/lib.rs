
mod helpers;
mod main_app;
pub mod audio_manager;

use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::WindowAttributes;
use app::App;
pub use helpers::load_icon;

use crate::main_app::MainApp;

pub fn run(){
    let config =WindowAttributes::default()
        .with_title("ThreDe Engine" )
        .with_window_icon(Some(load_icon("assets/icon.png")))
        .with_inner_size(PhysicalSize::new(1280, 768));



    App::<String,MainApp>::new(MainApp::new(),config).run();
}