// src/app.rs
// Application principale xfinder

use eframe::egui;
use std::path::PathBuf;
use std::sync::Arc;
use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::search::{FileScanner, SearchIndex, SearchResult, FileWatcher, SearchOptions};
use crate::ui::{render_main_ui, render_side_panel, render_top_panel, render_preview_panel, render_settings_modal, render_statistics_modal};
use crate::audio_player::AudioPlayer;
use crate::database::Database;
use chrono::{DateTime, Local, NaiveDate};

// Message de progression de l'indexation
#[derive(Debug, Clone)]
pub struct IndexProgress {
    pub indexed_count: usize,
    pub total_files: usize,
    pub current_path: String,
}

// Type de fichier pour filtrage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileTypeFilter {
    All,
    Documents,  // pdf, docx, txt, md, odt, rtf
    Images,     // jpg, png, gif, svg, bmp, webp
    Videos,     // mp4, avi, mkv, mov, wmv
    Audio,      // mp3, wav, ogg, flac, m4a
    Archives,   // zip, rar, 7z, tar, gz
    Code,       // rs, js, py, java, cpp, etc.
    Other,
}

impl FileTypeFilter {
    pub fn matches(&self, filename: &str) -> bool {
        let ext = std::path::Path::new(filename)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        match self {
            FileTypeFilter::All => true,
            FileTypeFilter::Documents => matches!(ext.as_str(),
                "pdf" | "docx" | "doc" | "txt" | "md" | "odt" | "rtf" | "xlsx" | "xls" | "pptx" | "ppt"),
            FileTypeFilter::Images => matches!(ext.as_str(),
                "jpg" | "jpeg" | "png" | "gif" | "svg" | "bmp" | "webp" | "ico" | "tiff"),
            FileTypeFilter::Videos => matches!(ext.as_str(),
                "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v"),
            FileTypeFilter::Audio => matches!(ext.as_str(),
                "mp3" | "wav" | "ogg" | "flac" | "m4a" | "wma" | "aac"),
            FileTypeFilter::Archives => matches!(ext.as_str(),
                "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz"),
            FileTypeFilter::Code => matches!(ext.as_str(),
                "rs" | "js" | "ts" | "py" | "java" | "cpp" | "c" | "h" | "cs" | "go" | "rb" | "php" | "html" | "css" | "json" | "xml"),
            FileTypeFilter::Other => !matches!(self, FileTypeFilter::All) &&
                !FileTypeFilter::Documents.matches(filename) &&
                !FileTypeFilter::Images.matches(filename) &&
                !FileTypeFilter::Videos.matches(filename) &&
                !FileTypeFilter::Audio.matches(filename) &&
                !FileTypeFilter::Archives.matches(filename) &&
                !FileTypeFilter::Code.matches(filename),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            FileTypeFilter::All => "Tous",
            FileTypeFilter::Documents => "Documents",
            FileTypeFilter::Images => "Images",
            FileTypeFilter::Videos => "Vidéos",
            FileTypeFilter::Audio => "Audio",
            FileTypeFilter::Archives => "Archives",
            FileTypeFilter::Code => "Code",
            FileTypeFilter::Other => "Autres",
        }
    }
}

// Ordre de tri des résultats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    Relevance,    // Score Tantivy (défaut)
    NameAsc,      // A→Z
    NameDesc,     // Z→A
    DateAsc,      // Ancien → Récent
    DateDesc,     // Récent → Ancien
    SizeAsc,      // Petit → Grand
    SizeDesc,     // Grand → Petit
}

impl SortBy {
    pub fn label(&self) -> &'static str {
        match self {
            SortBy::Relevance => "Pertinence",
            SortBy::NameAsc => "Nom (A→Z)",
            SortBy::NameDesc => "Nom (Z→A)",
            SortBy::DateAsc => "Date (ancien→récent)",
            SortBy::DateDesc => "Date (récent→ancien)",
            SortBy::SizeAsc => "Taille (petit→grand)",
            SortBy::SizeDesc => "Taille (grand→petit)",
        }
    }
}

