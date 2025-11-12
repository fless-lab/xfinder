# Architecture Technique - xfinder
**Technical Architecture Document**

---

## Vue d'ensemble

### Stack technologique

```
┌─────────────────────────────────────────────────────────┐
│                   FRONTEND (Tauri)                      │
│  React/TypeScript + TailwindCSS + shadcn/ui             │
│                                                          │
│  - Interface recherche                                  │
│  - Configuration dossiers                               │
│  - Mode Assist Me                                       │
│  - Affichage résultats                                  │
└────────────────────┬────────────────────────────────────┘
                     │ IPC (Tauri Commands)
┌────────────────────▼────────────────────────────────────┐
│              BACKEND CORE (Rust)                        │
│                                                          │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │  Watchdog   │  │   Indexer    │  │ContentExtract │  │
│  │  (notify)   │  │  (Tantivy)   │  │ (multi-format)│  │
│  └─────────────┘  └──────────────┘  └───────────────┘  │
│                                                          │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │ EmailParser │  │  AIEngine    │  │SearchEngine   │  │
│  │(libpff/mbox)│  │   (LEANN)    │  │ (orchestrator)│  │
│  └─────────────┘  └──────────────┘  └───────────────┘  │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│                 STORAGE LAYER                           │
│                                                          │
│  ┌──────────────────┐  ┌─────────────┐  ┌────────────┐ │
│  │   index.db       │  │ vectors.db  │  │content.db  │ │
│  │   (SQLite)       │  │  (LEANN)    │  │ (SQLite)   │ │
│  │                  │  │             │  │            │ │
│  │ - Métadonnées    │  │ - Embeddings│  │ - Textes   │ │
│  │ - Chemins        │  │ - Index ANN │  │ - OCR      │ │
│  │ - Config         │  │             │  │            │ │
│  └──────────────────┘  └─────────────┘  └────────────┘ │
└─────────────────────────────────────────────────────────┘
```

---

## Architecture des modules

### 1. Frontend (Tauri + React)

#### Structure dossiers
```
src-ui/
├── src/
│   ├── components/
│   │   ├── Search/
│   │   │   ├── SearchBar.tsx          # Barre recherche principale
│   │   │   ├── SearchFilters.tsx      # Filtres (type, date, taille)
│   │   │   └── SearchResults.tsx      # Affichage résultats
│   │   ├── AssistMe/
│   │   │   ├── ChatInterface.tsx      # Interface conversationnelle
│   │   │   ├── SourceCard.tsx         # Carte citation source
│   │   │   └── ResponseView.tsx       # Affichage réponse IA
│   │   ├── Config/
│   │   │   ├── FolderTree.tsx         # Arborescence dossiers
│   │   │   ├── ExclusionRules.tsx     # Gestion exclusions
│   │   │   └── OcrSettings.tsx        # Config OCR
│   │   └── Common/
│   │       ├── FilePreview.tsx        # Preview fichier
│   │       ├── ProgressBar.tsx        # Barre progression
│   │       └── Shortcuts.tsx          # Raccourcis clavier
│   ├── hooks/
│   │   ├── useSearch.ts               # Hook recherche
│   │   ├── useIndexing.ts             # Hook état indexation
│   │   └── useConfig.ts               # Hook configuration
│   ├── api/
│   │   └── tauri.ts                   # Wrappers Tauri commands
│   ├── types/
│   │   └── index.ts                   # Types TypeScript
│   └── App.tsx
├── package.json
└── vite.config.ts
```

#### Tauri Commands (IPC)

```typescript
// src/api/tauri.ts
import { invoke } from '@tauri-apps/api/tauri';

// Recherche
export async function searchFiles(query: string, filters: SearchFilters): Promise<SearchResult[]> {
  return invoke('search_files', { query, filters });
}

export async function assistMeQuery(question: string): Promise<AssistMeResponse> {
  return invoke('assist_me_query', { question });
}

// Configuration
export async function getWatchedFolders(): Promise<WatchedFolder[]> {
  return invoke('get_watched_folders');
}

export async function updateWatchedFolders(folders: WatchedFolder[]): Promise<void> {
  return invoke('update_watched_folders', { folders });
}

// Indexation
export async function startIndexing(paths: string[]): Promise<void> {
  return invoke('start_indexing', { paths });
}

export async function getIndexingProgress(): Promise<IndexingProgress> {
  return invoke('get_indexing_progress');
}

// OCR
export async function updateOcrConfig(config: OcrConfig): Promise<void> {
  return invoke('update_ocr_config', { config });
}

// Emails
export async function indexEmails(source: EmailSource): Promise<void> {
  return invoke('index_emails', { source });
}
```

#### Types principaux

```typescript
// src/types/index.ts

export interface SearchResult {
  id: string;
  path: string;
  filename: string;
  extension: string;
  size: number;
  modified: Date;
  snippet?: string;
  score: number;
  type: 'file' | 'email';
}

export interface AssistMeResponse {
  answer: string;
  sources: Source[];
  confidence: number;
}

export interface Source {
  id: string;
  path: string;
  title: string;
  type: 'file' | 'email';
  snippet: string;
  page?: number;
  relevance: number;
}

export interface WatchedFolder {
  path: string;
  mode: 'full' | 'metadata' | 'excluded';
  exclusions: Exclusion[];
}

export interface Exclusion {
  type: 'folder' | 'file' | 'extension' | 'pattern';
  value: string;
}

export interface IndexingProgress {
  total_files: number;
  indexed_files: number;
  current_file: string;
  status: 'idle' | 'indexing' | 'paused' | 'error';
  eta_seconds: number;
}

export interface OcrConfig {
  enabled: boolean;
  languages: string[];
  file_types: string[];
  min_size_kb: number;
  folders: string[];
}
```

