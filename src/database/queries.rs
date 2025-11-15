// Queries optimisées avec batch operations pour performance maximale

use rusqlite::{Connection, Result, params};
use std::path::Path;

// ==================== File Operations ====================

/// Représente un fichier dans la DB
#[derive(Debug, Clone)]
pub struct FileRecord {
    pub id: String,
    pub path: String,
    pub filename: String,
    pub extension: Option<String>,
    pub size: u64,
    pub modified: i64,
    pub created: i64,
    pub hash: Option<String>,
    pub indexed_at: i64,
}

/// Insère un fichier (ou le met à jour si existe)
pub fn upsert_file(conn: &Connection, file: &FileRecord) -> Result<()> {
    conn.execute(
        "INSERT INTO files (id, path, filename, extension, size, modified, created, hash, indexed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
         ON CONFLICT(path) DO UPDATE SET
            filename = excluded.filename,
            extension = excluded.extension,
            size = excluded.size,
            modified = excluded.modified,
            hash = excluded.hash,
            indexed_at = excluded.indexed_at",
        params![
            file.id,
            file.path,
            file.filename,
            file.extension,
            file.size as i64,
            file.modified,
            file.created,
            file.hash,
            file.indexed_at,
        ],
    )?;
    Ok(())
}

/// Batch insert optimisé - 1000x plus rapide que inserts individuels
pub fn batch_upsert_files(conn: &Connection, files: &[FileRecord]) -> Result<()> {
    let tx = conn.unchecked_transaction()?;

    {
        let mut stmt = tx.prepare_cached(
            "INSERT INTO files (id, path, filename, extension, size, modified, created, hash, indexed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             ON CONFLICT(path) DO UPDATE SET
                filename = excluded.filename,
                extension = excluded.extension,
                size = excluded.size,
                modified = excluded.modified,
                hash = excluded.hash,
                indexed_at = excluded.indexed_at"
        )?;

        for file in files {
            stmt.execute(params![
                file.id,
                file.path,
                file.filename,
                file.extension,
                file.size as i64,
                file.modified,
                file.created,
                file.hash,
                file.indexed_at,
            ])?;
        }
    }

    tx.commit()?;
    Ok(())
}

/// Récupère un fichier par chemin
pub fn get_file_by_path(conn: &Connection, path: &str) -> Result<Option<FileRecord>> {
    let mut stmt = conn.prepare_cached(
        "SELECT id, path, filename, extension, size, modified, created, hash, indexed_at
         FROM files WHERE path = ?1"
    )?;

    let mut rows = stmt.query(params![path])?;

    if let Some(row) = rows.next()? {
        Ok(Some(FileRecord {
            id: row.get(0)?,
            path: row.get(1)?,
            filename: row.get(2)?,
            extension: row.get(3)?,
            size: row.get::<_, i64>(4)? as u64,
            modified: row.get(5)?,
            created: row.get(6)?,
            hash: row.get(7)?,
            indexed_at: row.get(8)?,
        }))
    } else {
        Ok(None)
    }
}

/// Supprime un fichier
pub fn delete_file(conn: &Connection, path: &str) -> Result<()> {
    conn.execute("DELETE FROM files WHERE path = ?1", params![path])?;
    Ok(())
}

/// Supprime plusieurs fichiers (batch)
pub fn batch_delete_files(conn: &Connection, paths: &[String]) -> Result<()> {
    let tx = conn.unchecked_transaction()?;

    {
        let mut stmt = tx.prepare_cached("DELETE FROM files WHERE path = ?1")?;
        for path in paths {
            stmt.execute(params![path])?;
        }
    }

    tx.commit()?;
    Ok(())
}

/// Compte le nombre total de fichiers
pub fn count_files(conn: &Connection) -> Result<u64> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM files",
        [],
        |row| row.get(0)
    )?;
    Ok(count as u64)
}

/// Statistiques par extension
pub fn stats_by_extension(conn: &Connection) -> Result<Vec<(String, u64, u64)>> {
    let mut stmt = conn.prepare(
        "SELECT
            COALESCE(extension, 'no_ext') as ext,
            COUNT(*) as count,
            SUM(size) as total_size
         FROM files
         GROUP BY extension
         ORDER BY count DESC"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)? as u64,
            row.get::<_, i64>(2)? as u64,
        ))
    })?;

    rows.collect()
}

