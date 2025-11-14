// tests/e2e_phase1.rs
// Tests E2E Phase 1 - MVP Indexation
//
// TS-001: Test workflow complet indexation → recherche
// TS-002: Test watchdog détection changements
// TS-003: Test performance sur 10k fichiers

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use xfinder::search::{FileScanner, SearchIndex, SearchOptions};
use xfinder::database::Database;

// TS-001: Workflow complet indexation → recherche
#[test]
fn test_e2e_indexing_and_search() -> Result<()> {
    println!("✅ TS-001: Test workflow indexation → recherche");

    // 1. Setup: créer dossier temporaire avec fichiers
    let temp_dir = TempDir::new()?;
    let test_files = vec![
        ("readme.md", "# README\nProject documentation"),
        ("config.toml", "[app]\nname = \"xfinder\""),
        ("main.rs", "fn main() { println!(\"Hello\"); }"),
        ("test.txt", "Test file content"),
        ("data.json", "{\"name\": \"test\"}"),
    ];

    for (filename, content) in &test_files {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, content)?;
    }

    // 2. Indexation: scanner + index
    let index_dir = TempDir::new()?;
    let index = SearchIndex::new(index_dir.path(), 2, 20)?;

    let scanner = FileScanner::new();
    let files = scanner.scan_directory(
        temp_dir.path(),
        100,
        &[],
        &[],
        &[],
    )?;

    println!("   Scanned {} files", files.len());
    assert_eq!(files.len(), 5, "Should find 5 files");

    // Indexer tous les fichiers
    let mut writer = index.create_writer()?;
    for file in &files {
        index.add_file(&mut writer, &file.path, &file.filename)?;
    }
    writer.commit()?;

    // 3. Recherche: tester différentes requêtes
    // Note: seuls les noms de fichiers sont indexés, pas le contenu
    let test_queries = vec![
        ("readme", 1),    // readme.md
        ("md", 1),        // readme.md
        ("toml", 1),      // config.toml
        ("rs", 1),        // main.rs
        ("test", 1),      // test.txt seulement
    ];

    for (query, expected_min) in test_queries {
        let results = index.search(query, 50, SearchOptions::default())?;
        println!("   Search '{}': {} results", query, results.len());
        assert!(
            results.len() >= expected_min,
            "Query '{}' should return at least {} results, got {}",
            query,
            expected_min,
            results.len()
        );
    }

    println!("   ✅ Indexation and search workflow working!");
    Ok(())
}

// TS-002: Test database integration
#[test]
fn test_e2e_database_integration() -> Result<()> {
    println!("✅ TS-002: Test database integration");

    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");

    // 1. Créer database
    let db = Database::new(&db_path)?;

    // 2. Créer quelques fichiers de test
    let test_dir = TempDir::new()?;
    for i in 0..10 {
        let file_path = test_dir.path().join(format!("file_{}.txt", i));
        fs::write(&file_path, format!("Content {}", i))?;
    }

    // 3. Scanner et indexer dans DB
    let scanner = FileScanner::new();
    let files = scanner.scan_directory(
        test_dir.path(),
        100,
        &[],
        &[],
        &[],
    )?;

    println!("   Scanned {} files", files.len());

    // 4. Insérer dans DB avec hash
    let mut file_records = Vec::new();
    for file in &files {
        let file_record = xfinder::database::queries::FileRecord {
            id: format!("{:x}", file.path.as_bytes().iter().fold(0u64, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u64))),
            path: file.path.clone(),
            filename: file.filename.clone(),
            extension: PathBuf::from(&file.path)
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| format!(".{}", s)),
            size: 100,
            modified: chrono::Utc::now().timestamp(),
            created: chrono::Utc::now().timestamp(),
            hash: Some("dummy_hash".to_string()),
            indexed_at: chrono::Utc::now().timestamp(),
        };
        file_records.push(file_record);
    }

    db.batch_upsert_files(&file_records)?;

    // 5. Vérifier le nombre de fichiers
    let file_count = db.count_files()?;
    println!("   Database stats: {} files", file_count);
    assert_eq!(file_count, 10);

    println!("   ✅ Database integration working!");
    Ok(())
}

