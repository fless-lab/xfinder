# Guide des Décisions Techniques - xfinder
**Technical Decision Record (TDR)**

Ce document explique les choix technologiques majeurs, leurs alternatives, et les raisons de chaque décision.

---

## Décision 1 : Framework Application - Tauri vs Electron

### Contexte
Besoin d'une application desktop Windows avec interface moderne, performante et légère.

### Options évaluées

| Critère | Tauri | Electron | .NET WPF | Qt |
|---------|-------|----------|----------|-----|
| **Taille bundle** | ~10MB | ~150MB | ~30MB | ~20MB |
| **Mémoire (idle)** | ~80MB | ~200MB | ~50MB | ~60MB |
| **Performance** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Sécurité** | ⭐⭐⭐⭐⭐ (Rust) | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **UI moderne** | ⭐⭐⭐⭐⭐ (Web) | ⭐⭐⭐⭐⭐ (Web) | ⭐⭐⭐ | ⭐⭐⭐ |
| **Écosystème** | ⭐⭐⭐ (jeune) | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Dev velocity** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **Cross-platform** | ✅ | ✅ | ❌ (Windows) | ✅ |
| **Intégration Rust** | ⭐⭐⭐⭐⭐ | ⭐⭐ (NAPI) | ❌ | ⭐⭐ |

### ✅ Décision : Tauri

**Raisons :**
1. **Taille** : 10MB vs 150MB (Electron) = crucial pour distribution entreprise
2. **Performance** : Backend Rust = idéal pour indexation intensive
3. **Sécurité** : Memory safety Rust + sandboxing = essentiel pour données sensibles
4. **Moderne** : Frontend web (React) = UI riche sans compromis
5. **Futur** : Multi-plateforme possible (Linux si demande)

**Trade-offs acceptés :**
- Écosystème plus jeune (mais mature pour production depuis Tauri 1.0)
- Moins de libs tierces (mais suffisant pour nos besoins)

**Risques :**
- Bugs potentiels Tauri 2.x (nouveau) → **Mitigation** : Tests extensifs, fallback Tauri 1.x stable

---

## Décision 2 : Moteur de recherche - Tantivy vs Elasticsearch vs SQLite FTS5

### Contexte
Besoin de recherche full-text rapide sur métadonnées ET contenu fichiers.

### Options évaluées

| Critère | Tantivy (Rust) | Elasticsearch | SQLite FTS5 | Meilisearch |
|---------|----------------|---------------|-------------|-------------|
| **Performance** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Taille index** | Compact | Élevé | Très compact | Compact |
| **Complexité** | Moyenne | Élevée | Faible | Faible |
| **Embedded** | ✅ | ❌ (serveur) | ✅ | ❌ (serveur) |
| **Intégration Rust** | Native | HTTP API | Binding | HTTP API |
| **Français** | ✅ | ✅ | ✅ | ✅ |
| **Fuzzy search** | ✅ | ✅ | Limité | ✅ |
| **RAM usage** | Faible | Élevé | Très faible | Moyen |

### ✅ Décision : Tantivy (métadonnées) + SQLite FTS5 (contenu)

**Stratégie hybride :**

1. **Tantivy** pour :
   - Recherche rapide nom fichier
   - Ranking sophistiqué
   - Fuzzy matching

2. **SQLite FTS5** pour :
   - Recherche full-text contenu
   - Simplicité backup (fichier unique)
   - Transactions ACID

**Raisons :**
- **Embedded** : Pas de serveur séparé (simplicité déploiement)
- **Performance** : Tantivy = Lucene-like en Rust (ultra rapide)
- **Taille** : Pas de JVM, pas de serveur lourd
- **Rust-native** : Intégration seamless

**Trade-offs :**
- Pas de réplication distribuée (OK, app locale)
- Moins de features avancées qu'Elasticsearch (suffisant pour nos besoins)