---

### 2. Backend Rust

#### Structure projet

```
src-tauri/
├── src/
│   ├── main.rs                        # Entry point Tauri
│   ├── lib.rs                         # Modules publics
│   ├── commands.rs                    # Tauri commands
│   ├── modules/
│   │   ├── watchdog/
│   │   │   ├── mod.rs                 # Surveillance filesystem
│   │   │   ├── watcher.rs             # notify-rs wrapper
│   │   │   └── event_handler.rs       # Traitement événements
│   │   ├── indexer/
│   │   │   ├── mod.rs
│   │   │   ├── file_indexer.rs        # Indexation fichiers
│   │   │   ├── metadata.rs            # Extraction métadonnées
│   │   │   └── tantivy_index.rs       # Tantivy wrapper
│   │   ├── content_extractor/
│   │   │   ├── mod.rs
│   │   │   ├── text.rs                # TXT, MD, LOG
│   │   │   ├── pdf.rs                 # PDF (texte + OCR)
│   │   │   ├── office.rs              # DOCX, XLSX, PPT
│   │   │   └── ocr.rs                 # Tesseract wrapper
│   │   ├── search_engine/
│   │   │   ├── mod.rs
│   │   │   ├── query_parser.rs        # Parsing requêtes
│   │   │   ├── ranker.rs              # Scoring résultats
│   │   │   └── filters.rs             # Filtres (date, taille, etc.)
│   │   ├── ai_engine/
│   │   │   ├── mod.rs
│   │   │   ├── leann.rs               # Intégration LEANN
│   │   │   ├── embeddings.rs          # Génération embeddings
│   │   │   ├── vector_search.rs       # Recherche similarité
│   │   │   └── llm.rs                 # LLM local (optionnel)
│   │   ├── email_parser/
│   │   │   ├── mod.rs
│   │   │   ├── pst.rs                 # Outlook PST
│   │   │   ├── mbox.rs                # Thunderbird
│   │   │   └── imap.rs                # IMAP
│   │   ├── database/
│   │   │   ├── mod.rs
│   │   │   ├── schema.rs              # Schéma SQLite
│   │   │   ├── queries.rs             # Requêtes DB
│   │   │   └── migrations.rs          # Migrations
│   │   └── config/
│   │       ├── mod.rs
│   │       ├── settings.rs            # Configuration app
│   │       └── storage.rs             # Persistance config
│   └── utils/
│       ├── hash.rs                     # Hashing fichiers
│       ├── logger.rs                   # Logging
│       └── errors.rs                   # Gestion erreurs
├── Cargo.toml
└── tauri.conf.json
```

#### Cargo.toml (dépendances principales)

```toml
[package]
name = "xfinder"
version = "0.1.0"
edition = "2021"

[dependencies]
# Tauri
tauri = { version = "2.0", features = ["shell-open"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async runtime
tokio = { version = "1", features = ["full"] }
futures = "0.3"

# Filesystem watching
notify = "6.0"
walkdir = "2.4"

# Indexation
tantivy = "0.22"
rusqlite = { version = "0.32", features = ["bundled", "vtab"] }

# Content extraction
pdf-extract = "0.7"
lopdf = "0.32"
docx-rs = "0.4"
calamine = "0.25"  # Excel

# OCR
leptess = "0.14"  # Tesseract binding

# Embeddings
candle-core = "0.6"  # ML framework
candle-nn = "0.6"
tokenizers = "0.19"

# Email parsing
mailparse = "0.15"
# libpff-rs = "0.1"  # Pour PST (si lib existe, sinon FFI)

# Utilities
chrono = "0.4"
uuid = { version = "1.0", features = ["v4"] }
blake3 = "1.5"  # Hashing rapide
rayon = "1.10"  # Parallélisme
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

[build-dependencies]
tauri-build = { version = "2.0" }
```

---

### 3. Module Watchdog

#### Architecture

