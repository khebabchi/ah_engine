use image::GenericImageView;
use winit::event::WindowEvent;
use winit::window::Icon;
use ah_app::{AHAppCmd, AHAppCmdBuffer, AHAppCmdHandler, AHEvent, AHEvents, Actions};
use ah_app::app_instance::AHApp;

fn main() {
    let mut app: AHApp<Option<()>>  =AHApp::new(&event_handler);
    app.set_title("Ah App");
    app.set_icon(load_icon("assets/favicon.png"));
    app.run();
}

fn event_handler(events: AHEvents)->AHAppCmdBuffer {
    let mut cmd_buffer =AHAppCmdBuffer::new();
    for event in events {
        match event {
            AHEvent::App(_) => {}
            AHEvent::Window(win_event) => {
                match win_event {
                    WindowEvent::CloseRequested => {
                        cmd_buffer.register_cmd(AHAppCmd::Exit);
                    }
                    _=>{}
                }
            }
        }
    }

    cmd_buffer
}

pub fn load_icon(path:&str) -> Icon {
    // Load the icon image from a file (e.g., "icon.png")
    let icon_image = image::open(path).expect("Failed to open icon");
    let (width, height) = icon_image.dimensions();
    let rgba = icon_image.to_rgba8().into_raw();

    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}