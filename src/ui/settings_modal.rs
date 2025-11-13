// src/ui/settings_modal.rs
// Modal de paramètres avec plusieurs onglets

use eframe::egui;
use crate::app::{XFinderApp, SettingsTab};

pub fn render_settings_modal(ctx: &egui::Context, app: &mut XFinderApp) {
    if !app.show_settings_modal {
        return;
    }

    egui::Window::new("⚙️ Paramètres")
        .collapsible(false)
        .resizable(true)
        .default_width(800.0)
        .default_height(600.0)
        .show(ctx, |ui| {
            // Layout vertical principal
            ui.vertical(|ui| {
                // Top: Sélecteur d'onglets horizontal
                ui.horizontal(|ui| {
                    ui.heading("Paramètres");
                    ui.add_space(20.0);

                    if ui.selectable_label(app.settings_tab == SettingsTab::Exclusions, "🚫 Exclusions").clicked() {
                        app.settings_tab = SettingsTab::Exclusions;
                    }
                    if ui.selectable_label(app.settings_tab == SettingsTab::General, "⚙️ Général").clicked() {
                        app.settings_tab = SettingsTab::General;
                    }
                });

                ui.separator();
                ui.add_space(10.0);

                // Contenu scrollable
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        match app.settings_tab {
                            SettingsTab::Exclusions => render_exclusions_tab(ui, app),
                            SettingsTab::General => render_general_tab(ui, app),
                        }
                    });

                ui.add_space(10.0);
                ui.separator();

                // Footer avec boutons
                ui.horizontal(|ui| {
                    if ui.button("✓ Fermer").clicked() {
                        app.show_settings_modal = false;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.small("💾 Configuration sauvegardée automatiquement");
                    });
                });
            });
        });
}

fn render_exclusions_tab(ui: &mut egui::Ui, app: &mut XFinderApp) {
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
                    app.save_config();
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
                            app.save_config();
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
                    app.save_config();
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
                            app.save_config();
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

                // Liste des dossiers (boutons chips)
                let mut dir_to_remove = None;
                ui.horizontal_wrapped(|ui| {
                    for (idx, dir) in app.excluded_dirs.iter().enumerate() {
                        if ui.button(format!("✖ {}", dir)).clicked() {
                            dir_to_remove = Some(idx);
                        }
                    }
                });
                if let Some(idx) = dir_to_remove {
                    app.excluded_dirs.remove(idx);
                    app.save_config();
                }

                ui.add_space(5.0);

                // Bouton pour sélectionner un dossier
                if ui.button("📁 Sélectionner un dossier à exclure").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        let dir_str = path.to_string_lossy().to_string();
                        if !app.excluded_dirs.contains(&dir_str) {
                            app.excluded_dirs.push(dir_str);
                            app.save_config();
                        }
                    }
                }

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);

                // Bouton réinitialiser les exclusions
                if ui.button("🔄 Réinitialiser les exclusions").clicked() {
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
                    app.save_config();
                }

                ui.add_space(5.0);
                ui.colored_label(egui::Color32::from_rgb(200, 150, 50), "⚠ Réindexer pour appliquer les changements");
}

fn render_general_tab(ui: &mut egui::Ui, app: &mut XFinderApp) {
    ui.heading("Paramètres généraux");
    ui.add_space(10.0);

    // Limite d'affichage des résultats
    ui.label("Affichage des résultats:");
    ui.add_space(5.0);

    ui.horizontal(|ui| {
        ui.label("Limite:");
        if ui.add(egui::DragValue::new(&mut app.results_display_limit).speed(10).clamp_range(10..=1000)).changed() {
            app.save_config();
        }
        ui.label("résultats affichés");
    });
    ui.small("Affecte uniquement l'affichage dans la liste, pas la recherche");

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(15.0);

    // Info sur les autres paramètres
    ui.heading("Autres paramètres");
    ui.add_space(5.0);
    ui.label("Les paramètres suivants sont disponibles dans la barre latérale:");
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.label("•");
        ui.label("Watchdog (surveillance temps réel)");
    });
    ui.horizontal(|ui| {
        ui.label("•");
        ui.label("N-grams (configuration de l'indexation)");
    });
    ui.horizontal(|ui| {
        ui.label("•");
        ui.label("Limite de fichiers à indexer");
    });

    ui.add_space(15.0);
    ui.colored_label(egui::Color32::from_rgb(150, 150, 150),
        "💡 Ces paramètres sont dans la sidebar pour un accès rapide");
}
