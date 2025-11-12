# 🎯 Plan d'Implémentation - Tâches Immédiates

**Guide étape par étape pour implémenter xfinder**

---

## 📋 Comment utiliser ce document

- ✅ **Coche les tâches** au fur et à mesure
- 📝 **Commit après chaque tâche terminée**
- 🔄 **Reviens ici** après chaque tâche pour voir la suivante
- ⏭️ **Ne saute pas d'étapes** (chaque tâche dépend de la précédente)

---

## 🚀 PHASE 0 : Setup Initial (1-2h)

### Tâche 0.1 : Installer les outils
```bash
[ ] Installer Rust (https://rustup.rs/)
[ ] Vérifier : rustc --version
[ ] Vérifier : cargo --version
```

### Tâche 0.2 : Créer structure projet
```bash
cd D:\DataLab\xfinder

[ ] Créer Cargo.toml (copier depuis QUICKSTART.md)
[ ] Créer src/main.rs (copier Hello World egui)
```

**Cargo.toml à créer :**
```toml
[package]
name = "xfinder"
version = "0.1.0"
edition = "2021"

[dependencies]
eframe = "0.27"
egui = "0.27"
tantivy = "0.22"
rusqlite = { version = "0.32", features = ["bundled"] }
walkdir = "2.4"
anyhow = "1.0"
```

**src/main.rs à créer :**
```rust
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "xfinder",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    search_query: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            search_query: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔍 xfinder");
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.label("Rechercher :");
                ui.text_edit_singleline(&mut self.search_query);
            });

            if !self.search_query.is_empty() {
                ui.label(format!("Recherche : {}", self.search_query));
            }
        });
    }
}
```

### Tâche 0.3 : Premier test
```bash
[ ] cargo run
[ ] ✅ Fenêtre s'ouvre avec "xfinder" et barre recherche ?
```

**Si ça marche :**
```bash
git add .
git commit -m "feat: hello world egui fonctionne"
```

