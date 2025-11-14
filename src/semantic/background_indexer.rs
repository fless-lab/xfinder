// src/semantic/background_indexer.rs
// Thread d'indexation sémantique en arrière-plan (non-bloquant)

use anyhow::{Context, Result};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use super::SemanticIndexer;

/// Message pour le thread d'indexation background
#[derive(Debug, Clone)]
pub enum IndexingMessage {
    /// Indexer un fichier (file_path, file_id)
    IndexFile(PathBuf, i64),

    /// Indexer un batch de fichiers
    IndexBatch(Vec<(PathBuf, i64)>),

    /// Construire l'index LEANN final
    BuildIndex,

    /// Arrêter le thread
    Stop,
}

/// Statistiques d'indexation
#[derive(Debug, Clone, Default)]
pub struct IndexingStats {
    /// Nombre de fichiers indexés
    pub files_indexed: usize,

    /// Nombre de chunks créés
    pub chunks_created: usize,

    /// Nombre d'erreurs
    pub errors: usize,

    /// Indexation en cours
    pub is_indexing: bool,

    /// Fichier en cours
    pub current_file: Option<String>,
}

/// Thread d'indexation sémantique en background
pub struct BackgroundIndexer {
    /// Sender pour envoyer des messages au thread
    tx: Sender<IndexingMessage>,

    /// Handle du thread
    handle: Option<JoinHandle<()>>,

    /// Statistiques partagées
    stats: Arc<Mutex<IndexingStats>>,
}

impl BackgroundIndexer {
    /// Crée et démarre le thread d'indexation background
    ///
    /// # Arguments
    /// * `indexer` - SemanticIndexer partagé (Arc)
    /// * `batch_size` - Nombre de fichiers à accumuler avant traitement (0 = immédiat)
    pub fn start(
        indexer: Arc<Mutex<SemanticIndexer>>,
        batch_size: usize,
    ) -> Result<Self> {
        let (tx, rx) = unbounded::<IndexingMessage>();
        let stats = Arc::new(Mutex::new(IndexingStats::default()));
        let stats_clone = Arc::clone(&stats);

        let handle = thread::spawn(move || {
            Self::run_indexing_loop(rx, indexer, stats_clone, batch_size);
        });

        Ok(Self {
            tx,
            handle: Some(handle),
            stats,
        })
    }

    /// Boucle principale du thread d'indexation
    fn run_indexing_loop(
        rx: Receiver<IndexingMessage>,
        indexer: Arc<Mutex<SemanticIndexer>>,
        stats: Arc<Mutex<IndexingStats>>,
        batch_size: usize,
    ) {
        let mut pending_batch: Vec<(PathBuf, i64)> = Vec::new();

        loop {
            // Recevoir un message avec timeout
            let message = if pending_batch.is_empty() {
                // Si pas de batch en attente, attendre indéfiniment
                rx.recv().ok()
            } else {
                // Si batch en attente, timeout de 2 secondes
                rx.recv_timeout(Duration::from_secs(2)).ok()
            };

            match message {
                Some(IndexingMessage::IndexFile(path, file_id)) => {
                    if batch_size == 0 {
                        // Mode immédiat
                        Self::index_single_file(&indexer, &stats, path, file_id);
                    } else {
                        // Accumuler dans le batch
                        pending_batch.push((path, file_id));

                        // Si batch plein, traiter
                        if pending_batch.len() >= batch_size {
                            Self::index_batch(&indexer, &stats, &pending_batch);
                            pending_batch.clear();
                        }
                    }
                }

                Some(IndexingMessage::IndexBatch(files)) => {
                    // Indexer le batch immédiatement
                    Self::index_batch(&indexer, &stats, &files);
                }

                Some(IndexingMessage::BuildIndex) => {
                    // Traiter le batch en attente s'il existe
                    if !pending_batch.is_empty() {
                        Self::index_batch(&indexer, &stats, &pending_batch);
                        pending_batch.clear();
                    }

                    // Construire l'index LEANN final
                    Self::build_final_index(&indexer, &stats);
                }

                Some(IndexingMessage::Stop) => {
                    // Traiter le batch en attente avant de s'arrêter
                    if !pending_batch.is_empty() {
                        Self::index_batch(&indexer, &stats, &pending_batch);
                    }
                    break;
                }

                None => {
                    // Timeout : traiter le batch en attente
                    if !pending_batch.is_empty() {
                        Self::index_batch(&indexer, &stats, &pending_batch);
                        pending_batch.clear();
                    }
                }
            }
        }
    }

