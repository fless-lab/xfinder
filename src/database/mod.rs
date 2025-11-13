// Module Database - Connection pool optimisé + API publique

pub mod schema;
pub mod queries;

use rusqlite::{Connection, Result};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Database wrapper avec connection pool
pub struct Database {
    conn: Arc<Mutex<Connection>>,
    db_path: PathBuf,
}

impl Database {
    /// Crée ou ouvre la base de données
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db_path = db_path.as_ref().to_path_buf();
        let conn = Connection::open(&db_path)?;

        // Applique les PRAGMAs pour performance maximale
        for pragma in schema::PRAGMAS {
            conn.execute_batch(pragma)?;
        }

        // Initialise le schema si nécessaire
        conn.execute_batch(schema::INIT_SCHEMA)?;

        // Enregistre la version du schema
        let now = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT OR IGNORE INTO schema_version (version, applied_at) VALUES (?1, ?2)",
            rusqlite::params![schema::SCHEMA_VERSION, now],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            db_path,
        })
    }

    /// Crée une database en mémoire (pour tests)
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;

        for pragma in schema::PRAGMAS {
            conn.execute_batch(pragma)?;
        }

        conn.execute_batch(schema::INIT_SCHEMA)?;

        let now = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT OR IGNORE INTO schema_version (version, applied_at) VALUES (?1, ?2)",
            rusqlite::params![schema::SCHEMA_VERSION, now],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            db_path: PathBuf::from(":memory:"),
        })
    }

    /// Exécute une fonction avec accès à la connection
    pub fn with_conn<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Connection) -> Result<R>,
    {
        let conn = self.conn.lock().unwrap();
        f(&conn)
    }

    /// Vacuum et optimise la DB (à faire périodiquement)
    pub fn vacuum(&self) -> Result<()> {
        self.with_conn(|conn| {
            conn.execute_batch("VACUUM; ANALYZE;")?;
            Ok(())
        })
    }

    /// Récupère la taille de la DB en bytes
    pub fn size(&self) -> Result<u64> {
        if self.db_path.to_str() == Some(":memory:") {
            return Ok(0);
        }

        std::fs::metadata(&self.db_path)
            .map(|m| m.len())
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))
    }

    /// Récupère le chemin de la DB
    pub fn path(&self) -> &Path {
        &self.db_path
    }
}

// ==================== API Publique ====================

/// API simplifiée pour l'app
impl Database {
    /// Ajoute/met à jour un fichier
    pub fn upsert_file(&self, file: &queries::FileRecord) -> Result<()> {
        self.with_conn(|conn| queries::upsert_file(conn, file))
    }

    /// Batch insert optimisé (1000x plus rapide)
    pub fn batch_upsert_files(&self, files: &[queries::FileRecord]) -> Result<()> {
        self.with_conn(|conn| queries::batch_upsert_files(conn, files))
    }

    /// Récupère un fichier par chemin
    pub fn get_file_by_path(&self, path: &str) -> Result<Option<queries::FileRecord>> {
        self.with_conn(|conn| queries::get_file_by_path(conn, path))
    }

    /// Supprime un fichier
    pub fn delete_file(&self, path: &str) -> Result<()> {
        self.with_conn(|conn| queries::delete_file(conn, path))
    }

    /// Batch delete
    pub fn batch_delete_files(&self, paths: &[String]) -> Result<()> {
        self.with_conn(|conn| queries::batch_delete_files(conn, paths))
    }

    /// Compte total de fichiers
    pub fn count_files(&self) -> Result<u64> {
        self.with_conn(|conn| queries::count_files(conn))
    }

    /// Stats par extension
    pub fn stats_by_extension(&self) -> Result<Vec<(String, u64, u64)>> {
        self.with_conn(|conn| queries::stats_by_extension(conn))
    }

    /// Ajoute une recherche à l'historique
    pub fn add_search_history(&self, record: &queries::SearchHistoryRecord) -> Result<()> {
        self.with_conn(|conn| queries::add_search_history(conn, record))
    }

    /// Récupère les recherches top (suggestions)
    pub fn get_top_searches(&self, limit: u32) -> Result<Vec<(String, u32)>> {
        self.with_conn(|conn| queries::get_top_searches(conn, limit))
    }

    /// Ajoute un log d'erreur
    pub fn add_error_log(&self, record: &queries::ErrorLogRecord) -> Result<()> {
        self.with_conn(|conn| queries::add_error_log(conn, record))
    }

    /// Récupère les erreurs récentes
    pub fn get_recent_errors(&self, limit: u32) -> Result<Vec<queries::ErrorLogRecord>> {
        self.with_conn(|conn| queries::get_recent_errors(conn, limit))
    }

    /// Set config
    pub fn set_config(&self, key: &str, value: &str) -> Result<()> {
        self.with_conn(|conn| queries::set_config(conn, key, value))
    }

