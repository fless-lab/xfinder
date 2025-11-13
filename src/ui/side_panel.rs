// src/ui/side_panel.rs
// Panneau latéral avec statut de l'index

use eframe::egui;
use crate::app::XFinderApp;

pub fn render_side_panel(ctx: &egui::Context, app: &mut XFinderApp) {
    egui::SidePanel::left("side_panel")
        .min_width(280.0)
        .show(ctx, |ui| {
            ui.add_space(10.0);
            ui.heading("Statut de l'Index");
            ui.add_space(10.0);

            ui.separator();

            ui.label(format!(
                "Etat: {}",
                if app.index_status.is_ready {
                    "Pret"
                } else {
                    "Non charge"
                }
            ));

            ui.label(format!("Emplacement: {}", app.index_dir.display()));

            ui.label(format!(
                "Fichiers indexes: {}",
                app.index_status.file_count
            ));

            // Afficher la progression pendant l'indexation
            if app.indexing_in_progress {
                ui.add_space(5.0);
                ui.label(format!(
                    "Indexation: {}/{} fichiers",
                    app.index_status.current_indexed,
                    app.index_status.total_to_index
                ));

                // Progress bar (couleur cohérente avec l'UI)
                if app.index_status.total_to_index > 0 {
                    let progress = app.index_status.current_indexed as f32 /
                                  app.index_status.total_to_index as f32;
                    let mut pb = egui::ProgressBar::new(progress)
                        .show_percentage()
                        .animate(true);

                    // Couleur grise/orange comme le reste
                    pb = pb.fill(egui::Color32::from_rgb(200, 150, 100));
                    ui.add(pb);
                }
            }

            if let Some(ref last_update) = app.index_status.last_update {
                ui.label(format!("Derniere MAJ: {}", last_update));
            } else {
                ui.label("Derniere MAJ: Jamais");
            }

            if let Some(ref indexed_path) = app.index_status.indexed_path {
                ui.label("Chemin indexe:");
                ui.label(indexed_path).on_hover_text(indexed_path);
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Option "Scan tout le PC"
            if ui.checkbox(&mut app.scan_entire_pc, "Scan tout le PC (tous les lecteurs)").clicked() {
                if app.scan_entire_pc {
                    app.enable_scan_entire_pc();
                } else {
                    app.disable_scan_entire_pc();
                }
            }

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            ui.label("Dossiers a indexer:");

            // Afficher la liste des dossiers
            let mut to_remove = None;
            for (idx, path) in app.scan_paths.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}. {}", idx + 1, path));
                    if !app.scan_entire_pc && ui.button("X").clicked() {
                        to_remove = Some(idx);
                    }
                });
            }

            if let Some(idx) = to_remove {
                app.remove_scan_path(idx);
            }

            ui.add_space(5.0);

            // Désactiver les contrôles si "Scan tout le PC" est activé
            ui.add_enabled_ui(!app.scan_entire_pc, |ui| {
                // Ajouter un nouveau dossier
                if ui.button("+ Ajouter dossier").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        app.add_scan_path(path.to_string_lossy().to_string());
                    }
                }

                ui.add_space(5.0);

                // Boutons rapides pour ajouter des dossiers communs
                ui.label("Raccourcis:");
                ui.horizontal_wrapped(|ui| {
                    if ui.button("C:\\").clicked() {
                        app.add_scan_path("C:\\".to_string());
                    }
                    if ui.button("D:\\").clicked() {
                        app.add_scan_path("D:\\".to_string());
                    }
                    if ui.button("Downloads").clicked() {
                        if let Some(downloads) = dirs::download_dir() {
                            app.add_scan_path(downloads.to_string_lossy().to_string());
                        }
                    }
                    if ui.button("Documents").clicked() {
                        if let Some(docs) = dirs::document_dir() {
                            app.add_scan_path(docs.to_string_lossy().to_string());
                        }
                    }
                    if ui.button("Bureau").clicked() {
                        if let Some(desktop) = dirs::desktop_dir() {
                            app.add_scan_path(desktop.to_string_lossy().to_string());
                        }
                    }
                });
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Configuration de la limite
            ui.label("Limite de fichiers:");

            if ui.checkbox(&mut app.no_file_limit, "Pas de limite (indexer tous les fichiers)").clicked() {
                // Si on active "Pas de limite", on n'a plus besoin du slider
            }

            // Désactiver le slider si "Pas de limite" est activé
            ui.add_enabled_ui(!app.no_file_limit, |ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(&mut app.max_files_to_index, 1000..=5000000)
                        .logarithmic(true)
                        .text("fichiers"));
                });
                ui.label(format!("(Max: {} fichiers)", app.max_files_to_index));
            });

            if app.no_file_limit {
                ui.label("(Mode: Aucune limite)");
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label("Actions:");

            // Détection de changement de chemin
            let path_changed = app.is_path_changed();
            if path_changed && app.index_status.indexed_path.is_some() {
                ui.colored_label(
                    egui::Color32::from_rgb(255, 165, 0),
                    "ATTENTION: Chemin different de l'index actuel!"
                );
                ui.add_space(5.0);
            }

            ui.horizontal(|ui| {
                if ui.button("Nouvelle Indexation").clicked() {
                    app.start_indexing(true); // Efface l'ancien
                }

                if ui.button("Rafraichir").clicked() {
                    app.refresh_index(); // Ajoute par-dessus
                }
            });

            ui.add_space(5.0);

            if ui.button("Charger Index Existant").clicked() {
                app.load_index();
            }

            // Aide contextuelle
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);
            ui.label("Info:");
            ui.label("- Nouvelle: Efface l'ancien index");
            ui.label("- Rafraichir: Ajoute nouveaux fichiers");

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Watchdog
            ui.label("Surveillance en temps reel:");
            ui.horizontal(|ui| {
                if app.watchdog_enabled {
                    ui.label(format!("ACTIF ({} mises a jour)", app.watchdog_update_count));
                    if ui.button("Desactiver").clicked() {
                        app.disable_watchdog();
                    }
                } else {
                    ui.label("INACTIF");
                    if ui.button("Activer").clicked() {
                        app.enable_watchdog();
                    }
                }
            });

            if app.watchdog_enabled {
                ui.label("Detection auto: ajout/modification/suppression");
            }
        });
}
