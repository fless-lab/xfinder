// src/ui/top_panel.rs
// Panneau supérieur avec contrôles

use eframe::egui;
use crate::app::XFinderApp;

pub fn render_top_panel(ctx: &egui::Context, app: &mut XFinderApp) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.heading("xfinder - Recherche Intelligente");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("📊 Statistiques").clicked() {
                    app.show_statistics_modal = true;
                }
                if app.indexing_in_progress {
                    ui.spinner();
                    ui.label("Indexation en cours...");
                }
            });
        });
        ui.add_space(5.0);
        ui.separator();
    });
}
