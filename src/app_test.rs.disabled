// src/app_test.rs
// Tests pour la logique métier de l'app

#[cfg(test)]
mod tests {
    use crate::app::XFinderApp;
    use std::path::PathBuf;

    #[test]
    fn test_default_scan_paths_is_valid() {
        let app = XFinderApp::default();

        // Les scan paths par défaut doivent contenir au moins 1 chemin
        assert!(!app.scan_paths.is_empty());
        assert!(!app.scan_paths[0].is_empty());
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
    fn test_can_add_remove_scan_paths() {
        let mut app = XFinderApp::default();
        let initial_count = app.scan_paths.len();

        // Ajouter un chemin
        app.add_scan_path("C:\\Windows".to_string());
        assert_eq!(app.scan_paths.len(), initial_count + 1);

        // Ajouter le même chemin (ne doit pas dupliquer)
        app.add_scan_path("C:\\Windows".to_string());
        assert_eq!(app.scan_paths.len(), initial_count + 1);

        // Retirer un chemin
        app.remove_scan_path(0);
        assert_eq!(app.scan_paths.len(), initial_count);
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
