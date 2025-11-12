// src/main.rs
// xfinder - Recherche intelligente Windows
// Phase 1 : Intégration Tantivy

use eframe::egui;
use std::path::PathBuf;

mod search;
use search::{SearchIndex, SearchResult};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
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
    search_results: Vec<SearchResult>,
    search_index: Option<SearchIndex>,
    index_dir: PathBuf,
    index_status: IndexStatus,
    indexing_in_progress: bool,
    error_message: Option<String>,
}

#[derive(Default)]
struct IndexStatus {
    is_ready: bool,
    file_count: usize,
    last_update: Option<String>,
}

impl Default for XFinderApp {
    fn default() -> Self {
        let index_dir = std::env::current_dir()
            .unwrap_or_default()
            .join(".xfinder_index");

        Self {
            search_query: String::new(),
            search_results: Vec::new(),
            search_index: None,
            index_dir,
            index_status: IndexStatus::default(),
            indexing_in_progress: false,
            error_message: None,
        }
    }
}

impl XFinderApp {
    fn load_index(&mut self) {
        match SearchIndex::new(&self.index_dir) {
            Ok(index) => {
                self.search_index = Some(index);
                self.index_status.is_ready = true;
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(format!("Erreur chargement index: {}", e));
            }
        }
    }

    fn start_indexing(&mut self) {
        self.indexing_in_progress = true;
        self.error_message = None;

        if self.search_index.is_none() {
            self.load_index();
        }

        if let Some(ref index) = self.search_index {
            match index.create_writer() {
                Ok(mut writer) => {
                    // Indexation de fichiers test pour le moment
                    let test_files = vec![
                        ("C:\\Users\\Public\\test1.txt", "test1.txt"),
                        ("C:\\Users\\Public\\test2.pdf", "test2.pdf"),
                        ("C:\\Users\\Public\\document.docx", "document.docx"),
                    ];

                    let mut indexed_count = 0;
                    for (path, filename) in test_files {
                        if let Ok(_) = index.add_file(&mut writer, path, filename) {
                            indexed_count += 1;
                        }
                    }

                    match writer.commit() {
                        Ok(_) => {
                            self.index_status.file_count = indexed_count;
                            self.index_status.last_update = Some(
                                chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                            );
                            self.error_message = Some(format!("{} fichiers indexés avec succès", indexed_count));
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Erreur commit: {}", e));
                        }
                    }
                }
                Err(e) => {
                    self.error_message = Some(format!("Erreur création writer: {}", e));
                }
            }
        }

        self.indexing_in_progress = false;
    }

    fn perform_search(&mut self) {
        if self.search_query.trim().is_empty() {
            self.search_results.clear();
            return;
        }

        if let Some(ref index) = self.search_index {
            match index.search(&self.search_query, 50) {
                Ok(results) => {
                    self.search_results = results;
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("Erreur recherche: {}", e));
                    self.search_results.clear();
                }
            }
        } else {
            self.error_message = Some("Index non chargé. Lancez une indexation d'abord.".to_string());
        }
    }
}

impl eframe::App for XFinderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Panneau supérieur : Statut et contrôles
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.heading("xfinder - Recherche Intelligente");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Lancer Indexation").clicked() && !self.indexing_in_progress {
                        self.start_indexing();
                    }
                    if self.indexing_in_progress {
                        ui.spinner();
                        ui.label("Indexation en cours...");
                    }
                });
            });
            ui.add_space(5.0);
            ui.separator();
        });

        // Panneau latéral gauche : Statut de l'index
        egui::SidePanel::left("side_panel").min_width(280.0).show(ctx, |ui| {
            ui.add_space(10.0);
            ui.heading("Statut de l'Index");
            ui.add_space(10.0);

            ui.separator();

            ui.label(format!(
                "Etat: {}",
                if self.index_status.is_ready {
                    "Pret"
                } else {
                    "Non charge"
                }
            ));

            ui.label(format!(
                "Emplacement: {}",
                self.index_dir.display()
            ));

            ui.label(format!(
                "Fichiers indexes: {}",
                self.index_status.file_count
            ));

            if let Some(ref last_update) = self.index_status.last_update {
                ui.label(format!("Derniere MAJ: {}", last_update));
            } else {
                ui.label("Derniere MAJ: Jamais");
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label("Actions:");
            if ui.button("Charger Index Existant").clicked() {
                self.load_index();
            }

            if ui.button("Rafraichir Statistiques").clicked() {
                // TODO: implementer refresh stats
                self.error_message = Some("Fonctionnalite en cours".to_string());
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label("Informations:");
            ui.label(format!("Dossier courant: {}", std::env::current_dir().unwrap_or_default().display()));
        });

        // Panneau central : Recherche et résultats
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);

            // Barre de recherche
            ui.horizontal(|ui| {
                ui.label("Rechercher:");
                let response = ui.text_edit_singleline(&mut self.search_query);

                if response.changed() || response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.perform_search();
                }

                if ui.button("Rechercher").clicked() {
                    self.perform_search();
                }

                if ui.button("Effacer").clicked() {
                    self.search_query.clear();
                    self.search_results.clear();
                    self.error_message = None;
                }
            });

            ui.add_space(10.0);

            // Messages d'erreur ou de succès
            if let Some(ref msg) = self.error_message {
                ui.colored_label(egui::Color32::from_rgb(200, 100, 50), msg);
                ui.add_space(10.0);
            }

            ui.separator();
            ui.add_space(5.0);

            // Résultats de recherche
            ui.label(format!("Resultats: {} fichier(s) trouve(s)", self.search_results.len()));
            ui.add_space(5.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (idx, result) in self.search_results.iter().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("#{}", idx + 1));
                            ui.separator();
                            ui.vertical(|ui| {
                                ui.label(format!("Fichier: {}", result.filename));
                                ui.label(format!("Chemin: {}", result.path));
                                ui.label(format!("Score: {:.2}", result.score));
                            });
                        });
                    });
                    ui.add_space(5.0);
                }

                if self.search_results.is_empty() && !self.search_query.is_empty() {
                    ui.label("Aucun resultat. Lancez une indexation d'abord.");
                }
            });
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
