// src/app.rs
// Application principale xfinder

use eframe::egui;
use std::path::PathBuf;

use crate::search::{FileScanner, SearchIndex, SearchResult, FileWatcher};
use crate::ui::{render_main_ui, render_side_panel, render_top_panel, render_preview_panel};

pub struct XFinderApp {
    pub search_query: String,
    pub search_results: Vec<SearchResult>,
    pub search_index: Option<SearchIndex>,
    pub file_watcher: Option<FileWatcher>,
    pub index_dir: PathBuf,
    pub scan_paths: Vec<String>,
    pub index_status: IndexStatus,
    pub indexing_in_progress: bool,
    pub error_message: Option<String>,
    pub preview_file_path: Option<String>,
    pub max_files_to_index: usize,
    pub results_display_limit: usize,
    pub watchdog_enabled: bool,
    pub watchdog_update_count: usize,
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

        // Dossiers par défaut
        let default_paths = vec![
            dirs::download_dir()
                .unwrap_or_else(|| dirs::home_dir().unwrap_or_default())
                .to_string_lossy()
                .to_string()
        ];

        Self {
            search_query: String::new(),
            search_results: Vec::new(),
            search_index: None,
            file_watcher: None,
            index_dir,
            scan_paths: default_paths,
            index_status: IndexStatus::default(),
            indexing_in_progress: false,
            error_message: None,
            preview_file_path: None,
            max_files_to_index: 10000,
            results_display_limit: 50,
            watchdog_enabled: false,
            watchdog_update_count: 0,
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
            // Vérifier que tous les chemins existent
            for path_str in &self.scan_paths {
                let path = PathBuf::from(path_str);
                if !path.exists() {
                    self.error_message = Some(format!("Dossier inexistant: {}", path_str));
                    self.indexing_in_progress = false;
                    return;
                }
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
            let mut total_indexed = 0;
            let files_per_path = self.max_files_to_index / self.scan_paths.len().max(1);

            match index.create_writer() {
                Ok(mut writer) => {
                    // Scanner chaque dossier
                    for path_str in &self.scan_paths {
                        let scan_path = PathBuf::from(path_str);

                        match scanner.scan_directory(&scan_path, files_per_path) {
                            Ok(files) => {
                                for file in &files {
                                    if index.add_file(&mut writer, &file.path, &file.filename).is_ok() {
                                        total_indexed += 1;
                                    }
                                }
                            }
                            Err(e) => {
                                self.error_message = Some(format!("Erreur scan {}: {}", path_str, e));
                            }
                        }
                    }

                    match writer.commit() {
                        Ok(_) => {
                            self.index_status.file_count = total_indexed;
                            self.index_status.last_update = Some(
                                chrono::Local::now()
                                    .format("%Y-%m-%d %H:%M:%S")
                                    .to_string(),
                            );
                            self.index_status.indexed_path = Some(
                                self.scan_paths.join(", ")
                            );
                            self.error_message = Some(format!(
                                "{} fichiers indexes depuis {} dossiers",
                                total_indexed,
                                self.scan_paths.len()
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

        self.indexing_in_progress = false;
    }

    // Rafraîchit l'index actuel (ajoute nouveaux fichiers par-dessus)
    pub fn refresh_index(&mut self) {
        self.start_indexing(false);
    }

    // Vérifie si les chemins à indexer sont différents des derniers indexés
    pub fn is_path_changed(&self) -> bool {
        if let Some(ref indexed_path) = self.index_status.indexed_path {
            indexed_path != &self.scan_paths.join(", ")
        } else {
            false
        }
    }

    pub fn add_scan_path(&mut self, path: String) {
        if !self.scan_paths.contains(&path) {
            self.scan_paths.push(path);
        }
    }

    pub fn remove_scan_path(&mut self, index: usize) {
        if index < self.scan_paths.len() {
            self.scan_paths.remove(index);
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

    // Active le watchdog sur tous les dossiers surveillés
    pub fn enable_watchdog(&mut self) {
        if self.watchdog_enabled {
            return; // Déjà activé
        }

        match FileWatcher::new() {
            Ok(mut watcher) => {
                // Surveiller tous les dossiers
                for path_str in &self.scan_paths {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        if let Err(e) = watcher.watch_path(&path) {
                            self.error_message = Some(format!("Erreur watchdog {}: {}", path_str, e));
                            return;
                        }
                    }
                }

                self.file_watcher = Some(watcher);
                self.watchdog_enabled = true;
                self.error_message = Some(format!("Watchdog active sur {} dossiers", self.scan_paths.len()));
            }
            Err(e) => {
                self.error_message = Some(format!("Erreur init watchdog: {}", e));
            }
        }
    }

    // Désactive le watchdog
    pub fn disable_watchdog(&mut self) {
        self.file_watcher = None;
        self.watchdog_enabled = false;
        self.error_message = Some("Watchdog desactive".to_string());
    }

    // Appliquer les changements du watchdog à l'index
    // Appelé à chaque frame pour low latency
    pub fn process_watchdog_events(&mut self) {
        if !self.watchdog_enabled {
            return;
        }

        if let Some(ref watcher) = self.file_watcher {
            if let Some(ref index) = self.search_index {
                match watcher.apply_events_to_index(index) {
                    Ok(count) if count > 0 => {
                        self.watchdog_update_count += count;
                        self.index_status.file_count += count; // Approximatif
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Erreur watchdog: {}", e));
                    }
                    _ => {}
                }
            }
        }
    }
}

impl eframe::App for XFinderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Traiter les événements watchdog à chaque frame (low latency)
        self.process_watchdog_events();

        render_top_panel(ctx, self);
        render_side_panel(ctx, self);
        render_main_ui(ctx, self);
        render_preview_panel(ctx, self);

        // Redemander un repaint pour traiter les événements en continu
        if self.watchdog_enabled {
            ctx.request_repaint();
        }
    }
}
