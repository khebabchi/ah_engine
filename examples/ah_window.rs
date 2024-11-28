use image::GenericImageView;
use winit::dpi::PhysicalSize;
use winit::event::MouseButton;
use winit::keyboard::{KeyCode};
use winit::window::Icon;

fn main() {
    let mut app: AHApp<String> = AHWind::new(
        "App Example".to_string(),
        Some(load_icon("assets/favicon.png")),
        &event_handler,
    );

    app.run();
}

pub fn load_icon(path: &str) -> Icon {
    // Load the icon image from a file (e.g., "icon.png")
    let icon_image = image::open(path).expect("Failed to open icon");
    let (width, height) = icon_image.dimensions();
    let rgba = icon_image.to_rgba8().into_raw();
    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}
