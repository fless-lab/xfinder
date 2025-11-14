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
        .resizable(false)
        .fixed_size([800.0, 550.0])
        .show(ctx, |ui| {
            // Layout vertical principal avec hauteur contrainte
            ui.vertical(|ui| {
                // Top: Sélecteur d'onglets horizontal (hauteur fixe ~40px)
                ui.horizontal(|ui| {
                    ui.heading("Paramètres");
                    ui.add_space(20.0);

                    if ui.selectable_label(app.settings_tab == SettingsTab::Exclusions, "🚫 Exclusions").clicked() {
                        app.settings_tab = SettingsTab::Exclusions;
                    }
                    if ui.selectable_label(app.settings_tab == SettingsTab::General, "⚙️ Général").clicked() {
                        app.settings_tab = SettingsTab::General;
                    }
                    if ui.selectable_label(app.settings_tab == SettingsTab::System, "🖥️ Système").clicked() {
                        app.settings_tab = SettingsTab::System;
                    }
                });

                ui.separator();
                ui.add_space(5.0);

                // Contenu scrollable avec hauteur fixe (550 - 40 header - 50 footer = ~460px)
                egui::ScrollArea::vertical()
                    .max_height(450.0)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        match app.settings_tab {
                            SettingsTab::Exclusions => render_exclusions_tab(ui, app),
                            SettingsTab::General => render_general_tab(ui, app),
                            SettingsTab::System => render_system_tab(ui, app),
                        }
                    });

                ui.add_space(5.0);
                ui.separator();

                // Footer avec boutons (hauteur fixe ~40px)
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

fn render_system_tab(ui: &mut egui::Ui, app: &mut XFinderApp) {
    ui.heading("Paramètres système");
    ui.add_space(10.0);

    // Minimize to tray
    ui.label("Comportement de la fenêtre:");
    ui.add_space(5.0);

    let mut minimize_to_tray = app.config.ui.minimize_to_tray;
    if ui.checkbox(&mut minimize_to_tray, "Minimiser dans la barre système au lieu de quitter").changed() {
        app.config.ui.minimize_to_tray = minimize_to_tray;
        app.save_config();
    }
    ui.small("La fenêtre se masquera dans le system tray lors de la fermeture");

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(15.0);

    // Auto-start
    ui.heading("Démarrage automatique");
    ui.add_space(5.0);

    #[cfg(windows)]
    {
        use crate::system::autostart;

        let is_enabled = autostart::is_autostart_enabled();
        let mut autostart_enabled = app.config.system.autostart_enabled;

        if ui.checkbox(&mut autostart_enabled, "Démarrer xfinder au démarrage de Windows").changed() {
            app.config.system.autostart_enabled = autostart_enabled;

            // Appliquer immédiatement
            if autostart_enabled {
                if let Err(e) = autostart::enable_autostart() {
                    eprintln!("Erreur activation auto-start: {}", e);
                    app.config.system.autostart_enabled = false;
                }
            } else {
                if let Err(e) = autostart::disable_autostart() {
                    eprintln!("Erreur désactivation auto-start: {}", e);
                }
            }

            app.save_config();
        }

        if is_enabled {
            ui.small("✓ Actif dans le registre Windows");
        } else {
            ui.small("✗ Inactif");
        }
    }

    #[cfg(not(windows))]
    {
        ui.label("⚠️ Auto-start non supporté sur cette plateforme");
        ui.small("Cette fonctionnalité est uniquement disponible sur Windows");
    }

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(15.0);

    // Scheduler
    ui.heading("Planification des indexations");
    ui.add_space(5.0);

    let mut scheduler_enabled = app.config.system.scheduler_enabled;
    if ui.checkbox(&mut scheduler_enabled, "Activer l'indexation planifiée automatique").changed() {
        app.config.system.scheduler_enabled = scheduler_enabled;

        // TODO: Démarrer/arrêter le scheduler
        if scheduler_enabled {
            // Créer et démarrer le scheduler si pas déjà fait
            if app.scheduler.is_none() {
                use crate::system::Scheduler;
                let scheduler = Scheduler::new(
                    app.config.system.scheduler_hour,
                    app.config.system.scheduler_minute
                );
                // TODO: Démarrer avec callback d'indexation
                app.scheduler = Some(scheduler);
            }
        } else {
            // Arrêter le scheduler
            app.scheduler = None;
        }

        app.save_config();
    }

    ui.add_space(10.0);

    // Heure de planification
    ui.horizontal(|ui| {
        ui.label("Heure d'indexation:");
        ui.add_space(10.0);

        let mut hour = app.config.system.scheduler_hour;
        let mut minute = app.config.system.scheduler_minute;

        if ui.add(egui::DragValue::new(&mut hour).speed(1).clamp_range(0..=23)).changed() {
            app.config.system.scheduler_hour = hour;
            if let Some(ref scheduler) = app.scheduler {
                scheduler.set_schedule(hour, minute);
            }
            app.save_config();
        }

        ui.label("h");

        if ui.add(egui::DragValue::new(&mut minute).speed(1).clamp_range(0..=59)).changed() {
            app.config.system.scheduler_minute = minute;
            if let Some(ref scheduler) = app.scheduler {
                scheduler.set_schedule(hour, minute);
            }
            app.save_config();
        }

        ui.label("min");
    });

    ui.small(format!("L'indexation sera lancée automatiquement à {:02}:{:02}",
        app.config.system.scheduler_hour, app.config.system.scheduler_minute));

    ui.add_space(10.0);

    // Dernière exécution
    if let Some(ref scheduler) = app.scheduler {
        if let Some(last_run) = scheduler.last_run() {
            ui.small(format!("Dernière exécution: {}",
                last_run.format("%Y-%m-%d %H:%M:%S")));
        } else {
            ui.small("Aucune exécution encore");
        }
    }

    ui.add_space(15.0);
    ui.colored_label(egui::Color32::from_rgb(150, 150, 150),
        "💡 L'indexation planifiée s'exécutera en arrière-plan à l'heure configurée");
}
