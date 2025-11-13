// src/app.rs
// Application principale xfinder

use eframe::egui;
use std::path::PathBuf;
use std::sync::Arc;
use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::search::{FileScanner, SearchIndex, SearchResult, FileWatcher};
use crate::ui::{render_main_ui, render_side_panel, render_top_panel, render_preview_panel};
use crate::audio_player::AudioPlayer;

// Message de progression de l'indexation
#[derive(Debug, Clone)]
pub struct IndexProgress {
    pub indexed_count: usize,
    pub total_files: usize,
    pub current_path: String,
}

pub struct XFinderApp {
    pub search_query: String,
    pub search_results: Vec<SearchResult>,
    pub search_index: Option<SearchIndex>,
    pub file_watcher: Option<FileWatcher>,
    pub audio_player: Option<AudioPlayer>,
    pub index_dir: PathBuf,
    pub scan_paths: Vec<String>,
    pub index_status: IndexStatus,
    pub indexing_in_progress: bool,
    pub error_message: Option<String>,
    pub preview_file_path: Option<String>,
    pub max_files_to_index: usize,
    pub no_file_limit: bool,
    pub results_display_limit: usize,
    pub watchdog_enabled: bool,
    pub watchdog_update_count: usize,
    pub scan_entire_pc: bool,
    progress_rx: Option<Receiver<IndexProgress>>,
}

#[derive(Default)]
pub struct IndexStatus {
    pub is_ready: bool,
    pub file_count: usize,
    pub last_update: Option<String>,
    pub indexed_path: Option<String>,
    pub current_indexed: usize,
    pub total_to_index: usize,
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
            audio_player: AudioPlayer::new().ok(),
            index_dir,
            scan_paths: default_paths,
            index_status: IndexStatus::default(),
            indexing_in_progress: false,
            error_message: None,
            preview_file_path: None,
            max_files_to_index: 100000,
            no_file_limit: false,
            results_display_limit: 50,
            watchdog_enabled: false,
            watchdog_update_count: 0,
            scan_entire_pc: false,
            progress_rx: None,
        }
    }
}

impl XFinderApp {
    pub fn load_index(&mut self) {
        match SearchIndex::new(&self.index_dir) {
            Ok(index) => {
                self.search_index = Some(index);
                self.index_status.is_ready = true;
                // Ne pas effacer error_message ici pour garder le message de succès
            }
            Err(e) => {
                self.error_message = Some(format!("Erreur chargement index: {}", e));
                self.index_status.is_ready = false;
            }
        }
    }