// ==================== Watched Folders Operations ====================

#[derive(Debug, Clone)]
pub struct WatchedFolderRecord {
    pub path: String,
    pub last_scan: Option<i64>,
    pub file_count: u64,
    pub total_size: u64,
    pub enabled: bool,
    pub created_at: i64,
}

pub fn upsert_watched_folder(conn: &Connection, folder: &WatchedFolderRecord) -> Result<()> {
    conn.execute(
        "INSERT INTO watched_folders (path, last_scan, file_count, total_size, enabled, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(path) DO UPDATE SET
            last_scan = excluded.last_scan,
            file_count = excluded.file_count,
            total_size = excluded.total_size,
            enabled = excluded.enabled",
        params![
            folder.path,
            folder.last_scan,
            folder.file_count as i64,
            folder.total_size as i64,
            folder.enabled,
            folder.created_at,
        ],
    )?;
    Ok(())
}

pub fn get_watched_folders(conn: &Connection) -> Result<Vec<WatchedFolderRecord>> {
    let mut stmt = conn.prepare(
        "SELECT path, last_scan, file_count, total_size, enabled, created_at
         FROM watched_folders
         WHERE enabled = 1
         ORDER BY path"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(WatchedFolderRecord {
            path: row.get(0)?,
            last_scan: row.get(1)?,
            file_count: row.get::<_, i64>(2)? as u64,
            total_size: row.get::<_, i64>(3)? as u64,
            enabled: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;

    rows.collect()
}

// ==================== Config Operations ====================

pub fn set_config(conn: &Connection, key: &str, value: &str) -> Result<()> {
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "INSERT INTO config (key, value, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET
            value = excluded.value,
            updated_at = excluded.updated_at",
        params![key, value, now],
    )?;
    Ok(())
}

pub fn get_config(conn: &Connection, key: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare_cached("SELECT value FROM config WHERE key = ?1")?;
    let mut rows = stmt.query(params![key])?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

// ==================== Search History Operations ====================

#[derive(Debug, Clone)]
pub struct SearchHistoryRecord {
    pub id: String,
    pub query: String,
    pub results_count: u32,
    pub execution_time_ms: u32,
    pub timestamp: i64,
}

pub fn add_search_history(conn: &Connection, record: &SearchHistoryRecord) -> Result<()> {
    conn.execute(
        "INSERT INTO search_history (id, query, results_count, execution_time_ms, timestamp)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            record.id,
            record.query,
            record.results_count as i64,
            record.execution_time_ms as i64,
            record.timestamp,
        ],
    )?;
    Ok(())
}

/// Récupère les recherches les plus fréquentes (suggestions)
pub fn get_top_searches(conn: &Connection, limit: u32) -> Result<Vec<(String, u32)>> {
    let mut stmt = conn.prepare(
        "SELECT query, COUNT(*) as freq
         FROM search_history
         WHERE timestamp > ?1
         GROUP BY query
         ORDER BY freq DESC
         LIMIT ?2"
    )?;

    // Derniers 30 jours
    let since = chrono::Utc::now().timestamp() - (30 * 24 * 60 * 60);

    let rows = stmt.query_map(params![since, limit as i64], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)? as u32,
        ))
    })?;

    rows.collect()
}

// ==================== Error Log Operations ====================

#[derive(Debug, Clone)]
pub struct ErrorLogRecord {
    pub id: String,
    pub file_path: Option<String>,
    pub error_type: String,
    pub message: String,
    pub timestamp: i64,
}

pub fn add_error_log(conn: &Connection, record: &ErrorLogRecord) -> Result<()> {
    conn.execute(
        "INSERT INTO error_log (id, file_path, error_type, message, timestamp)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            record.id,
            record.file_path,
            record.error_type,
            record.message,
            record.timestamp,
        ],
    )?;
    Ok(())
}

/// Récupère les erreurs récentes
pub fn get_recent_errors(conn: &Connection, limit: u32) -> Result<Vec<ErrorLogRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, file_path, error_type, message, timestamp
         FROM error_log
         ORDER BY timestamp DESC
         LIMIT ?1"
    )?;

    let rows = stmt.query_map(params![limit as i64], |row| {
        Ok(ErrorLogRecord {
            id: row.get(0)?,
            file_path: row.get(1)?,
            error_type: row.get(2)?,
            message: row.get(3)?,
            timestamp: row.get(4)?,
        })
    })?;

    rows.collect()
}

