// src/ui/main_panel.rs
// Panneau central avec recherche et résultats

use eframe::egui;
use crate::app::XFinderApp;

pub fn render_main_ui(ctx: &egui::Context, app: &mut XFinderApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(10.0);

        // Barre de recherche
        ui.horizontal(|ui| {
            ui.label("Rechercher:");
            let response = ui.text_edit_singleline(&mut app.search_query);

            if response.changed()
                || response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
            {
                app.perform_search();
            }

            if ui.button("Rechercher").clicked() {
                app.perform_search();
            }

            if ui.button("Effacer").clicked() {
                app.search_query.clear();
                app.search_results.clear();
                app.error_message = None;
            }
        });

        ui.add_space(10.0);

        // Messages d'erreur ou de succès
        if let Some(ref msg) = app.error_message {
            ui.colored_label(egui::Color32::from_rgb(200, 100, 50), msg);
            ui.add_space(10.0);
        }

        ui.separator();
        ui.add_space(5.0);

        // Résultats de recherche
        ui.label(format!(
            "Resultats: {} fichier(s) trouve(s)",
            app.search_results.len()
        ));
        ui.add_space(5.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (idx, result) in app.search_results.iter().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("#{}", idx + 1));
                        ui.separator();
                        ui.vertical(|ui| {
                            ui.label(format!("Fichier: {}", result.filename));
                            ui.label(format!("Chemin: {}", result.path));
                            ui.label(format!("Score: {:.2}", result.score));
                        });
                    });
                });
                ui.add_space(5.0);
            }

            if app.search_results.is_empty() && !app.search_query.is_empty() {
                ui.label("Aucun resultat. Lancez une indexation d'abord.");
            }
        });
    });
}
