// src/main.rs
// xfinder - Point d'entrée de l'application

use eframe::egui;

mod app;
mod search;
mod ui;
mod audio_player;
mod database;
mod config;
mod system;
mod hash;

// Tests désactivés temporairement (à corriger)
// #[cfg(test)]
// mod app_test;

use app::XFinderApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("xfinder - Recherche intelligente"),
        ..Default::default()
    };

    eframe::run_native(
        "xfinder",
        options,
        Box::new(|_cc| Box::new(XFinderApp::default())),
    )
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = XFinderApp::default();
        assert_eq!(app.search_query, "");
        assert_eq!(app.search_results.len(), 0);
        assert!(!app.indexing_in_progress);
    }
}
