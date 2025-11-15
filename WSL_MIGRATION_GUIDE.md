# WSL Migration Guide - xfinder Assist Me Mode

## 🎯 Contexte du Projet

**xfinder** : Application desktop Rust (egui) de recherche de fichiers avec 2 modes :
1. **Classic Search** : Recherche fulltext avec Tantivy (✅ FONCTIONNE)
2. **Assist Me** : Recherche sémantique IA avec embeddings + LEANN (🚧 EN COURS)

---

## 📊 État actuel (Janvier 2025)

### ✅ Ce qui fonctionne

#### Mode Classic (100% opérationnel)
- ✅ Indexation Tantivy (n-grams 2-100)
- ✅ Recherche fulltext rapide
- ✅ Watchdog temps réel (détection auto des changements)
- ✅ Filtres avancés (date, taille, type de fichier)
- ✅ Preview de fichiers
- ✅ Database SQLite pour métadonnées
- ✅ Standalone `.exe` (~10 MB)

#### Mode Assist Me (Infrastructure prête)
- ✅ UI complète (barre recherche, suggestions, cartes de résultats)
- ✅ Sidebar dynamique selon le mode
- ✅ Messages d'erreur séparés par mode
- ✅ Architecture dual-mode (Classic ↔ Assist Me)
- ✅ Modules sémantiques (code complet) :
  - `ContentExtractor` : Extraction texte (PDF, DOCX, TXT, etc.)
  - `Chunker` : Découpage en chunks 500 tokens + 10% overlap
  - `EmbeddingGenerator` : Wrapper PyO3 pour sentence-transformers
  - `LeannIndex` : Wrapper PyO3 pour LEANN
  - `SemanticIndexer` : Pipeline complet
  - `BackgroundIndexer` : Thread non-bloquant avec queue
- ✅ Boutons UI ("Initialiser Assist Me", "Indexer maintenant")
- ✅ Progress tracking temps réel
- ✅ Channel async pour résultats de recherche

### ❌ Ce qui ne marche PAS (raison de la migration WSL)

**Problème** : LEANN **n'a pas de build Windows**
- `pip install leann` échoue car `leann-backend-hnsw` n'existe pas pour Windows
- PyO3 fonctionne mais LEANN manquant bloque l'init
- Solution : **WSL (Linux)** où LEANN s'installe normalement

---

## 🏗️ Architecture Technique

### Stack technologique

```
┌─────────────────────────────────────┐
│         xfinder.exe (Rust)          │
├─────────────────────────────────────┤
│  egui UI (natif, pas de navigateur)│
├─────────────────────────────────────┤
│  Mode Classic    │   Mode Assist Me │
│  ---------------│------------------│
│  Tantivy (Rust) │   PyO3 (Rust↔Python)
│  SQLite (Rust)  │   sentence-transformers
│  Watchdog       │   LEANN vector DB
└─────────────────────────────────────┘
```

### Dépendances clés

**Rust** (Cargo.toml) :
- `eframe` : UI framework
- `tantivy` : Fulltext search
- `rusqlite` : Database
- `walkdir` : File scanning
- `notify` : Watchdog
- `pyo3` : Python bindings (pour Assist Me)
- `crossbeam-channel` : Threading

**Python** (pour Assist Me) :
- `sentence-transformers` : Embeddings (all-MiniLM-L6-v2, 384 dim)
- `torch` : Backend ML
- `leann` : Low-storage vector index (97% économie mémoire)

---

## 📂 Structure du Code

```
xfinder/
├── src/
│   ├── main.rs                    # Entry point
│   ├── app.rs                     # État principal de l'app
│   ├── config/
│   │   └── mod.rs                 # Config TOML
│   ├── search/
│   │   ├── tantivy_index.rs       # Classic search
│   │   ├── file_watcher.rs        # Watchdog
│   │   └── scanner.rs             # File scanning
│   ├── semantic/                  # 🎯 ASSIST ME
│   │   ├── mod.rs
│   │   ├── content_extractor.rs   # PDF, DOCX → texte
│   │   ├── chunker.rs             # Texte → chunks
│   │   ├── embedding_generator.rs # PyO3 → sentence-transformers
│   │   ├── leann_wrapper.rs       # PyO3 → LEANN
│   │   ├── semantic_indexer.rs    # Pipeline complet
│   │   └── background_indexer.rs  # Thread + queue
│   ├── database/
│   │   ├── mod.rs
│   │   └── queries.rs
│   ├── ui/
│   │   ├── main_ui.rs             # Classic UI
│   │   ├── assist_me_ui.rs        # 🎯 Assist Me UI
│   │   ├── side_panel.rs          # Sidebar dynamique
│   │   ├── top_panel.rs           # Tabs mode switching
│   │   └── ...
│   └── system/
│       ├── tray.rs
│       ├── hotkey.rs
│       └── scheduler.rs
├── Cargo.toml
├── config.toml                    # Config utilisateur
└── DUAL_MODE_ARCHITECTURE.md      # Spécifications Phase 3
```