```rust
// src/modules/watchdog/mod.rs

use notify::{Watcher, RecursiveMode, Event, EventKind};
use std::path::PathBuf;
use tokio::sync::mpsc;

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    event_tx: mpsc::UnboundedSender<FileEvent>,
}

#[derive(Debug, Clone)]
pub enum FileEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    Renamed { from: PathBuf, to: PathBuf },
}

impl FileWatcher {
    pub fn new() -> Result<Self> {
        let (event_tx, mut event_rx) = mpsc::unbounded_channel();

        let watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            match res {
                Ok(event) => {
                    if let Some(file_event) = parse_event(event) {
                        let _ = event_tx.send(file_event);
                    }
                },
                Err(e) => error!("Watchdog error: {:?}", e),
            }
        })?;

        // Spawn event processor
        tokio::spawn(async move {
            let mut debouncer = EventDebouncer::new(500); // 500ms
            while let Some(event) = event_rx.recv().await {
                debouncer.add(event).await;
            }
        });

        Ok(Self { watcher, event_tx })
    }

    pub fn watch(&mut self, path: PathBuf) -> Result<()> {
        self.watcher.watch(&path, RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn unwatch(&mut self, path: PathBuf) -> Result<()> {
        self.watcher.unwatch(&path)?;
        Ok(())
    }
}

// Debouncer pour éviter spam événements
struct EventDebouncer {
    delay_ms: u64,
    pending: HashMap<PathBuf, (FileEvent, Instant)>,
}

impl EventDebouncer {
    async fn add(&mut self, event: FileEvent) {
        let path = event.path();
        self.pending.insert(path.clone(), (event, Instant::now()));

        // Flush après délai
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        if let Some((evt, time)) = self.pending.remove(&path) {
            if time.elapsed().as_millis() >= self.delay_ms {
                self.process_event(evt).await;
            }
        }
    }

    async fn process_event(&self, event: FileEvent) {
        match event {
            FileEvent::Created(path) => {
                // Trigger indexation
                let _ = index_file(path).await;
            },
            FileEvent::Deleted(path) => {
                // Remove from index
                let _ = remove_from_index(path).await;
            },
            FileEvent::Renamed { from, to } => {
                // Update path in DB
                let _ = update_file_path(from, to).await;
            },
            FileEvent::Modified(path) => {
                // Re-index if content changed
                let _ = reindex_if_changed(path).await;
            },
        }
    }
}
```

#### Gestion exclusions

```rust
// src/modules/watchdog/filters.rs

pub struct ExclusionFilter {
    rules: Vec<ExclusionRule>,
}

pub enum ExclusionRule {
    Extension(HashSet<String>),        // .tmp, .log
    Pattern(Vec<glob::Pattern>),       // node_modules, *.bak
    Path(PathBuf),                     // Dossier spécifique
    SizeRange { min: u64, max: u64 },  // Taille fichier
}

impl ExclusionFilter {
    pub fn should_index(&self, path: &Path, metadata: &Metadata) -> bool {
        // Vérifie chaque règle
        for rule in &self.rules {
            match rule {
                ExclusionRule::Extension(exts) => {
                    if let Some(ext) = path.extension() {
                        if exts.contains(ext.to_str().unwrap()) {
                            return false;
                        }
                    }
                },
                ExclusionRule::Pattern(patterns) => {
                    for pattern in patterns {
                        if pattern.matches_path(path) {
                            return false;
                        }
                    }
                },
                ExclusionRule::Path(excluded_path) => {
                    if path.starts_with(excluded_path) {
                        return false;
                    }
                },
                ExclusionRule::SizeRange { min, max } => {
                    let size = metadata.len();
                    if size < *min || size > *max {
                        return false;
                    }
                },
            }
        }
        true
    }
}
```

---

### 4. Module Indexer

#### Tantivy index

```rust
// src/modules/indexer/tantivy_index.rs

use tantivy::*;

pub struct FileIndex {
    index: Index,
    reader: IndexReader,
    writer: IndexWriter,
}

impl FileIndex {
    pub fn new(index_path: &Path) -> Result<Self> {
        let mut schema_builder = Schema::builder();

        // Champs indexés
        schema_builder.add_text_field("path", TEXT | STORED);
        schema_builder.add_text_field("filename", TEXT | STORED);
        schema_builder.add_text_field("content", TEXT);
        schema_builder.add_text_field("extension", STRING | STORED);
        schema_builder.add_u64_field("size", INDEXED | STORED);
        schema_builder.add_date_field("modified", INDEXED | STORED);
        schema_builder.add_bytes_field("file_id", STORED);

        let schema = schema_builder.build();
        let index = Index::create_in_dir(index_path, schema)?;
        let reader = index.reader()?;
        let writer = index.writer(50_000_000)?; // 50MB buffer

        Ok(Self { index, reader, writer })
    }

    pub fn add_file(&mut self, file: &FileMetadata, content: &str) -> Result<()> {
        let mut doc = Document::new();

        doc.add_text(self.schema().get_field("path")?, &file.path);
        doc.add_text(self.schema().get_field("filename")?, &file.filename);
        doc.add_text(self.schema().get_field("content")?, content);
        doc.add_text(self.schema().get_field("extension")?, &file.extension);
        doc.add_u64(self.schema().get_field("size")?, file.size);
        doc.add_date(self.schema().get_field("modified")?, DateTime::from(file.modified));
        doc.add_bytes(self.schema().get_field("file_id")?, file.id.as_bytes());

        self.writer.add_document(doc)?;
        self.writer.commit()?;
        Ok(())
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let searcher = self.reader.searcher();
        let query_parser = QueryParser::for_index(
            &self.index,
            vec![
                self.schema().get_field("filename")?,
                self.schema().get_field("content")?,
            ]
        );

        let query = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let doc = searcher.doc(doc_address)?;
            results.push(SearchResult::from_doc(doc, score));
        }

        Ok(results)
    }

    pub fn update_path(&mut self, old_path: &str, new_path: &str) -> Result<()> {
        // Delete old + add new (Tantivy ne supporte pas update direct)
        let term = Term::from_field_text(self.schema().get_field("path")?, old_path);
        self.writer.delete_term(term);
        // Re-add avec nouveau chemin (récupéré de DB)
        self.writer.commit()?;
        Ok(())
    }
}
```

#### SQLite metadata store