    /// Indexe un seul fichier
    fn index_single_file(
        indexer: &Arc<Mutex<SemanticIndexer>>,
        stats: &Arc<Mutex<IndexingStats>>,
        path: PathBuf,
        file_id: i64,
    ) {
        // Mettre à jour stats
        {
            let mut s = stats.lock().unwrap();
            s.is_indexing = true;
            s.current_file = Some(path.to_string_lossy().to_string());
        }

        // Indexer
        let result = {
            let idx = indexer.lock().unwrap();
            idx.index_file(&path, file_id)
        };

        // Mettre à jour stats
        let mut s = stats.lock().unwrap();
        match result {
            Ok(chunks) => {
                s.files_indexed += 1;
                s.chunks_created += chunks;
            }
            Err(e) => {
                eprintln!("Semantic indexing error for {:?}: {}", path, e);
                s.errors += 1;
            }
        }
        s.is_indexing = false;
        s.current_file = None;
    }

    /// Indexe un batch de fichiers
    fn index_batch(
        indexer: &Arc<Mutex<SemanticIndexer>>,
        stats: &Arc<Mutex<IndexingStats>>,
        files: &[(PathBuf, i64)],
    ) {
        {
            let mut s = stats.lock().unwrap();
            s.is_indexing = true;
        }

        for (path, file_id) in files {
            let result = {
                let idx = indexer.lock().unwrap();
                idx.index_file(path, *file_id)
            };

            let mut s = stats.lock().unwrap();
            match result {
                Ok(chunks) => {
                    s.files_indexed += 1;
                    s.chunks_created += chunks;
                    s.current_file = Some(path.to_string_lossy().to_string());
                }
                Err(e) => {
                    eprintln!("Semantic indexing error for {:?}: {}", path, e);
                    s.errors += 1;
                }
            }
        }

        let mut s = stats.lock().unwrap();
        s.is_indexing = false;
        s.current_file = None;
    }

    /// Construit l'index LEANN final
    fn build_final_index(
        indexer: &Arc<Mutex<SemanticIndexer>>,
        stats: &Arc<Mutex<IndexingStats>>,
    ) {
        {
            let mut s = stats.lock().unwrap();
            s.is_indexing = true;
            s.current_file = Some("Building LEANN index...".to_string());
        }

        let result = {
            let idx = indexer.lock().unwrap();
            idx.build_index()
        };

        if let Err(e) = result {
            eprintln!("Failed to build LEANN index: {}", e);
            let mut s = stats.lock().unwrap();
            s.errors += 1;
        }

        let mut s = stats.lock().unwrap();
        s.is_indexing = false;
        s.current_file = None;
    }

    /// Envoie un fichier à indexer dans la queue
    pub fn queue_file(&self, path: PathBuf, file_id: i64) -> Result<()> {
        self.tx
            .send(IndexingMessage::IndexFile(path, file_id))
            .context("Failed to send IndexFile message")
    }

    /// Envoie un batch de fichiers à indexer
    pub fn queue_batch(&self, files: Vec<(PathBuf, i64)>) -> Result<()> {
        self.tx
            .send(IndexingMessage::IndexBatch(files))
            .context("Failed to send IndexBatch message")
    }

    /// Demande la construction de l'index final
    pub fn build_index(&self) -> Result<()> {
        self.tx
            .send(IndexingMessage::BuildIndex)
            .context("Failed to send BuildIndex message")
    }

    /// Arrête le thread d'indexation
    pub fn stop(&mut self) -> Result<()> {
        self.tx
            .send(IndexingMessage::Stop)
            .context("Failed to send Stop message")?;

        if let Some(handle) = self.handle.take() {
            handle
                .join()
                .map_err(|_| anyhow::anyhow!("Failed to join background thread"))?;
        }

        Ok(())
    }

    /// Retourne les statistiques actuelles
    pub fn stats(&self) -> IndexingStats {
        self.stats.lock().unwrap().clone()
    }

    /// Réinitialise les statistiques
    pub fn reset_stats(&self) {
        let mut s = self.stats.lock().unwrap();
        s.files_indexed = 0;
        s.chunks_created = 0;
        s.errors = 0;
    }
}

impl Drop for BackgroundIndexer {
    fn drop(&mut self) {
        // Arrêter proprement le thread si pas déjà fait
        let _ = self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    #[ignore] // Nécessite sentence-transformers et LEANN
    fn test_background_indexer_basic() {
        let dir = tempdir().unwrap();
        let index_path = dir.path().join("test_index");

        // Créer un indexer
        let indexer = SemanticIndexer::new(&index_path, "all-MiniLM-L6-v2").unwrap();
        let indexer_arc = Arc::new(Mutex::new(indexer));

        // Démarrer le background indexer
        let mut bg_indexer = BackgroundIndexer::start(indexer_arc, 5).unwrap();

        // Les stats devraient être à 0
        let stats = bg_indexer.stats();
        assert_eq!(stats.files_indexed, 0);

        // Arrêter
        bg_indexer.stop().unwrap();
    }

    #[test]
    fn test_indexing_stats_default() {
        let stats = IndexingStats::default();
        assert_eq!(stats.files_indexed, 0);
        assert_eq!(stats.chunks_created, 0);
        assert_eq!(stats.errors, 0);
        assert!(!stats.is_indexing);
        assert!(stats.current_file.is_none());
    }
}