// TS-003: Test performance avec 1000 fichiers
#[test]
#[ignore] // Long test - run with --ignored
fn test_e2e_performance_1000_files() -> Result<()> {
    use std::time::Instant;

    println!("✅ TS-003: Test performance 1000 fichiers");

    // 1. Créer 1000 fichiers de test
    let temp_dir = TempDir::new()?;
    let start_create = Instant::now();

    for i in 0..1000 {
        let file_path = temp_dir.path().join(format!("test_file_{}.txt", i));
        fs::write(&file_path, format!("Test content {}", i))?;
    }

    let create_time = start_create.elapsed();
    println!("   Created 1000 files in {:?}", create_time);

    // 2. Scanner les fichiers
    let start_scan = Instant::now();
    let scanner = FileScanner::new();
    let files = scanner.scan_directory(
        temp_dir.path(),
        10000,
        &[],
        &[],
        &[],
    )?;

    let scan_time = start_scan.elapsed();
    println!("   Scanned {} files in {:?} ({:.0} files/sec)",
        files.len(), scan_time, files.len() as f64 / scan_time.as_secs_f64());

    assert_eq!(files.len(), 1000);

    // 3. Indexation
    let index_dir = TempDir::new()?;
    let index = SearchIndex::new(index_dir.path(), 2, 20)?;

    let start_index = Instant::now();
    let mut writer = index.create_writer()?;
    for file in &files {
        index.add_file(&mut writer, &file.path, &file.filename)?;
    }
    writer.commit()?;

    let index_time = start_index.elapsed();
    println!("   Indexed 1000 files in {:?} ({:.0} files/sec)",
        index_time, 1000.0 / index_time.as_secs_f64());

    // 4. Recherche
    let start_search = Instant::now();
    let results = index.search("test", 50, SearchOptions::default())?;
    let search_time = start_search.elapsed();

    println!("   Search found {} results in {:?} ({:.2}ms)",
        results.len(), search_time, search_time.as_secs_f64() * 1000.0);

    // Vérifier les performances cibles
    assert!(scan_time.as_secs() < 10, "Scan should complete in <10s");
    assert!(index_time.as_secs() < 60, "Indexing should complete in <60s (1000 files/min)");
    assert!(search_time.as_millis() < 100, "Search should complete in <100ms");

    println!("   ✅ Performance targets met!");
    println!("      - Scan: {:?} (target: <10s)", scan_time);
    println!("      - Index: {:?} (target: <60s)", index_time);
    println!("      - Search: {:?} (target: <100ms)", search_time);

    Ok(())
}

// Test de fuzzy matching E2E
#[test]
fn test_e2e_fuzzy_search() -> Result<()> {
    println!("✅ Test fuzzy search E2E");

    let temp_dir = TempDir::new()?;
    let test_files = vec![
        "document.pdf",
        "documents.txt",
        "documant.md", // typo volontaire
    ];

    for filename in &test_files {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, "test")?;
    }

    // Indexation
    let index_dir = TempDir::new()?;
    let index = SearchIndex::new(index_dir.path(), 2, 20)?;

    let scanner = FileScanner::new();
    let files = scanner.scan_directory(
        temp_dir.path(),
        100,
        &[],
        &[],
        &[],
    )?;

    let mut writer = index.create_writer()?;
    for file in &files {
        index.add_file(&mut writer, &file.path, &file.filename)?;
    }
    writer.commit()?;

    // Recherche fuzzy avec typo
    let fuzzy_options = SearchOptions {
        fuzzy_search: true,
        fuzzy_distance: 1,
        ..Default::default()
    };

    let results = index.search("documnt", 50, fuzzy_options)?; // typo: documnt au lieu de document
    println!("   Fuzzy search 'documnt' found {} results", results.len());

    // Devrait trouver au moins document et documant
    assert!(results.len() >= 2, "Fuzzy search should find similar filenames");

    println!("   ✅ Fuzzy search working!");
    Ok(())
}
