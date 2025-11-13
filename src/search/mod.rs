// src/search/mod.rs
// Module de recherche avec Tantivy

pub mod scanner;
pub mod tantivy_index;
pub mod file_watcher;

#[cfg(test)]
mod search_test;

pub use scanner::{FileEntry, FileScanner};
pub use tantivy_index::{SearchIndex, SearchOptions};
pub use file_watcher::{FileWatcher, FileEvent};

// Résultat de recherche avec métadonnées
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: String,
    pub filename: String,
    pub score: f32,
    pub size_bytes: u64,
    pub created: Option<String>,
    pub modified: Option<String>,
}

impl SearchResult {
    pub fn new(path: String, filename: String, score: f32) -> Self {
        // Récupérer les métadonnées du fichier
        let metadata = std::fs::metadata(&path).ok();
        let size_bytes = metadata.as_ref().map(|m| m.len()).unwrap_or(0);

        let created = metadata.as_ref().and_then(|m| {
            m.created().ok().map(|t| {
                let datetime: chrono::DateTime<chrono::Local> = t.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            })
        });

        let modified = metadata.as_ref().and_then(|m| {
            m.modified().ok().map(|t| {
                let datetime: chrono::DateTime<chrono::Local> = t.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            })
        });

        Self {
            path,
            filename,
            score,
            size_bytes,
            created,
            modified,
        }
    }
}

// Tests du module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_creation() {
        let result = SearchResult::new(
            "C:\\test\\file.txt".to_string(),
            "file.txt".to_string(),
            0.95,
        );
        assert_eq!(result.filename, "file.txt");
        assert_eq!(result.score, 0.95);
    }
}
