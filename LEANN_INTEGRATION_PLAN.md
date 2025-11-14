# Plan d'Intégration LEANN - Mode "Assist Me"
**xfinder - Phase 3**

---

## 📋 Table des matières

1. [Vue d'ensemble](#vue-densemble)
2. [Architecture technique](#architecture-technique)
3. [Plan d'implémentation détaillé](#plan-dimplémentation-détaillé)
4. [Flux utilisateur](#flux-utilisateur)
5. [Détails techniques](#détails-techniques)
6. [Timeline et estimation](#timeline-et-estimation)

---

## 🎯 Vue d'ensemble

### Objectif

Implémenter le mode "Assist Me" permettant à l'utilisateur de poser des questions en langage naturel et recevoir des réponses avec sources vérifiables, alimentées par LEANN (recherche vectorielle à faible stockage).

### Requirements clés (d'après PRD F6)

**Fonctionnalités obligatoires :**
- ✅ Questions en langage naturel
- ✅ Réponses avec sources cliquables
- ✅ Recherche sémantique (pas seulement mots-clés)
- ✅ 100% offline (pas de cloud)
- ✅ Sources vérifiables (fichier + page + extrait)
- ✅ Historique des questions

**Critères de performance :**
- ⚡ <3s pour réponse (mode sans LLM)
- ⚡ <10s pour réponse (mode avec LLM optionnel)
- 🎯 80%+ de pertinence sur top-5 sources
- 💾 Réduction storage 97% vs index vectoriel classique (grâce à LEANN)

**Exemples d'usage :**
```
Q: "Quels sont les budgets formation validés en 2024 ?"
→ Retourne 3 budgets avec montants, sources PDF + emails

Q: "Retrouve les échanges avec Marie sur le projet RGPD"
→ Retourne 7 conversations avec dates, pièces jointes
```

---

## 🏗️ Architecture technique

### Stack complète

```
┌─────────────────────────────────────────────────────────┐
│                    FRONTEND (egui)                      │
│  - ChatInterface (questions/réponses)                   │
│  - SourceCard (sources cliquables)                      │
│  - HistoryPanel (historique)                            │
└────────────┬────────────────────────────────────────────┘
             │
             │ IPC (in-process calls)
             ↓
┌─────────────────────────────────────────────────────────┐
│              BACKEND RUST (xfinder)                     │
│                                                          │
│  ┌──────────────────────────────────────────────────┐  │
│  │ AssistMeEngine                                   │  │
│  │  - answer_question(query) → Vec<Source>         │  │
│  │  - get_history() → Vec<QAPair>                  │  │
│  └──────────┬───────────────────────────────────────┘  │
│             │                                            │
│     ┌───────┴────────┬─────────────┬──────────────┐    │
│     ↓                ↓             ↓              ↓    │
│  ┌──────┐      ┌──────────┐   ┌────────┐   ┌────────┐ │
│  │Embed │      │  LEANN   │   │Content │   │Database│ │
│  │Model │      │  Index   │   │Extract │   │(SQLite)│ │
│  │(MiniLM)│    │ (PyO3)   │   │        │   │        │ │
│  └──────┘      └──────────┘   └────────┘   └────────┘ │
│     │               │              │            │       │
└─────┼───────────────┼──────────────┼────────────┼───────┘
      │               │              │            │
      ↓               ↓              ↓            ↓
┌──────────┐   ┌──────────┐   ┌─────────┐  ┌──────────┐
│ Candle   │   │ Python   │   │  Files  │  │ xfinder  │
│(Rust ML) │   │  LEANN   │   │ (.txt,  │  │   .db    │
│          │   │  Lib     │   │  .pdf)  │  │          │
└──────────┘   └──────────┘   └─────────┘  └──────────┘
```

### Composants principaux

1. **EmbeddingModel** (Candle + all-MiniLM-L6-v2)
   - Convertit texte → vecteur 384 dimensions
   - Utilisé pour : questions utilisateur + chunks de documents

2. **LeannIndex** (PyO3 wrapper)
   - Interface Rust → Python LEANN
   - Stocke graphe HNSW (pas les embeddings complets)
   - Recherche top-k similarité

3. **ContentExtractor**
   - Extrait texte de fichiers (.txt, .pdf, .docx, .md)
   - Chunking intelligent (500 tokens avec overlap)

4. **AssistMeEngine**
   - Orchestrateur principal
   - Gère workflow complet : question → recherche → sources

5. **Database (SQLite)**
   - Stocke mapping: file_id → path, metadata
   - Stocke chunks: chunk_id → file_id, text, page_num
   - Historique questions/réponses

---

## 📅 Plan d'implémentation détaillé

### Phase A : Setup & Infrastructure (3 jours)

#### **Jour 1 : Setup PyO3 + Installation LEANN**

**Tâches :**
1. Ajouter dépendances Cargo.toml
2. Installer LEANN Python
3. Créer module `src/semantic/`
4. Test PyO3 basique

**Code à créer :**

```toml
# Cargo.toml
[dependencies]
pyo3 = { version = "0.20", features = ["auto-initialize"] }
anyhow = "1.0"

# Pour embeddings
candle-core = "0.4"
candle-nn = "0.4"
candle-transformers = "0.4"
tokenizers = "0.15"
```

```bash
# Installation LEANN
pip install leann

# Vérification
python -c "import leann; print(leann.__version__)"
```

**Fichiers à créer :**
- `src/semantic/mod.rs`
- `src/semantic/leann_wrapper.rs` (stub)

**Test de validation :**
```rust
#[test]
fn test_pyo3_leann_import() {
    use pyo3::prelude::*;

    Python::with_gil(|py| {
        let leann = py.import("leann").unwrap();
        assert!(leann.hasattr("LeannBuilder").unwrap());
    });
}
```

---

#### **Jour 2 : Wrapper LEANN en Rust**

**Tâches :**
1. Implémenter `LeannIndex` struct
2. Wrapper méthodes Python: create, add, search, save, load
3. Tests unitaires

**Code complet :**

```rust
// src/semantic/leann_wrapper.rs

use pyo3::prelude::*;
use pyo3::types::PyList;
use anyhow::{Result, Context};

pub struct LeannIndex {
    py_builder: Option<PyObject>,
    py_searcher: Option<PyObject>,
    index_path: String,
    dim: usize,
}

impl LeannIndex {
    /// Crée un nouvel index LEANN
    pub fn new(index_path: &str, dim: usize) -> Result<Self> {
        Ok(Self {
            py_builder: None,
            py_searcher: None,
            index_path: index_path.to_string(),
            dim,
        })
    }

    /// Initialise le builder pour indexation
    pub fn init_builder(&mut self, model_name: &str) -> Result<()> {
        Python::with_gil(|py| {
            let leann = py.import("leann")
                .context("Failed to import leann")?;

            // Créer LeannBuilder
            let builder = leann
                .getattr("LeannBuilder")?
                .call1((
                    &self.index_path,
                    self.dim,
                    model_name,  // "sentence-transformers/all-MiniLM-L6-v2"
                ))?;

            self.py_builder = Some(builder.into());
            Ok(())
        })
    }

    /// Ajoute un document à l'index
    pub fn add_document(&mut self, doc_id: i64, text: &str) -> Result<()> {
        Python::with_gil(|py| {
            let builder = self.py_builder.as_ref()
                .context("Builder not initialized")?
                .as_ref(py);

            builder.call_method1("add_text", (doc_id, text))?;
            Ok(())
        })
    }

    /// Finalise et construit l'index
    pub fn build(&mut self) -> Result<()> {
        Python::with_gil(|py| {
            let builder = self.py_builder.as_ref()
                .context("Builder not initialized")?
                .as_ref(py);

            builder.call_method0("build")?;
            self.py_builder = None; // Builder consommé
            Ok(())
        })
    }

    /// Charge un index existant
    pub fn load(&mut self, model_name: &str) -> Result<()> {
        Python::with_gil(|py| {
            let leann = py.import("leann")?;

            // Créer LeannSearcher
            let searcher = leann
                .getattr("LeannSearcher")?
                .call1((
                    &self.index_path,
                    model_name,
                ))?;

            self.py_searcher = Some(searcher.into());
            Ok(())
        })
    }

    /// Recherche sémantique
    pub fn search(&self, query: &str, k: usize) -> Result<Vec<(i64, f32)>> {
        Python::with_gil(|py| {
            let searcher = self.py_searcher.as_ref()
                .context("Searcher not initialized")?
                .as_ref(py);

            // Appeler search(query, k)
            let results = searcher.call_method1("search", (query, k))?;

            // Convertir résultats Python → Rust
            // Format attendu: List[Tuple[int, float]]
            let py_list: &PyList = results.downcast()?;

            let mut rust_results = Vec::new();
            for item in py_list.iter() {
                let tuple: &pyo3::types::PyTuple = item.downcast()?;
                let doc_id: i64 = tuple.get_item(0)?.extract()?;
                let score: f32 = tuple.get_item(1)?.extract()?;
                rust_results.push((doc_id, score));
            }

            Ok(rust_results)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_leann_basic_workflow() {
        let temp_dir = TempDir::new().unwrap();
        let index_path = temp_dir.path().join("test_index");

        // 1. Créer et construire index
        let mut index = LeannIndex::new(
            index_path.to_str().unwrap(),
            384
        ).unwrap();

        index.init_builder("sentence-transformers/all-MiniLM-L6-v2").unwrap();

        index.add_document(1, "Budget formation 2024: 45000 euros").unwrap();
        index.add_document(2, "Contrat Dupont signé le 15 mars").unwrap();
        index.add_document(3, "Rapport RGPD article 30 conformité").unwrap();

        index.build().unwrap();

        // 2. Charger et rechercher
        let mut searcher = LeannIndex::new(
            index_path.to_str().unwrap(),
            384
        ).unwrap();

        searcher.load("sentence-transformers/all-MiniLM-L6-v2").unwrap();

        let results = searcher.search("budget formation", 2).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, 1); // Doc 1 devrait être le plus pertinent
    }
}
```

**Test de validation :**
```bash
cargo test test_leann_basic_workflow -- --nocapture
```

---

#### **Jour 3 : Module Content Extraction**

**Tâches :**
1. Créer `src/content/mod.rs`
2. Implémenter extraction TXT, MD, PDF
3. Implémenter chunking intelligent
4. Tests

**Code :**

```rust
// src/content/mod.rs

use anyhow::{Result, Context};
use std::path::Path;

pub struct ContentExtractor;

impl ContentExtractor {
    pub fn extract_text(path: &Path) -> Result<String> {
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        match ext.to_lowercase().as_str() {
            "txt" | "md" | "log" | "rs" | "toml" | "json" => {
                // Lecture directe
                std::fs::read_to_string(path)
                    .context("Failed to read text file")
            }
            "pdf" => {
                // TODO: Utiliser pdf-extract
                Self::extract_pdf(path)
            }
            "docx" => {
                // TODO: Utiliser docx-rs
                Self::extract_docx(path)
            }
            _ => {
                // Fallback: essayer lecture texte
                std::fs::read_to_string(path)
                    .or_else(|_| Ok(String::new()))
            }
        }
    }

    fn extract_pdf(path: &Path) -> Result<String> {
        // Pour l'instant stub
        // TODO Phase suivante: pdf-extract
        Ok(String::from("[PDF extraction not implemented yet]"))
    }

    fn extract_docx(path: &Path) -> Result<String> {
        // Pour l'instant stub
        // TODO Phase suivante: docx-rs
        Ok(String::from("[DOCX extraction not implemented yet]"))
    }

    /// Split text into chunks of ~500 tokens with overlap
    pub fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
        // Approximation simple: 1 token ~= 4 caractères
        let chunk_chars = chunk_size * 4;
        let overlap_chars = overlap * 4;

        let mut chunks = Vec::new();
        let text_len = text.len();

        let mut start = 0;
        while start < text_len {
            let end = (start + chunk_chars).min(text_len);
            let chunk = &text[start..end];
            chunks.push(chunk.to_string());

            if end >= text_len {
                break;
            }

            start = end - overlap_chars;
        }

        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_extract_txt() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        fs::write(&file_path, "Hello world").unwrap();

        let content = ContentExtractor::extract_text(&file_path).unwrap();
        assert_eq!(content, "Hello world");
    }

    #[test]
    fn test_chunking() {
        let text = "a".repeat(1000);
        let chunks = ContentExtractor::chunk_text(&text, 100, 20);

        // chunk_size=100 tokens = 400 chars
        // overlap=20 tokens = 80 chars
        assert!(chunks.len() > 1);
        assert!(chunks[0].len() <= 400);
    }
}
```

---

### Phase B : Embeddings & Indexation (4 jours)

#### **Jour 4-5 : Intégration Candle + all-MiniLM-L6-v2**

**Tâches :**
1. Setup Candle
2. Télécharger modèle all-MiniLM-L6-v2
3. Implémenter `EmbeddingModel`
4. Tests qualité embeddings

**Code :**

```rust
// src/semantic/embeddings.rs

use candle_core::{Device, Tensor, Result};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use tokenizers::Tokenizer;
use std::path::Path;

pub struct EmbeddingModel {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl EmbeddingModel {
    /// Charge le modèle all-MiniLM-L6-v2
    pub fn load(model_path: &Path) -> Result<Self> {
        let device = Device::Cpu; // TODO: GPU si disponible

        // Charger config
        let config_path = model_path.join("config.json");
        let config = Config::from_file(config_path)?;

        // Charger poids
        let weights_path = model_path.join("model.safetensors");
        let vb = VarBuilder::from_safetensors(&[weights_path], Default::default(), &device)?;

        // Construire modèle
        let model = BertModel::load(vb, &config)?;

        // Charger tokenizer
        let tokenizer_path = model_path.join("tokenizer.json");
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| candle_core::Error::Msg(e.to_string()))?;

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }

    /// Encode texte → vecteur 384 dimensions
    pub fn encode(&self, text: &str) -> Result<Vec<f32>> {
        // 1. Tokenize
        let encoding = self.tokenizer
            .encode(text, true)
            .map_err(|e| candle_core::Error::Msg(e.to_string()))?;

        let input_ids = Tensor::new(encoding.get_ids(), &self.device)?
            .unsqueeze(0)?;

        let attention_mask = Tensor::new(encoding.get_attention_mask(), &self.device)?
            .unsqueeze(0)?;

        // 2. Forward pass
        let embeddings = self.model.forward(&input_ids, &attention_mask)?;

        // 3. Mean pooling
        let pooled = Self::mean_pooling(&embeddings, &attention_mask)?;

        // 4. L2 normalize
        let normalized = Self::normalize(&pooled)?;

        // 5. Convert to Vec<f32>
        normalized.to_vec1()
    }

    fn mean_pooling(embeddings: &Tensor, attention_mask: &Tensor) -> Result<Tensor> {
        // Moyenne des tokens (en ignorant padding)
        let masked = (embeddings * attention_mask.unsqueeze(2)?)?;
        let summed = masked.sum(1)?;
        let mask_sum = attention_mask.sum(1)?.unsqueeze(1)?;
        summed.div(&mask_sum)
    }

    fn normalize(tensor: &Tensor) -> Result<Tensor> {
        let norm = tensor.sqr()?.sum_all()?.sqrt()?;
        tensor.div(&norm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requiert modèle téléchargé
    fn test_embedding_generation() {
        let model_path = Path::new("models/all-MiniLM-L6-v2");
        let model = EmbeddingModel::load(model_path).unwrap();

        let embedding = model.encode("Hello world").unwrap();

        assert_eq!(embedding.len(), 384);

        // Vérifier normalisation L2
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.01);
    }
}
```

**Setup modèle :**
```bash
# Télécharger all-MiniLM-L6-v2
mkdir -p models
cd models
git clone https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2
```

---

#### **Jour 6 : Indexation avec LEANN**

**Tâches :**
1. Intégrer ContentExtractor + EmbeddingModel + LeannIndex
2. Créer pipeline d'indexation
3. Stocker mapping chunks → files dans DB

**Code :**

```rust
// src/semantic/indexer.rs

use super::{LeannIndex, ContentExtractor};
use crate::database::Database;
use anyhow::Result;
use std::path::Path;
use std::sync::Arc;

pub struct SemanticIndexer {
    leann: LeannIndex,
    db: Arc<Database>,
}

impl SemanticIndexer {
    pub fn new(index_path: &str, db: Arc<Database>) -> Result<Self> {
        let mut leann = LeannIndex::new(index_path, 384)?;
        leann.init_builder("sentence-transformers/all-MiniLM-L6-v2")?;

        Ok(Self { leann, db })
    }

    /// Index un fichier : extraction → chunking → embedding → LEANN
    pub fn index_file(&mut self, file_id: i64, file_path: &Path) -> Result<()> {
        // 1. Extraire contenu
        let content = ContentExtractor::extract_text(file_path)?;

        if content.is_empty() {
            return Ok(()); // Skip fichiers vides
        }

        // 2. Chunking
        let chunks = ContentExtractor::chunk_text(&content, 500, 50);

        // 3. Stocker chunks dans DB + indexer dans LEANN
        for (chunk_idx, chunk_text) in chunks.iter().enumerate() {
            // Générer chunk_id unique
            let chunk_id = format!("{}_{}", file_id, chunk_idx);

            // Stocker chunk dans DB
            self.db.insert_chunk(
                &chunk_id,
                file_id,
                chunk_text,
                chunk_idx as i32,
            )?;

            // Indexer dans LEANN
            // Note: LEANN calculera l'embedding automatiquement
            self.leann.add_document(
                file_id * 1000 + chunk_idx as i64, // ID unique pour chunk
                chunk_text
            )?;
        }

        Ok(())
    }

    /// Finalise l'indexation
    pub fn finalize(&mut self) -> Result<()> {
        self.leann.build()
    }
}
```

**Schema DB pour chunks :**
```sql
-- À ajouter dans src/database/schema.rs

CREATE TABLE IF NOT EXISTS chunks (
    chunk_id TEXT PRIMARY KEY,
    file_id INTEGER NOT NULL,
    chunk_text TEXT NOT NULL,
    chunk_index INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);

CREATE INDEX idx_chunks_file_id ON chunks(file_id);
```

---

#### **Jour 7 : AssistMe Engine**

**Tâches :**
1. Créer `AssistMeEngine`
2. Implémenter `answer_question()`
3. Tests end-to-end

**Code :**

```rust
// src/semantic/assist_me.rs

use super::LeannIndex;
use crate::database::Database;
use anyhow::Result;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Source {
    pub file_path: String,
    pub filename: String,
    pub snippet: String,
    pub chunk_index: i32,
    pub relevance_score: f32,
}

pub struct AssistMeEngine {
    leann: LeannIndex,
    db: Arc<Database>,
}

impl AssistMeEngine {
    pub fn new(index_path: &str, db: Arc<Database>) -> Result<Self> {
        let mut leann = LeannIndex::new(index_path, 384)?;
        leann.load("sentence-transformers/all-MiniLM-L6-v2")?;

        Ok(Self { leann, db })
    }

    /// Répond à une question
    pub fn answer_question(&self, query: &str, top_k: usize) -> Result<Vec<Source>> {
        // 1. Recherche sémantique via LEANN
        let results = self.leann.search(query, top_k)?;

        // 2. Récupérer les sources depuis DB
        let mut sources = Vec::new();

        for (chunk_internal_id, score) in results {
            // Décoder: file_id * 1000 + chunk_idx
            let file_id = chunk_internal_id / 1000;
            let chunk_idx = (chunk_internal_id % 1000) as i32;

            // Récupérer chunk depuis DB
            if let Some(chunk) = self.db.get_chunk_by_file_and_index(file_id, chunk_idx)? {
                // Récupérer info fichier
                if let Some(file_info) = self.db.get_file_by_id(file_id)? {
                    sources.push(Source {
                        file_path: file_info.path,
                        filename: file_info.filename,
                        snippet: chunk.text,
                        chunk_index: chunk_idx,
                        relevance_score: score,
                    });
                }
            }
        }

        Ok(sources)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    #[ignore] // Requiert index complet
    fn test_assist_me_query() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(Database::new(temp_dir.path().join("test.db")).unwrap());

        let engine = AssistMeEngine::new("path/to/index", db).unwrap();

        let sources = engine.answer_question(
            "Quels sont les budgets formation ?",
            5
        ).unwrap();

        assert!(sources.len() > 0);
        assert!(sources[0].relevance_score > 0.0);
    }
}
```

---

### Phase C : UI & Intégration (3 jours)

#### **Jour 8 : UI AssistMe Panel**

**Tâches :**
1. Créer `src/ui/assist_me_panel.rs`
2. Interface question/réponse
3. Affichage sources cliquables

**Code :**

```rust
// src/ui/assist_me_panel.rs

use eframe::egui;
use crate::app::XFinderApp;
use crate::semantic::Source;

pub fn render_assist_me_panel(app: &mut XFinderApp, ctx: &egui::Context) {
    egui::SidePanel::right("assist_me_panel")
        .default_width(400.0)
        .show(ctx, |ui| {
            ui.heading("🤖 Assist Me");
            ui.separator();

            // Input question
            ui.horizontal(|ui| {
                ui.label("Question:");
                if ui.text_edit_singleline(&mut app.assist_me_query).lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                {
                    // Lancer recherche
                    app.process_assist_me_query();
                }
            });

            ui.add_space(5.0);

            if ui.button("🔍 Rechercher").clicked() {
                app.process_assist_me_query();
            }

            ui.separator();

            // Afficher résultats
            if app.assist_me_loading {
                ui.spinner();
                ui.label("Recherche en cours...");
            } else if !app.assist_me_sources.is_empty() {
                ui.label(format!("📚 {} sources trouvées", app.assist_me_sources.len()));
                ui.add_space(10.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (idx, source) in app.assist_me_sources.iter().enumerate() {
                        render_source_card(ui, idx + 1, source);
                        ui.add_space(5.0);
                    }
                });
            } else if !app.assist_me_query.is_empty() {
                ui.label("❌ Aucune source trouvée");
            }
        });
}

fn render_source_card(ui: &mut egui::Ui, rank: usize, source: &Source) {
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(245, 245, 245))
        .inner_margin(8.0)
        .rounding(5.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("#{}", rank)).strong());
                ui.label("📄");
                if ui.link(&source.filename).clicked() {
                    // Ouvrir fichier
                    let _ = open::that(&source.file_path);
                }
            });

            ui.add_space(3.0);

            // Snippet
            ui.label(
                egui::RichText::new(&source.snippet)
                    .small()
                    .color(egui::Color32::DARK_GRAY)
            );

            ui.add_space(3.0);

            // Score
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(
                    format!("Pertinence: {:.1}%", source.relevance_score * 100.0)
                ).small());

                if ui.small_button("📂 Ouvrir dossier").clicked() {
                    let folder = std::path::Path::new(&source.file_path)
                        .parent()
                        .unwrap();
                    let _ = open::that(folder);
                }
            });
        });
}
```

---

#### **Jour 9 : Intégration dans XFinderApp**

**Tâches :**
1. Ajouter champs AssistMe dans `XFinderApp`
2. Méthode `process_assist_me_query()`
3. Thread background pour recherche

**Code :**

```rust
// src/app.rs - Ajouts

pub struct XFinderApp {
    // ... champs existants

    // Assist Me
    pub assist_me_engine: Option<AssistMeEngine>,
    pub assist_me_query: String,
    pub assist_me_sources: Vec<Source>,
    pub assist_me_loading: bool,
    pub assist_me_enabled: bool, // Config utilisateur
}

impl XFinderApp {
    pub fn init_assist_me(&mut self) {
        if !self.assist_me_enabled {
            return;
        }

        let index_path = self.index_dir.join("leann_index");
        let db = self.database.clone().unwrap();

        match AssistMeEngine::new(index_path.to_str().unwrap(), db) {
            Ok(engine) => {
                self.assist_me_engine = Some(engine);
                self.error_message = Some("✅ Assist Me activé".to_string());
            }
            Err(e) => {
                self.error_message = Some(format!("❌ Assist Me: {}", e));
            }
        }
    }

    pub fn process_assist_me_query(&mut self) {
        if self.assist_me_query.is_empty() {
            return;
        }

        if let Some(ref engine) = self.assist_me_engine {
            self.assist_me_loading = true;

            let query = self.assist_me_query.clone();
            let engine_clone = engine.clone(); // Besoin Arc<> pour clone

            // Thread background
            std::thread::spawn(move || {
                match engine_clone.answer_question(&query, 10) {
                    Ok(sources) => {
                        // TODO: Send via channel
                        println!("Found {} sources", sources.len());
                    }
                    Err(e) => {
                        eprintln!("Assist Me error: {}", e);
                    }
                }
            });
        }
    }
}
```

---

#### **Jour 10 : Tests & Polish**

**Tâches :**
1. Tests end-to-end complets
2. Gestion erreurs
3. Loading states
4. Documentation

---

### Phase D : Optimisations & Production (2 jours)

#### **Jour 11 : Optimisations Performance**

**Tâches :**
1. Caching résultats
2. Batch embeddings
3. Profiling latence
4. Optimiser chunking

---

#### **Jour 12 : Documentation & Livraison**

**Tâches :**
1. README Assist Me
2. Guide utilisateur
3. Tests finaux
4. Merge dans master

---

## 👤 Flux utilisateur

### Workflow complet

```
1. ACTIVATION (première fois)
   User: Ouvre Settings → onglet "Assist Me"
   User: Coche "Activer recherche sémantique"
   App: Lance indexation embeddings (background)
   App: Affiche progression "Indexation sémantique: 234/1000 fichiers"

2. UTILISATION
   User: Clic bouton "🤖 Assist Me" (ou Ctrl+Shift+A)
   App: Ouvre panneau latéral AssistMe

   User: Tape "Quels sont les budgets formation 2024 ?"
   User: Appuie Enter (ou clic "Rechercher")

   App: Affiche spinner "Recherche en cours..."
   App: (Background) Query → Embedding → LEANN search → DB lookup
   App: Affiche 5-10 sources avec extraits

   User: Clic sur source #1 "Budget_Formation_2024.pdf"
   App: Ouvre le PDF à la bonne page

   User: Clic "Ouvrir dossier" sur source #2
   App: Ouvre l'explorateur Windows

3. HISTORIQUE
   User: Clic "Historique" (ou flèche haut dans input)
   App: Affiche questions précédentes
   User: Sélectionne question passée
   App: Ré-affiche les sources
```

---

## 🔧 Détails techniques

### Structure des données

**Chunks DB Schema:**
```sql
CREATE TABLE chunks (
    chunk_id TEXT PRIMARY KEY,        -- "file_id_chunk_idx"
    file_id INTEGER NOT NULL,
    chunk_text TEXT NOT NULL,         -- 500 tokens ~2000 chars
    chunk_index INTEGER NOT NULL,     -- Position dans le fichier
    created_at INTEGER NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
);
```

**LEANN Storage:**
```
~/.xfinder_index/leann_index/
  ├─ graph.bin          (Graphe HNSW pruné, ~5 MB pour 100k docs)
  ├─ metadata.json      (Config: dim, model_name, etc.)
  └─ ids_mapping.bin    (Mapping doc_id → internal_id)
```

### Calculs de storage

**Pour 10,000 fichiers :**
- Contenu texte moyen : 10 KB/fichier = 100 MB total
- Chunks (500 tokens) : ~20 chunks/fichier = 200k chunks
- Embeddings classiques : 200k × 384 × 4 bytes = **307 MB**
- LEANN (97% réduction) : **~9 MB** 💾

**Économie : 298 MB saved !**

---

## ⏱️ Timeline et estimation

### Planning détaillé

| Phase | Jours | Tâches | Risques |
|-------|-------|--------|---------|
| **Phase A: Setup** | 3j | PyO3, LEANN wrapper, ContentExtractor | Installation Python/LEANN |
| **Phase B: Embeddings** | 4j | Candle, modèle, indexation, AssistMe | Compatibilité Candle |
| **Phase C: UI** | 3j | Panel UI, intégration, sources cliquables | - |
| **Phase D: Polish** | 2j | Optimisations, tests, doc | - |
| **TOTAL** | **12j** | (~2.5 semaines) | |

### Checkpoints validation

✅ **Checkpoint 1 (Jour 3):** PyO3 + LEANN fonctionne
✅ **Checkpoint 2 (Jour 7):** Indexation complète OK
✅ **Checkpoint 3 (Jour 10):** UI fonctionnelle
✅ **Checkpoint 4 (Jour 12):** Production ready

---

## 🎯 Critères de succès

### Performance

- [x] Indexation: <1s per fichier (extraction + chunking)
- [x] Recherche: <3s (query → sources affichées)
- [x] Storage: <10 MB pour 10k fichiers (LEANN graph)
- [x] RAM: <500 MB idle, <2 GB pendant indexation

### Qualité

- [x] Top-5 pertinence: >80% sur 50 questions test
- [x] Crash rate: 0% sur tests manuels
- [x] Sources cliquables: 100% fonctionnelles

### UX

- [x] Loading states clairs
- [x] Erreurs explicatives
- [x] Liens ouvrent bon fichier
- [x] Panel responsive (<100ms input lag)

---

## 📚 Références

- [LEANN GitHub](https://github.com/yichuan-w/LEANN)
- [PyO3 Documentation](https://pyo3.rs/)
- [Candle ML Framework](https://github.com/huggingface/candle)
- [all-MiniLM-L6-v2 Model](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2)
- PRD xfinder - F6: Mode "Assist Me"
- Backlog Phase 3

---

**Document version:** 1.0
**Dernière mise à jour:** 2025-11-14
**Auteur:** xfinder Team
**Status:** 🚧 En cours d'implémentation
