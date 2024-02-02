use client::window::Window;

fn main() {
    common::log_init();
    let mut window = Window::new(800, 600);
    window.grab_cursor(true);
    window.run();
}
