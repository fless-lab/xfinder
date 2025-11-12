# API & Schémas de Données - xfinder
**API Reference & Data Schemas**

---

## Table des matières

1. [Tauri Commands (IPC)](#tauri-commands-ipc)
2. [Types TypeScript Frontend](#types-typescript-frontend)
3. [Structures Rust Backend](#structures-rust-backend)
4. [Schéma Base de Données](#schéma-base-de-données)
5. [Events (Frontend ← Backend)](#events-frontend--backend)
6. [Configuration Files](#configuration-files)

---

## Tauri Commands (IPC)

### Convention de nommage
- Format snake_case : `search_files`, `get_config`
- Préfixe par module : `index_*`, `search_*`, `config_*`, `email_*`

### Search Module

#### `search_files`
Recherche rapide par nom/métadonnées.

**Frontend (TypeScript) :**
```typescript
import { invoke } from '@tauri-apps/api/tauri';

interface SearchFilters {
  extensions?: string[];      // [".pdf", ".docx"]
  date_after?: number;        // Unix timestamp
  date_before?: number;
  size_min?: number;          // Bytes
  size_max?: number;
  folder?: string;            // Recherche dans dossier spécifique
}

interface SearchResult {
  id: string;
  path: string;
  filename: string;
  extension: string;
  size: number;
  modified: number;           // Unix timestamp
  snippet?: string;
  score: number;
  type: 'file' | 'email';
}

const results = await invoke<SearchResult[]>('search_files', {
  query: 'contrat dupont',
  filters: {
    extensions: ['.pdf'],
    date_after: Date.now() - 30 * 24 * 60 * 60 * 1000, // 30 jours
  },
  limit: 100,
});
```

**Backend (Rust) :**
```rust
#[tauri::command]
async fn search_files(
    query: String,
    filters: SearchFilters,
    limit: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, String> {
    let engine = state.search_engine.lock().await;
    engine.search(&query, SearchMode::Fast, filters, limit.unwrap_or(100))
        .await
        .map_err(|e| e.to_string())
}
```

---

#### `search_content`
Recherche full-text dans contenu.

**Frontend :**
```typescript
const results = await invoke<SearchResult[]>('search_content', {
  query: 'budget formation 2024',
  filters: {},
  limit: 50,
});
```

**Backend :**
```rust
#[tauri::command]
async fn search_content(
    query: String,
    filters: SearchFilters,
    limit: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, String> {
    let engine = state.search_engine.lock().await;
    engine.search(&query, SearchMode::Content, filters, limit.unwrap_or(50))
        .await
        .map_err(|e| e.to_string())
}
```

---

#### `assist_me_query`
Mode conversationnel avec sources.

**Frontend :**
```typescript
interface AssistMeResponse {
  answer: string;
  sources: Source[];
  confidence: number;
  processing_time_ms: number;
}

interface Source {
  id: string;
  path: string;
  filename: string;
  type: 'file' | 'email';
  snippet: string;
  page?: number;
  relevance: number;
}

const response = await invoke<AssistMeResponse>('assist_me_query', {
  question: 'Quels sont les budgets formation 2024 ?',
  include_emails: true,
});
```

**Backend :**
```rust
#[tauri::command]
async fn assist_me_query(
    question: String,
    include_emails: bool,
    state: State<'_, AppState>,
) -> Result<AssistMeResponse, String> {
    let assist_me = state.assist_me_engine.lock().await;
    assist_me.answer_question(&question, include_emails)
        .await
        .map_err(|e| e.to_string())
}
```

---

### Indexing Module

#### `start_indexing`
Démarre indexation d'un ou plusieurs dossiers.

**Frontend :**
```typescript
interface IndexingOptions {
  paths: string[];
  mode: 'full' | 'metadata' | 'incremental';
  enable_ocr: boolean;
  enable_embeddings: boolean;
}

await invoke('start_indexing', {
  options: {
    paths: ['C:\\Users\\Admin\\Documents'],
    mode: 'full',
    enable_ocr: true,
    enable_embeddings: true,
  },
});
```

**Backend :**
```rust
#[tauri::command]
async fn start_indexing(
    options: IndexingOptions,
    window: Window,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let indexer = state.indexer.lock().await;

    // Lance indexation async, émet events vers frontend
    tokio::spawn(async move {
        let result = indexer.index_folders(options).await;
        window.emit("indexing_complete", result).ok();
    });

    Ok(())
}
```

---

#### `get_indexing_progress`
Récupère état progression indexation.

**Frontend :**
```typescript
interface IndexingProgress {
  status: 'idle' | 'indexing' | 'paused' | 'error';
  total_files: number;
  indexed_files: number;
  current_file: string;
  speed_files_per_min: number;
  eta_seconds: number;
  errors: ErrorSummary[];
}

const progress = await invoke<IndexingProgress>('get_indexing_progress');
```

**Backend :**
```rust
#[tauri::command]
fn get_indexing_progress(
    state: State<'_, AppState>,
) -> Result<IndexingProgress, String> {
    let progress = state.indexing_progress.lock().unwrap();
    Ok(progress.clone())
}
```

---

#### `pause_indexing` / `resume_indexing`

**Frontend :**
```typescript
await invoke('pause_indexing');
await invoke('resume_indexing');
```

**Backend :**
```rust
#[tauri::command]
fn pause_indexing(state: State<'_, AppState>) -> Result<(), String> {
    state.indexer.lock().await.pause()?;
    Ok(())
}

#[tauri::command]
fn resume_indexing(state: State<'_, AppState>) -> Result<(), String> {
    state.indexer.lock().await.resume()?;
    Ok(())
}
```

---

### Configuration Module

#### `get_config`
Récupère configuration complète.

**Frontend :**
```typescript
interface AppConfig {
  watched_folders: WatchedFolder[];
  global_exclusions: GlobalExclusions;
  ocr_config: OcrConfig;
  email_sources: EmailSource[];
  search_preferences: SearchPreferences;
}

const config = await invoke<AppConfig>('get_config');
```

**Backend :**
```rust
#[tauri::command]
fn get_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let config = state.config.lock().unwrap();
    Ok(config.clone())
}
```

---

#### `update_watched_folders`
Met à jour dossiers surveillés.

**Frontend :**
```typescript
interface WatchedFolder {
  path: string;
  mode: 'full' | 'metadata' | 'excluded';
  exclusions: Exclusion[];
  last_scan?: number;
}

interface Exclusion {
  type: 'folder' | 'file' | 'extension' | 'pattern';
  value: string;
}

await invoke('update_watched_folders', {
  folders: [
    {
      path: 'C:\\Users\\Admin\\Documents',
      mode: 'full',
      exclusions: [
        { type: 'extension', value: 'tmp' },
        { type: 'folder', value: 'Archives\\Old' },
      ],
    },
  ],
});
```

**Backend :**
```rust
#[tauri::command]
async fn update_watched_folders(
    folders: Vec<WatchedFolder>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    config.watched_folders = folders.clone();
    config.save()?;

    // Update watchdog
    let mut watchdog = state.watchdog.lock().await;
    watchdog.reload_config(&folders).await?;

    Ok(())
}
```

---

#### `update_ocr_config`

**Frontend :**
```typescript
interface OcrConfig {
  enabled: boolean;
  languages: string[];          // ['fra', 'eng']
  file_types: string[];         // ['pdf', 'jpg', 'png']
  min_size_kb: number;
  folders: string[];            // Dossiers où OCR actif
}

await invoke('update_ocr_config', {
  config: {
    enabled: true,
    languages: ['fra', 'eng'],
    file_types: ['pdf', 'jpg'],
    min_size_kb: 500,
    folders: ['C:\\Users\\Admin\\Documents'],
  },
});
```

---

### Email Module

#### `index_emails`
Indexe emails depuis source.

**Frontend :**
```typescript
interface EmailSource {
  type: 'outlook_pst' | 'outlook_mapi' | 'thunderbird' | 'imap';
  path?: string;                // Pour PST/Thunderbird
  server?: string;              // Pour IMAP
  username?: string;
  password?: string;            // Stocké chiffré
}

await invoke('index_emails', {
  source: {
    type: 'outlook_pst',
    path: 'C:\\Users\\Admin\\Documents\\Outlook.pst',
  },
});
```

**Backend :**
```rust
#[tauri::command]
async fn index_emails(
    source: EmailSource,
    window: Window,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let email_parser = state.email_parser.lock().await;

    tokio::spawn(async move {
        let result = email_parser.index_source(source).await;
        window.emit("email_indexing_complete", result).ok();
    });

    Ok(())
}
```

---

#### `search_emails`

**Frontend :**
```typescript
interface EmailResult {
  id: string;
  message_id: string;
  subject: string;
  from: string;
  to: string[];
  date: number;
  snippet: string;
  has_attachments: boolean;
  attachments: Attachment[];
  score: number;
}

const emails = await invoke<EmailResult[]>('search_emails', {
  query: 'marie budget',
  filters: {
    date_after: Date.now() - 90 * 24 * 60 * 60 * 1000,
  },
});
```

---

### File Operations

#### `open_file`
Ouvre fichier dans application native.

**Frontend :**
```typescript
await invoke('open_file', {
  path: 'C:\\Users\\Admin\\Documents\\Contrat.pdf',
  page: 3,  // Optionnel : ouvre à la page spécifique
});
```

**Backend :**
```rust
use tauri::api::shell;

#[tauri::command]
fn open_file(path: String, page: Option<usize>) -> Result<(), String> {
    if let Some(p) = page {
        // TODO: Support ouverture à page spécifique (PDF)
        // Dépend de lecteur PDF (Adobe, Edge, etc.)
    }

    shell::open(&path).map_err(|e| e.to_string())
}
```

---

#### `open_file_location`
Ouvre dossier contenant le fichier.

**Frontend :**
```typescript
await invoke('open_file_location', {
  path: 'C:\\Users\\Admin\\Documents\\Contrat.pdf',
});
```

**Backend :**
```rust
#[tauri::command]
fn open_file_location(path: String) -> Result<(), String> {
    let dir = Path::new(&path).parent()
        .ok_or("Invalid path")?;

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
```

---

#### `get_file_preview`
Récupère preview fichier (texte, image, etc.).

**Frontend :**
```typescript
interface FilePreview {
  type: 'text' | 'image' | 'pdf' | 'unsupported';
  content: string;              // Base64 pour images, texte brut pour TXT
  page_count?: number;          // Pour PDF
}

const preview = await invoke<FilePreview>('get_file_preview', {
  path: 'C:\\Users\\Admin\\Documents\\Note.txt',
  max_size_kb: 100,
});
```

---

### Statistics & Monitoring

#### `get_statistics`

**Frontend :**
```typescript
interface Statistics {
  total_files_indexed: number;
  total_emails_indexed: number;
  index_size_mb: number;
  last_indexing: number;
  search_count_today: number;
  avg_search_time_ms: number;
  top_searches: Array<{ query: string; count: number }>;
}

const stats = await invoke<Statistics>('get_statistics');
```

---

## Types TypeScript Frontend

### Complet `src/types/index.ts`

```typescript
// ==================== Search Types ====================

export interface SearchResult {
  id: string;
  path: string;
  filename: string;
  extension: string;
  size: number;
  modified: number;
  created: number;
  snippet?: string;
  score: number;
  type: ResultType;
  highlight_ranges?: [number, number][];
}

export type ResultType =
  | { type: 'file' }
  | { type: 'email'; message_id: string }
  | { type: 'email_attachment'; email_id: string };

export interface SearchFilters {
  extensions?: string[];
  date_after?: number;
  date_before?: number;
  size_min?: number;
  size_max?: number;
  folder?: string;
  include_emails?: boolean;
}

export enum SearchMode {
  Fast = 'fast',
  Content = 'content',
  Semantic = 'semantic',
}

// ==================== Assist Me Types ====================

export interface AssistMeResponse {
  answer: string;
  sources: Source[];
  confidence: number;
  processing_time_ms: number;
}

export interface Source {
  id: string;
  path: string;
  filename: string;
  type: 'file' | 'email';
  snippet: string;
  page?: number;
  relevance: number;
  highlight?: string;
}

// ==================== Configuration Types ====================

export interface AppConfig {
  watched_folders: WatchedFolder[];
  global_exclusions: GlobalExclusions;
  ocr_config: OcrConfig;
  email_sources: EmailSource[];
  search_preferences: SearchPreferences;
}

export interface WatchedFolder {
  path: string;
  mode: WatchMode;
  exclusions: Exclusion[];
  last_scan?: number;
}

export type WatchMode = 'full' | 'metadata' | 'excluded';

export interface Exclusion {
  type: 'folder' | 'file' | 'extension' | 'pattern';
  value: string;
}

export interface GlobalExclusions {
  extensions: string[];
  patterns: string[];
  max_size_mb?: number;
}

export interface OcrConfig {
  enabled: boolean;
  languages: string[];
  file_types: string[];
  min_size_kb: number;
  folders: string[];
}

export interface EmailSource {
  id: string;
  type: 'outlook_pst' | 'outlook_mapi' | 'thunderbird' | 'imap';
  path?: string;
  server?: string;
  username?: string;
  // password NOT exposed to frontend (backend only)
}

export interface SearchPreferences {
  fuzzy_matching: boolean;
  max_results: number;
  snippet_length: number;
  enable_suggestions: boolean;
}

// ==================== Indexing Types ====================

export interface IndexingProgress {
  status: IndexingStatus;
  total_files: number;
  indexed_files: number;
  current_file: string;
  speed_files_per_min: number;
  eta_seconds: number;
  errors: ErrorSummary[];
  phase: IndexingPhase;
}

export type IndexingStatus = 'idle' | 'indexing' | 'paused' | 'error';

export type IndexingPhase =
  | 'scanning'
  | 'extracting_metadata'
  | 'extracting_content'
  | 'running_ocr'
  | 'generating_embeddings'
  | 'finalizing';

export interface ErrorSummary {
  file_path: string;
  error_type: string;
  message: string;
  timestamp: number;
}

export interface IndexingOptions {
  paths: string[];
  mode: 'full' | 'metadata' | 'incremental';
  enable_ocr: boolean;
  enable_embeddings: boolean;
}

// ==================== Email Types ====================

export interface EmailResult {
  id: string;
  message_id: string;
  subject: string;
  from: string;
  to: string[];
  cc?: string[];
  date: number;
  snippet: string;
  has_attachments: boolean;
  attachments: Attachment[];
  folder: string;
  score: number;
}

export interface Attachment {
  id: string;
  filename: string;
  size: number;
  mime_type: string;
  extracted_path?: string;
}

// ==================== Statistics Types ====================

export interface Statistics {
  total_files_indexed: number;
  total_emails_indexed: number;
  index_size_mb: number;
  vector_index_size_mb: number;
  last_indexing: number;
  search_count_today: number;
  search_count_total: number;
  avg_search_time_ms: number;
  top_searches: TopSearch[];
}

export interface TopSearch {
  query: string;
  count: number;
  last_used: number;
}

// ==================== File Preview Types ====================

export interface FilePreview {
  type: 'text' | 'image' | 'pdf' | 'unsupported';
  content: string;
  page_count?: number;
  size: number;
}
```

---

## Structures Rust Backend

### Complet `src/types.rs`

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::path::PathBuf;

// ==================== Search Types ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub path: String,
    pub filename: String,
    pub extension: String,
    pub size: u64,
    pub modified: i64,
    pub created: i64,
    pub snippet: Option<String>,
    pub score: f32,
    #[serde(rename = "type")]
    pub result_type: ResultType,
    pub highlight_ranges: Option<Vec<(usize, usize)>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResultType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "email")]
    Email { message_id: String },
    #[serde(rename = "email_attachment")]
    EmailAttachment { email_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub extensions: Option<Vec<String>>,
    pub date_after: Option<i64>,
    pub date_before: Option<i64>,
    pub size_min: Option<u64>,
    pub size_max: Option<u64>,
    pub folder: Option<String>,
    pub include_emails: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SearchMode {
    Fast,
    Content,
    Semantic,
}

// ==================== Assist Me Types ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistMeResponse {
    pub answer: String,
    pub sources: Vec<Source>,
    pub confidence: f32,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub path: String,
    pub filename: String,
    #[serde(rename = "type")]
    pub source_type: String,
    pub snippet: String,
    pub page: Option<usize>,
    pub relevance: f32,
    pub highlight: Option<String>,
}

// ==================== Configuration Types ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub watched_folders: Vec<WatchedFolder>,
    pub global_exclusions: GlobalExclusions,
    pub ocr_config: OcrConfig,
    pub email_sources: Vec<EmailSource>,
    pub search_preferences: SearchPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchedFolder {
    pub path: PathBuf,
    pub mode: WatchMode,
    pub exclusions: Vec<Exclusion>,
    pub last_scan: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WatchMode {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "metadata")]
    MetadataOnly,
    #[serde(rename = "excluded")]
    Excluded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exclusion {
    #[serde(rename = "type")]
    pub exclusion_type: ExclusionType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExclusionType {
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "extension")]
    Extension,
    #[serde(rename = "pattern")]
    Pattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalExclusions {
    pub extensions: Vec<String>,
    pub patterns: Vec<String>,
    pub max_size_mb: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrConfig {
    pub enabled: bool,
    pub languages: Vec<String>,
    pub file_types: Vec<String>,
    pub min_size_kb: u64,
    pub folders: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSource {
    pub id: String,
    #[serde(rename = "type")]
    pub source_type: EmailSourceType,
    pub path: Option<PathBuf>,
    pub server: Option<String>,
    pub username: Option<String>,
    #[serde(skip_serializing)] // Pas exposé au frontend
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailSourceType {
    #[serde(rename = "outlook_pst")]
    OutlookPst,
    #[serde(rename = "outlook_mapi")]
    OutlookMapi,
    #[serde(rename = "thunderbird")]
    Thunderbird,
    #[serde(rename = "imap")]
    Imap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPreferences {
    pub fuzzy_matching: bool,
    pub max_results: usize,
    pub snippet_length: usize,
    pub enable_suggestions: bool,
}

// ==================== Indexing Types ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingProgress {
    pub status: IndexingStatus,
    pub total_files: u64,
    pub indexed_files: u64,
    pub current_file: String,
    pub speed_files_per_min: f64,
    pub eta_seconds: u64,
    pub errors: Vec<ErrorSummary>,
    pub phase: IndexingPhase,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum IndexingStatus {
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "indexing")]
    Indexing,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IndexingPhase {
    #[serde(rename = "scanning")]
    Scanning,
    #[serde(rename = "extracting_metadata")]
    ExtractingMetadata,
    #[serde(rename = "extracting_content")]
    ExtractingContent,
    #[serde(rename = "running_ocr")]
    RunningOcr,
    #[serde(rename = "generating_embeddings")]
    GeneratingEmbeddings,
    #[serde(rename = "finalizing")]
    Finalizing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorSummary {
    pub file_path: String,
    pub error_type: String,
    pub message: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingOptions {
    pub paths: Vec<PathBuf>,
    pub mode: IndexingMode,
    pub enable_ocr: bool,
    pub enable_embeddings: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IndexingMode {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "incremental")]
    Incremental,
}

// ==================== Database Models ====================

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub id: String,
    pub path: PathBuf,
    pub filename: String,
    pub extension: String,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub created: DateTime<Utc>,
    pub hash: String,
    pub indexed_at: DateTime<Utc>,
    pub has_embedding: bool,
}

#[derive(Debug, Clone)]
pub struct Email {
    pub id: String,
    pub message_id: String,
    pub subject: String,
    pub from: String,
    pub to: Vec<String>,
    pub cc: Option<Vec<String>>,
    pub date: DateTime<Utc>,
    pub body_text: String,
    pub body_html: Option<String>,
    pub folder: String,
    pub source: String,
    pub has_attachments: bool,
    pub indexed_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Attachment {
    pub id: String,
    pub email_id: String,
    pub filename: String,
    pub size: u64,
    pub mime_type: String,
    pub extracted_path: Option<PathBuf>,
    pub file_id: Option<String>,
}
```

---

## Schéma Base de Données

### SQLite Schema `schema.sql`

```sql
-- ==================== Files Table ====================

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

CREATE INDEX idx_files_path ON files(path);
CREATE INDEX idx_files_modified ON files(modified);
CREATE INDEX idx_files_extension ON files(extension);
CREATE INDEX idx_files_filename ON files(filename);
CREATE INDEX idx_files_has_embedding ON files(has_embedding);

-- ==================== Embeddings Table ====================

CREATE TABLE IF NOT EXISTS embeddings (
    file_id TEXT PRIMARY KEY,
    chunk_count INTEGER NOT NULL,
    model TEXT NOT NULL,
    dimension INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);

-- Chunks embeddings (stockés dans LEANN vector DB, metadata ici)
CREATE TABLE IF NOT EXISTS embedding_chunks (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    chunk_idx INTEGER NOT NULL,
    text_preview TEXT,
    token_count INTEGER,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);

CREATE INDEX idx_chunks_file_id ON embedding_chunks(file_id);

-- ==================== Content Table ====================

CREATE TABLE IF NOT EXISTS content (
    file_id TEXT PRIMARY KEY,
    text TEXT NOT NULL,
    ocr_used BOOLEAN DEFAULT 0,
    extraction_method TEXT,
    page_count INTEGER,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);

-- Full-text search index
CREATE VIRTUAL TABLE IF NOT EXISTS content_fts USING fts5(
    file_id UNINDEXED,
    text,
    content='content',
    tokenize='porter unicode61'
);

-- Triggers pour sync FTS
CREATE TRIGGER IF NOT EXISTS content_ai AFTER INSERT ON content BEGIN
  INSERT INTO content_fts(rowid, file_id, text) VALUES (new.rowid, new.file_id, new.text);
END;

CREATE TRIGGER IF NOT EXISTS content_ad AFTER DELETE ON content BEGIN
  DELETE FROM content_fts WHERE rowid = old.rowid;
END;

CREATE TRIGGER IF NOT EXISTS content_au AFTER UPDATE ON content BEGIN
  UPDATE content_fts SET text = new.text WHERE rowid = new.rowid;
END;

-- ==================== Emails Table ====================

CREATE TABLE IF NOT EXISTS emails (
    id TEXT PRIMARY KEY,
    message_id TEXT UNIQUE,
    subject TEXT,
    from_addr TEXT,
    to_addrs TEXT,
    cc_addrs TEXT,
    date INTEGER,
    body_text TEXT,
    body_html TEXT,
    folder TEXT,
    source TEXT,
    has_attachments BOOLEAN DEFAULT 0,
    indexed_at INTEGER
);

CREATE INDEX idx_emails_from ON emails(from_addr);
CREATE INDEX idx_emails_date ON emails(date);
CREATE INDEX idx_emails_subject ON emails(subject);
CREATE INDEX idx_emails_folder ON emails(folder);

-- Email FTS
CREATE VIRTUAL TABLE IF NOT EXISTS emails_fts USING fts5(
    email_id UNINDEXED,
    subject,
    body_text,
    from_addr,
    to_addrs
);

-- ==================== Attachments Table ====================

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

CREATE INDEX idx_attachments_email_id ON email_attachments(email_id);
CREATE INDEX idx_attachments_file_id ON email_attachments(file_id);

-- ==================== Configuration Tables ====================

CREATE TABLE IF NOT EXISTS watched_folders (
    path TEXT PRIMARY KEY,
    mode TEXT NOT NULL,
    last_scan INTEGER,
    exclusions TEXT
);

CREATE TABLE IF NOT EXISTS config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

-- ==================== Statistics Tables ====================

CREATE TABLE IF NOT EXISTS search_history (
    id TEXT PRIMARY KEY,
    query TEXT NOT NULL,
    mode TEXT NOT NULL,
    results_count INTEGER,
    execution_time_ms INTEGER,
    timestamp INTEGER NOT NULL
);

CREATE INDEX idx_search_history_timestamp ON search_history(timestamp);
CREATE INDEX idx_search_history_query ON search_history(query);

CREATE TABLE IF NOT EXISTS error_log (
    id TEXT PRIMARY KEY,
    file_path TEXT,
    error_type TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp INTEGER NOT NULL
);

CREATE INDEX idx_error_log_timestamp ON error_log(timestamp);
```

---

## Events (Frontend ← Backend)

Tauri events pour communication asynchrone backend → frontend.

### `indexing_progress`
Émis régulièrement pendant indexation.

```typescript
// Frontend listener
import { listen } from '@tauri-apps/api/event';

listen<IndexingProgress>('indexing_progress', (event) => {
  console.log(`Progress: ${event.payload.indexed_files}/${event.payload.total_files}`);
  updateProgressBar(event.payload);
});
```

```rust
// Backend emitter
window.emit("indexing_progress", IndexingProgress {
    status: IndexingStatus::Indexing,
    indexed_files: 1500,
    total_files: 10000,
    ...
})?;
```

---

### `indexing_complete`

```typescript
interface IndexingComplete {
  success: boolean;
  total_files: number;
  total_time_seconds: number;
  errors: ErrorSummary[];
}

listen<IndexingComplete>('indexing_complete', (event) => {
  if (event.payload.success) {
    toast.success('Indexation terminée !');
  }
});
```

---

### `file_changed`
Watchdog détecte changement fichier.

```typescript
interface FileChangedEvent {
  path: string;
  event_type: 'created' | 'modified' | 'deleted' | 'renamed';
  old_path?: string;
}

listen<FileChangedEvent>('file_changed', (event) => {
  // Rafraîchit résultats si impacté
  if (currentSearchResults.some(r => r.path === event.payload.path)) {
    refreshSearch();
  }
});
```

---

## Configuration Files

### `config.json` (AppData)

```json
{
  "version": "1.0.0",
  "watched_folders": [
    {
      "path": "C:\\Users\\Admin\\Documents",
      "mode": "full",
      "exclusions": [
        {
          "type": "extension",
          "value": "tmp"
        },
        {
          "type": "folder",
          "value": "Archives\\Old"
        }
      ],
      "last_scan": 1699999999
    }
  ],
  "global_exclusions": {
    "extensions": [".tmp", ".log", ".cache"],
    "patterns": ["node_modules", ".git", "*_backup.*"],
    "max_size_mb": 500
  },
  "ocr_config": {
    "enabled": true,
    "languages": ["fra", "eng"],
    "file_types": ["pdf", "jpg", "png"],
    "min_size_kb": 500,
    "folders": ["C:\\Users\\Admin\\Documents"]
  },
  "email_sources": [
    {
      "id": "outlook-main",
      "type": "outlook_mapi",
      "path": null,
      "server": null,
      "username": null
    }
  ],
  "search_preferences": {
    "fuzzy_matching": true,
    "max_results": 100,
    "snippet_length": 200,
    "enable_suggestions": true
  },
  "telemetry": {
    "enabled": false,
    "anonymous": true
  }
}
```

---

### `tauri.conf.json` (App configuration)

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "package": {
    "productName": "xfinder",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "open": true
      },
      "dialog": {
        "open": true,
        "save": true
      },
      "fs": {
        "scope": ["$APPDATA/*", "$HOME/*"]
      }
    },
    "bundle": {
      "active": true,
      "identifier": "com.xfinder.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/icon.ico"
      ],
      "windows": {
        "wix": {
          "language": ["fr-FR", "en-US"]
        }
      }
    },
    "security": {
      "csp": "default-src 'self'; style-src 'self' 'unsafe-inline'"
    },
    "updater": {
      "active": true,
      "endpoints": ["https://updates.xfinder.app/{{target}}/{{current_version}}"],
      "dialog": true,
      "pubkey": "YOUR_PUBLIC_KEY_HERE"
    },
    "windows": [
      {
        "title": "xfinder",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true,
        "fullscreen": false
      }
    ]
  }
}
```

---

## Exemples d'utilisation

### Recherche complète avec filtres

```typescript
// Frontend component
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

function Search() {
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);

  const handleSearch = async (query: string) => {
    setLoading(true);
    try {
      const results = await invoke<SearchResult[]>('search_files', {
        query,
        filters: {
          extensions: ['.pdf', '.docx'],
          date_after: Date.now() - 30 * 24 * 60 * 60 * 1000,
        },
        limit: 100,
      });
      setResults(results);
    } catch (error) {
      console.error('Search failed:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <input onChange={(e) => handleSearch(e.target.value)} />
      {loading && <p>Recherche...</p>}
      {results.map(result => (
        <div key={result.id} onClick={() => openFile(result.path)}>
          <h3>{result.filename}</h3>
          <p>{result.path}</p>
        </div>
      ))}
    </div>
  );
}
```

---

**Document version :** 1.0
**Dernière mise à jour :** 2025-11-12