**Si ça marche PAS :**
- Vérifie Cargo.toml (pas d'erreur syntaxe)
- Vérifie src/main.rs (copié exactement)
- Lance : `cargo clean && cargo build`

---

## 📁 SEMAINE 1 : Indexation Tantivy Basique

### Tâche 1.1 : Créer module search/
```bash
[ ] mkdir src/search
[ ] Créer src/search/mod.rs
[ ] Créer src/search/tantivy_index.rs
```

**src/search/mod.rs :**
```rust
pub mod tantivy_index;

pub use tantivy_index::SearchIndex;
```

### Tâche 1.2 : Setup Tantivy basique

**src/search/tantivy_index.rs :**
```rust
use tantivy::*;
use anyhow::Result;
use std::path::Path;

pub struct SearchIndex {
    index: Index,
    schema: Schema,
}

impl SearchIndex {
    pub fn new(index_dir: &Path) -> Result<Self> {
        // Créer schéma
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("path", TEXT | STORED);
        schema_builder.add_text_field("filename", TEXT | STORED);
        let schema = schema_builder.build();

        // Créer index
        std::fs::create_dir_all(index_dir)?;
        let index = Index::create_in_dir(index_dir, schema.clone())?;

        Ok(Self { index, schema })
    }

    pub fn add_file(&self, path: &str, filename: &str) -> Result<()> {
        let mut index_writer = self.index.writer(50_000_000)?;

        let path_field = self.schema.get_field("path").unwrap();
        let filename_field = self.schema.get_field("filename").unwrap();

        let mut doc = Document::new();
        doc.add_text(path_field, path);
        doc.add_text(filename_field, filename);

        index_writer.add_document(doc)?;
        index_writer.commit()?;

        Ok(())
    }

    pub fn search(&self, query_text: &str, limit: usize) -> Result<Vec<(String, String)>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();

        let filename_field = self.schema.get_field("filename").unwrap();
        let query_parser = QueryParser::for_index(&self.index, vec![filename_field]);
        let query = query_parser.parse_query(query_text)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        for (_score, doc_address) in top_docs {
            let doc = searcher.doc(doc_address)?;
            let path = doc.get_first(self.schema.get_field("path").unwrap())
                .and_then(|v| v.as_text())
                .unwrap_or("");
            let filename = doc.get_first(self.schema.get_field("filename").unwrap())
                .and_then(|v| v.as_text())
                .unwrap_or("");

            results.push((path.to_string(), filename.to_string()));
        }

        Ok(results)
    }
}
```

**Checklist :**
```bash
[ ] Créer les fichiers ci-dessus
[ ] cargo build
[ ] ✅ Compile sans erreur ?
```

**Commit :**
```bash
git add src/search/
git commit -m "feat: ajout module search avec Tantivy basique"
```

---

### Tâche 1.3 : Indexer dossier test

**Modifier src/main.rs :**
```rust
use eframe::egui;
mod search;
use search::SearchIndex;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    // Créer index au démarrage
    let index_dir = PathBuf::from("./test_index");
    let index = SearchIndex::new(&index_dir)
        .expect("Échec création index");

    // Indexer quelques fichiers test
    index_test_files(&index);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "xfinder",
        options,
        Box::new(move |_cc| Box::new(MyApp::new(index))),
    )
}

fn index_test_files(index: &SearchIndex) {
    // Crée quelques fichiers test pour commencer
    let test_files = vec![
        ("C:\\test\\document.pdf", "document.pdf"),
        ("C:\\test\\rapport.docx", "rapport.docx"),
        ("C:\\test\\photo.jpg", "photo.jpg"),
    ];

    for (path, filename) in test_files {
        let _ = index.add_file(path, filename);
    }

    println!("✅ 3 fichiers test indexés");
}

struct MyApp {
    search_query: String,
    results: Vec<(String, String)>,
    index: SearchIndex,
}

impl MyApp {
    fn new(index: SearchIndex) -> Self {
        Self {
            search_query: String::new(),
            results: Vec::new(),
            index,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔍 xfinder");
            ui.add_space(20.0);

            // Barre recherche
            let response = ui.horizontal(|ui| {
                ui.label("Rechercher :");
                ui.text_edit_singleline(&mut self.search_query)
            }).inner;

            // Recherche quand on tape
            if response.changed() && !self.search_query.is_empty() {
                if let Ok(results) = self.index.search(&self.search_query, 10) {
                    self.results = results;
                }
            }

            ui.add_space(10.0);
            ui.separator();

            // Affiche résultats
            if !self.results.is_empty() {
                ui.label(format!("📁 {} résultats", self.results.len()));
                ui.add_space(5.0);

                for (path, filename) in &self.results {
                    ui.label(format!("📄 {}", filename));
                    ui.label(format!("   {}", path));
                    ui.add_space(5.0);
                }
            }
        });
    }
}
```

**Checklist :**
```bash
[ ] Modifier src/main.rs
[ ] cargo run
[ ] ✅ Console affiche "✅ 3 fichiers test indexés" ?
[ ] ✅ Tape "document" → affiche "document.pdf" ?
[ ] ✅ Tape "rapport" → affiche "rapport.docx" ?
```

**Commit :**
```bash
git add src/main.rs
git commit -m "feat: recherche Tantivy fonctionne avec fichiers test"
```

---

### Tâche 1.4 : Indexer VRAI dossier (ex: Downloads/)

**Ajouter dans Cargo.toml :**
```toml
walkdir = "2.4"
```

**Créer src/indexer/mod.rs :**
```rust
use walkdir::WalkDir;
use std::path::Path;
use crate::search::SearchIndex;
use anyhow::Result;

pub fn index_folder(index: &SearchIndex, folder_path: &Path) -> Result<usize> {
    let mut count = 0;

    for entry in WalkDir::new(folder_path)
        .max_depth(3) // Limite à 3 niveaux pour test
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path().to_string_lossy().to_string();
            let filename = entry.file_name().to_string_lossy().to_string();

            index.add_file(&path, &filename)?;
            count += 1;

            if count % 100 == 0 {
                println!("📁 {} fichiers indexés...", count);
            }
        }
    }

    println!("✅ Indexation terminée : {} fichiers", count);
    Ok(count)
}
```

**Modifier src/main.rs :**
```rust
// Ajouter en haut
mod indexer;

// Remplacer index_test_files() par :
fn index_real_folder(index: &SearchIndex) {
    use std::path::PathBuf;

    // Change ce chemin vers TON dossier à tester
    let test_folder = PathBuf::from("C:\\Users\\TON_USER\\Downloads");

    println!("🔍 Indexation de : {:?}", test_folder);

    match indexer::index_folder(index, &test_folder) {
        Ok(count) => println!("✅ {} fichiers indexés", count),
        Err(e) => eprintln!("❌ Erreur indexation : {}", e),
    }
}

// Remplacer dans main() :
// index_test_files(&index);
index_real_folder(&index);
```

**Checklist :**
```bash
[ ] mkdir src/indexer
[ ] Créer src/indexer/mod.rs
[ ] Modifier src/main.rs (ajoute mod indexer;)
[ ] CHANGER le chemin dans index_real_folder() vers TON dossier
[ ] cargo run
[ ] ✅ Console affiche progression "📁 X fichiers indexés..." ?
[ ] ✅ Recherche fonctionne sur TES fichiers ?
```

**Commit :**
```bash
git add .
git commit -m "feat: indexation dossier réel fonctionne"
```

---

### Tâche 1.5 : Améliorer UI résultats

**Modifier src/main.rs (partie affichage résultats) :**
```rust
// Dans impl eframe::App for MyApp, remplacer section résultats par :

// Affiche résultats
egui::ScrollArea::vertical()
    .max_height(400.0)
    .show(ui, |ui| {
        if !self.results.is_empty() {
            ui.label(format!("📁 {} résultat(s)", self.results.len()));
            ui.add_space(10.0);

            for (idx, (path, filename)) in self.results.iter().enumerate() {
                // Carte résultat
                egui::Frame::none()
                    .fill(if idx % 2 == 0 {
                        egui::Color32::from_rgb(240, 240, 240)
                    } else {
                        egui::Color32::from_rgb(250, 250, 250)
                    })
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("📄");
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(filename).strong());
                                ui.label(
                                    egui::RichText::new(path)
                                        .small()
                                        .color(egui::Color32::GRAY)
                                );
                            });
                        });

                        // Bouton ouvrir
                        if ui.button("📂 Ouvrir").clicked() {
                            open_file(path);
                        }
                    });

                ui.add_space(5.0);
            }
        } else if !self.search_query.is_empty() {
            ui.label("❌ Aucun résultat");
        }
    });

// Ajouter cette fonction en dehors de impl :
fn open_file(path: &str) {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let _ = Command::new("cmd")
            .args(&["/C", "start", "", path])
            .spawn();
    }
}
```

**Checklist :**
```bash
[ ] Modifier src/main.rs
[ ] cargo run
[ ] ✅ Résultats dans cartes grises/blanches alternées ?
[ ] ✅ Bouton "📂 Ouvrir" visible ?
[ ] ✅ Clic bouton ouvre le fichier ?
```

**Commit :**
```bash
git add src/main.rs
git commit -m "feat: amélioration UI résultats avec bouton ouvrir"
```

---

## ✅ FIN SEMAINE 1 : Bilan

**Tu as maintenant :**
- ✅ Interface egui fonctionnelle
- ✅ Indexation Tantivy basique
- ✅ Recherche en temps réel
- ✅ Affichage résultats
- ✅ Ouverture fichiers

**Prochaine étape (Semaine 2) :** SQLite pour métadonnées

---

## 📊 SEMAINE 2 : Base de données SQLite

### Tâche 2.1 : Créer module database/

```bash
[ ] mkdir src/database
[ ] Créer src/database/mod.rs
```

**src/database/mod.rs :**
```rust
use rusqlite::{Connection, Result, params};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Créer tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL UNIQUE,
                filename TEXT NOT NULL,
                size INTEGER,
                modified INTEGER,
                indexed_at INTEGER
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn insert_file(&self, path: &str, filename: &str, size: u64, modified: i64) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO files (path, filename, size, modified, indexed_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![path, filename, size as i64, modified, chrono::Utc::now().timestamp()],
        )?;
        Ok(())
    }

    pub fn get_file_info(&self, path: &str) -> Result<Option<FileInfo>> {
        let mut stmt = self.conn.prepare(
            "SELECT filename, size, modified FROM files WHERE path = ?1"
        )?;

        let mut rows = stmt.query(params![path])?;

        if let Some(row) = rows.next()? {
            Ok(Some(FileInfo {
                filename: row.get(0)?,
                size: row.get(1)?,
                modified: row.get(2)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn count_files(&self) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM files",
            [],
            |row| row.get(0)
        )?;
        Ok(count)
    }
}

pub struct FileInfo {
    pub filename: String,
    pub size: i64,
    pub modified: i64,
}
```

**Ajouter dans Cargo.toml :**
```toml
chrono = "0.4"
```

**Checklist :**
```bash
[ ] Créer src/database/mod.rs
[ ] Ajouter chrono dans Cargo.toml
[ ] cargo build
[ ] ✅ Compile ?
```

**Commit :**
```bash
git add src/database/
git commit -m "feat: ajout module database SQLite"
```

---

### Tâche 2.2 : Intégrer DB dans indexation

**Modifier src/indexer/mod.rs :**
```rust
use std::fs;
use crate::database::Database;

pub fn index_folder_with_db(
    search_index: &SearchIndex,
    db: &Database,
    folder_path: &Path
) -> Result<usize> {
    let mut count = 0;

    for entry in WalkDir::new(folder_path)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path().to_string_lossy().to_string();
            let filename = entry.file_name().to_string_lossy().to_string();

            // Métadonnées
            if let Ok(metadata) = fs::metadata(entry.path()) {
                let size = metadata.len();
                let modified = metadata.modified()
                    .unwrap_or(std::time::SystemTime::now())
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;

                // Indexe dans Tantivy
                search_index.add_file(&path, &filename)?;

                // Stocke dans DB
                db.insert_file(&path, &filename, size, modified)?;

                count += 1;

                if count % 100 == 0 {
                    println!("📁 {} fichiers indexés...", count);
                }
            }
        }
    }

    println!("✅ {} fichiers indexés", count);
    Ok(count)
}
```

**Modifier src/main.rs :**
```rust
mod database;
use database::Database;

// Dans main(), ajouter :
let db = Database::new(&PathBuf::from("./xfinder.db"))
    .expect("Échec création DB");

// Modifier appel indexation :
match indexer::index_folder_with_db(&index, &db, &test_folder) {
    Ok(count) => {
        println!("✅ {} fichiers indexés", count);
        let total = db.count_files().unwrap_or(0);
        println!("📊 Total en DB : {}", total);
    },
    Err(e) => eprintln!("❌ Erreur : {}", e),
}

// Passer db à MyApp
Box::new(move |_cc| Box::new(MyApp::new(index, db)))

// Modifier struct MyApp :
struct MyApp {
    search_query: String,
    results: Vec<(String, String)>,
    index: SearchIndex,
    db: Database,
}

impl MyApp {
    fn new(index: SearchIndex, db: Database) -> Self {
        Self {
            search_query: String::new(),
            results: Vec::new(),
            index,
            db,
        }
    }
}
```

**Checklist :**
```bash
[ ] Modifier src/indexer/mod.rs
[ ] Modifier src/main.rs
[ ] cargo run
[ ] ✅ Console affiche "📊 Total en DB : X" ?
[ ] ✅ Fichier xfinder.db créé dans dossier ?
```

**Commit :**
```bash
git add .
git commit -m "feat: intégration SQLite dans indexation"
```

---

### Tâche 2.3 : Afficher métadonnées dans résultats

**Modifier src/main.rs (struct MyApp) :**
```rust
// Changer type results :
results: Vec<(String, String, Option<(u64, i64)>)>, // (path, filename, (size, modified))

// Dans update(), quand recherche :
if response.changed() && !self.search_query.is_empty() {
    if let Ok(search_results) = self.index.search(&self.search_query, 10) {
        self.results = search_results.into_iter().map(|(path, filename)| {
            // Récupère infos DB
            let metadata = self.db.get_file_info(&path)
                .ok()
                .flatten()
                .map(|info| (info.size as u64, info.modified));
            (path, filename, metadata)
        }).collect();
    }
}

// Dans affichage résultats :
for (idx, (path, filename, metadata)) in self.results.iter().enumerate() {
    egui::Frame::none()
        .fill(...)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("📄");
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(filename).strong());
                    ui.label(
                        egui::RichText::new(path)
                            .small()
                            .color(egui::Color32::GRAY)
                    );

                    // Afficher métadonnées
                    if let Some((size, modified)) = metadata {
                        ui.horizontal(|ui| {
                            ui.label(format!("📊 {}", format_size(*size)));
                            ui.label("•");
                            ui.label(format!("🕒 {}", format_date(*modified)));
                        });
                    }
                });
            });

            if ui.button("📂 Ouvrir").clicked() {
                open_file(path);
            }
        });
    ui.add_space(5.0);
}

// Ajouter fonctions helper :
fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn format_date(timestamp: i64) -> String {
    use chrono::{DateTime, Utc};
    let dt = DateTime::<Utc>::from_timestamp(timestamp, 0)
        .unwrap_or_else(|| Utc::now());
    dt.format("%d/%m/%Y %H:%M").to_string()
}
```

**Checklist :**
```bash
[ ] Modifier src/main.rs
[ ] cargo run
[ ] ✅ Résultats affichent taille (ex: "2.3 MB") ?
[ ] ✅ Résultats affichent date (ex: "12/11/2025 14:30") ?
```

**Commit :**
```bash
git add src/main.rs
git commit -m "feat: affichage métadonnées (taille, date) dans résultats"
```

---

## ✅ FIN SEMAINE 2 : Bilan

**Tu as maintenant :**
- ✅ SQLite intégré
- ✅ Métadonnées stockées (taille, date)
- ✅ Résultats enrichis
- ✅ ~100-500 fichiers indexés

**Prochaine étape (Semaine 3) :** Watchdog auto-indexation

---

## 🔄 SEMAINE 3 : Watchdog (auto-indexation)

### Tâche 3.1 : Créer module watchdog/

**À suivre après Semaine 2...**

---

## 📝 Comment utiliser ce plan

### Règle d'or :
**1 tâche à la fois, teste, commit, passe à la suivante**

### Si tu bloques >30 min :
1. Lis l'erreur cargo
2. Google : "rust [ton erreur]"
3. Vérifie que tu as bien copié le code
4. Reviens en arrière : `git reset --hard`

### Fréquence commit :
- ✅ Après chaque tâche terminée
- ✅ Avant de tester quelque chose de nouveau
- ✅ En fin de session

---

## 🎯 Progression

```
Semaine 1 : [████████░░] 80% - Recherche basique
Semaine 2 : [████░░░░░░] 40% - SQLite métadonnées
Semaine 3 : [░░░░░░░░░░]  0% - Watchdog
Semaine 4 : [░░░░░░░░░░]  0% - Configuration
```

**Reviens ici après chaque tâche pour cocher et voir la suite ! ✅**

---

**Document version :** 1.0
**Dernière mise à jour :** 2025-11-12
**Usage :** Plan pas-à-pas implémentation
