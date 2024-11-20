use image::GenericImageView;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Icon;
use ah_app::{AHAppCmd, AHAppCmdBuffer, AHAppCmdHandler, AHEvent, AHEvents, AHSize, Actions};
use ah_app::app_instance::AHApp;

fn main() {
    let mut app: AHApp<()>  =AHApp::new("App Example".to_string(),Some(load_icon("assets/favicon.png")),&event_handler);

    app.run();
}

fn event_handler(events: AHEvents<()>)->AHAppCmdBuffer {
    let mut cmd_buffer =AHAppCmdBuffer::new();
    for event in events {
        match event {
            AHEvent::App(_) => {}
            AHEvent::Window(win_event) => {
                match win_event {
                    WindowEvent::CloseRequested => {
                        cmd_buffer.register_cmd(AHAppCmd::Exit);
                    },WindowEvent::KeyboardInput {
                        event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(code),
                            ..
                        },
                        ..
                    }  => {
                        match code {
                            KeyCode::KeyW=>{
                                cmd_buffer.register_cmd(AHAppCmd::ResizeWindow(AHSize::new(700,700)));
                            }
                            KeyCode::KeyS=>{
                                cmd_buffer.register_cmd(AHAppCmd::ResizeWindow(AHSize::new(500,500)));
                            }
                            KeyCode::Enter => {
                                cmd_buffer.register_cmd(AHAppCmd::FullScreen);
                            }
                            KeyCode::Escape => {
                                cmd_buffer.register_cmd(AHAppCmd::Windowed);
                            }
                            _=>{}
                        }

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