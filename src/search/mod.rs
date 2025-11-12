// src/search/mod.rs
// Module de recherche avec Tantivy

pub mod scanner;
pub mod tantivy_index;

pub use scanner::{FileEntry, FileScanner};
pub use tantivy_index::SearchIndex;

// Résultat de recherche
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: String,
    pub filename: String,
    pub score: f32,
}

impl SearchResult {
    pub fn new(path: String, filename: String, score: f32) -> Self {
        Self {
            path,
            filename,
            score,
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
