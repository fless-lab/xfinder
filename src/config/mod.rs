// Module de configuration persistante

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_scan_paths")]
    pub scan_paths: Vec<String>,

    #[serde(default)]
    pub exclusions: ExclusionsConfig,

    #[serde(default)]
    pub indexing: IndexingConfig,

    #[serde(default)]
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExclusionsConfig {
    #[serde(default = "default_excluded_extensions")]
    pub extensions: Vec<String>,

    #[serde(default = "default_excluded_patterns")]
    pub patterns: Vec<String>,

    #[serde(default)]
    pub dirs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingConfig {
    #[serde(default = "default_min_ngram")]
    pub min_ngram_size: usize,

    #[serde(default = "default_max_ngram")]
    pub max_ngram_size: usize,

    #[serde(default = "default_max_files")]
    pub max_files_to_index: usize,

    #[serde(default)]
    pub no_file_limit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_results_display_limit")]
    pub results_display_limit: usize,

    #[serde(default)]
    pub watchdog_enabled: bool,
}

// === Defaults ===

fn default_scan_paths() -> Vec<String> {
    vec![
        dirs::download_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_default())
            .to_string_lossy()
            .to_string()
    ]
}

fn default_excluded_extensions() -> Vec<String> {
    vec![
        ".tmp".to_string(),
        ".log".to_string(),
        ".cache".to_string(),
        ".bak".to_string(),
    ]
}

fn default_excluded_patterns() -> Vec<String> {
    vec![
        "node_modules".to_string(),
        ".git".to_string(),
        "__pycache__".to_string(),
        "target/debug".to_string(),
        "target/release".to_string(),
    ]
}

fn default_min_ngram() -> usize {
    2
}

fn default_max_ngram() -> usize {
    20
}

fn default_max_files() -> usize {
    100000
}

fn default_results_display_limit() -> usize {
    50
}

impl Default for ExclusionsConfig {
    fn default() -> Self {
        Self {
            extensions: default_excluded_extensions(),
            patterns: default_excluded_patterns(),
            dirs: Vec::new(),
        }
    }
}

impl Default for IndexingConfig {
    fn default() -> Self {
        Self {
            min_ngram_size: default_min_ngram(),
            max_ngram_size: default_max_ngram(),
            max_files_to_index: default_max_files(),
            no_file_limit: false,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            results_display_limit: default_results_display_limit(),
            watchdog_enabled: false,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            scan_paths: default_scan_paths(),
            exclusions: ExclusionsConfig::default(),
            indexing: IndexingConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl AppConfig {
    /// Charge la config depuis un fichier TOML, ou crée les defaults
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if path.exists() {
            // Charger depuis le fichier
            let contents = std::fs::read_to_string(path)?;
            let config: AppConfig = toml::from_str(&contents)?;
            Ok(config)
        } else {
            // Créer config par défaut et la sauvegarder
            let config = AppConfig::default();
            config.save(path)?;
            Ok(config)
        }
    }

    /// Sauvegarde la config dans un fichier TOML
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        // Créer le dossier parent si nécessaire
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let toml_string = toml::to_string_pretty(self)?;
        std::fs::write(path, toml_string)?;

        Ok(())
    }

    /// Chemin par défaut du fichier de config
    pub fn default_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
            .join(".xfinder_index")
            .join("config.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert!(!config.scan_paths.is_empty());
        assert_eq!(config.indexing.min_ngram_size, 2);
        assert_eq!(config.indexing.max_ngram_size, 20);
        assert_eq!(config.ui.results_display_limit, 50);
    }

    #[test]
    fn test_save_load_config() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let config_path = dir.path().join("test_config.toml");

        // Créer et sauvegarder une config
        let mut config = AppConfig::default();
        config.scan_paths.push("C:\\Test".to_string());
        config.indexing.min_ngram_size = 3;

        config.save(&config_path).unwrap();

        // Recharger
        let loaded = AppConfig::load(&config_path).unwrap();
        assert!(loaded.scan_paths.contains(&"C:\\Test".to_string()));
        assert_eq!(loaded.indexing.min_ngram_size, 3);
    }

    #[test]
    fn test_toml_serialization() {
        let config = AppConfig::default();
        let toml_str = toml::to_string(&config).unwrap();

        // Vérifier que c'est du TOML valide
        assert!(toml_str.contains("[exclusions]"));
        assert!(toml_str.contains("[indexing]"));
        assert!(toml_str.contains("[ui]"));
    }
}
