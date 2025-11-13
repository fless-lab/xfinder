// src/ui/mod.rs
// Modules UI

mod main_panel;
mod side_panel;
mod top_panel;
mod preview_panel;
pub mod icons;

pub use main_panel::render_main_ui;
pub use side_panel::render_side_panel;
pub use top_panel::render_top_panel;
pub use preview_panel::render_preview_panel;
