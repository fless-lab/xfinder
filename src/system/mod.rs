// src/system/mod.rs
// Module pour l'intégration système (tray, auto-start, scheduler)

pub mod tray;
pub mod autostart;
pub mod scheduler;
pub mod window_restore;

pub use tray::SystemTray;
pub use scheduler::Scheduler;
pub use window_restore::{restore_window, hide_from_taskbar, show_in_taskbar};