---

## 🔧 Ce qui a été fait récemment

### Session du 2025-01-XX (avant WSL)

**Commits** :
- `7bef496` : Complete Assist Me mode implementation with dynamic UI
- `76a37e5` : Implement basic semantic search in Assist Me UI
- `17df3bc` : Add manual semantic indexing trigger
- `bf370fc` : Add comprehensive logging for debug (dernier commit)

**Modifications clés** :

1. **app.rs** :
   - Ajout `assist_me_error: Option<String>` (messages séparés)
   - Ajout `search_results_rx: Option<Receiver<Vec<AssistMeSource>>>` (channel async)
   - Méthode `init_semantic_indexing()` avec logs détaillés
   - Méthode `start_semantic_indexing()` (scan + enqueue)
   - Méthode `perform_semantic_search()` (query → LEANN)
   - Méthode `process_search_results()` (channel → UI)
   - Méthode `process_semantic_indexing_stats()` (progress tracking)

2. **ui/top_panel.rs** :
   - **Changement important** : Retiré l'auto-init au switch d'onglet
   - L'init se fait maintenant via bouton sidebar

3. **ui/side_panel.rs** :
   - `render_classic_sidebar()` : sidebar pour Classic
   - `render_assist_me_sidebar()` : sidebar pour Assist Me
   - Boutons "Initialiser Assist Me" et "Indexer maintenant"
   - Stats temps réel (fichiers, chunks, erreurs)
   - Toggle "Auto-indexer nouveaux fichiers"

4. **ui/assist_me_ui.rs** :
   - Affichage `assist_me_error` avec couleurs
   - Suggestions cliquables
   - Cartes de résultats avec scores colorés
   - Liens cliquables (opener crate)

5. **semantic/semantic_indexer.rs** :
   - Logs détaillés à chaque étape
   - Gestion erreurs explicite

---

## 🚨 Problèmes rencontrés (Windows)

### 1. PyO3 avec venv
- **Problème** : PyO3 `auto-initialize` ne trouve pas le venv
- **Tenté** : `PYTHONHOME`, `PYO3_PYTHON`, `PATH`
- **Résultat** : Crash avec `Fatal Python error: init_fs_encoding`

### 2. LEANN incompatible Windows
- **Erreur** : `leann-backend-hnsw` n'a pas de wheel Windows
- **Toutes les versions** (0.1.0 à 0.3.5) échouent
- **Raison** : Dépendance C++ pas compilée pour Windows

### 3. Conflit versions torch
- **Problème** : `torch 2.1.1` incompatible avec `transformers 4.57.1`
- **Fix** : `pip install --upgrade torch --index-url https://download.pytorch.org/whl/cpu`

---

## ✅ Plan sur WSL

### Étape 1 : Setup environnement

```bash
# Dans WSL Ubuntu
cd /mnt/d/DataLab/xfinder  # Ou cloner depuis GitHub

# Installer Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Installer Python + pip
sudo apt update
sudo apt install python3 python3-pip python3-venv -y

# Créer venv
python3 -m venv .venv
source .venv/bin/activate

# Installer dépendances Python
pip install sentence-transformers torch leann

# Tester LEANN
python -c "import leann; print('✅ LEANN OK')"
```

### Étape 2 : Compiler

```bash
# Avec venv activé
export PYO3_PYTHON=$(pwd)/.venv/bin/python3

# Build
cargo build --release

# Lancer
./target/release/xfinder
```

### Étape 3 : Tester Assist Me

1. Lancer l'app
2. Cliquer onglet "🤖 Assist Me"
3. Dans sidebar : cliquer "🚀 Initialiser Assist Me"
4. **Observer la console** : doit afficher tous les logs
5. Cliquer "📚 Indexer maintenant"
6. Vérifier progression temps réel
7. Taper une question et chercher

---

## 📝 TODO List (après WSL setup)

### Sprint 1 : Finaliser retrieval (PRIORITAIRE)