```rust
// src/modules/database/schema.rs

pub struct Database {
    conn: rusqlite::Connection,
}

impl Database {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = rusqlite::Connection::open(db_path)?;
        Self::create_tables(&conn)?;
        Ok(Self { conn })
    }

    fn create_tables(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS files (
                id TEXT PRIMARY KEY,
                path TEXT NOT NULL UNIQUE,
                filename TEXT NOT NULL,
                extension TEXT,
                size INTEGER NOT NULL,
                modified INTEGER NOT NULL,
                created INTEGER NOT NULL,
                hash TEXT,
                indexed_at INTEGER NOT NULL,
                has_embedding BOOLEAN DEFAULT 0
            );

            CREATE INDEX idx_path ON files(path);
            CREATE INDEX idx_modified ON files(modified);
            CREATE INDEX idx_extension ON files(extension);

            CREATE TABLE IF NOT EXISTS embeddings (
                file_id TEXT PRIMARY KEY,
                vector BLOB NOT NULL,
                model TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS content (
                file_id TEXT PRIMARY KEY,
                text TEXT NOT NULL,
                ocr_used BOOLEAN DEFAULT 0,
                extraction_method TEXT,
                FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS content_fts USING fts5(
                file_id UNINDEXED,
                text,
                content=content,
                content_rowid=rowid
            );

            CREATE TABLE IF NOT EXISTS emails (
                id TEXT PRIMARY KEY,
                message_id TEXT UNIQUE,
                subject TEXT,
                from_addr TEXT,
                to_addrs TEXT,
                date INTEGER,
                body_text TEXT,
                folder TEXT,
                source TEXT,
                has_attachments BOOLEAN,
                indexed_at INTEGER
            );

            CREATE TABLE IF NOT EXISTS email_attachments (
                id TEXT PRIMARY KEY,
                email_id TEXT NOT NULL,
                filename TEXT,
                size INTEGER,
                mime_type TEXT,
                extracted_path TEXT,
                file_id TEXT,
                FOREIGN KEY (email_id) REFERENCES emails(id) ON DELETE CASCADE,
                FOREIGN KEY (file_id) REFERENCES files(id)
            );

            CREATE TABLE IF NOT EXISTS watched_folders (
                path TEXT PRIMARY KEY,
                mode TEXT NOT NULL,
                last_scan INTEGER,
                exclusions TEXT
            );

            CREATE TABLE IF NOT EXISTS config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            "#
        )?;
        Ok(())
    }

    pub fn insert_file(&self, file: &FileMetadata) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO files
             (id, path, filename, extension, size, modified, created, hash, indexed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                file.id,
                file.path,
                file.filename,
                file.extension,
                file.size,
                file.modified.timestamp(),
                file.created.timestamp(),
                file.hash,
                chrono::Utc::now().timestamp(),
            ],
        )?;
        Ok(())
    }

    pub fn update_path(&self, file_id: &str, new_path: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE files SET path = ?1 WHERE id = ?2",
            params![new_path, file_id],
        )?;
        Ok(())
    }

    pub fn get_file_by_path(&self, path: &str) -> Result<Option<FileMetadata>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, path, filename, extension, size, modified, created, hash
             FROM files WHERE path = ?1"
        )?;

        let mut rows = stmt.query(params![path])?;
        if let Some(row) = rows.next()? {
            Ok(Some(FileMetadata::from_row(row)?))
        } else {
            Ok(None)
        }
    }
}
```

---

### 5. Module Content Extractor

#### OCR avec Tesseract

```rust
// src/modules/content_extractor/ocr.rs

use leptess::{LepTess, Variable};
use std::path::Path;

pub struct OcrEngine {
    tesseract: LepTess,
    config: OcrConfig,
}

impl OcrEngine {
    pub fn new(config: OcrConfig) -> Result<Self> {
        // Initialise Tesseract avec langues configurées
        let lang = config.languages.join("+"); // "fra+eng"
        let mut tesseract = LepTess::new(None, &lang)?;

        // Optimisations
        tesseract.set_variable(Variable::TesseditPagesegMode, "3")?; // Auto
        tesseract.set_variable(Variable::TesseditOcrEngineMode, "2")?; // LSTM

        Ok(Self { tesseract, config })
    }

    pub fn extract_text_from_image(&mut self, image_path: &Path) -> Result<String> {
        // Preprocessing (optionnel, améliore précision)
        let preprocessed = self.preprocess_image(image_path)?;

        self.tesseract.set_image(&preprocessed)?;
        let text = self.tesseract.get_utf8_text()?;

        Ok(text)
    }

    pub fn extract_from_pdf(&mut self, pdf_path: &Path) -> Result<Vec<PageText>> {
        // Détecte si PDF a une couche texte
        if pdf_has_text_layer(pdf_path)? {
            // Extraction directe (plus rapide)
            return extract_pdf_text(pdf_path);
        }

        // PDF scanné → OCR page par page
        let images = pdf_to_images(pdf_path)?;
        let mut pages = Vec::new();

        for (page_num, image) in images.iter().enumerate() {
            let text = self.extract_text_from_image(image)?;
            pages.push(PageText {
                page: page_num + 1,
                text,
            });
        }

        Ok(pages)
    }

    fn preprocess_image(&self, path: &Path) -> Result<PathBuf> {
        // Leptonica preprocessing
        // - Deskew (redressement)
        // - Binarization (noir & blanc)
        // - Noise removal
        // TODO: Implementation via leptonica-sys
        Ok(path.to_path_buf())
    }

    pub fn should_ocr(&self, file: &FileMetadata) -> bool {
        if !self.config.enabled {
            return false;
        }

        // Vérif extension
        if !self.config.file_types.contains(&file.extension) {
            return false;
        }

        // Vérif taille min
        if file.size < self.config.min_size_kb * 1024 {
            return false;
        }

        // Vérif dossier surveillé
        self.config.folders.iter().any(|folder| {
            file.path.starts_with(folder)
        })
    }
}

// Helper: Détecte si PDF a du texte
fn pdf_has_text_layer(path: &Path) -> Result<bool> {
    use lopdf::Document;
    let doc = Document::load(path)?;

    // Vérifie première page
    if let Ok(page) = doc.get_page(1) {
        if let Ok(content) = page.get_content() {
            // Si contenu > 100 chars, assume texte présent
            return Ok(content.len() > 100);
        }
    }
    Ok(false)
}

// Helper: Convertit PDF en images (via pdf2image ou poppler)
fn pdf_to_images(path: &Path) -> Result<Vec<PathBuf>> {
    // TODO: Implementation avec pdf2image ou poppler-rs
    unimplemented!("Requires pdf-to-image library")
}
```

