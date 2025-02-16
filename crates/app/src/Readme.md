# ah_app

`ah_app` is a lightweight Rust crate designed for building simple and flexible applications with event-driven architectures. It provides easy-to-use APIs for handling window events, keyboard input, and application commands. Ideal for creating desktop applications and games that require custom event handling and window management.
> Made specially for **AH Engine** needs.

## Features

- **Event Handling**: Capture window events as they trigger, and handles them once per frame, built in a pretty optimized way (to be improuved)
- **Command Buffers**: Efficiently manage commands that trigger various actions within the app, like resizing the window or switching to fullscreen.
- **Cross-platform Support**: Built with portability in mind, supporting major platforms such as Windows, macOS, and Linux.
- **Extensible**: Easily extendable to suit your specific needs by adding custom events and commands.

## Example

```Rust
use ah_app::app_instance::AHApp;
use ah_app::{AHAppCmd, AHAppCmdBuffer, AHEvents};
use winit::{keyboard::{KeyCode}, window::Icon, dpi::PhysicalSize};
use image::GenericImageView;

fn main() {
    let mut app: AHApp<()> = AHApp::new("ThreDe Engine".to_string(), Some(load_icon("assets/icon.png")), &event_handler);
    app.run();
}

fn event_handler(events: AHEvents<()>) -> AHAppCmdBuffer {
    let mut cmd_buffer = AHAppCmdBuffer::new();

    if events.close_requested() {
        cmd_buffer.register_cmd(AHAppCmd::Exit);
    }
    if events.key_held(KeyCode::KeyW) {
        cmd_buffer.register_cmd(AHAppCmd::ResizeWindow(PhysicalSize::new(700, 700)));
    }
    if events.key_held(KeyCode::KeyS) {
        cmd_buffer.register_cmd(AHAppCmd::ResizeWindow(PhysicalSize::new(500, 500)));
    }
    if events.key_pressed(KeyCode::Enter) {
        cmd_buffer.register_cmd(AHAppCmd::FullScreen);
    }
    if events.key_pressed(KeyCode::Escape) {
        cmd_buffer.register_cmd(AHAppCmd::Windowed);
    }

    cmd_buffer
}

// -------------------------------- helper function --------------------------------

pub fn load_icon(path: &str) -> Icon {
    let icon_image = image::open(path).expect("Failed to open icon");
    let (width, height) = icon_image.dimensions();
    let rgba = icon_image.to_rgba8().into_raw();
    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}
```
