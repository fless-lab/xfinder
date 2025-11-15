// src/semantic/content_extractor.rs
// Extraction de contenu texte depuis différents formats de fichiers

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Extracteur de contenu pour différents types de fichiers
pub struct ContentExtractor;

impl ContentExtractor {
    /// Extrait le texte d'un fichier selon son extension
    ///
    /// # Arguments
    /// * `path` - Chemin du fichier à extraire
    ///
    /// # Returns
    /// Contenu texte du fichier, ou erreur si extraction impossible
    ///
    /// # Supported formats
    /// - `.txt`, `.md`, `.log`, `.json`, `.xml`, `.yaml`, `.toml` : texte brut
    /// - `.pdf` : extraction via pdf-extract
    /// - `.docx` : extraction via docx-rs
    /// - Autres : tentative de lecture comme texte UTF-8
    pub fn extract_text<P: AsRef<Path>>(path: P) -> Result<String> {
        let path = path.as_ref();

        // Vérifier que le fichier existe
        if !path.exists() {
            anyhow::bail!("File not found: {}", path.display());
        }

        // Obtenir l'extension
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        // Router selon l'extension
        match ext.as_str() {
            // Texte brut
            "txt" | "md" | "log" | "json" | "xml" | "yaml" | "yml" | "toml" | "ini" | "cfg" => {
                Self::extract_plain_text(path)
            }

            // PDF
            "pdf" => Self::extract_pdf(path),

            // DOCX
            "docx" => Self::extract_docx(path),

            // Code source (traité comme texte)
            "rs" | "js" | "ts" | "py" | "java" | "cpp" | "c" | "h" | "cs" | "go" | "rb" | "php" | "html" | "css" => {
                Self::extract_plain_text(path)
            }

            // Par défaut : tentative de lecture comme texte
            _ => Self::extract_plain_text(path)
                .or_else(|_| Ok(format!("[Unsupported format: {}]", ext))),
        }
    }

    /// Extrait le texte d'un fichier texte brut
    fn extract_plain_text<P: AsRef<Path>>(path: P) -> Result<String> {
        fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read text file: {}", path.as_ref().display()))
    }

    /// Extrait le texte d'un fichier PDF
    fn extract_pdf<P: AsRef<Path>>(path: P) -> Result<String> {
        use pdf_extract as pdf;
        let path = path.as_ref();

        // Utiliser pdf-extract pour extraire le texte
        let content = pdf::extract_text(path)
            .with_context(|| format!("Failed to extract PDF: {}", path.display()))?;

        if content.trim().is_empty() {
            // PDF vide ou scanné (pas de texte extrait)
            Ok(format!("[PDF sans texte extractible: {}]", path.file_name().unwrap_or_default().to_string_lossy()))
        } else {
            Ok(content)
        }
    }

    /// Extrait le texte d'un fichier DOCX
    fn extract_docx<P: AsRef<Path>>(path: P) -> Result<String> {
        use dotext::{Docx, MsDoc};
        use std::io::Read;

        let path = path.as_ref();

        // Ouvrir le fichier DOCX avec dotext
        let mut docx_file = Docx::open(path)
            .with_context(|| format!("Failed to open DOCX file: {}", path.display()))?;

        // Lire le contenu texte
        let mut text = String::new();
        docx_file.read_to_string(&mut text)
            .with_context(|| format!("Failed to read DOCX content: {}", path.display()))?;

        if text.trim().is_empty() {
            Ok(format!("[DOCX vide: {}]", path.file_name().unwrap_or_default().to_string_lossy()))
        } else {
            Ok(text)
        }
    }

    /// Vérifie si un fichier est supporté pour l'extraction
    ///
    /// # Arguments
    /// * `path` - Chemin du fichier à vérifier
    ///
    /// # Returns
    /// `true` si le format est supporté, `false` sinon
    pub fn is_supported<P: AsRef<Path>>(path: P) -> bool {
        let ext = path
            .as_ref()
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        matches!(
            ext.as_str(),
            // Texte
            "txt" | "md" | "log" | "json" | "xml" | "yaml" | "yml" | "toml" | "ini" | "cfg" |
            // Documents
            "pdf" | "docx" |
            // Code
            "rs" | "js" | "ts" | "py" | "java" | "cpp" | "c" | "h" | "cs" | "go" | "rb" | "php" | "html" | "css"
        )
    }

    /// Nettoie le texte extrait (supprime caractères invisibles, normalise espaces)
    ///
    /// # Arguments
    /// * `text` - Texte brut à nettoyer
    ///
    /// # Returns
    /// Texte nettoyé
    pub fn clean_text(text: &str) -> String {
        text
            // Remplacer les retours à la ligne multiples par un seul
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
            // Normaliser les espaces multiples
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_extract_plain_text() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "Hello World").unwrap();
        writeln!(file, "Test content").unwrap();
        file.flush().unwrap();

        let content = ContentExtractor::extract_text(file.path()).unwrap();
        assert!(content.contains("Hello World"));
        assert!(content.contains("Test content"));
    }

    #[test]
    fn test_is_supported() {
        assert!(ContentExtractor::is_supported("test.txt"));
        assert!(ContentExtractor::is_supported("test.pdf"));
        assert!(ContentExtractor::is_supported("test.docx"));
        assert!(ContentExtractor::is_supported("test.md"));
        assert!(ContentExtractor::is_supported("test.rs"));
        assert!(!ContentExtractor::is_supported("test.exe"));
        assert!(!ContentExtractor::is_supported("test.dll"));
    }

    #[test]
    fn test_clean_text() {
        let dirty = "Hello   World\n\n\nTest    Content\n  \n";
        let clean = ContentExtractor::clean_text(dirty);
        assert_eq!(clean, "Hello World Test Content");
    }

    #[test]
    fn test_extract_nonexistent_file() {
        let result = ContentExtractor::extract_text("/nonexistent/file.txt");
        assert!(result.is_err());
    }
}
