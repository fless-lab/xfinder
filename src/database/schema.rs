// Schema SQLite - Phase 1 (Fichiers uniquement)

pub const SCHEMA_VERSION: i32 = 1;

pub const INIT_SCHEMA: &str = r#"
-- ==================== Schema Version ====================
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY,
    applied_at INTEGER NOT NULL
);

-- ==================== Files Table ====================
-- Métadonnées des fichiers indexés
CREATE TABLE IF NOT EXISTS files (
    id TEXT PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    filename TEXT NOT NULL,
    extension TEXT,
    size INTEGER NOT NULL,
    modified INTEGER NOT NULL,
    created INTEGER NOT NULL,
    hash TEXT,
    indexed_at INTEGER NOT NULL
);

-- Indexes pour performance
CREATE INDEX IF NOT EXISTS idx_files_path ON files(path);
CREATE INDEX IF NOT EXISTS idx_files_modified ON files(modified);
CREATE INDEX IF NOT EXISTS idx_files_extension ON files(extension);
CREATE INDEX IF NOT EXISTS idx_files_filename ON files(filename);
CREATE INDEX IF NOT EXISTS idx_files_hash ON files(hash);

-- ==================== Watched Folders Table ====================
-- Dossiers surveillés par le watchdog
CREATE TABLE IF NOT EXISTS watched_folders (
    path TEXT PRIMARY KEY,
    last_scan INTEGER,
    file_count INTEGER DEFAULT 0,
    total_size INTEGER DEFAULT 0,
    enabled BOOLEAN DEFAULT 1,
    created_at INTEGER NOT NULL
);

-- ==================== Configuration Table ====================
-- Configuration clé-valeur persistante
CREATE TABLE IF NOT EXISTS config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

-- ==================== Search History Table ====================
-- Historique des recherches pour suggestions
CREATE TABLE IF NOT EXISTS search_history (
    id TEXT PRIMARY KEY,
    query TEXT NOT NULL,
    results_count INTEGER,
    execution_time_ms INTEGER,
    timestamp INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_search_history_timestamp ON search_history(timestamp);
CREATE INDEX IF NOT EXISTS idx_search_history_query ON search_history(query);

-- ==================== Error Log Table ====================
-- Logs d'erreurs structurés
CREATE TABLE IF NOT EXISTS error_log (
    id TEXT PRIMARY KEY,
    file_path TEXT,
    error_type TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_error_log_timestamp ON error_log(timestamp);
CREATE INDEX IF NOT EXISTS idx_error_log_type ON error_log(error_type);

-- ==================== Stats View ====================
-- Vue pour statistiques rapides
CREATE VIEW IF NOT EXISTS files_stats AS
SELECT
    COUNT(*) as total_files,
    SUM(size) as total_size,
    MAX(indexed_at) as last_indexed,
    extension,
    COUNT(*) as count_by_ext,
    SUM(size) as size_by_ext
FROM files
GROUP BY extension;
"#;

/// Optimisations SQLite pour performance maximale
pub const PRAGMAS: &[&str] = &[
    "PRAGMA journal_mode = WAL;",          // Write-Ahead Logging (non-bloquant)
    "PRAGMA synchronous = NORMAL;",        // Balance perf/sécurité
    "PRAGMA cache_size = -64000;",         // 64MB cache
    "PRAGMA temp_store = MEMORY;",         // Temp tables en RAM
    "PRAGMA mmap_size = 268435456;",       // 256MB memory-mapped I/O
    "PRAGMA page_size = 4096;",            // Taille page optimale
    "PRAGMA auto_vacuum = INCREMENTAL;",   // Auto-nettoyage progressif
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_version() {
        assert_eq!(SCHEMA_VERSION, 1);
    }

    #[test]
    fn test_schema_not_empty() {
        assert!(!INIT_SCHEMA.is_empty());
        assert!(INIT_SCHEMA.contains("CREATE TABLE"));
    }

    #[test]
    fn test_pragmas_not_empty() {
        assert!(!PRAGMAS.is_empty());
        assert!(PRAGMAS.iter().any(|p| p.contains("WAL")));
    }
}