/// Nettoie les vieux logs (>30 jours)
pub fn cleanup_old_logs(conn: &Connection) -> Result<usize> {
    let cutoff = chrono::Utc::now().timestamp() - (30 * 24 * 60 * 60);
    let deleted = conn.execute(
        "DELETE FROM error_log WHERE timestamp < ?1",
        params![cutoff],
    )?;
    Ok(deleted)
}

// ==================== Duplicate Detection ====================

/// Représente un groupe de fichiers dupliqués
#[derive(Debug, Clone)]
pub struct DuplicateGroup {
    pub hash: String,
    pub files: Vec<FileRecord>,
    pub total_size: u64,
    pub duplicate_count: usize,
}

/// Trouve tous les fichiers dupliqués (même hash blake3)
///
/// Retourne une liste de groupes, chaque groupe contenant tous les fichiers
/// ayant le même hash. Seuls les hash ayant 2+ fichiers sont retournés.
pub fn find_duplicates(conn: &Connection) -> Result<Vec<DuplicateGroup>> {
    // 1. Récupérer tous les hash qui apparaissent plus d'une fois
    let mut stmt = conn.prepare(
        "SELECT hash, COUNT(*) as count
         FROM files
         WHERE hash IS NOT NULL
         GROUP BY hash
         HAVING count > 1
         ORDER BY count DESC"
    )?;

    let duplicate_hashes: Vec<String> = stmt.query_map([], |row| {
        row.get::<_, String>(0)
    })?.collect::<Result<Vec<_>>>()?;

    // 2. Pour chaque hash dupliqué, récupérer tous les fichiers
    let mut groups = Vec::new();
    for hash in duplicate_hashes {
        let mut file_stmt = conn.prepare_cached(
            "SELECT id, path, filename, extension, size, modified, created, hash, indexed_at
             FROM files
             WHERE hash = ?1"
        )?;

        let files: Vec<FileRecord> = file_stmt.query_map(params![&hash], |row| {
            Ok(FileRecord {
                id: row.get(0)?,
                path: row.get(1)?,
                filename: row.get(2)?,
                extension: row.get(3)?,
                size: row.get::<_, i64>(4)? as u64,
                modified: row.get(5)?,
                created: row.get(6)?,
                hash: row.get(7)?,
                indexed_at: row.get(8)?,
            })
        })?.collect::<Result<Vec<_>>>()?;

        let total_size = files.iter().map(|f| f.size).sum();
        let duplicate_count = files.len();

        groups.push(DuplicateGroup {
            hash,
            files,
            total_size,
            duplicate_count,
        });
    }

    Ok(groups)
}

/// Compte le nombre total de fichiers dupliqués
pub fn count_duplicates(conn: &Connection) -> Result<(usize, u64)> {
    let groups = find_duplicates(conn)?;
    let total_files: usize = groups.iter().map(|g| g.duplicate_count).sum();
    let total_size: u64 = groups.iter().map(|g| g.total_size).sum();
    Ok((total_files, total_size))
}

// ==================== Semantic File Mapping Operations ====================

/// Enregistre ou met à jour le mapping file_id -> path pour la recherche sémantique
pub fn upsert_semantic_file_mapping(conn: &Connection, file_id: i64, path: &str) -> Result<()> {
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "INSERT INTO semantic_file_mapping (file_id, path, indexed_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(file_id) DO UPDATE SET
            path = excluded.path,
            indexed_at = excluded.indexed_at",
        params![file_id, path, now],
    )?;
    Ok(())
}

