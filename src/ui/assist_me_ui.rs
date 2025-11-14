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
                    // TODO: Lancer recherche sémantique
                    app.assist_me_loading = true;
                }

                if ui.button("🔍 Rechercher").clicked() {
                    // TODO: Lancer recherche sémantique
                    app.assist_me_loading = true;
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

fn render_suggestions(ui: &mut egui::Ui, _app: &mut XFinderApp) {
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
                // TODO: Remplir query avec suggestion
            }
            ui.add_space(5.0);
        }

        ui.add_space(30.0);
        ui.separator();
        ui.add_space(10.0);

        // Info sur Assist Me
        ui.label("ℹ️ Mode Assist Me - Recherche sémantique");
        ui.label("Posez des questions en langage naturel pour trouver des documents pertinents.");
        ui.label("L'IA analyse le contenu de vos fichiers pour comprendre le sens de votre question.");
    });
}

fn render_source_card(ui: &mut egui::Ui, index: usize, source: &str) {
    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .inner_margin(egui::Margin::same(10.0))
        .rounding(egui::Rounding::same(5.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("#{}", index));
                ui.separator();
                ui.label(source);
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                if ui.button("📄 Ouvrir").clicked() {
                    // TODO: Ouvrir le fichier
                }
                if ui.button("📁 Dossier").clicked() {
                    // TODO: Ouvrir le dossier
                }
            });
        });
}
