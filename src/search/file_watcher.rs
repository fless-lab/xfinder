// src/search/file_watcher.rs
// Surveillance des changements de fichiers en temps réel avec notify

use anyhow::Result;
use crossbeam_channel::{bounded, Receiver, Sender};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

use super::SearchIndex;

#[derive(Debug, Clone)]
pub enum FileEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Removed(PathBuf),
    Renamed { from: PathBuf, to: PathBuf },
}

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    event_rx: Receiver<FileEvent>,
    _watcher_thread: Option<thread::JoinHandle<()>>,
}

impl FileWatcher {
    pub fn new() -> Result<Self> {
        let (event_tx, event_rx) = bounded::<FileEvent>(1000);
        let (notify_tx, notify_rx) = bounded(1000);

        // Thread pour traiter les événements notify et les convertir
        let event_tx_clone = event_tx.clone();
        let watcher_thread = thread::spawn(move || {
            while let Ok(result) = notify_rx.recv() {
                if let Ok(event) = result {
                    if let Some(file_event) = Self::process_notify_event(event) {
                        let _ = event_tx_clone.send(file_event);
                    }
                }
            }
        });

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = notify_tx.send(res);
            },
            Config::default(),
        )?;

        Ok(Self {
            watcher,
            event_rx,
            _watcher_thread: Some(watcher_thread),
        })
    }

    // Surveiller un dossier
    pub fn watch_path(&mut self, path: &Path) -> Result<()> {
        self.watcher.watch(path, RecursiveMode::Recursive)?;
        Ok(())
    }

    // Arrêter de surveiller un dossier
    pub fn unwatch_path(&mut self, path: &Path) -> Result<()> {
        self.watcher.unwatch(path)?;
        Ok(())
    }

    // Récupérer les événements en attente
    pub fn poll_events(&self) -> Vec<FileEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.event_rx.try_recv() {
            events.push(event);
        }
        events
    }

    // Convertir événement notify en FileEvent
    fn process_notify_event(event: Event) -> Option<FileEvent> {
        match event.kind {
            EventKind::Create(_) => {
                if let Some(path) = event.paths.first() {
                    if path.is_file() {
                        return Some(FileEvent::Created(path.clone()));
                    }
                }
            }
            EventKind::Modify(_) => {
                if let Some(path) = event.paths.first() {
                    if path.is_file() {
                        return Some(FileEvent::Modified(path.clone()));
                    }
                }
            }
            EventKind::Remove(_) => {
                if let Some(path) = event.paths.first() {
                    return Some(FileEvent::Removed(path.clone()));
                }
            }
            EventKind::Access(_) => {
                // Rename events sur Windows
                if event.paths.len() == 2 {
                    return Some(FileEvent::Renamed {
                        from: event.paths[0].clone(),
                        to: event.paths[1].clone(),
                    });
                }
            }
            _ => {}
        }
        None
    }

    // Vérifie si un fichier doit être exclu (similaire au scanner)
    fn should_exclude(
        path: &Path,
        filename: &str,
        excluded_extensions: &[String],
        excluded_patterns: &[String],
        excluded_dirs: &[String],
    ) -> bool {
        let path_str = path.to_string_lossy().to_string();

        // 1. Vérifier les dossiers exclus
        for excluded_dir in excluded_dirs {
            if path_str.starts_with(excluded_dir) || path_str.contains(&format!("\\{}", excluded_dir)) {
                return true;
            }
        }

        // 2. Vérifier les extensions exclues
        if let Some(ext) = path.extension() {
            let ext_with_dot = format!(".{}", ext.to_string_lossy());
            if excluded_extensions.contains(&ext_with_dot) {
                return true;
            }
        }

        // 3. Vérifier les patterns
        for pattern in excluded_patterns {
            if path_str.contains(pattern) || filename.contains(pattern) {
                return true;
            }
        }

        false
    }

    // Appliquer les événements à l'index (avec respect des exclusions)
    pub fn apply_events_to_index(
        &self,
        index: &SearchIndex,
        database: Option<&std::sync::Arc<crate::database::Database>>,
        excluded_extensions: &[String],
        excluded_patterns: &[String],
        excluded_dirs: &[String],
    ) -> Result<usize> {
        let events = self.poll_events();
        let mut updated_count = 0;

        for event in events {
            match event {
                FileEvent::Created(path) => {
                    if let Some(filename) = path.file_name() {
                        let path_str = path.to_string_lossy().to_string();
                        let filename_str = filename.to_string_lossy().to_string();

                        // Vérifier si le fichier doit être exclu
                        if Self::should_exclude(&path, &filename_str, excluded_extensions, excluded_patterns, excluded_dirs) {
                            continue; // Skip ce fichier
                        }

                        if let Ok(mut writer) = index.create_writer() {
                            if index.add_file(&mut writer, &path_str, &filename_str).is_ok() {
                                let _ = writer.commit();
                                updated_count += 1;

                                // Ajouter dans SQLite aussi
                                if let Some(db) = database {
                                    if let Ok(metadata) = std::fs::metadata(&path) {
                                        let now = chrono::Utc::now().timestamp();
                                        let file_record = crate::database::queries::FileRecord {
                                            id: format!("{:x}", path_str.as_bytes().iter().fold(0u64, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u64))),
                                            path: path_str.clone(),
                                            filename: filename_str.clone(),
                                            extension: path.extension()
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
                                            hash: None,
                                            indexed_at: now,
                                        };
                                        let _ = db.upsert_file(&file_record);
                                    }
                                }
                            }
                        }
                    }
                }
                FileEvent::Modified(path) => {
                    if let Some(filename) = path.file_name() {
                        let path_str = path.to_string_lossy().to_string();
                        let filename_str = filename.to_string_lossy().to_string();

                        // Vérifier si le fichier doit être exclu
                        if Self::should_exclude(&path, &filename_str, excluded_extensions, excluded_patterns, excluded_dirs) {
                            // Si le fichier est maintenant exclu, le supprimer de l'index et de la DB
                            let _ = index.delete_file_by_path(&path_str);
                            if let Some(db) = database {
                                let _ = db.delete_file(&path_str);
                            }
                            continue;
                        }

                        if index.update_file(&path_str, &filename_str).is_ok() {
                            updated_count += 1;

                            // Mettre à jour dans SQLite aussi
                            if let Some(db) = database {
                                if let Ok(metadata) = std::fs::metadata(&path) {
                                    let now = chrono::Utc::now().timestamp();
                                    let file_record = crate::database::queries::FileRecord {
                                        id: format!("{:x}", path_str.as_bytes().iter().fold(0u64, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u64))),
                                        path: path_str.clone(),
                                        filename: filename_str.clone(),
                                        extension: path.extension()
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
                                        hash: None,
                                        indexed_at: now,
                                    };
                                    let _ = db.upsert_file(&file_record);
                                }
                            }
                        }
                    }
                }
                FileEvent::Removed(path) => {
                    // Toujours supprimer, même si exclu (au cas où il était indexé avant)
                    let path_str = path.to_string_lossy().to_string();
                    if index.delete_file_by_path(&path_str).is_ok() {
                        updated_count += 1;
                    }
                    // Supprimer de SQLite aussi
                    if let Some(db) = database {
                        let _ = db.delete_file(&path_str);
                    }
                }
                FileEvent::Renamed { from, to } => {
                    if let Some(filename) = to.file_name() {
                        let from_str = from.to_string_lossy().to_string();
                        let to_str = to.to_string_lossy().to_string();
                        let filename_str = filename.to_string_lossy().to_string();

                        // Vérifier si le nouveau fichier doit être exclu
                        if Self::should_exclude(&to, &filename_str, excluded_extensions, excluded_patterns, excluded_dirs) {
                            // Si renommé vers un nom exclu, supprimer l'ancien
                            let _ = index.delete_file_by_path(&from_str);
                            continue;
                        }

                        if index.update_file_path(&from_str, &to_str, &filename_str).is_ok() {
                            updated_count += 1;
                        }
                    }
                }
            }
        }

        Ok(updated_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_watcher_creation() {
        let watcher = FileWatcher::new();
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_watch_path() {
        let mut watcher = FileWatcher::new().unwrap();
        let temp_dir = TempDir::new().unwrap();

        let result = watcher.watch_path(temp_dir.path());
        assert!(result.is_ok());

        let result = watcher.unwatch_path(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_file_creation_event() {
        let mut watcher = FileWatcher::new().unwrap();
        let temp_dir = TempDir::new().unwrap();

        watcher.watch_path(temp_dir.path()).unwrap();

        // Créer un fichier
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        // Attendre que l'événement soit traité
        std::thread::sleep(std::time::Duration::from_millis(500));

        let events = watcher.poll_events();
        // Au moins un événement Create devrait être présent
        assert!(events.iter().any(|e| matches!(e, FileEvent::Created(_))));
    }
}
