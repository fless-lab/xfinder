// tests/config_test.rs
// Tests d'intégration pour la configuration

use xfinder::config::AppConfig;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_config_default() {
    let config = AppConfig::default();

    // Vérifier les valeurs par défaut
    assert_eq!(config.indexing.min_ngram_size, 2);
    assert_eq!(config.indexing.max_ngram_size, 20);
    assert_eq!(config.ui.results_display_limit, 50);
    assert_eq!(config.system.scheduler_hour, 2);
    assert_eq!(config.system.scheduler_minute, 0);
    assert!(!config.system.autostart_enabled);
    assert!(!config.system.scheduler_enabled);
}

#[test]
fn test_config_save_and_load() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("test_config.toml");

    // Créer une config personnalisée
    let mut config = AppConfig::default();
    config.system.scheduler_hour = 15;
    config.system.scheduler_minute = 30;
    config.system.autostart_enabled = true;
    config.ui.minimize_to_tray = true;

    // Sauvegarder
    config.save(&config_path).unwrap();

    // Recharger
    let loaded = AppConfig::load(&config_path).unwrap();

    // Vérifier que les valeurs sont préservées
    assert_eq!(loaded.system.scheduler_hour, 15);
    assert_eq!(loaded.system.scheduler_minute, 30);
    assert!(loaded.system.autostart_enabled);
    assert!(loaded.ui.minimize_to_tray);
}

#[test]
fn test_config_toml_format() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("test_config.toml");

    let mut config = AppConfig::default();
    config.system.scheduler_hour = 10;
    config.save(&config_path).unwrap();

    // Lire le fichier TOML directement
    let content = fs::read_to_string(&config_path).unwrap();

    // Vérifier que le fichier contient bien les sections
    assert!(content.contains("[system]"));
    assert!(content.contains("scheduler_hour = 10"));
}

#[test]
fn test_config_missing_fields_use_defaults() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("partial_config.toml");

    // Écrire un fichier TOML partiel (manque la section system)
    fs::write(&config_path, r#"
[indexing]
min_ngram_size = 2

[ui]
results_display_limit = 50
"#).unwrap();

    // Charger - devrait utiliser les defaults pour system
    let loaded = AppConfig::load(&config_path).unwrap();

    assert_eq!(loaded.ui.results_display_limit, 50);
    assert_eq!(loaded.system.scheduler_hour, 2); // Default
    assert_eq!(loaded.system.scheduler_minute, 0); // Default
}
