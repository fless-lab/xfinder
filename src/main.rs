// src/main.rs
// xfinder - Recherche intelligente Windows
// Phase 0 : Hello World egui

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("xfinder - Recherche intelligente"),
        ..Default::default()
    };

    eframe::run_native(
        "xfinder",
        options,
        Box::new(|_cc| Box::new(XFinderApp::default())),
    )
}

struct XFinderApp {
    search_query: String,
}

impl Default for XFinderApp {
    fn default() -> Self {
        Self {
            search_query: String::new(),
        }
    }
}

impl eframe::App for XFinderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔍 xfinder - Recherche intelligente");

            ui.add_space(20.0);

            // Barre de recherche
            ui.horizontal(|ui| {
                ui.label("Rechercher :");
                let response = ui.text_edit_singleline(&mut self.search_query);

                // Focus automatique sur la barre de recherche au démarrage
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.search_query.clear();
                }

                if response.changed() {
                    // TODO: Déclencher la recherche Tantivy (Semaine 1)
                }
            });

            ui.add_space(10.0);

            // Affiche ce que tu tapes (pour tester)
            if !self.search_query.is_empty() {
                ui.label(format!("Recherche : {}", self.search_query));
                ui.add_space(10.0);
                ui.label("⏭️ Prochaine étape : Intégrer Tantivy (Semaine 1)");
            } else {
                ui.label("💡 Tape quelque chose pour commencer...");
            }
        });
    }
}

// ============================================================================
// TESTS (TDD - Test Driven Development)
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = XFinderApp::default();
        assert_eq!(app.search_query, "");
    }

    #[test]
    fn test_search_query_update() {
        let mut app = XFinderApp::default();
        app.search_query = "test.txt".to_string();
        assert_eq!(app.search_query, "test.txt");
    }
}