/// Récupère le chemin d'un fichier à partir de son file_id
pub fn get_path_by_file_id(conn: &Connection, file_id: i64) -> Result<Option<String>> {
    let mut stmt = conn.prepare_cached(
        "SELECT path FROM semantic_file_mapping WHERE file_id = ?1"
    )?;
    let mut rows = stmt.query(params![file_id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

/// Supprime le mapping d'un fichier
pub fn delete_semantic_file_mapping(conn: &Connection, file_id: i64) -> Result<()> {
    conn.execute("DELETE FROM semantic_file_mapping WHERE file_id = ?1", params![file_id])?;
    Ok(())
}

// ==================== Semantic Chunks Operations ====================

#[derive(Debug, Clone)]
pub struct SemanticChunkRecord {
    pub chunk_id: i64,
    pub file_id: i64,
    pub chunk_index: usize,
    pub text: String,
    pub start_pos: usize,
    pub end_pos: usize,
    pub indexed_at: i64,
}

/// Insère un chunk sémantique
pub fn insert_semantic_chunk(conn: &Connection, chunk: &SemanticChunkRecord) -> Result<()> {
    conn.execute(
        "INSERT INTO semantic_chunks (chunk_id, file_id, chunk_index, text, start_pos, end_pos, indexed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(chunk_id) DO UPDATE SET
            text = excluded.text,
            indexed_at = excluded.indexed_at",
        params![
            chunk.chunk_id,
            chunk.file_id,
            chunk.chunk_index as i64,
            chunk.text,
            chunk.start_pos as i64,
            chunk.end_pos as i64,
            chunk.indexed_at,
        ],
    )?;
    Ok(())
}

/// Récupère un chunk par son ID
pub fn get_chunk_by_id(conn: &Connection, chunk_id: i64) -> Result<Option<SemanticChunkRecord>> {
    let mut stmt = conn.prepare_cached(
        "SELECT chunk_id, file_id, chunk_index, text, start_pos, end_pos, indexed_at
         FROM semantic_chunks WHERE chunk_id = ?1"
    )?;
    let mut rows = stmt.query(params![chunk_id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(SemanticChunkRecord {
            chunk_id: row.get(0)?,
            file_id: row.get(1)?,
            chunk_index: row.get::<_, i64>(2)? as usize,
            text: row.get(3)?,
            start_pos: row.get::<_, i64>(4)? as usize,
            end_pos: row.get::<_, i64>(5)? as usize,
            indexed_at: row.get(6)?,
        }))
    } else {
        Ok(None)
    }
}

/// Récupère tous les chunks d'un fichier
pub fn get_chunks_by_file_id(conn: &Connection, file_id: i64) -> Result<Vec<SemanticChunkRecord>> {
    let mut stmt = conn.prepare_cached(
        "SELECT chunk_id, file_id, chunk_index, text, start_pos, end_pos, indexed_at
         FROM semantic_chunks WHERE file_id = ?1 ORDER BY chunk_index"
    )?;

    let rows = stmt.query_map(params![file_id], |row| {
        Ok(SemanticChunkRecord {
            chunk_id: row.get(0)?,
            file_id: row.get(1)?,
            chunk_index: row.get::<_, i64>(2)? as usize,
            text: row.get(3)?,
            start_pos: row.get::<_, i64>(4)? as usize,
            end_pos: row.get::<_, i64>(5)? as usize,
            indexed_at: row.get(6)?,
        })
    })?;

    rows.collect()
}

/// Supprime tous les chunks d'un fichier
pub fn delete_chunks_by_file_id(conn: &Connection, file_id: i64) -> Result<()> {
    conn.execute("DELETE FROM semantic_chunks WHERE file_id = ?1", params![file_id])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(crate::database::schema::INIT_SCHEMA).unwrap();
        conn
    }

    #[test]
    fn test_file_operations() {
        let conn = create_test_db();
        let now = chrono::Utc::now().timestamp();

        let file = FileRecord {
            id: "test-id".to_string(),
            path: "C:\\test.txt".to_string(),
            filename: "test.txt".to_string(),
            extension: Some(".txt".to_string()),
            size: 1024,
            modified: now,
            created: now,
            hash: Some("abc123".to_string()),
            indexed_at: now,
        };

        upsert_file(&conn, &file).unwrap();
        let retrieved = get_file_by_path(&conn, "C:\\test.txt").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().filename, "test.txt");
    }

    #[test]
    fn test_batch_insert() {
        let conn = create_test_db();
        let now = chrono::Utc::now().timestamp();

        let files: Vec<FileRecord> = (0..100).map(|i| FileRecord {
            id: format!("id-{}", i),
            path: format!("C:\\file{}.txt", i),
            filename: format!("file{}.txt", i),
            extension: Some(".txt".to_string()),
            size: 1024,
            modified: now,
            created: now,
            hash: Some(format!("hash{}", i)),
            indexed_at: now,
        }).collect();

        batch_upsert_files(&conn, &files).unwrap();
        let count = count_files(&conn).unwrap();
        assert_eq!(count, 100);
    }
}
