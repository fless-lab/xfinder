// src/ui/settings_modal.rs
// Modal de paramètres pour les exclusions

use eframe::egui;
use crate::app::XFinderApp;

pub fn render_settings_modal(ctx: &egui::Context, app: &mut XFinderApp) {
    if !app.show_settings_modal {
        return;
    }

    egui::Window::new("⚙️ Paramètres d'exclusion")
        .collapsible(false)
        .resizable(true)
        .default_width(600.0)
        .default_height(500.0)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Extensions exclues
                ui.heading("Extensions exclues");
                ui.label("Fichiers avec ces extensions ne seront pas indexés");
                ui.add_space(5.0);

                // Liste des extensions (boutons chips)
                let mut ext_to_remove = None;
                ui.horizontal_wrapped(|ui| {
                    for (idx, ext) in app.excluded_extensions.iter().enumerate() {
                        if ui.button(format!("✖ {}", ext)).clicked() {
                            ext_to_remove = Some(idx);
                        }
                    }
                });
                if let Some(idx) = ext_to_remove {
                    app.excluded_extensions.remove(idx);
                }

                ui.add_space(5.0);

                // Input pour ajouter une nouvelle extension
                ui.horizontal(|ui| {
                    ui.label("Ajouter:");
                    let response = ui.text_edit_singleline(&mut app.new_extension_input);

                    let can_add = !app.new_extension_input.trim().is_empty()
                        && !app.excluded_extensions.contains(&app.new_extension_input.trim().to_string());

                    if ui.add_enabled(can_add, egui::Button::new("➕ Ajouter")).clicked()
                        || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) && can_add) {
                        let mut ext = app.new_extension_input.trim().to_string();
                        // Ajouter le point si absent
                        if !ext.starts_with('.') {
                            ext = format!(".{}", ext);
                        }
                        if !app.excluded_extensions.contains(&ext) {
                            app.excluded_extensions.push(ext);
                            app.new_extension_input.clear();
                        }
                    }
                });
                ui.small("Format: .ext (ex: .tmp, .log, .bak)");

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(15.0);

                // Patterns exclus
                ui.heading("Patterns exclus");
                ui.label("Fichiers/dossiers correspondant à ces patterns ne seront pas indexés");
                ui.add_space(5.0);

                // Liste des patterns (boutons chips)
                let mut pattern_to_remove = None;
                ui.horizontal_wrapped(|ui| {
                    for (idx, pattern) in app.excluded_patterns.iter().enumerate() {
                        if ui.button(format!("✖ {}", pattern)).clicked() {
                            pattern_to_remove = Some(idx);
                        }
                    }
                });
                if let Some(idx) = pattern_to_remove {
                    app.excluded_patterns.remove(idx);
                }

                ui.add_space(5.0);

                // Input pour ajouter un nouveau pattern
                ui.horizontal(|ui| {
                    ui.label("Ajouter:");
                    let response = ui.text_edit_singleline(&mut app.new_pattern_input);

                    let can_add = !app.new_pattern_input.trim().is_empty()
                        && !app.excluded_patterns.contains(&app.new_pattern_input.trim().to_string());

                    if ui.add_enabled(can_add, egui::Button::new("➕ Ajouter")).clicked()
                        || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) && can_add) {
                        let pattern = app.new_pattern_input.trim().to_string();
                        if !app.excluded_patterns.contains(&pattern) {
                            app.excluded_patterns.push(pattern);
                            app.new_pattern_input.clear();
                        }
                    }
                });
                ui.small("Ex: node_modules, .git, __pycache__, *.tmp");

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(15.0);

                // Dossiers exclus
                ui.heading("Dossiers exclus");
                ui.label("Ces dossiers spécifiques ne seront pas indexés");
                ui.add_space(5.0);

                // Liste des dossiers (boutons, un par ligne car chemins longs)
                let mut dir_to_remove = None;
                for (idx, dir) in app.excluded_dirs.iter().enumerate() {
                    if ui.button(format!("✖ {}", dir)).clicked() {
                        dir_to_remove = Some(idx);
                    }
                }
                if let Some(idx) = dir_to_remove {
                    app.excluded_dirs.remove(idx);
                }

                ui.add_space(5.0);

                // Bouton pour sélectionner un dossier
                if ui.button("📁 Sélectionner un dossier à exclure").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        let dir_str = path.to_string_lossy().to_string();
                        if !app.excluded_dirs.contains(&dir_str) {
                            app.excluded_dirs.push(dir_str);
                        }
                    }
                }

                ui.add_space(20.0);
            });

            ui.separator();
            ui.add_space(5.0);

            // Boutons d'action en bas
            ui.horizontal(|ui| {
                if ui.button("✓ Fermer").clicked() {
                    app.show_settings_modal = false;
                }

                ui.add_space(10.0);

                if ui.button("🔄 Réinitialiser tout").clicked() {
                    // Réinitialiser aux valeurs par défaut
                    app.excluded_extensions = vec![
                        ".tmp".to_string(),
                        ".log".to_string(),
                        ".cache".to_string(),
                        ".bak".to_string(),
                    ];
                    app.excluded_patterns = vec![
                        "node_modules".to_string(),
                        ".git".to_string(),
                        "__pycache__".to_string(),
                        "target/debug".to_string(),
                        "target/release".to_string(),
                    ];
                    app.excluded_dirs.clear();
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("⚠ Réindexer pour appliquer les changements");
                });
            });
        });
}
