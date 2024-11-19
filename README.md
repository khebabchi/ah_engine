# AH Engine

`ah_engine` is a game engine in its early stages, built using Rust and WGPU with simplicity in mind. This project aims to provide a streamlined and accessible foundation for developing custom game engines or high-performance applications with modern graphics rendering.

## Current Status

- **Initial Development**: The project is just getting started, and the first major milestone, the `ah_app` crate, has been completed.
- **Simplicity First**: The focus is on creating a minimal and clear codebase that can be easily understood and expanded upon.

## Features (Planned)

- **Modular Design**: A simple, modular architecture that allows developers to integrate and modify components effortlessly.
- **WGPU Renderer**: Leveraging WGPU for modern, cross-platform rendering capabilities.
- **Lightweight ECS**: Basic entity-component-system (ECS) functionality for game logic management.
- **User-Friendly**: Minimalistic design to ensure ease of learning and integration.

## Getting Started  with ah_app example

1. **Install Rust**: Ensure you have Rust installed. Follow the instructions on the [official website](https://www.rust-lang.org/).
   
2. **Clone the Repository**:
    ```bash
    git clone https://github.com/yourusername/ah_engine.git
    cd ah_engine
    
    ```

3. **Go to ah_app folder**:
    ```bash
    cd./crates/ah_app
    ```

4. **Run the Example (when available)**:
    ```bash
    cargo run --example main
    ```

## Future Plans

- Add more modules for rendering, input handling, and asset management.
- Implement a basic ECS for handling game logic in a clean, maintainable way.
- Create simple demos to showcase the capabilities of the engine.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
