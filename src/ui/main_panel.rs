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

        // Options de recherche avancée
        ui.horizontal(|ui| {
            ui.label("Options:");

            let mut changed = false;

            if ui.checkbox(&mut app.search_exact_match, "Match exact").changed() {
                changed = true;
            }

            if ui.checkbox(&mut app.search_case_sensitive, "Respecter la casse").changed() {
                changed = true;
            }

            ui.separator();
            ui.label("Chercher dans:");

            if ui.checkbox(&mut app.search_in_filename, "Nom").changed() {
                changed = true;
            }

            if ui.checkbox(&mut app.search_in_path, "Chemin").changed() {
                changed = true;
            }

            // Relancer la recherche si une option a changé et qu'il y a une query
            if changed && !app.search_query.trim().is_empty() {
                app.perform_search();
            }
        });

        ui.add_space(10.0);

        // Filtres et tri
        ui.label("Filtres:");
        let mut filters_changed = false;
        ui.horizontal(|ui| {
            // Type de fichier
            ui.label("Type:");
            let old_type = app.filter_file_type;
            egui::ComboBox::from_id_source("filter_type")
                .selected_text(app.filter_file_type.label())
                .show_ui(ui, |ui| {
                    use crate::app::FileTypeFilter;
                    ui.selectable_value(&mut app.filter_file_type, FileTypeFilter::All, FileTypeFilter::All.label());
                    ui.selectable_value(&mut app.filter_file_type, FileTypeFilter::Documents, FileTypeFilter::Documents.label());
                    ui.selectable_value(&mut app.filter_file_type, FileTypeFilter::Images, FileTypeFilter::Images.label());
                    ui.selectable_value(&mut app.filter_file_type, FileTypeFilter::Videos, FileTypeFilter::Videos.label());
                    ui.selectable_value(&mut app.filter_file_type, FileTypeFilter::Audio, FileTypeFilter::Audio.label());
                    ui.selectable_value(&mut app.filter_file_type, FileTypeFilter::Archives, FileTypeFilter::Archives.label());
                    ui.selectable_value(&mut app.filter_file_type, FileTypeFilter::Code, FileTypeFilter::Code.label());
                    ui.selectable_value(&mut app.filter_file_type, FileTypeFilter::Other, FileTypeFilter::Other.label());
                });
            if old_type != app.filter_file_type {
                filters_changed = true;
            }

            ui.separator();

            // Trier par
            ui.label("Trier:");
            let old_sort = app.sort_by;
            egui::ComboBox::from_id_source("sort_by")
                .selected_text(app.sort_by.label())
                .show_ui(ui, |ui| {
                    use crate::app::SortBy;
                    ui.selectable_value(&mut app.sort_by, SortBy::Relevance, SortBy::Relevance.label());
                    ui.selectable_value(&mut app.sort_by, SortBy::NameAsc, SortBy::NameAsc.label());
                    ui.selectable_value(&mut app.sort_by, SortBy::NameDesc, SortBy::NameDesc.label());
                    ui.selectable_value(&mut app.sort_by, SortBy::DateDesc, SortBy::DateDesc.label());
                    ui.selectable_value(&mut app.sort_by, SortBy::DateAsc, SortBy::DateAsc.label());
                    ui.selectable_value(&mut app.sort_by, SortBy::SizeDesc, SortBy::SizeDesc.label());
                    ui.selectable_value(&mut app.sort_by, SortBy::SizeAsc, SortBy::SizeAsc.label());
                });
            if old_sort != app.sort_by {
                filters_changed = true;
            }
        });

        // Re-filtrer et re-trier si changement
        if filters_changed && !app.raw_search_results.is_empty() {
            // Réappliquer les filtres sur les résultats bruts (pas de nouvelle recherche Tantivy)
            app.apply_filters_and_sort();
        }

        ui.add_space(5.0);

        // Filtres avancés (date et taille) dans une section pliable
        egui::CollapsingHeader::new("Filtres avancés")
            .default_open(false)
            .show(ui, |ui| {
                let mut advanced_filters_changed = false;

                // Filtre par date de modification
                ui.horizontal(|ui| {
                    ui.label("Modifié après:");

                    let mut date_enabled = app.filter_date_after.is_some();
                    if ui.checkbox(&mut date_enabled, "").changed() {
                        if date_enabled {
                            // Activer avec date par défaut (30 jours avant aujourd'hui)
                            app.filter_date_after = Some(
                                chrono::Local::now().naive_local().date() - chrono::Duration::days(30)
                            );
                        } else {
                            app.filter_date_after = None;
                        }
                        advanced_filters_changed = true;
                    }

                    if let Some(date) = app.filter_date_after {
                        // Mode édition ou affichage
                        if !app.editing_date_filter {
                            // Affichage normal: label cliquable (mais ne ressemble pas à un input)
                            let date_str = date.format("%Y-%m-%d").to_string();
                            let response = ui.label(&date_str);
                            if response.clicked() {
                                app.editing_date_filter = true;
                                app.date_filter_input = date_str;
                            }
                        } else {
                            // Mode édition: TextEdit
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut app.date_filter_input)
                                    .desired_width(100.0)
                            );

                            // Valider si Enter ou perte de focus
                            let should_validate = response.lost_focus() ||
                                (response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)));

                            if should_validate {
                                // Parser et appliquer la nouvelle date
                                if let Ok(new_date) = chrono::NaiveDate::parse_from_str(&app.date_filter_input, "%Y-%m-%d") {
                                    app.filter_date_after = Some(new_date);
                                    advanced_filters_changed = true;
                                }
                                app.editing_date_filter = false;
                                app.date_filter_input.clear();
                            }

                            // Auto-focus le champ quand on entre en mode édition
                            if !response.has_focus() {
                                response.request_focus();
                            }
                        }

                        // Boutons pour incrémenter/décrémenter les jours
                        if ui.button("-7j").clicked() {
                            app.filter_date_after = Some(date - chrono::Duration::days(7));
                            advanced_filters_changed = true;
                        }
                        if ui.button("-1j").clicked() {
                            app.filter_date_after = Some(date - chrono::Duration::days(1));
                            advanced_filters_changed = true;
                        }
                        if ui.button("+1j").clicked() {
                            app.filter_date_after = Some(date + chrono::Duration::days(1));
                            advanced_filters_changed = true;
                        }
                        if ui.button("+7j").clicked() {
                            app.filter_date_after = Some(date + chrono::Duration::days(7));
                            advanced_filters_changed = true;
                        }
                    } else {
                        ui.label("(désactivé)");
                    }
                });

                ui.add_space(5.0);

                // Filtre par taille de fichier
                ui.label("Taille du fichier:");
                ui.small("(Désactivé temporairement - en cours de refonte)");

                // Bouton pour réinitialiser tous les filtres avancés
                if ui.button("Réinitialiser filtres avancés").clicked() {
                    app.filter_date_after = None;
                    app.filter_size_min = None;
                    app.filter_size_max = None;
                    advanced_filters_changed = true;
                }

                // Appliquer les changements
                if advanced_filters_changed && !app.raw_search_results.is_empty() {
                    app.apply_filters_and_sort();
                }
            });

        ui.add_space(5.0);

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

        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| {
            // Force toute la largeur disponible
            ui.set_width(ui.available_width());

            // N'afficher que jusqu'à la limite
            for (idx, result) in app.search_results.iter()
                .take(app.results_display_limit)
                .enumerate() {
                ui.push_id(idx, |ui| {
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
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
                                    // Sur Windows: ouvrir avec le fichier sélectionné
                                    // Autres OS: ouvrir juste le dossier parent
                                    #[cfg(target_os = "windows")]
                                    {
                                        let _ = std::process::Command::new("explorer")
                                            .args(["/select,", &result.path])
                                            .spawn();
                                    }
                                    #[cfg(not(target_os = "windows"))]
                                    {
                                        if let Some(parent) = std::path::Path::new(&result.path).parent() {
                                            let _ = opener::open(parent);
                                        }
                                    }
                                }
                                if ui.button("Copier chemin").clicked() {
                                    // Copier le chemin dans le presse-papiers
                                    ui.output_mut(|o| o.copied_text = result.path.clone());
                                }
                            });
                        });
                    });
                });
                }); // Fin push_id
                ui.add_space(5.0);
            }

            if app.search_results.is_empty() && !app.search_query.is_empty() {
                if app.search_index.is_some() {
                    ui.label("Aucun resultat pour cette recherche.");
                } else {
                    ui.label("Index non charge. Lancez une indexation d'abord.");
                }
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