#### PDF text extraction

```rust
// src/modules/content_extractor/pdf.rs

use pdf_extract::extract_text;
use std::path::Path;

pub fn extract_pdf_text(path: &Path) -> Result<Vec<PageText>> {
    // Méthode 1: pdf-extract (simple mais parfois incomplet)
    match extract_text(path) {
        Ok(text) => Ok(vec![PageText { page: 1, text }]),
        Err(_) => {
            // Fallback: lopdf (plus bas niveau)
            extract_with_lopdf(path)
        }
    }
}

fn extract_with_lopdf(path: &Path) -> Result<Vec<PageText>> {
    use lopdf::Document;
    let doc = Document::load(path)?;
    let mut pages = Vec::new();

    let num_pages = doc.get_pages().len();
    for page_num in 1..=num_pages {
        if let Ok(text) = doc.extract_text(&[page_num as u32]) {
            pages.push(PageText {
                page: page_num,
                text,
            });
        }
    }

    Ok(pages)
}

pub struct PageText {
    pub page: usize,
    pub text: String,
}
```

#### DOCX extraction

```rust
// src/modules/content_extractor/office.rs

use docx_rs::*;
use std::path::Path;

pub fn extract_docx_text(path: &Path) -> Result<String> {
    let file = std::fs::read(path)?;
    let docx = read_docx(&file)?;

    let mut text = String::new();

    // Parcourt les paragraphes
    for child in docx.document.children {
        if let DocumentChild::Paragraph(para) = child {
            for run in para.children {
                if let ParagraphChild::Run(r) = run {
                    for rc in r.children {
                        if let RunChild::Text(t) = rc {
                            text.push_str(&t.text);
                            text.push(' ');
                        }
                    }
                }
            }
            text.push('\n');
        }
    }

    Ok(text)
}

pub fn extract_xlsx_text(path: &Path) -> Result<String> {
    use calamine::{Reader, open_workbook, Xlsx};

    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let mut text = String::new();

    for sheet_name in workbook.sheet_names() {
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            for row in range.rows() {
                for cell in row {
                    text.push_str(&cell.to_string());
                    text.push('\t');
                }
                text.push('\n');
            }
        }
    }

    Ok(text)
}
```

---

### 6. Module AI Engine (LEANN)

#### Intégration LEANN

```rust
// src/modules/ai_engine/leann.rs

// NOTE: LEANN est un projet récent, l'API peut varier
// Ceci est une structure approximative basée sur le concept

pub struct LeannIndex {
    index: leann::Index,
    model: EmbeddingModel,
}

impl LeannIndex {
    pub fn new(index_path: &Path) -> Result<Self> {
        let model = EmbeddingModel::load("all-MiniLM-L6-v2")?;
        let index = leann::Index::create(index_path, model.dimension())?;

        Ok(Self { index, model })
    }

    pub fn add_document(&mut self, file_id: &str, text: &str) -> Result<()> {
        // Split en chunks (500 tokens)
        let chunks = self.split_into_chunks(text, 500);

        for (idx, chunk) in chunks.iter().enumerate() {
            let embedding = self.model.encode(chunk)?;
            let doc_id = format!("{}#{}", file_id, idx);

            self.index.add(doc_id, embedding)?;
        }

        Ok(())
    }

    pub fn search(&self, query: &str, top_k: usize) -> Result<Vec<SearchMatch>> {
        let query_embedding = self.model.encode(query)?;
        let results = self.index.search(query_embedding, top_k)?;

        let mut matches = Vec::new();
        for result in results {
            // Parse doc_id (format: file_id#chunk_idx)
            let parts: Vec<&str> = result.id.split('#').collect();
            let file_id = parts[0];
            let chunk_idx: usize = parts[1].parse()?;

            matches.push(SearchMatch {
                file_id: file_id.to_string(),
                chunk_idx,
                score: result.score,
            });
        }

        Ok(matches)
    }

    fn split_into_chunks(&self, text: &str, max_tokens: usize) -> Vec<String> {
        // Tokenize et split en chunks avec overlap
        let tokens = self.model.tokenize(text);
        let overlap = 50; // Tokens overlap entre chunks

        let mut chunks = Vec::new();
        let mut start = 0;

        while start < tokens.len() {
            let end = (start + max_tokens).min(tokens.len());
            let chunk_tokens = &tokens[start..end];
            let chunk_text = self.model.detokenize(chunk_tokens);
            chunks.push(chunk_text);

            start += max_tokens - overlap;
        }

        chunks
    }
}
```

