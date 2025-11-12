// src/app.rs
// Application principale xfinder

use eframe::egui;
use std::path::PathBuf;

use crate::search::{FileScanner, SearchIndex, SearchResult};
use crate::ui::{render_main_ui, render_side_panel, render_top_panel, render_preview_panel};

pub struct XFinderApp {
    pub search_query: String,
    pub search_results: Vec<SearchResult>,
    pub search_index: Option<SearchIndex>,
    pub index_dir: PathBuf,
    pub scan_path: String,
    pub index_status: IndexStatus,
    pub indexing_in_progress: bool,
    pub error_message: Option<String>,
    pub preview_file_path: Option<String>,
    pub max_files_to_index: usize,
    pub results_display_limit: usize,
}

#[derive(Default)]
pub struct IndexStatus {
    pub is_ready: bool,
    pub file_count: usize,
    pub last_update: Option<String>,
    pub indexed_path: Option<String>,
}

impl Default for XFinderApp {
    fn default() -> Self {
        // Index dans le home dir de l'utilisateur
        let index_dir = dirs::home_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
            .join(".xfinder_index");

        // Dossier par défaut = Downloads
        let default_scan = dirs::download_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_default())
            .to_string_lossy()
            .to_string();

        Self {
            search_query: String::new(),
            search_results: Vec::new(),
            search_index: None,
            index_dir,
            scan_path: default_scan,
            index_status: IndexStatus::default(),
            indexing_in_progress: false,
            error_message: None,
            preview_file_path: None,
            max_files_to_index: 10000,
            results_display_limit: 50, // Affiche 50 résultats au départ
        }
    }
}

impl XFinderApp {
    pub fn load_index(&mut self) {
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

    // Lance une nouvelle indexation complète (efface l'ancien index)
    pub fn start_indexing(&mut self, clear_existing: bool) {
        self.indexing_in_progress = true;
        self.error_message = None;

        if self.search_index.is_none() {
            self.load_index();
        }

        if let Some(ref index) = self.search_index {
            let scan_path = PathBuf::from(&self.scan_path);

            if !scan_path.exists() {
                self.error_message = Some("Dossier inexistant".to_string());
                self.indexing_in_progress = false;
                return;
            }

            // Si demandé, effacer l'index existant
            if clear_existing {
                if let Err(e) = index.clear() {
                    self.error_message = Some(format!("Erreur nettoyage index: {}", e));
                    self.indexing_in_progress = false;
                    return;
                }
            }

            let scanner = FileScanner::new();

            match scanner.scan_directory(&scan_path, self.max_files_to_index) {
                Ok(files) => {
                    match index.create_writer() {
                        Ok(mut writer) => {
                            let mut indexed_count = 0;

                            for file in &files {
                                if index.add_file(&mut writer, &file.path, &file.filename).is_ok() {
                                    indexed_count += 1;
                                }
                            }

                            match writer.commit() {
                                Ok(_) => {
                                    self.index_status.file_count = indexed_count;
                                    self.index_status.last_update = Some(
                                        chrono::Local::now()
                                            .format("%Y-%m-%d %H:%M:%S")
                                            .to_string(),
                                    );
                                    self.index_status.indexed_path = Some(scan_path.display().to_string());
                                    self.error_message = Some(format!(
                                        "{} fichiers indexes ({})",
                                        indexed_count,
                                        scan_path.display()
                                    ));
                                }
                                Err(e) => {
                                    self.error_message = Some(format!("Erreur commit: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Erreur writer: {}", e));
                        }
                    }
                }
                Err(e) => {
                    self.error_message = Some(format!("Erreur scan: {}", e));
                }
            }
        }

        self.indexing_in_progress = false;
    }

    // Rafraîchit l'index actuel (ajoute nouveaux fichiers par-dessus)
    pub fn refresh_index(&mut self) {
        self.start_indexing(false);
    }

    // Vérifie si le chemin à indexer est différent du dernier indexé
    pub fn is_path_changed(&self) -> bool {
        if let Some(ref indexed_path) = self.index_status.indexed_path {
            indexed_path != &self.scan_path
        } else {
            false
        }
    }

    pub fn perform_search(&mut self) {
        if self.search_query.trim().is_empty() {
            self.search_results.clear();
            return;
        }

        if let Some(ref index) = self.search_index {
            // Cherche jusqu'à 10000 résultats pour infinite scroll
            match index.search(&self.search_query, 10000) {
                Ok(results) => {
                    self.search_results = results;
                    self.results_display_limit = 50; // Reset à 50
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("Erreur recherche: {}", e));
                    self.search_results.clear();
                }
            }
        } else {
            self.error_message =
                Some("Index non charge. Lancez une indexation d'abord.".to_string());
        }
    }

    pub fn load_more_results(&mut self) {
        self.results_display_limit += 50;
    }
}

impl eframe::App for XFinderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        render_top_panel(ctx, self);
        render_side_panel(ctx, self);
        render_main_ui(ctx, self);
        render_preview_panel(ctx, self);
    }
}
