use client::window::Window;

fn main() {
    common::tracing_init();
    let mut window = Window::new(800, 600);
    window.run();
}