#### Embedding model (all-MiniLM-L6-v2)

```rust
// src/modules/ai_engine/embeddings.rs

use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use tokenizers::Tokenizer;

pub struct EmbeddingModel {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl EmbeddingModel {
    pub fn load(model_name: &str) -> Result<Self> {
        let device = Device::Cpu; // Ou Device::Cuda(0) si GPU

        // Charge modèle depuis Hugging Face ou local
        let model_path = download_model(model_name)?;
        let tokenizer = Tokenizer::from_file(model_path.join("tokenizer.json"))?;

        let vb = VarBuilder::from_pth(&model_path.join("model.safetensors"), device)?;
        let model = BertModel::load(vb)?;

        Ok(Self { model, tokenizer, device })
    }

    pub fn encode(&self, text: &str) -> Result<Vec<f32>> {
        // Tokenize
        let encoding = self.tokenizer.encode(text, true)?;
        let token_ids = encoding.get_ids();

        // Convertit en tenseur
        let input_ids = Tensor::new(token_ids, &self.device)?;

        // Forward pass
        let outputs = self.model.forward(&input_ids)?;

        // Mean pooling
        let embeddings = mean_pooling(outputs)?;

        // Normalize
        let normalized = normalize(embeddings)?;

        // Convertit en Vec<f32>
        Ok(normalized.to_vec1()?)
    }

    pub fn dimension(&self) -> usize {
        384 // all-MiniLM-L6-v2 dimension
    }

    pub fn tokenize(&self, text: &str) -> Vec<u32> {
        self.tokenizer.encode(text, false).unwrap().get_ids().to_vec()
    }

    pub fn detokenize(&self, tokens: &[u32]) -> String {
        self.tokenizer.decode(tokens, true).unwrap()
    }
}

fn mean_pooling(tensor: Tensor) -> Result<Tensor> {
    // Moyenne des embeddings tokens
    tensor.mean(1)
}

fn normalize(tensor: Tensor) -> Result<Tensor> {
    // L2 normalization
    let norm = tensor.sqr()?.sum(1)?.sqrt()?;
    tensor.broadcast_div(&norm)
}

// Helper: Télécharge modèle depuis HF
fn download_model(name: &str) -> Result<PathBuf> {
    // TODO: Utilise hf-hub ou téléchargement manuel
    // Pour l'instant, assume modèle local
    Ok(PathBuf::from(format!("models/{}", name)))
}
```

---

### 7. Module Search Engine

#### Query orchestration

```rust
// src/modules/search_engine/mod.rs

pub struct SearchEngine {
    file_index: FileIndex,
    leann_index: LeannIndex,
    db: Database,
}

pub enum SearchMode {
    Fast,      // Nom fichier uniquement
    Content,   // Full-text search
    Semantic,  // LEANN vector search
}

impl SearchEngine {
    pub async fn search(
        &self,
        query: &str,
        mode: SearchMode,
        filters: SearchFilters,
    ) -> Result<Vec<SearchResult>> {
        match mode {
            SearchMode::Fast => self.search_fast(query, filters).await,
            SearchMode::Content => self.search_content(query, filters).await,
            SearchMode::Semantic => self.search_semantic(query, filters).await,
        }
    }

    async fn search_fast(&self, query: &str, filters: SearchFilters) -> Result<Vec<SearchResult>> {
        // Recherche Tantivy sur filename uniquement
        let results = self.file_index.search(query, 100)?;
        Ok(self.apply_filters(results, filters))
    }

    async fn search_content(&self, query: &str, filters: SearchFilters) -> Result<Vec<SearchResult>> {
        // SQLite FTS5 sur contenu
        let results = self.db.search_content_fts(query, 100)?;
        Ok(self.apply_filters(results, filters))
    }

    async fn search_semantic(&self, query: &str, filters: SearchFilters) -> Result<Vec<SearchResult>> {
        // LEANN vector search
        let matches = self.leann_index.search(query, 20)?;

        let mut results = Vec::new();
        for m in matches {
            // Récupère metadata depuis DB
            if let Some(file) = self.db.get_file_by_id(&m.file_id)? {
                // Récupère chunk texte
                let chunk = self.db.get_content_chunk(&m.file_id, m.chunk_idx)?;

                results.push(SearchResult {
                    id: file.id,
                    path: file.path,
                    filename: file.filename,
                    snippet: chunk,
                    score: m.score,
                    ..Default::default()
                });
            }
        }

        Ok(self.apply_filters(results, filters))
    }

    fn apply_filters(&self, mut results: Vec<SearchResult>, filters: SearchFilters) -> Vec<SearchResult> {
        results.retain(|r| {
            // Filtre extension
            if let Some(ref exts) = filters.extensions {
                if !exts.contains(&r.extension) {
                    return false;
                }
            }

            // Filtre date
            if let Some(after) = filters.date_after {
                if r.modified < after {
                    return false;
                }
            }

            // Filtre taille
            if let Some(min) = filters.size_min {
                if r.size < min {
                    return false;
                }
            }

            true
        });

        results
    }
}
```