**Alternative si problème :**
- Fallback 100% SQLite FTS5 (plus simple mais moins performant)

---

## Décision 3 : Vector DB - LEANN vs FAISS vs ChromaDB

### Contexte
Recherche sémantique via embeddings pour mode "Assist Me".

### Options évaluées

| Critère | LEANN | FAISS | ChromaDB | Qdrant |
|---------|-------|-------|----------|--------|
| **Taille index** | ⭐⭐⭐⭐⭐ (97% réduc) | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **Vitesse** | ⭐⭐⭐⭐ (à valider) | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Embedded** | ✅ | ✅ | ❌ (serveur) | ❌ (serveur) |
| **Rust support** | ⭐⭐⭐ | ⭐⭐⭐ (binding) | ❌ | ✅ (natif) |
| **Maturité** | ⭐⭐ (nouveau) | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Facilité** | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |

### ✅ Décision : LEANN (avec fallback FAISS)

**Raisons :**
1. **Taille** : Claim 97% réduction = game-changer (500MB vs 20GB pour 100k docs)
2. **Innovation** : Approche HNSW optimisée mentionnée sur Reddit
3. **Cible** : Parfait pour app locale (vs serveur lourd)

**Plan de validation :**
```
Phase 1: POC LEANN (2 semaines)
  - Benchmark 10k documents
  - Mesurer taille index, vitesse, recall@10
  - Comparer vs FAISS baseline

Si échec (recall <85% ou vitesse <acceptable):
  → Fallback FAISS (mature, éprouvé)

Si succès:
  → Production LEANN
```

**Critères succès POC :**
- Taille index <50% de FAISS
- Recall@10 >90%
- Latency <500ms (recherche)

**Fallback FAISS :**
```rust
use faiss::{Index, IndexImpl, MetricType};

let index = faiss::index_factory(
    384,                    // Dimension (all-MiniLM-L6-v2)
    "IVF100,PQ64",         // Quantization
    MetricType::InnerProduct
)?;
```

---

## Décision 4 : OCR Engine - Tesseract vs PaddleOCR vs Azure OCR

### Contexte
Extraction texte PDF scannés et images pour indexation.

### Options évaluées

| Critère | Tesseract 5 | PaddleOCR | Azure OCR | Windows OCR API |
|---------|-------------|-----------|-----------|-----------------|
| **Précision FR** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Vitesse** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Taille** | 30MB | 8MB | 0MB (cloud) | 0MB (OS) |
| **Offline** | ✅ | ✅ | ❌ | ✅ |
| **Coût** | Gratuit | Gratuit | Payant | Gratuit |
| **Confidentialité** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ (cloud) | ⭐⭐⭐⭐ |
| **Rust binding** | ✅ (leptess) | ⭐⭐ (ONNX) | HTTP | Win32 API |

### ✅ Décision : Tesseract 5 (avec option Azure pour pro)

**Raisons primaires :**
1. **Offline** : Essentiel pour données confidentielles administration
2. **Précision** : Excellent français (tessdata_best trained)
3. **Mature** : Référence industrie depuis 15+ ans
4. **Binding Rust** : `leptess` stable et maintenu

**Configuration optimale :**
```toml
# Cargo.toml
leptess = "0.14"

# Téléchargement tessdata
fra.traineddata (best) = 14MB
eng.traineddata (best) = 15MB
Total: ~30MB
```

**Preprocessing pour meilleure qualité :**
```rust
// Via Leptonica (inclus dans leptess)
fn preprocess_for_ocr(image: &Path) -> PathBuf {
    // 1. Deskew (redressement)
    // 2. Binarization (Otsu threshold)
    // 3. Noise removal
    // 4. Contrast enhancement
}
```

**Option premium (V2) :**
- **Azure Computer Vision OCR** : Pour clients avec budget
- Mode hybride : Tesseract local par défaut, Azure si activé
- Configuration :
```json
{
  "ocr_provider": "tesseract", // ou "azure"
  "azure_key": "...",
  "fallback_to_tesseract": true
}
```

