// Test d'intégration pour vérifier que la recherche marche vraiment

#[cfg(test)]
mod integration_tests {
    use crate::search::SearchIndex;
    use std::path::PathBuf;

    #[test]
    fn test_partial_search_filtrag() {
        let temp_dir = std::env::temp_dir().join("xfinder_test_partial");
        let _ = std::fs::remove_dir_all(&temp_dir);

        let index = SearchIndex::new(&temp_dir).unwrap();
        let mut writer = index.create_writer().unwrap();

        // Ajouter des fichiers avec "filtrage" dans le nom
        index.add_file(&mut writer, "C:\\test\\cours_filtrage.pdf", "cours_filtrage.pdf").unwrap();
        index.add_file(&mut writer, "C:\\test\\filtrage_actif.pdf", "filtrage_actif.pdf").unwrap();
        index.add_file(&mut writer, "C:\\test\\autre_fichier.pdf", "autre_fichier.pdf").unwrap();
        writer.commit().unwrap();

        // Test 1: recherche complète "filtrage"
        let results = index.search("filtrage", 10).unwrap();
        println!("Test 'filtrage': {} résultats", results.len());
        assert!(results.len() >= 2, "Devrait trouver au moins 2 fichiers avec 'filtrage'");

        // Test 2: recherche partielle "filtrag" (sans le e final)
        let results = index.search("filtrag", 10).unwrap();
        println!("Test 'filtrag': {} résultats", results.len());
        assert!(results.len() >= 2, "FAIL: 'filtrag' devrait trouver 'filtrage'");

        // Test 3: recherche partielle "filtr"
        let results = index.search("filtr", 10).unwrap();
        println!("Test 'filtr': {} résultats", results.len());
        assert!(results.len() >= 2, "FAIL: 'filtr' devrait trouver 'filtrage'");

        // Test 4: recherche "trag" (milieu du mot)
        let results = index.search("trag", 10).unwrap();
        println!("Test 'trag': {} résultats", results.len());
        assert!(results.len() >= 2, "FAIL: 'trag' devrait trouver 'filtrage'");

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_partial_search_xfinder() {
        let temp_dir = std::env::temp_dir().join("xfinder_test_xfinder");
        let _ = std::fs::remove_dir_all(&temp_dir);

        let index = SearchIndex::new(&temp_dir).unwrap();
        let mut writer = index.create_writer().unwrap();

        // Ajouter fichiers avec "xfinder"
        index.add_file(&mut writer, "C:\\test\\xfinder.exe", "xfinder.exe").unwrap();
        index.add_file(&mut writer, "C:\\test\\xfinder_config.toml", "xfinder_config.toml").unwrap();
        writer.commit().unwrap();

        // Test recherche partielle "xfinde" (sans r final)
        let results = index.search("xfinde", 10).unwrap();
        println!("Test 'xfinde': {} résultats", results.len());
        assert!(results.len() >= 2, "FAIL: 'xfinde' devrait trouver 'xfinder'");

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_incremental_typing_filtra_to_filtrag() {
        let temp_dir = std::env::temp_dir().join("xfinder_test_incremental");
        let _ = std::fs::remove_dir_all(&temp_dir);

        let index = SearchIndex::new(&temp_dir).unwrap();
        let mut writer = index.create_writer().unwrap();

        // Fichiers réels de l'utilisateur
        index.add_file(&mut writer, "C:\\Users\\ATD\\Downloads\\Documents\\Filtrage.pdf", "Filtrage.pdf").unwrap();
        index.add_file(&mut writer, "C:\\Users\\ATD\\Downloads\\Documents\\cours_filtrage.pdf", "cours_filtrage.pdf").unwrap();
        index.add_file(&mut writer, "C:\\Users\\ATD\\Downloads\\Documents\\le-filtrage-actif-passif.pdf", "le-filtrage-actif-passif.pdf").unwrap();
        writer.commit().unwrap();

        // Test progression "filtra" -> "filtrag"
        println!("\n=== Test Incremental Typing ===");

        let results = index.search("filtra", 10).unwrap();
        println!("'filtra': {} résultats", results.len());
        for r in &results {
            println!("  - {} (score: {:.2})", r.filename, r.score);
        }
        assert!(results.len() >= 3, "FAIL: 'filtra' devrait trouver 3 fichiers");

        let results = index.search("filtrag", 10).unwrap();
        println!("'filtrag': {} résultats", results.len());
        for r in &results {
            println!("  - {} (score: {:.2})", r.filename, r.score);
        }
        assert!(results.len() >= 3, "FAIL: 'filtrag' devrait AUSSI trouver 3 fichiers!");

        let results = index.search("filtrage", 10).unwrap();
        println!("'filtrage': {} résultats", results.len());
        assert!(results.len() >= 3, "FAIL: 'filtrage' devrait trouver 3 fichiers");

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
