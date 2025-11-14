// src/ui/assist_me_ui.rs
// Interface du mode Assist Me (recherche sémantique IA)

use eframe::egui;
use crate::app::XFinderApp;

pub fn render_assist_me_ui(ctx: &egui::Context, app: &mut XFinderApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading("🤖 Assist Me - Recherche Intelligente");
            ui.add_space(20.0);

            // Input question
            ui.horizontal(|ui| {
                let response = ui.add_sized(
                    [ui.available_width() - 120.0, 40.0],
                    egui::TextEdit::singleline(&mut app.assist_me_query)
                        .hint_text("💬 Posez votre question en langage naturel...")
                        .font(egui::FontId::proportional(16.0))
                );

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    app.perform_semantic_search();
                }

                if ui.button("🔍 Rechercher").clicked() {
                    app.perform_semantic_search();
                }
            });

            ui.add_space(20.0);
        });

        ui.separator();

        // Affichage des résultats ou suggestions
        if app.assist_me_loading {
            // État de chargement
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.spinner();
                ui.label("🔍 Recherche sémantique en cours...");
                ui.label("Analyse des documents et calcul de pertinence...");
            });
        } else if !app.assist_me_results.is_empty() {
            // Afficher les résultats
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(10.0);
                ui.label(format!("📚 {} sources pertinentes trouvées", app.assist_me_results.len()));
                ui.add_space(10.0);

                for (idx, source) in app.assist_me_results.iter().enumerate() {
                    render_source_card(ui, idx + 1, source);
                    ui.add_space(10.0);
                }
            });
        } else if app.assist_me_query.is_empty() {
            // État vide : afficher suggestions
            render_suggestions(ui, app);
        } else {
            // Aucun résultat
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label("❌ Aucune source trouvée pour cette question");
                ui.label("💡 Essayez de reformuler ou d'utiliser d'autres mots-clés");
            });
        }
    });
}

fn render_suggestions(ui: &mut egui::Ui, app: &mut XFinderApp) {
    ui.vertical_centered(|ui| {
        ui.add_space(40.0);
        ui.heading("💡 Exemples de questions");
        ui.add_space(20.0);

        let suggestions = vec![
            "Trouve mes factures EDF de 2024",
            "Quels sont les contrats signés ce mois ?",
            "Emails avec pièces jointes importantes",
            "Documents RGPD modifiés récemment",
            "Budget formation validé en janvier",
        ];

        for suggestion in suggestions {
            if ui.button(format!("💬 {}", suggestion)).clicked() {
                app.assist_me_query = suggestion.to_string();
                app.perform_semantic_search();
            }
            ui.add_space(5.0);
        }

        ui.add_space(30.0);
        ui.separator();
        ui.add_space(10.0);

        // Bouton d'indexation manuelle
        ui.horizontal(|ui| {
            if ui.button("🚀 Démarrer l'indexation sémantique").clicked() {
                // Trigger semantic indexing
                app.start_semantic_indexing();
            }

            if app.semantic_indexing_in_progress {
                ui.spinner();
                ui.label(format!("📊 {} fichiers indexés", app.semantic_stats.files_indexed));
            }
        });

        ui.add_space(10.0);

        // Info sur Assist Me
        ui.label("ℹ️ Mode Assist Me - Recherche sémantique");
        ui.label("Posez des questions en langage naturel pour trouver des documents pertinents.");
        ui.label("L'IA analyse le contenu de vos fichiers pour comprendre le sens de votre question.");
    });
}

fn render_source_card(ui: &mut egui::Ui, index: usize, source: &crate::app::AssistMeSource) {
    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .inner_margin(egui::Margin::same(12.0))
        .rounding(egui::Rounding::same(6.0))
        .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
        .show(ui, |ui| {
            // Header : Score + Filename
            ui.horizontal(|ui| {
                // Score badge
                let score_color = if source.score > 0.8 {
                    egui::Color32::from_rgb(40, 167, 69) // vert
                } else if source.score > 0.6 {
                    egui::Color32::from_rgb(255, 193, 7) // orange
                } else {
                    egui::Color32::from_rgb(220, 53, 69) // rouge
                };

                ui.label(
                    egui::RichText::new(format!("#{} • {:.0}%", index, source.score * 100.0))
                        .color(score_color)
                        .strong()
                );

                ui.separator();

                // Filename (cliquable)
                if ui.link(egui::RichText::new(&source.filename).strong()).clicked() {
                    // Ouvrir le fichier
                    let _ = opener::open(&source.file_path);
                }
            });

            ui.add_space(8.0);

            // Excerpt (extrait du chunk)
            ui.label(
                egui::RichText::new(&source.excerpt)
                    .italics()
                    .color(ui.visuals().text_color())
            );

            ui.add_space(8.0);

            // Footer : Actions
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("📁 {}", &source.file_path))
                        .small()
                        .color(ui.visuals().weak_text_color())
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("📂 Dossier").clicked() {
                        // Ouvrir le dossier parent
                        if let Some(parent) = std::path::Path::new(&source.file_path).parent() {
                            let _ = opener::open(parent);
                        }
                    }
                });
            });
        });
}