**Benchmarks attendus :**
- Page A4 (300 DPI) : <5s (Tesseract) vs <1s (Azure)
- Précision doc admin FR : >95% (Tesseract) vs >98% (Azure)

---

## Décision 5 : Embedding Model - all-MiniLM-L6-v2 vs others

### Contexte
Génération embeddings pour recherche sémantique, doit être petit et rapide.

### Options évaluées

| Modèle | Taille | Dimension | Vitesse | Multilingue | Qualité FR |
|--------|--------|-----------|---------|-------------|-----------|
| **all-MiniLM-L6-v2** | 80MB | 384 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| all-mpnet-base-v2 | 420MB | 768 | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| multilingual-e5-small | 120MB | 384 | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| CamemBERT (FR only) | 440MB | 768 | ⭐⭐⭐ | ❌ | ⭐⭐⭐⭐⭐ |

### ✅ Décision : all-MiniLM-L6-v2 (MVP) → multilingual-e5-small (V2)

**Phase 1 (MVP) : all-MiniLM-L6-v2**

**Raisons :**
- **Taille** : 80MB = acceptable pour distribution
- **Vitesse** : Très rapide (384 dim vs 768)
- **Qualité** : Bon compromis (MTEB score 56.3)
- **Populaire** : Très utilisé, bien testé

**Phase 2 : multilingual-e5-small**

**Upgrade si feedback :**
- Meilleur français natif
- Même vitesse (384 dim)
- +40MB acceptable

**Intégration Rust (Candle) :**
```rust
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use tokenizers::Tokenizer;

pub struct EmbeddingModel {
    model: BertModel,
    tokenizer: Tokenizer,
}

impl EmbeddingModel {
    pub fn load_minilm() -> Result<Self> {
        let model_path = "models/all-MiniLM-L6-v2";
        let tokenizer = Tokenizer::from_file(
            format!("{}/tokenizer.json", model_path)
        )?;

        let device = Device::Cpu; // Ou Cuda si GPU
        let vb = VarBuilder::from_pth(
            format!("{}/model.safetensors", model_path),
            device
        )?;

        let model = BertModel::load(vb)?;
        Ok(Self { model, tokenizer })
    }

    pub fn encode(&self, text: &str) -> Result<Vec<f32>> {
        // 1. Tokenize
        let encoding = self.tokenizer.encode(text, true)?;
        let tokens = Tensor::new(encoding.get_ids(), &Device::Cpu)?;

        // 2. Forward pass
        let output = self.model.forward(&tokens)?;

        // 3. Mean pooling
        let embedding = mean_pooling(output)?;

        // 4. Normalize L2
        normalize(embedding)?.to_vec1()
    }
}
```

**Performance cible :**
- Encoding : <50ms par document (512 tokens)
- Batch 100 docs : <2s

---

## Décision 6 : LLM local - Llama 3.2 1B vs Alternatives

### Contexte
Génération réponses mode "Assist Me" (optionnel).

### Options évaluées

| Modèle | Taille | Qualité | Vitesse (CPU) | Français | RAM |
|--------|--------|---------|---------------|----------|-----|
| **Llama 3.2 1B** | 1.3GB | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | 2GB |
| Llama 3.2 3B | 3.7GB | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | 5GB |
| Phi-3 Mini | 2.4GB | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | 3GB |
| Mistral 7B Q4 | 4GB | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ | 6GB |

### ✅ Décision : Feature OPTIONNELLE avec Llama 3.2 1B

**Stratégie :**

**Mode 1 (défaut) : Sans LLM**
- Affiche extraits pertinents + liens
- Instantané, léger
- Suffisant pour 80% cas usage

**Mode 2 (optionnel) : Avec LLM**
- Téléchargement séparé (+1.3GB)
- Génération réponse synthétique
- Meilleure UX mais plus lent

