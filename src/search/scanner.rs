// src/search/scanner.rs
// Scan de fichiers avec walkdir
// TODO: Ajouter ignore crate plus tard pour skip intelligent

use anyhow::Result;
use walkdir::WalkDir;
use std::path::Path;

pub struct FileEntry {
    pub path: String,
    pub filename: String,
}

pub struct FileScanner;

impl FileScanner {
    pub fn new() -> Self {
        Self
    }

    // Scan un dossier récursivement
    // Limite à max_files pour éviter de surcharger
    pub fn scan_directory(&self, root: &Path, max_files: usize) -> Result<Vec<FileEntry>> {
        let mut entries = Vec::new();
        let mut count = 0;

        for entry in WalkDir::new(root)
            .max_depth(5) // Limite profondeur
            .follow_links(false)
        {
            if count >= max_files {
                break;
            }

            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    if let Some(filename) = entry.path().file_name() {
                        entries.push(FileEntry {
                            path: entry.path().to_string_lossy().to_string(),
                            filename: filename.to_string_lossy().to_string(),
                        });
                        count += 1;
                    }
                }
            }
        }

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_creation() {
        let _scanner = FileScanner::new();
        // Juste vérifier que ça compile
        assert!(true);
    }

    #[test]
    fn test_scan_temp_directory() {
        let temp_dir = std::env::temp_dir();
        let scanner = FileScanner::new();

        let result = scanner.scan_directory(&temp_dir, 10);
        assert!(result.is_ok());

        let files = result.unwrap();
        assert!(files.len() <= 10);
    }
}