#### Assist Me implementation

```rust
// src/modules/search_engine/assist_me.rs

pub struct AssistMeEngine {
    search_engine: SearchEngine,
    llm: Option<LocalLLM>,  // Optionnel
}

impl AssistMeEngine {
    pub async fn answer_question(&self, question: &str) -> Result<AssistMeResponse> {
        // 1. Recherche sémantique top sources
        let sources = self.search_engine.search(
            question,
            SearchMode::Semantic,
            SearchFilters::default(),
        ).await?;

        // Limite à top 5
        let top_sources = sources.into_iter().take(5).collect::<Vec<_>>();

        // 2. Option A: Sans LLM (liste sources)
        if self.llm.is_none() {
            return Ok(AssistMeResponse {
                answer: self.format_sources_only(&top_sources),
                sources: top_sources.into_iter().map(Source::from).collect(),
                confidence: 0.8,
            });
        }

        // 2. Option B: Avec LLM (génération réponse)
        let context = self.build_context(&top_sources);
        let prompt = format!(
            "Question: {}\n\nContexte:\n{}\n\nRéponds en citant les sources.",
            question, context
        );

        let answer = self.llm.as_ref().unwrap().generate(&prompt).await?;

        Ok(AssistMeResponse {
            answer,
            sources: top_sources.into_iter().map(Source::from).collect(),
            confidence: 0.9,
        })
    }

    fn build_context(&self, results: &[SearchResult]) -> String {
        results.iter().enumerate().map(|(i, r)| {
            format!(
                "[Source {}] {}\nChemin: {}\nExtrait: {}\n",
                i + 1, r.filename, r.path, r.snippet
            )
        }).collect::<Vec<_>>().join("\n")
    }

    fn format_sources_only(&self, results: &[SearchResult]) -> String {
        format!(
            "J'ai trouvé {} documents pertinents :\n\n{}",
            results.len(),
            results.iter().enumerate().map(|(i, r)| {
                format!(
                    "{}. {}\n   📄 {}\n   Extrait: {}\n",
                    i + 1, r.filename, r.path, r.snippet
                )
            }).collect::<Vec<_>>().join("\n")
        )
    }
}

pub struct AssistMeResponse {
    pub answer: String,
    pub sources: Vec<Source>,
    pub confidence: f32,
}

pub struct Source {
    pub id: String,
    pub path: String,
    pub filename: String,
    pub snippet: String,
    pub page: Option<usize>,
    pub relevance: f32,
}
```

---

### 8. Module Email Parser

#### Outlook PST

```rust
// src/modules/email_parser/pst.rs

// NOTE: libpff-rs n'existe pas encore, utilisation via FFI ou alternative

use std::path::Path;

pub struct OutlookParser {
    // Wrapper autour libpff (C library)
}

impl OutlookParser {
    pub fn parse_pst(path: &Path) -> Result<Vec<Email>> {
        // Option 1: FFI vers libpff (C)
        // Option 2: Utiliser readpst (CLI) via process
        // Option 3: mailpst-rs (si existe)

        // Pour l'instant, placeholder
        unimplemented!("PST parsing requires libpff binding")
    }
}

// Alternative: MAPI (Windows uniquement)
#[cfg(target_os = "windows")]
pub fn read_outlook_mapi() -> Result<Vec<Email>> {
    // Utilise win32 API MAPI
    // Requiert Outlook installé
    unimplemented!("MAPI integration")
}
```

#### Thunderbird MBOX

```rust
// src/modules/email_parser/mbox.rs

use mailparse::{parse_mail, MailHeaderMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_mbox(path: &Path) -> Result<Vec<Email>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut emails = Vec::new();
    let mut current_email = Vec::new();

    for line in reader.lines() {
        let line = line?;

        // MBOX format: emails séparés par "From "
        if line.starts_with("From ") && !current_email.is_empty() {
            // Parse email précédent
            let raw = current_email.join("\n");
            if let Ok(mail) = parse_mail(raw.as_bytes()) {
                emails.push(Email::from_parsed_mail(mail)?);
            }
            current_email.clear();
        }

        current_email.push(line);
    }

    // Dernier email
    if !current_email.is_empty() {
        let raw = current_email.join("\n");
        if let Ok(mail) = parse_mail(raw.as_bytes()) {
            emails.push(Email::from_parsed_mail(mail)?);
        }
    }

    Ok(emails)
}

pub struct Email {
    pub id: String,
    pub subject: String,
    pub from: String,
    pub to: Vec<String>,
    pub date: DateTime<Utc>,
    pub body_text: String,
    pub attachments: Vec<Attachment>,
}

impl Email {
    fn from_parsed_mail(mail: mailparse::ParsedMail) -> Result<Self> {
        let headers = mail.headers;

        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            subject: headers.get_first_value("Subject").unwrap_or_default(),
            from: headers.get_first_value("From").unwrap_or_default(),
            to: headers.get_all_values("To"),
            date: parse_date(&headers.get_first_value("Date").unwrap_or_default())?,
            body_text: mail.get_body()?,
            attachments: extract_attachments(&mail)?,
        })
    }
}

fn extract_attachments(mail: &mailparse::ParsedMail) -> Result<Vec<Attachment>> {
    let mut attachments = Vec::new();

    for subpart in &mail.subparts {
        if let Some(filename) = subpart.get_content_disposition().params.get("filename") {
            attachments.push(Attachment {
                filename: filename.clone(),
                data: subpart.get_body_raw()?,
                mime_type: subpart.ctype.mimetype.clone(),
            });
        }
    }

    Ok(attachments)
}
```

