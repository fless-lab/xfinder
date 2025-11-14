// src/ui/top_panel.rs
// Panneau supérieur avec contrôles et onglets de mode

use eframe::egui;
use crate::app::{XFinderApp, AppMode};

pub fn render_top_panel(ctx: &egui::Context, app: &mut XFinderApp) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(10.0);

        // Première ligne : Titre + Boutons
        ui.horizontal(|ui| {
            ui.heading("xfinder");

            ui.add_space(10.0);

            // Onglets de mode
            if ui.selectable_label(
                matches!(app.current_mode, AppMode::ClassicSearch),
                "🔍 Recherche"
            ).clicked() {
                app.current_mode = AppMode::ClassicSearch;
            }

            if ui.selectable_label(
                matches!(app.current_mode, AppMode::AssistMe),
                "🤖 Assist Me"
            ).clicked() {
                app.current_mode = AppMode::AssistMe;
                // Initialiser le système sémantique à la demande
                app.init_semantic_indexing();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("📊 Statistiques").clicked() {
                    app.show_statistics_modal = true;
                }

                // Afficher l'état d'indexation selon le mode
                if app.indexing_in_progress {
                    ui.spinner();
                    ui.label("Indexation classique...");
                }

                if app.semantic_indexing_in_progress {
                    ui.spinner();
                    ui.label(format!("Indexation IA ({} fichiers)...", app.semantic_stats.files_indexed));
                }
            });
        });

        ui.add_space(5.0);
        ui.separator();
    });
}