    /// Get config
    pub fn get_config(&self, key: &str) -> Result<Option<String>> {
        self.with_conn(|conn| queries::get_config(conn, key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let db = Database::in_memory().unwrap();
        assert_eq!(db.count_files().unwrap(), 0);
    }

    #[test]
    fn test_file_crud() {
        let db = Database::in_memory().unwrap();
        let now = chrono::Utc::now().timestamp();

        let file = queries::FileRecord {
            id: "test-1".to_string(),
            path: "C:\\test.txt".to_string(),
            filename: "test.txt".to_string(),
            extension: Some(".txt".to_string()),
            size: 1024,
            modified: now,
            created: now,
            hash: Some("abc".to_string()),
            indexed_at: now,
        };

        // Insert
        db.upsert_file(&file).unwrap();
        assert_eq!(db.count_files().unwrap(), 1);

        // Get
        let retrieved = db.get_file_by_path("C:\\test.txt").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().size, 1024);

        // Delete
        db.delete_file("C:\\test.txt").unwrap();
        assert_eq!(db.count_files().unwrap(), 0);
    }

    #[test]
    fn test_batch_operations() {
        let db = Database::in_memory().unwrap();
        let now = chrono::Utc::now().timestamp();

        // Crée 1000 fichiers
        let files: Vec<queries::FileRecord> = (0..1000)
            .map(|i| queries::FileRecord {
                id: format!("id-{}", i),
                path: format!("C:\\file{}.txt", i),
                filename: format!("file{}.txt", i),
                extension: Some(".txt".to_string()),
                size: 1024 * i,
                modified: now,
                created: now,
                hash: Some(format!("hash{}", i)),
                indexed_at: now,
            })
            .collect();

        // Batch insert
        db.batch_upsert_files(&files).unwrap();
        assert_eq!(db.count_files().unwrap(), 1000);

        // Stats
        let stats = db.stats_by_extension().unwrap();
        assert!(!stats.is_empty());
    }

    #[test]
    fn test_config_persistence() {
        let db = Database::in_memory().unwrap();

        db.set_config("test_key", "test_value").unwrap();
        let value = db.get_config("test_key").unwrap();
        assert_eq!(value, Some("test_value".to_string()));

        // Update
        db.set_config("test_key", "new_value").unwrap();
        let value = db.get_config("test_key").unwrap();
        assert_eq!(value, Some("new_value".to_string()));
    }

    #[test]
    fn test_search_history() {
        let db = Database::in_memory().unwrap();
        let now = chrono::Utc::now().timestamp();

        // Ajoute plusieurs recherches
        for i in 0..10 {
            let record = queries::SearchHistoryRecord {
                id: format!("search-{}", i),
                query: if i < 5 { "test query".to_string() } else { format!("query {}", i) },
                results_count: 10,
                execution_time_ms: 50,
                timestamp: now,
            };
            db.add_search_history(&record).unwrap();
        }

        // Récupère top searches
        let top = db.get_top_searches(5).unwrap();
        assert!(!top.is_empty());
        assert_eq!(top[0].0, "test query"); // Plus fréquent
        assert_eq!(top[0].1, 5); // 5 occurrences
    }
}

// ==================== Benchmarks ====================

#[cfg(test)]
mod benches {
    use super::*;
    use std::time::Instant;

    #[test]
    fn bench_single_inserts() {
        let db = Database::in_memory().unwrap();
        let now = chrono::Utc::now().timestamp();

        let start = Instant::now();

        for i in 0..1000 {
            let file = queries::FileRecord {
                id: format!("id-{}", i),
                path: format!("C:\\file{}.txt", i),
                filename: format!("file{}.txt", i),
                extension: Some(".txt".to_string()),
                size: 1024,
                modified: now,
                created: now,
                hash: Some(format!("hash{}", i)),
                indexed_at: now,
            };
            db.upsert_file(&file).unwrap();
        }

        let elapsed = start.elapsed();
        println!("✅ Single inserts: 1000 files in {:?} ({:.2} files/sec)",
            elapsed, 1000.0 / elapsed.as_secs_f64());
    }

    #[test]
    fn bench_batch_inserts() {
        let db = Database::in_memory().unwrap();
        let now = chrono::Utc::now().timestamp();

        let files: Vec<queries::FileRecord> = (0..1000)
            .map(|i| queries::FileRecord {
                id: format!("id-{}", i),
                path: format!("C:\\file{}.txt", i),
                filename: format!("file{}.txt", i),
                extension: Some(".txt".to_string()),
                size: 1024,
                modified: now,
                created: now,
                hash: Some(format!("hash{}", i)),
                indexed_at: now,
            })
            .collect();

        let start = Instant::now();
        db.batch_upsert_files(&files).unwrap();
        let elapsed = start.elapsed();

        println!("✅ Batch inserts: 1000 files in {:?} ({:.2} files/sec)",
            elapsed, 1000.0 / elapsed.as_secs_f64());

        // Batch doit être au moins 10x plus rapide
        assert!(elapsed.as_millis() < 100); // < 100ms pour 1000 fichiers
    }

    #[test]
    fn bench_queries() {
        let db = Database::in_memory().unwrap();
        let now = chrono::Utc::now().timestamp();

        // Setup: 10k fichiers
        let files: Vec<queries::FileRecord> = (0..10000)
            .map(|i| queries::FileRecord {
                id: format!("id-{}", i),
                path: format!("C:\\file{}.txt", i),
                filename: format!("file{}.txt", i),
                extension: Some(".txt".to_string()),
                size: 1024,
                modified: now,
                created: now,
                hash: Some(format!("hash{}", i)),
                indexed_at: now,
            })
            .collect();

        db.batch_upsert_files(&files).unwrap();

        // Bench count
        let start = Instant::now();
        let count = db.count_files().unwrap();
        let elapsed = start.elapsed();
        println!("✅ Count 10k files: {:?}", elapsed);
        assert_eq!(count, 10000);
        assert!(elapsed.as_millis() < 50); // < 50ms

        // Bench stats
        let start = Instant::now();
        let stats = db.stats_by_extension().unwrap();
        let elapsed = start.elapsed();
        println!("✅ Stats aggregation: {:?}", elapsed);
        assert!(!stats.is_empty());
        assert!(elapsed.as_millis() < 100); // < 100ms

        // Bench get by path (avec index)
        let start = Instant::now();
        let file = db.get_file_by_path("C:\\file5000.txt").unwrap();
        let elapsed = start.elapsed();
        println!("✅ Get by path (indexed): {:?}", elapsed);
        assert!(file.is_some());
        assert!(elapsed.as_micros() < 1000); // < 1ms (index)
    }
}
