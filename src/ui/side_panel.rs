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

            ui.label("Dossier a indexer:");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut app.scan_path);
                if ui.button("Parcourir...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        app.scan_path = path.to_string_lossy().to_string();
                    }
                }
            });

            ui.add_space(5.0);

            // Boutons rapides
            ui.horizontal(|ui| {
                if ui.button("Downloads").clicked() {
                    if let Some(downloads) = dirs::download_dir() {
                        app.scan_path = downloads.to_string_lossy().to_string();
                    }
                }

                if ui.button("Documents").clicked() {
                    if let Some(docs) = dirs::document_dir() {
                        app.scan_path = docs.to_string_lossy().to_string();
                    }
                }

                if ui.button("Bureau").clicked() {
                    if let Some(desktop) = dirs::desktop_dir() {
                        app.scan_path = desktop.to_string_lossy().to_string();
                    }
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Configuration de la limite
            ui.label("Limite de fichiers:");
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut app.max_files_to_index, 100..=100000)
                    .logarithmic(true)
                    .text("fichiers"));
            });
            ui.label(format!("(Actuellement: {} fichiers max)", app.max_files_to_index));

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
        });
}