pub struct XFinderApp {
    pub search_query: String,
    pub search_results: Vec<SearchResult>,      // Résultats filtrés/triés (affichés)
    pub raw_search_results: Vec<SearchResult>,  // Résultats bruts de Tantivy (originaux)
    pub search_index: Option<SearchIndex>,
    pub database: Option<Arc<Database>>,         // Base SQLite pour métadonnées
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
    // Options de recherche avancée
    pub search_exact_match: bool,
    pub search_case_sensitive: bool,
    pub search_in_filename: bool,
    pub search_in_path: bool,
    // Configuration de l'indexation (n-grams)
    pub min_ngram_size: usize,
    pub max_ngram_size: usize,
    // Filtres et tri
    pub filter_file_type: FileTypeFilter,
    pub filter_date_after: Option<NaiveDate>,
    pub filter_size_min: Option<u64>,  // en bytes
    pub filter_size_max: Option<u64>,  // en bytes
    pub sort_by: SortBy,
    // Exclusions d'indexation
    pub excluded_dirs: Vec<String>,      // Dossiers à exclure
    pub excluded_extensions: Vec<String>, // Extensions à exclure (.tmp, .log, etc.)
    pub excluded_patterns: Vec<String>,   // Patterns glob (node_modules, .git, etc.)
    // UI state
    pub show_settings_modal: bool,         // Afficher la fenêtre de paramètres
    pub show_statistics_modal: bool,       // Afficher la fenêtre de statistiques
    pub new_extension_input: String,       // Input temporaire pour ajouter une extension
    pub new_pattern_input: String,         // Input temporaire pour ajouter un pattern
    pub editing_date_filter: bool,         // Mode édition pour le filtre de date
    pub date_filter_input: String,         // Input temporaire pour éditer la date
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

        // Initialiser la database SQLite
        let db_path = dirs::home_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
            .join(".xfinder_index")
            .join("xfinder.db");

        let database = Database::new(&db_path)
            .ok()
            .map(Arc::new);

        Self {
            search_query: String::new(),
            search_results: Vec::new(),
            raw_search_results: Vec::new(),
            search_index: None,
            database,
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
            // Options de recherche par défaut
            search_exact_match: false,
            search_case_sensitive: false,
            search_in_filename: true,
            search_in_path: true,
            // Par défaut: n-grams 2-20 (bon équilibre vitesse/flexibilité)
            min_ngram_size: 2,
            max_ngram_size: 20,
            // Filtres et tri par défaut
            filter_file_type: FileTypeFilter::All,
            filter_date_after: None,
            filter_size_min: None,
            filter_size_max: None,
            sort_by: SortBy::Relevance,
            // Exclusions par défaut (patterns courants)
            excluded_dirs: vec![],
            excluded_extensions: vec![
                ".tmp".to_string(),
                ".log".to_string(),
                ".cache".to_string(),
                ".bak".to_string(),
            ],
            excluded_patterns: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "__pycache__".to_string(),
                "target/debug".to_string(),  // Rust builds
                "target/release".to_string(),
            ],
            // UI state
            show_settings_modal: false,
            show_statistics_modal: false,
            new_extension_input: String::new(),
            new_pattern_input: String::new(),
            editing_date_filter: false,
            date_filter_input: String::new(),
            progress_rx: None,
        }
    }
}