    // Lance une nouvelle indexation dans un thread séparé (pas de freeze UI)
    pub fn start_indexing(&mut self, clear_existing: bool) {
        if self.indexing_in_progress {
            return; // Déjà en cours
        }

        self.indexing_in_progress = true;
        self.error_message = None;
        self.index_status.current_indexed = 0;
        self.index_status.total_to_index = 0;

        // CRITIQUE: Fermer l'ancien index AVANT de le supprimer
        // Sinon les fichiers restent verrouillés et delete_completely() échoue
        if clear_existing {
            self.search_index = None; // Drop l'ancien index pour libérer les fichiers
            self.file_watcher = None; // Fermer le watchdog aussi
            self.index_status.is_ready = false;
        }

        // Ne charger l'index existant QUE si on fait un refresh (pas une nouvelle indexation)
        if self.search_index.is_none() && !clear_existing {
            self.load_index();
        }

        // Vérifier que tous les chemins existent
        for path_str in &self.scan_paths {
            let path = PathBuf::from(path_str);
            if !path.exists() {
                self.error_message = Some(format!("Dossier inexistant: {}", path_str));
                self.indexing_in_progress = false;
                return;
            }
        }

        // Cloner les données nécessaires pour le thread
        let index_dir = self.index_dir.clone();
        let scan_paths = self.scan_paths.clone();
        let max_files = if self.no_file_limit {
            usize::MAX
        } else {
            self.max_files_to_index
        };

        // Créer le channel de progression
        let (progress_tx, progress_rx) = unbounded::<IndexProgress>();
        self.progress_rx = Some(progress_rx);

        // Lancer l'indexation dans un thread séparé
        std::thread::spawn(move || {
            // Effacer complètement si demandé (pour forcer nouveau schéma/tokenizer)
            if clear_existing {
                let _ = SearchIndex::delete_completely(&index_dir);
            }

            // Charger l'index (nouveau schéma si on a effacé)
            let index = match SearchIndex::new(&index_dir) {
                Ok(idx) => idx,
                Err(_) => return,
            };

            let scanner = FileScanner::new();
            let files_per_path = max_files / scan_paths.len().max(1);

            let mut writer = match index.create_writer() {
                Ok(w) => w,
                Err(_) => return,
            };

            let mut total_indexed = 0;

            // Scanner chaque dossier
            for path_str in &scan_paths {
                let scan_path = PathBuf::from(path_str);

                if let Ok(files) = scanner.scan_directory(&scan_path, files_per_path) {
                    let total_files = files.len();

                    for (i, file) in files.iter().enumerate() {
                        if index.add_file(&mut writer, &file.path, &file.filename).is_ok() {
                            total_indexed += 1;

                            // Envoyer progression tous les 10 fichiers
                            if i % 10 == 0 {
                                let _ = progress_tx.send(IndexProgress {
                                    indexed_count: total_indexed,
                                    total_files,
                                    current_path: file.filename.clone(),
                                });
                            }
                        }
                    }
                }
            }

            // Commit final
            let _ = writer.commit();

            // Envoyer progression finale
            let _ = progress_tx.send(IndexProgress {
                indexed_count: total_indexed,
                total_files: total_indexed,
                current_path: "Termine".to_string(),
            });
        });
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

    // Activer le scan de tout le PC (tous les lecteurs)
    pub fn enable_scan_entire_pc(&mut self) {
        self.scan_entire_pc = true;
        self.scan_paths.clear();

        // Détecter tous les lecteurs Windows (A: à Z:)
        for letter in b'A'..=b'Z' {
            let drive = format!("{}:\\", letter as char);
            let path = PathBuf::from(&drive);
            if path.exists() {
                self.scan_paths.push(drive);
            }
        }

        self.error_message = Some(format!("Scan PC complet: {} lecteurs detectes", self.scan_paths.len()));
    }

    pub fn disable_scan_entire_pc(&mut self) {
        self.scan_entire_pc = false;
        self.scan_paths.clear();

        // Remettre le dossier par défaut
        if let Some(downloads) = dirs::download_dir() {
            self.scan_paths.push(downloads.to_string_lossy().to_string());
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
                    // Ne pas effacer error_message pour garder les infos d'indexation
                }
                Err(e) => {
                    self.error_message = Some(format!("Erreur recherche: {}", e));
                    self.search_results.clear();
                }
            }
        } else {
            // Index pas chargé - essayer de le charger
            self.load_index();
            if self.search_index.is_some() {
                // Retry la recherche après chargement
                self.perform_search();
                return;
            }
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

    // Traiter les messages de progression de l'indexation
    fn process_indexing_progress(&mut self) {
        let mut is_done = false;
        let mut final_count = 0;

        if let Some(ref rx) = self.progress_rx {
            while let Ok(progress) = rx.try_recv() {
                self.index_status.current_indexed = progress.indexed_count;
                self.index_status.total_to_index = progress.total_files;

                // Si terminé
                if progress.current_path == "Termine" {
                    is_done = true;
                    final_count = progress.indexed_count;
                }
            }
        }

        if is_done {
            self.indexing_in_progress = false;
            self.index_status.file_count = final_count;
            self.index_status.last_update = Some(
                chrono::Local::now()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            );
            self.index_status.indexed_path = Some(self.scan_paths.join(", "));
            self.error_message = Some(format!(
                "{} fichiers indexes depuis {} dossiers",
                final_count,
                self.scan_paths.len()
            ));
            self.progress_rx = None;

            // Recharger le nouvel index créé par le thread
            self.load_index();
        }
    }
}

impl eframe::App for XFinderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Traiter les événements watchdog à chaque frame (low latency)
        self.process_watchdog_events();

        // Traiter la progression de l'indexation
        self.process_indexing_progress();

        render_top_panel(ctx, self);
        render_side_panel(ctx, self);
        render_main_ui(ctx, self);
        render_preview_panel(ctx, self);

        // Redemander un repaint pour traiter les événements en continu
        if self.watchdog_enabled || self.indexing_in_progress {
            ctx.request_repaint();
        }
    }
}