---

## Schémas de données

### File metadata
```rust
pub struct FileMetadata {
    pub id: String,              // UUID
    pub path: String,            // Chemin complet
    pub filename: String,        // Nom fichier
    pub extension: String,       // Extension (sans point)
    pub size: u64,               // Octets
    pub modified: DateTime<Utc>, // Dernière modif
    pub created: DateTime<Utc>,  // Création
    pub hash: String,            // Blake3 hash
    pub indexed_at: DateTime<Utc>,
    pub has_embedding: bool,
}
```

### Search result
```rust
pub struct SearchResult {
    pub id: String,
    pub path: String,
    pub filename: String,
    pub extension: String,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub snippet: Option<String>,
    pub score: f32,
    pub result_type: ResultType,
}

pub enum ResultType {
    File,
    Email { message_id: String },
    EmailAttachment { email_id: String },
}
```

### Configuration
```rust
pub struct AppConfig {
    pub watched_folders: Vec<WatchedFolder>,
    pub global_exclusions: GlobalExclusions,
    pub ocr_config: OcrConfig,
    pub email_sources: Vec<EmailSource>,
    pub search_preferences: SearchPreferences,
}

pub struct WatchedFolder {
    pub path: PathBuf,
    pub mode: WatchMode,
    pub exclusions: Vec<ExclusionRule>,
    pub last_scan: Option<DateTime<Utc>>,
}

pub enum WatchMode {
    Full,        // Métadonnées + contenu + embeddings
    MetadataOnly, // Juste nom/date/taille
    Excluded,    // Ignoré
}

pub struct OcrConfig {
    pub enabled: bool,
    pub languages: Vec<String>,  // ["fra", "eng"]
    pub file_types: Vec<String>, // ["jpg", "png", "pdf"]
    pub min_size_kb: u64,
    pub folders: Vec<PathBuf>,   // Dossiers où OCR actif
}
```

---

## Performance & Optimisations

### Indexation initiale
**Objectif : 1000 fichiers/min**

Stratégies :
- **Parallélisme** : Rayon pour traiter N fichiers simultanément
- **Queue prioritaire** : Dossiers récents en premier
- **Batch DB inserts** : 1000 inserts → 1 transaction
- **Lazy embeddings** : Génération en background après métadonnées

```rust
pub async fn index_folder_parallel(path: &Path, pool_size: usize) -> Result<()> {
    let files: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .collect();

    // Process par batches parallèles
    files.par_chunks(pool_size)
        .for_each(|chunk| {
            for file in chunk {
                let _ = index_file(file);
            }
        });

    Ok(())
}
```

### Mémoire
**Objectif : <500MB idle, <2GB indexation**

- LEANN index sur disque (memory-mapped)
- Tantivy index avec buffer limité (50MB)
- Streaming content extraction (pas tout en RAM)

### Stockage
**Objectif : Index <5% taille corpus**

| Corpus | Index SQLite | Tantivy | LEANN | Total | Ratio |
|--------|-------------|---------|-------|-------|-------|
| 100GB | 500MB | 800MB | 200MB | 1.5GB | 1.5% |
| 500GB | 2GB | 3.5GB | 800MB | 6.3GB | 1.3% |

---

## Sécurité

### Données locales
- Tout stocké localement (pas de cloud)
- Chiffrement optionnel DB (SQLCipher)
- Passwords emails chiffrés (Windows DPAPI)

### Permissions
- Élévation UAC si besoin (accès fichiers système)
- Respect ACL Windows
- Logs accès fichiers sensibles

### Code safety
- Rust = Memory safety
- Input validation (chemins, queries)
- Sandboxing Tauri (frontend isolé)

---

## Monitoring & Logs

### Telemetry (optionnelle, opt-in)
```rust
pub struct Telemetry {
    // Anonymisé, local only
    pub search_count: u64,
    pub avg_search_time_ms: f64,
    pub indexing_time_total: Duration,
    pub files_indexed: u64,
    pub errors_count: u64,
}
```

### Logs
```rust
// tracing framework
use tracing::{info, warn, error};

info!(file_path = %path, "Indexing file");
warn!(error = ?e, "OCR failed, skipping");
error!(db_error = ?e, "Database corruption detected");
```

---

## Déploiement

### Installateur Windows (.msi)

- **WiX Toolset** : Génération MSI
- **Silent install** : Support GPO entreprise
- **Auto-update** : Tauri updater

### Taille distribution
- Base app : ~10MB
- Tesseract + langues : +30MB
- Modèle embeddings : +80MB
- **Total : ~120MB**

Option : Téléchargement components on-demand
- Base : 10MB
- OCR : +30MB (si activé)
- AI : +80MB (si Assist Me activé)

---

## Prochaines étapes techniques

1. **POC LEANN** : Valider performance (benchmark vs FAISS)
2. **Test Tesseract** : Précision français, vitesse
3. **Prototype Tauri** : Setup projet, IPC basique
4. **Schema DB finalisé** : Migrations, indexes optimisés
5. **Benchmarks** : Indexation 100k fichiers, mémoire, vitesse recherche

---

**Document version :** 1.0
**Dernière mise à jour :** 2025-11-12