**Implémentation :**
```rust
// Via llama.cpp (binding Rust)
use llama_cpp_rs::{LlamaModel, LlamaContext};

pub struct LocalLLM {
    model: LlamaModel,
    context: LlamaContext,
}

impl LocalLLM {
    pub fn load_llama32_1b() -> Result<Self> {
        let model = LlamaModel::from_file(
            "models/llama-3.2-1B-Q4_K_M.gguf",
            Default::default()
        )?;

        let context = LlamaContext::new(&model, 2048)?;
        Ok(Self { model, context })
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        let tokens = self.context.tokenize(prompt, true)?;
        let output = self.context.generate(tokens, 512)?;
        Ok(self.context.detokenize(output)?)
    }
}
```

**Prompt template :**
```
Tu es un assistant de recherche. Réponds à la question en te basant UNIQUEMENT sur les sources fournies. Cite les sources [1], [2], etc.

Question: {question}

Sources:
[1] {source_1_text} (fichier: {path_1})
[2] {source_2_text} (fichier: {path_2})
...

Réponse concise (3-5 phrases) :
```

**Trade-off :**
- +1.3GB download
- +2GB RAM
- +5-10s génération (CPU i5)

**Acceptable car :**
- Optionnel (pas obligatoire)
- Améliore drastiquement UX
- Trend marché (users habitués attendre LLM)

---

## Décision 7 : Email Parsing - PST Strategy

### Contexte
Parsing Outlook PST (format propriétaire Microsoft).

### Options évaluées

| Approche | Complexité | Fiabilité | Performance |
|----------|------------|-----------|-------------|
| **libpff (C) via FFI** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| readpst CLI wrapper | ⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| MAPI Win32 API | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| Python + pypff | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |

### ✅ Décision : MAPI Win32 API (priorité) + libpff (fallback)

**Stratégie à 2 niveaux :**

**1. MAPI (Windows COM) - Préféré**
```rust
// Via windows-rs
use windows::Win32::System::Mapi::*;

pub fn read_outlook_via_mapi() -> Result<Vec<Email>> {
    // Requiert Outlook installé
    // Accès direct profil actif
    // Plus fiable que PST parsing
}
```

**Avantages :**
- Support Microsoft officiel
- Gère PST + OST + Exchange
- Pas de parsing format complexe

**Inconvénients :**
- Requiert Outlook installé
- Windows uniquement

**2. libpff (C library) - Fallback**
```rust
// FFI vers libpff
#[link(name = "pff")]
extern "C" {
    fn libpff_file_open(filename: *const c_char, ...);
    fn libpff_file_get_number_of_messages(...);
}
```

**Avantages :**
- Pas besoin Outlook
- Parsing direct PST

**Inconvénients :**
- FFI complexe
- Bugs potentiels PST corrompu

**Décision finale :**
1. Détecte si Outlook installé → MAPI
2. Sinon → libpff
3. User peut choisir dans config

---

## Décision 8 : Database - SQLite vs PostgreSQL vs Custom

### Contexte
Stockage métadonnées, configuration, contenu extrait.

### Options évaluées

| Critère | SQLite | PostgreSQL | Sled (Rust KV) |
|---------|--------|------------|----------------|
| **Simplicité** | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ |
| **Performance** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Full-text** | ✅ (FTS5) | ✅ | ❌ |
| **ACID** | ✅ | ✅ | ✅ |
| **Embedded** | ✅ | ❌ (serveur) | ✅ |
| **Backup** | Fichier unique | Dump/restore | Fichier unique |
| **Taille** | Compact | Moyen | Compact |

### ✅ Décision : SQLite (rusqlite)

**Raisons :**
1. **Embedded** : Pas de serveur, fichier unique
2. **FTS5** : Full-text search natif (bonus pour contenu)
3. **Mature** : Utilisé partout, ultra fiable
4. **Backup** : Copie fichier = backup complet
5. **Rust binding** : `rusqlite` excellent

