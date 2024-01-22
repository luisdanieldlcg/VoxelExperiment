use apecs::{ok, NoDefault, Write};
use common::SysResult;

use crate::render::{resources::DebugRender, Renderer};

pub const DEBUG_SYSTEM: &str = "debug";

pub fn debug_update_system(
    (renderer, debug_render): (Write<Renderer, NoDefault>, Write<DebugRender>),
) -> SysResult {
    log::info!("Debug update system");
    ok()
}
