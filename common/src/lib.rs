pub mod block;
pub mod chunk;
pub mod clock;
pub mod components;
pub mod dir;
pub mod event;
pub mod net;
pub mod resources;
pub mod state;
pub mod uid;

pub type SysResult = apecs::anyhow::Result<apecs::ShouldContinue>;

pub fn init_logger(filters: &str) {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .parse_filters(filters)
        .init();
}
