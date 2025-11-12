// src/app_test.rs
// Tests pour la logique métier de l'app

#[cfg(test)]
mod tests {
    use crate::app::XFinderApp;
    use std::path::PathBuf;

    #[test]
    fn test_default_scan_path_is_valid() {
        let app = XFinderApp::default();

        // Le scan path par défaut doit être un chemin valide
        let _path = PathBuf::from(&app.scan_path);
        assert!(!app.scan_path.is_empty());
        // On ne teste pas exists() car le dossier peut ne pas exister
    }

    #[test]
    fn test_index_dir_not_in_target() {
        let app = XFinderApp::default();

        // L'index ne doit PAS être dans target/
        let index_path = app.index_dir.to_string_lossy();
        assert!(!index_path.contains("target"));
        assert!(!index_path.contains("debug"));
        assert!(!index_path.contains("release"));
    }

    #[test]
    fn test_scan_path_can_be_updated() {
        let mut app = XFinderApp::default();
        let old_path = app.scan_path.clone();

        // On peut changer le scan path
        app.scan_path = "C:\\Windows".to_string();

        assert_ne!(app.scan_path, old_path);
        assert_eq!(app.scan_path, "C:\\Windows");
    }

    #[test]
    fn test_initial_state() {
        let app = XFinderApp::default();

        assert!(app.search_query.is_empty());
        assert!(app.search_results.is_empty());
        assert!(app.search_index.is_none());
        assert!(!app.indexing_in_progress);
        assert_eq!(app.index_status.file_count, 0);
        assert!(app.preview_file_path.is_none());
    }
}