**Configuration optimale :**
```rust
use rusqlite::{Connection, params};

let conn = Connection::open("xfinder.db")?;

// Optimisations performance
conn.execute_batch(
    "PRAGMA journal_mode = WAL;       -- Write-Ahead Logging
     PRAGMA synchronous = NORMAL;     -- Balance perf/durabilité
     PRAGMA cache_size = 10000;       -- Cache 10MB
     PRAGMA temp_store = MEMORY;      -- Temp en RAM
     PRAGMA mmap_size = 30000000000;  -- Memory-mapped I/O (30GB)"
)?;
```

**Structure :**
```
storage/
├── index.db         # SQLite principal (métadonnées + config)
├── index.db-wal     # Write-Ahead Log
├── index.db-shm     # Shared memory
└── vectors/         # LEANN index (séparé)
```

---

## Décision 9 : Frontend Framework - React vs Vue vs Svelte

### Contexte
UI Tauri frontend.

### Options évaluées

| Critère | React | Vue | Svelte |
|---------|-------|-----|--------|
| **Bundle size** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Performance** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Écosystème** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Dev velocity** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Communauté** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **TypeScript** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |

### ✅ Décision : React + TypeScript

**Raisons :**
1. **Écosystème** : shadcn/ui, Radix UI (components qualité)
2. **Ressources** : Plus facile recruter devs React
3. **Tauri examples** : Majorité en React
4. **TypeScript** : Excellent support

**Stack complète :**
```json
{
  "dependencies": {
    "react": "^18",
    "react-dom": "^18",
    "@tanstack/react-query": "^5",  // Data fetching
    "zustand": "^4",                 // State management
    "@radix-ui/react-*": "^1",      // Primitives UI
    "tailwindcss": "^3",            // Styling
    "lucide-react": "^0.300"        // Icons
  }
}
```

**Architecture state :**
- **Zustand** : State global (config, indexing status)
- **React Query** : Server state (recherches, IPC Tauri)
- **Local state** : useState pour UI éphémère

**Alternative si problème :**
- Svelte = plus léger, mais écosystème moins riche

---

## Décision 10 : Build & Packaging - MSI vs EXE vs MSIX

### Contexte
Distribution Windows administration.

### Options évaluées

| Format | GPO support | Auto-update | User install | Admin install | Sandboxing |
|--------|-------------|-------------|--------------|---------------|------------|
| **MSI** | ✅ | ⭐⭐⭐ | ✅ | ✅ | ❌ |
| EXE | ⭐⭐ | ✅ | ✅ | ✅ | ❌ |
| MSIX | ✅ | ✅ | ✅ | ✅ | ✅ |
| AppX | ✅ | ✅ | ✅ | ✅ | ✅ |

### ✅ Décision : MSI (WiX) + Auto-updater Tauri

**Raisons :**
1. **GPO** : Essential pour déploiement masse administration
2. **Familier** : IT admins connaissent MSI
3. **WiX** : Intégration Tauri native
4. **Auto-update** : Géré par Tauri (pas MSI)

**Workflow :**
```
Build → MSI (installation initiale)
Runtime → Tauri updater (mises à jour)
```

**Configuration WiX :**
```xml
<!-- tauri.conf.json -->
{
  "tauri": {
    "bundle": {
      "windows": {
        "wix": {
          "language": ["fr-FR", "en-US"],
          "template": "wix/main.wxs",
          "enableElevatedUpdateTask": true
        }
      }
    },
    "updater": {
      "active": true,
      "endpoints": ["https://updates.xfinder.app/{{target}}/{{current_version}}"],
      "dialog": true,
      "pubkey": "..."
    }
  }
}
```

**Installation silencieuse (GPO) :**
```powershell
msiexec /i xfinder-1.0.0.msi /quiet /qn /norestart
```

---

