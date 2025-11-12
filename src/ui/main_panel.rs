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
        let total_results = app.search_results.len();
        let displayed = app.results_display_limit.min(total_results);

        ui.label(format!(
            "Resultats: {} trouve(s) - Affichage: {}/{}",
            total_results, displayed, total_results
        ));
        ui.add_space(5.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            // N'afficher que jusqu'à la limite
            for (idx, result) in app.search_results.iter()
                .take(app.results_display_limit)
                .enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("#{}", idx + 1));
                        ui.separator();
                        ui.vertical(|ui| {
                            ui.label(format!("Fichier: {}", result.filename));
                            ui.label(format!("Chemin: {}", result.path));

                            // Métadonnées: taille, dates
                            let size_kb = result.size_bytes as f64 / 1024.0;
                            let size_str = if size_kb > 1024.0 {
                                format!("{:.2} MB", size_kb / 1024.0)
                            } else {
                                format!("{:.2} KB", size_kb)
                            };
                            ui.label(format!("Taille: {}", size_str));

                            if let Some(ref created) = result.created {
                                ui.label(format!("Cree: {}", created));
                            }
                            if let Some(ref modified) = result.modified {
                                ui.label(format!("Modifie: {}", modified));
                            }

                            ui.label(format!("Score: {:.2}", result.score));

                            // Boutons d'action
                            ui.horizontal(|ui| {
                                if ui.button("Ouvrir").clicked() {
                                    // Ouvrir le fichier avec l'app par défaut
                                    let _ = opener::open(&result.path);
                                }
                                if ui.button("Previsualiser").clicked() {
                                    app.preview_file_path = Some(result.path.clone());
                                }
                                if ui.button("Dossier").clicked() {
                                    // Ouvrir le dossier contenant le fichier
                                    if let Some(parent) = std::path::Path::new(&result.path).parent() {
                                        let _ = opener::open(parent);
                                    }
                                }
                            });
                        });
                    });
                });
                ui.add_space(5.0);
            }

            if app.search_results.is_empty() && !app.search_query.is_empty() {
                ui.label("Aucun resultat. Lancez une indexation d'abord.");
            }

            // Bouton "Charger plus" si il y a encore des résultats
            if displayed < total_results {
                ui.add_space(10.0);
                ui.separator();
                if ui.button(format!("Charger {} resultats supplementaires...",
                    (total_results - displayed).min(50))).clicked() {
                    app.load_more_results();
                }
            }
        });
    });
}
