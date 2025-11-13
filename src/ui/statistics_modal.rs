// src/ui/statistics_modal.rs
// Fenêtre modale pour afficher les statistiques de la base de données

use eframe::egui;
use crate::app::XFinderApp;

pub fn render_statistics_modal(ctx: &egui::Context, app: &mut XFinderApp) {
    if !app.show_statistics_modal {
        return;
    }

    egui::Window::new("📊 Statistiques de l'Index")
        .default_width(600.0)
        .default_height(500.0)
        .resizable(true)
        .collapsible(false)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Section: Vue d'ensemble
                ui.heading("📈 Vue d'ensemble");
                ui.add_space(10.0);

                if let Some(ref db) = app.database {
                    // Nombre total de fichiers
                    match db.count_files() {
                        Ok(count) => {
                            ui.horizontal(|ui| {
                                ui.label("Total fichiers indexés :");
                                ui.strong(format!("{}", count));
                            });
                        }
                        Err(e) => {
                            ui.colored_label(
                                egui::Color32::from_rgb(255, 100, 100),
                                format!("Erreur lecture DB: {}", e)
                            );
                        }
                    }

                    ui.add_space(5.0);

                    // Stats par extension
                    match db.stats_by_extension() {
                        Ok(stats) if !stats.is_empty() => {
                            // Calculer taille totale
                            let total_size: u64 = stats.iter().map(|(_, _, size)| size).sum();
                            ui.horizontal(|ui| {
                                ui.label("Taille totale :");
                                ui.strong(format_size(total_size));
                            });

                            ui.add_space(15.0);
                            ui.separator();
                            ui.add_space(10.0);

                            // Section: Répartition par extension
                            ui.heading("📁 Répartition par extension");
                            ui.add_space(10.0);

                            // Afficher jusqu'à 10 extensions max
                            let display_count = stats.len().min(10);

                            ui.push_id("stats_table", |ui| {
                                egui::Grid::new("stats_grid")
                                    .num_columns(3)
                                    .spacing([20.0, 8.0])
                                    .striped(true)
                                    .show(ui, |ui| {
                                        // Header
                                        ui.strong("Extension");
                                        ui.strong("Fichiers");
                                        ui.strong("Taille");
                                        ui.end_row();

                                        // Données
                                        for (ext, count, size) in stats.iter().take(display_count) {
                                            let ext_display = if ext == "no_ext" {
                                                "(sans extension)".to_string()
                                            } else {
                                                ext.clone()
                                            };

                                            ui.label(ext_display);
                                            ui.label(format!("{}", count));
                                            ui.label(format_size(*size));
                                            ui.end_row();
                                        }
                                    });
                            });

                            if stats.len() > 10 {
                                ui.add_space(5.0);
                                ui.label(format!("... et {} autres extensions", stats.len() - 10));
                            }
                        }
                        Ok(_) => {
                            ui.label("Aucune statistique disponible");
                        }
                        Err(e) => {
                            ui.colored_label(
                                egui::Color32::from_rgb(255, 100, 100),
                                format!("Erreur stats: {}", e)
                            );
                        }
                    }

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Section: Recherches récentes
                    ui.heading("🔍 Recherches fréquentes");
                    ui.add_space(10.0);

                    match db.get_top_searches(10) {
                        Ok(searches) if !searches.is_empty() => {
                            ui.push_id("search_history_list", |ui| {
                                for (query, count) in searches {
                                    ui.horizontal(|ui| {
                                        ui.label("•");
                                        ui.label(&query);
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.label(format!("({} fois)", count));
                                        });
                                    });
                                }
                            });
                        }
                        Ok(_) => {
                            ui.label("Aucun historique de recherche");
                        }
                        Err(e) => {
                            ui.colored_label(
                                egui::Color32::from_rgb(255, 100, 100),
                                format!("Erreur historique: {}", e)
                            );
                        }
                    }
                } else {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 200, 100),
                        "⚠ Base de données non disponible"
                    );
                    ui.label("La base de données SQLite n'est pas initialisée.");
                }

                ui.add_space(20.0);

                // Bouton fermer
                ui.separator();
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("Fermer").clicked() {
                        app.show_statistics_modal = false;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("💡 Les données sont automatiquement mises à jour");
                    });
                });
            });
        });
}

/// Formate une taille en bytes en format lisible (Ko, Mo, Go)
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} Go", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} Mo", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} Ko", bytes as f64 / KB as f64)
    } else {
        format!("{} o", bytes)
    }
}
