use image::GenericImageView;
use winit::event::WindowEvent;
use winit::window::Icon;
use ah_app::{AHEvent, Actions};
use ah_app::app_instance::AHApp;

fn main() {
    let mut app =AHApp::new();
    app.set_title("Ah App");
    app.set_icon(load_icon("assets/favicon.png"));
    app.handle_events(&event_handler);
    app.run();
}

fn event_handler(event: &AHEvent,actions:Actions) {
    match event {
        AHEvent::App(_) => {}
        AHEvent::Window(win_event) => {
            match win_event {
                WindowEvent::CloseRequested => {
                    println!(" handled exit event");
                    actions.exit_app()
                },
                WindowEvent::CursorMoved {..}=>{
                    println!(" handled cursor event")
                }
                _=>{}
            }
        }
    }
}

pub fn load_icon(path:&str) -> Icon {
    // Load the icon image from a file (e.g., "icon.png")
    let icon_image = image::open(path).expect("Failed to open icon");
    let (width, height) = icon_image.dimensions();
    let rgba = icon_image.to_rgba8().into_raw();

    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}