## Décision 11 : Logging & Telemetry - Opt-in vs Opt-out

### Contexte
Besoin metrics usage pour amélioration, mais confidentialité critique.

### Options

| Approche | Privacy | Utilité | Compliance RGPD |
|----------|---------|---------|-----------------|
| **Opt-in** | ⭐⭐⭐⭐⭐ | ⭐⭐ | ✅ |
| Opt-out | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⚠️ |
| Aucune | ⭐⭐⭐⭐⭐ | ❌ | ✅ |
| Anonymisée locale | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ✅ |

### ✅ Décision : Telemetry locale anonymisée (opt-in)

**Principe :**
- **Stockage local uniquement** (jamais envoyé cloud)
- **Opt-in explicite** à l'installation
- **Anonymisé** (pas de PII, chemins hashés)
- **Exportable** pour support

**Données collectées :**
```rust
pub struct AnonymousTelemetry {
    // Performance
    pub avg_search_time_ms: f64,
    pub avg_indexing_speed_files_per_min: f64,

    // Usage
    pub total_searches: u64,
    pub search_mode_distribution: HashMap<SearchMode, u64>,
    pub total_files_indexed: u64,

    // Erreurs (pas de PII)
    pub error_counts: HashMap<ErrorType, u64>,

    // Système (anonymisé)
    pub os_version: String,
    pub cpu_cores: usize,
    pub ram_gb: u64,

    // JAMAIS collecté:
    // - Chemins fichiers
    // - Noms fichiers
    // - Contenu
    // - Queries utilisateur
}
```

**Interface :**
```
☐ Envoyer des statistiques anonymes d'usage
  Aide à améliorer xfinder. Aucune donnée personnelle n'est collectée.
  [En savoir plus]

  [Voir les données collectées] [Exporter pour support]
```

**Export support :**
- User peut exporter JSON pour ticket support
- Volontaire uniquement

---

## Décisions restantes (TBD)

### À valider par POC

1. **LEANN vs FAISS** : Benchmark attendu semaine 3-4
2. **Tesseract performance réelle** : Tests sur corpus admin
3. **Llama 3.2 1B qualité FR** : Evaluation Q&A

### À décider avec utilisateurs

1. **Langues OCR** : Français seul ou +Anglais/Allemand ?
2. **LLM opt-in ou opt-out** : Feature activée par défaut ?
3. **Fréquence mise à jour** : Hebdo, mensuel, manuel ?

### Questions ouvertes techniques

1. **GPU support** : CUDA pour embeddings ? (gain 10x vitesse mais +500MB)
2. **Compression index** : ZSTD sur content DB ? (gain 60% espace)
3. **Incremental backup** : SQLite backup API ou rsync-like ?

---

## Matrice de décision finale

| Composant | Technologie choisie | Alternative fallback |
|-----------|---------------------|---------------------|
| App framework | Tauri 2.0 | Electron (si bugs bloquants) |
| Frontend | React + TS | Svelte (si perf UI critique) |
| Backend | Rust | - |
| Search (metadata) | Tantivy | SQLite FTS5 |
| Search (content) | SQLite FTS5 | - |
| Vector DB | LEANN | FAISS |
| Embeddings | all-MiniLM-L6-v2 | multilingual-e5-small |
| OCR | Tesseract 5 | Azure OCR (premium) |
| LLM | Llama 3.2 1B (opt) | Pas de LLM |
| Email (Outlook) | MAPI Win32 | libpff |
| Email (Thunderbird) | mailparse | - |
| Database | SQLite | - |
| Packaging | MSI (WiX) | - |
| Updater | Tauri updater | Manuel |

---

## Changelog décisions

| Date | Décision | Raison changement |
|------|----------|-------------------|
| 2025-11-12 | Initial | Première version |

---

**Document version :** 1.0
**Dernière mise à jour :** 2025-11-12
**Prochaine revue :** Après POC LEANN (semaine 4)