impl XFinderApp {
    pub fn load_index(&mut self) {
        match SearchIndex::new(&self.index_dir, self.min_ngram_size, self.max_ngram_size) {
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
        let min_ngram_size = self.min_ngram_size;
        let max_ngram_size = self.max_ngram_size;
        // Cloner les exclusions pour le thread
        let excluded_extensions = self.excluded_extensions.clone();
        let excluded_patterns = self.excluded_patterns.clone();
        let excluded_dirs = self.excluded_dirs.clone();
        // Cloner la database pour le thread
        let database = self.database.clone();

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
            let index = match SearchIndex::new(&index_dir, min_ngram_size, max_ngram_size) {
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
            let mut db_batch: Vec<crate::database::queries::FileRecord> = Vec::with_capacity(1000);

            // Scanner chaque dossier
            for path_str in &scan_paths {
                let scan_path = PathBuf::from(path_str);

                if let Ok(files) = scanner.scan_directory(
                    &scan_path,
                    files_per_path,
                    &excluded_extensions,
                    &excluded_patterns,
                    &excluded_dirs
                ) {
                    let total_files = files.len();

                    for (i, file) in files.iter().enumerate() {
                        if index.add_file(&mut writer, &file.path, &file.filename).is_ok() {
                            total_indexed += 1;

                            // Collecter métadonnées pour SQLite
                            if let Some(ref db) = database {
                                if let Ok(metadata) = std::fs::metadata(&file.path) {
                                    let now = chrono::Utc::now().timestamp();
                                    let file_record = crate::database::queries::FileRecord {
                                        id: format!("{:x}", file.path.as_bytes().iter().fold(0u64, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u64))),
                                        path: file.path.clone(),
                                        filename: file.filename.clone(),
                                        extension: std::path::Path::new(&file.path)
                                            .extension()
                                            .and_then(|s| s.to_str())
                                            .map(|s| format!(".{}", s)),
                                        size: metadata.len(),
                                        modified: metadata.modified()
                                            .ok()
                                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                            .map(|d| d.as_secs() as i64)
                                            .unwrap_or(now),
                                        created: metadata.created()
                                            .ok()
                                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                            .map(|d| d.as_secs() as i64)
                                            .unwrap_or(now),
                                        hash: None, // Pas de hash pour l'instant (performance)
                                        indexed_at: now,
                                    };
                                    db_batch.push(file_record);

                                    // Batch insert tous les 1000 fichiers
                                    if db_batch.len() >= 1000 {
                                        let _ = db.batch_upsert_files(&db_batch);
                                        db_batch.clear();
                                    }
                                }
                            }

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

            // Flush dernier batch SQLite
            if !db_batch.is_empty() {
                if let Some(ref db) = database {
                    let _ = db.batch_upsert_files(&db_batch);
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
            self.raw_search_results.clear();
            return;
        }

        if let Some(ref index) = self.search_index {
            // Construire les options de recherche
            let options = SearchOptions {
                exact_match: self.search_exact_match,
                case_sensitive: self.search_case_sensitive,
                search_in_filename: self.search_in_filename,
                search_in_path: self.search_in_path,
            };

            // Cherche jusqu'à 10000 résultats pour infinite scroll
            match index.search(&self.search_query, 10000, options) {
                Ok(results) => {
                    // Stocker les résultats bruts de Tantivy
                    self.raw_search_results = results;
                    self.results_display_limit = 50; // Reset à 50
                    // Appliquer les filtres et le tri (copie depuis raw_search_results)
                    self.apply_filters_and_sort();
                    // Ne pas effacer error_message pour garder les infos d'indexation
                }
                Err(e) => {
                    self.error_message = Some(format!("Erreur recherche: {}", e));
                    self.search_results.clear();
                    self.raw_search_results.clear();
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

    // Applique les filtres et le tri sur les résultats de recherche
    pub fn apply_filters_and_sort(&mut self) {
        // Toujours partir d'une copie fraîche des résultats bruts de Tantivy
        // Cela permet de changer de filtre sans perdre les résultats originaux
        self.search_results = self.raw_search_results.clone();

        // 1. Filtrer par type de fichier
        if self.filter_file_type != FileTypeFilter::All {
            self.search_results.retain(|result| {
                self.filter_file_type.matches(&result.filename)
            });
        }

        // 2. Filtrer par date (après une certaine date)
        if let Some(ref date_after) = self.filter_date_after {
            self.search_results.retain(|result| {
                if let Some(ref modified_str) = result.modified {
                    // Parse la date au format "YYYY-MM-DD HH:MM:SS"
                    if let Some(date_part) = modified_str.split(' ').next() {
                        if let Ok(file_date) = chrono::NaiveDate::parse_from_str(date_part, "%Y-%m-%d") {
                            return file_date >= *date_after;
                        }
                    }
                }
                // Garder si pas de date de modification ou erreur de parsing
                true
            });
        }

        // 3. Filtrer par taille
        if let Some(min_size) = self.filter_size_min {
            self.search_results.retain(|result| result.size_bytes >= min_size);
        }
        if let Some(max_size) = self.filter_size_max {
            self.search_results.retain(|result| result.size_bytes <= max_size);
        }

        // 4. Trier les résultats
        match self.sort_by {
            SortBy::Relevance => {
                // Déjà trié par score de Tantivy
            },
            SortBy::NameAsc => {
                self.search_results.sort_by(|a, b| a.filename.to_lowercase().cmp(&b.filename.to_lowercase()));
            },
            SortBy::NameDesc => {
                self.search_results.sort_by(|a, b| b.filename.to_lowercase().cmp(&a.filename.to_lowercase()));
            },
            SortBy::DateAsc => {
                self.search_results.sort_by(|a, b| {
                    match (&a.modified, &b.modified) {
                        (Some(d1), Some(d2)) => d1.cmp(d2),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                });
            },
            SortBy::DateDesc => {
                self.search_results.sort_by(|a, b| {
                    match (&a.modified, &b.modified) {
                        (Some(d1), Some(d2)) => d2.cmp(d1),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                });
            },
            SortBy::SizeAsc => {
                self.search_results.sort_by(|a, b| a.size_bytes.cmp(&b.size_bytes));
            },
            SortBy::SizeDesc => {
                self.search_results.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
            },
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
                match watcher.apply_events_to_index(
                    index,
                    self.database.as_ref(),
                    &self.excluded_extensions,
                    &self.excluded_patterns,
                    &self.excluded_dirs
                ) {
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
        render_settings_modal(ctx, self);
        render_statistics_modal(ctx, self);

        // Redemander un repaint pour traiter les événements en continu
        if self.watchdog_enabled || self.indexing_in_progress {
            ctx.request_repaint();
        }
    }
}
