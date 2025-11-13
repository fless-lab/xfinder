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
        .default_width(700.0)
        .default_height(550.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Menu latéral pour les onglets
                ui.vertical(|ui| {
                    ui.set_min_width(150.0);
                    ui.heading("Sections");
                    ui.separator();
                    ui.add_space(10.0);

                    if ui.selectable_label(app.settings_tab == SettingsTab::Exclusions, "🚫 Exclusions").clicked() {
                        app.settings_tab = SettingsTab::Exclusions;
                    }
                    if ui.selectable_label(app.settings_tab == SettingsTab::Indexation, "📚 Indexation").clicked() {
                        app.settings_tab = SettingsTab::Indexation;
                    }
                    if ui.selectable_label(app.settings_tab == SettingsTab::Interface, "🖥 Interface").clicked() {
                        app.settings_tab = SettingsTab::Interface;
                    }
                });

                ui.separator();

                // Contenu de l'onglet sélectionné
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.set_min_width(500.0);

                    match app.settings_tab {
                        SettingsTab::Exclusions => render_exclusions_tab(ui, app),
                        SettingsTab::Indexation => render_indexation_tab(ui, app),
                        SettingsTab::Interface => render_interface_tab(ui, app),
                    }
                });
            });

            ui.separator();
            ui.add_space(5.0);

            // Bouton fermer en bas
            ui.horizontal(|ui| {
                if ui.button("✓ Fermer").clicked() {
                    app.show_settings_modal = false;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("💾 Configuration sauvegardée automatiquement");
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

fn render_indexation_tab(ui: &mut egui::Ui, app: &mut XFinderApp) {
    ui.heading("Configuration de l'indexation");
    ui.add_space(10.0);

    // N-grams
    ui.label("Taille des n-grams pour la recherche:");
    ui.add_space(5.0);

    ui.horizontal(|ui| {
        ui.label("Min:");
        if ui.add(egui::DragValue::new(&mut app.min_ngram_size).speed(1).clamp_range(1..=10)).changed() {
            if app.min_ngram_size > app.max_ngram_size {
                app.max_ngram_size = app.min_ngram_size;
            }
            app.save_config();
        }

        ui.add_space(20.0);

        ui.label("Max:");
        if ui.add(egui::DragValue::new(&mut app.max_ngram_size).speed(1).clamp_range(1..=30)).changed() {
            if app.max_ngram_size < app.min_ngram_size {
                app.min_ngram_size = app.max_ngram_size;
            }
            app.save_config();
        }
    });
    ui.small(format!("Actuel: {}-{} caractères (redémarrage requis)", app.min_ngram_size, app.max_ngram_size));

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(15.0);

    // Limite de fichiers
    ui.label("Limite de fichiers à indexer:");
    ui.add_space(5.0);

    if ui.checkbox(&mut app.no_file_limit, "Pas de limite (indexer tous les fichiers)").changed() {
        app.save_config();
    }

    ui.add_space(5.0);

    ui.add_enabled_ui(!app.no_file_limit, |ui| {
        ui.horizontal(|ui| {
            ui.label("Limite:");
            if ui.add(egui::DragValue::new(&mut app.max_files_to_index).speed(1000).clamp_range(1000..=10000000)).changed() {
                app.save_config();
            }
            ui.label("fichiers");
        });
    });

    ui.add_space(15.0);
    ui.colored_label(egui::Color32::from_rgb(200, 150, 50), "⚠ Réindexer pour appliquer les changements de n-grams");
}

fn render_interface_tab(ui: &mut egui::Ui, app: &mut XFinderApp) {
    ui.heading("Configuration de l'interface");
    ui.add_space(10.0);

    // Limite d'affichage des résultats
    ui.label("Nombre de résultats à afficher:");
    ui.add_space(5.0);

    ui.horizontal(|ui| {
        ui.label("Limite:");
        if ui.add(egui::DragValue::new(&mut app.results_display_limit).speed(10).clamp_range(10..=1000)).changed() {
            app.save_config();
        }
        ui.label("résultats");
    });
    ui.small("Affecte uniquement l'affichage, pas la recherche");

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(15.0);

    // Watchdog
    ui.label("Watchdog (surveillance temps réel):");
    ui.add_space(5.0);

    let was_enabled = app.watchdog_enabled;
    ui.checkbox(&mut app.watchdog_enabled, "Activer la surveillance en temps réel");

    if was_enabled != app.watchdog_enabled {
        if app.watchdog_enabled {
            app.enable_watchdog();
        } else {
            app.disable_watchdog();
        }
    }

    ui.add_space(5.0);
    ui.small("Le watchdog surveille les changements de fichiers et met à jour l'index automatiquement");

    if app.watchdog_enabled {
        ui.add_space(10.0);
        ui.colored_label(egui::Color32::from_rgb(100, 200, 100),
            format!("✓ Watchdog actif ({} mises à jour)", app.watchdog_update_count));
    }
}
