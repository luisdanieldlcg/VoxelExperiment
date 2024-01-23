use tracing::Level;

pub fn tracing_init() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
}
