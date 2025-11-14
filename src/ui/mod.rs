// src/ui/mod.rs
// Modules UI

mod main_panel;
mod side_panel;
mod top_panel;
mod preview_panel;
mod settings_modal;
mod statistics_modal;
mod assist_me_ui;
pub mod icons;

pub use main_panel::render_main_ui;
pub use side_panel::render_side_panel;
pub use top_panel::render_top_panel;
pub use preview_panel::render_preview_panel;
pub use settings_modal::render_settings_modal;
pub use statistics_modal::render_statistics_modal;
pub use assist_me_ui::render_assist_me_ui;