- [ ] **Récupérer vrais chemins de fichiers**
  - Actuellement : `file_path = format!("file_{}.txt", file_id)` (fake)
  - Nécessaire : `database.get_file_path(file_id)?`
  - Ajouter méthode dans `database/queries.rs`

- [ ] **Extraire vrai texte des chunks**
  - Actuellement : `excerpt = format!("Chunk #{}", chunk_index)` (fake)
  - Nécessaire : relire fichier + extraire chunk
  - Utiliser `ContentExtractor` + positions start/end

- [ ] **Tester avec vrais documents**
  - Indexer 100-1000 fichiers réels
  - Chercher "factures EDF 2024"
  - Vérifier pertinence des résultats

### Sprint 2 : Watchdog auto-indexing

- [ ] **Connecter watchdog au semantic indexing**
  - Modifier `process_watchdog_events()` dans app.rs
  - Si Assist Me mode + auto_index activé → enqueue vers BackgroundIndexer
  - Gérer create/modify/delete

### Sprint 3 : Config dual-mode

- [ ] **Séparer config Classic vs Assist Me**
  - Actuellement : `scan_paths` partagé
  - Créer `ClassicConfig` et `AssistMeConfig` séparés
  - Toggle "Sync config" dans Settings

### Sprint 4 : Polish & tests

- [ ] Cache des chunks en mémoire (LRU)
- [ ] Progress bar pour recherche
- [ ] Highlights dans excerpts
- [ ] Tests end-to-end

---

## 🐛 Debug Tips

### Logs importants

Avec les logs ajoutés, tu verras :

```
🔧 init_semantic_indexing() called
📍 LEANN index path: /home/user/.xfinder_index/leann_index
📍 Model: all-MiniLM-L6-v2
🔄 Creating SemanticIndexer...
📦 SemanticIndexer::new()
🔄 Creating EmbeddingGenerator...
🔄 Loading Sentence Transformer model...
[sentence-transformers télécharge le modèle...]
✅ Model loaded successfully!
📐 Embedding dimension: 384
🔄 Creating LEANN index...
✅ LEANN index created!
🔄 Initializing LEANN builder...
✅ LEANN builder initialized!
✅ SemanticIndexer::new() completed successfully!
🔄 Starting BackgroundIndexer (batch_size=10)...
✅ BackgroundIndexer started successfully!
✅ ✅ ✅ Semantic indexing system initialized successfully!
```

**Si échec**, les logs affichent :
```
❌ Failed to create SemanticIndexer: [erreur détaillée]
   Details: [stacktrace]

💡 PRÉREQUIS:
   1. Python 3.8+ installé
   2. pip install sentence-transformers
   3. pip install leann
```

### Commandes de debug

```bash
# Vérifier Python
python --version
which python

# Vérifier packages
pip list | grep -E "(sentence|leann|torch)"

# Tester imports
python -c "from sentence_transformers import SentenceTransformer; print('OK')"
python -c "import leann; print('OK')"

# Compiler avec logs Rust
RUST_LOG=debug cargo run --release
```

---

## 📦 Configuration

### config.toml (utilisateur)

```toml
[scan_paths]
paths = ["/home/user/Documents", "/home/user/Downloads"]

[assist_me]
enabled = true
auto_index_new_files = false
batch_size = 10
model_path = "sentence-transformers/all-MiniLM-L6-v2"
leann_index_path = "/home/user/.xfinder_index/leann_index"
```

---

## 🎯 Objectif Final

**Application standalone** avec :
- ✅ `.exe` Windows (Classic search seulement)
- ✅ Binary Linux (Classic + Assist Me avec LEANN)
- Future : Migration vers Rust pur (rust-bert + usearch) pour vrai standalone multi-OS

---

## 📞 Points de reprise

**Quand tu reviens sur ce projet** :

1. Lire ce document en entier
2. Vérifier le dernier commit : `git log -1`
3. Lire `DUAL_MODE_ARCHITECTURE.md` (spécifications Phase 3)
4. Compiler et tester sur WSL
5. Continuer le TODO List ci-dessus

**Questions fréquentes** :

- **Où en est-on ?** → Infra complète, retrieval à finaliser
- **Quel est le blocage ?** → LEANN pas dispo Windows, migration WSL nécessaire
- **Prochaine étape ?** → Setup WSL + tester init LEANN + récupérer vrais fichiers

---

**Date de création** : 2025-01-XX
**Dernier commit** : `bf370fc` - "Add comprehensive logging for semantic indexing debug"
**Statut** : 🚧 Migration WSL en cours
