use ah_app::window::AHApp;
use ah_app::{AHAppCmd, AHAppCmdBuffer, AHEvents};
use image::GenericImageView;
use winit::dpi::PhysicalSize;
use winit::event::MouseButton;
use winit::keyboard::{KeyCode};
use winit::window::Icon;

fn main() {
    let mut app: AHApp<String> = AHApp::new(
        "App Example".to_string(),
        Some(load_icon("assets/favicon.png")),
        &event_handler,
    );

    app.run();
}

fn event_handler(events: AHEvents<String>) -> AHAppCmdBuffer<String> {
    let mut cmd_buffer = AHAppCmdBuffer::new();

    if events.close_requested() {
        cmd_buffer.register_cmd(AHAppCmd::Exit);
    }
    if events.key_held(KeyCode::KeyW) {
        cmd_buffer.register_cmd(AHAppCmd::ResizeWindow(PhysicalSize::new(700, 700)));

    }
    if events.key_released(KeyCode::KeyS) {
        cmd_buffer.register_cmd(AHAppCmd::ResizeWindow(PhysicalSize::new(500, 500)));
    }
    if events.mouse_pressed(MouseButton::Left) {
        cmd_buffer.register_cmd(AHAppCmd::SendEvent("hello".to_string()));
    }
    if events.key_pressed(KeyCode::Enter) {
        cmd_buffer.register_cmd(AHAppCmd::FullScreen);
    }
    if events.key_pressed(KeyCode::Escape) {
        cmd_buffer.register_cmd(AHAppCmd::Windowed);
    }
    if events.user_event_received("hello".to_string()){
        println!("hello received yaaaaaaay");
    }
    cmd_buffer
}

pub fn load_icon(path: &str) -> Icon {
    // Load the icon image from a file (e.g., "icon.png")
    let icon_image = image::open(path).expect("Failed to open icon");
    let (width, height) = icon_image.dimensions();
    let rgba = icon_image.to_rgba8().into_raw();
    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}
