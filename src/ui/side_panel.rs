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

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label("Actions:");
            if ui.button("Charger Index Existant").clicked() {
                app.load_index();
            }

            if ui.button("Rafraichir Statistiques").clicked() {
                app.error_message = Some("Fonctionnalite en cours".to_string());
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label("Informations:");
            ui.label(format!(
                "Dossier courant: {}",
                std::env::current_dir().unwrap_or_default().display()
            ));
        });
}
