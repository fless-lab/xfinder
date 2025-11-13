// src/search/scanner.rs
// Scan de fichiers avec walkdir

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

    // Vérifie si un chemin doit être exclu
    fn should_exclude(
        &self,
        path: &Path,
        filename: &str,
        excluded_extensions: &[String],
        excluded_patterns: &[String],
        excluded_dirs: &[String],
    ) -> bool {
        let path_str = path.to_string_lossy().to_string();

        // 1. Vérifier les dossiers exclus (chemins exacts)
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

        // 3. Vérifier les patterns (noms de fichiers/dossiers)
        for pattern in excluded_patterns {
            // Pattern simple: si le chemin contient le pattern
            if path_str.contains(pattern) || filename.contains(pattern) {
                return true;
            }
        }

        false
    }

    // Scan un dossier récursivement avec exclusions
    // Limite à max_files pour éviter de surcharger
    pub fn scan_directory(
        &self,
        root: &Path,
        max_files: usize,
        excluded_extensions: &[String],
        excluded_patterns: &[String],
        excluded_dirs: &[String],
    ) -> Result<Vec<FileEntry>> {
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
                let path = entry.path();

                // Exclure les dossiers avant de les parcourir
                if entry.file_type().is_dir() {
                    if self.should_exclude(path, "", excluded_extensions, excluded_patterns, excluded_dirs) {
                        continue;
                    }
                }

                if entry.file_type().is_file() {
                    if let Some(filename) = path.file_name() {
                        let filename_str = filename.to_string_lossy().to_string();

                        // Vérifier si le fichier doit être exclu
                        if self.should_exclude(path, &filename_str, excluded_extensions, excluded_patterns, excluded_dirs) {
                            continue;
                        }

                        entries.push(FileEntry {
                            path: path.to_string_lossy().to_string(),
                            filename: filename_str,
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

        // Pas d'exclusions pour ce test
        let result = scanner.scan_directory(&temp_dir, 10, &[], &[], &[]);
        assert!(result.is_ok());

        let files = result.unwrap();
        assert!(files.len() <= 10);
    }

    #[test]
    fn test_exclusions() {
        let scanner = FileScanner::new();
        let temp_dir = std::env::temp_dir();

        // Test avec exclusion d'extensions
        let excluded_ext = vec![".log".to_string()];
        let result = scanner.scan_directory(&temp_dir, 100, &excluded_ext, &[], &[]);
        assert!(result.is_ok());

        // Vérifier qu'aucun fichier .log n'est présent
        let files = result.unwrap();
        for file in &files {
            assert!(!file.filename.ends_with(".log"));
        }
    }
}
