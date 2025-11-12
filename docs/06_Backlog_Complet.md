# Backlog Complet - xfinder
**Liste exhaustive de toutes les tâches du projet**

---

## Table des matières

1. [Phase 0 : Documentation](#phase-0--documentation)
2. [Phase 1 : MVP Indexation](#phase-1--mvp-indexation)
3. [Phase 2 : OCR + Contenu](#phase-2--ocr--contenu)
4. [Phase 3 : IA Assist Me](#phase-3--ia-assist-me)
5. [Phase 4 : Emails](#phase-4--emails)
6. [Phase 5 : Production](#phase-5--production)
7. [Tâches transverses](#tâches-transverses)
8. [Backlog futur](#backlog-futur)

---

## Légende

| Symbole | Signification |
|---------|---------------|
| 📝 | Documentation |
| 🏗️ | Architecture / Setup |
| 💻 | Développement |
| 🧪 | Tests |
| 🎨 | Design / UI |
| 🔧 | Configuration |
| ✅ | Validation / Review |
| 🚀 | Déploiement |
| 📊 | Métriques / Analytics |

**Priorités :**
- 🔴 **MUST** : Critique, bloquant
- 🟠 **SHOULD** : Important, recommandé
- 🟢 **COULD** : Nice to have
- 🔵 **FUTURE** : Backlog futur

---

## Phase 0 : Documentation

### ✅ Documentation initiale (TERMINÉ)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| D-001 | Rédiger PRD (Product Requirements) | 📝 | 🔴 | ✅ | 1j |
| D-002 | Rédiger Architecture Technique | 📝 | 🔴 | ✅ | 1j |
| D-003 | Rédiger Décisions Techniques | 📝 | 🔴 | ✅ | 0.5j |
| D-004 | Rédiger API & Schémas DB | 📝 | 🔴 | ✅ | 1j |
| D-005 | Rédiger Plan Tests & Métriques | 📝 | 🔴 | ✅ | 0.5j |
| D-006 | Créer README.md projet | 📝 | 🔴 | ✅ | 0.5j |
| D-007 | Créer Index documentation | 📝 | 🟠 | ✅ | 0.5j |
| D-008 | Créer Backlog complet | 📝 | 🔴 | ✅ | 0.5j |

### ⏭️ Validation documentation

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| V-001 | Review PRD avec sponsor/équipe | ✅ | 🔴 | ⏳ | 2h |
| V-002 | Review Architecture avec Tech Lead | ✅ | 🔴 | ⏳ | 2h |
| V-003 | Valider stack technique finale | ✅ | 🔴 | ⏳ | 1h |
| V-004 | Valider roadmap & timeline | ✅ | 🔴 | ⏳ | 1h |
| V-005 | Approval budget/ressources | ✅ | 🔴 | ⏳ | - |

### 📝 Documentation complémentaire

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| D-100 | Créer CONTRIBUTING.md | 📝 | 🟠 | ⏳ | 2h |
| D-101 | Créer SETUP.md (env dev) | 📝 | 🔴 | ⏳ | 3h |
| D-102 | Créer CODE_STYLE.md | 📝 | 🟠 | ⏳ | 1h |
| D-103 | Créer SECURITY.md | 📝 | 🟠 | ⏳ | 2h |
| D-104 | Choisir LICENSE (GPL-3/Apache-2) | 📝 | 🔴 | ⏳ | 1h |
| D-105 | Créer templates Issues GitHub | 📝 | 🟠 | ⏳ | 1h |
| D-106 | Créer template PR GitHub | 📝 | 🟠 | ⏳ | 1h |

---

## Phase 1 : MVP Indexation (Semaines 3-8)

### 🏗️ Setup projet

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| S-001 | Initialiser projet Tauri 2.0 | 🏗️ | 🔴 | ⏳ | 2h |
| S-002 | Setup React + TypeScript + Vite | 🏗️ | 🔴 | ⏳ | 2h |
| S-003 | Configurer TailwindCSS + shadcn/ui | 🎨 | 🔴 | ⏳ | 3h |
| S-004 | Setup Cargo workspace (modules) | 🏗️ | 🔴 | ⏳ | 2h |
| S-005 | Configurer Git + .gitignore | 🔧 | 🔴 | ⏳ | 1h |
| S-006 | Setup CI/CD GitHub Actions | 🔧 | 🔴 | ⏳ | 4h |
| S-007 | Configurer tests Rust (cargo test) | 🧪 | 🔴 | ⏳ | 2h |
| S-008 | Configurer tests Frontend (Vitest) | 🧪 | 🔴 | ⏳ | 2h |
| S-009 | Setup Playwright (E2E) | 🧪 | 🟠 | ⏳ | 3h |
| S-010 | Setup coverage (tarpaulin, vitest) | 📊 | 🟠 | ⏳ | 2h |
| S-011 | Créer structure dossiers backend | 🏗️ | 🔴 | ⏳ | 1h |
| S-012 | Créer structure dossiers frontend | 🏗️ | 🔴 | ⏳ | 1h |

### 💾 Database & Storage

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| DB-001 | Créer schéma SQLite initial | 💻 | 🔴 | ⏳ | 3h |
| DB-002 | Implémenter module database/schema.rs | 💻 | 🔴 | ⏳ | 4h |
| DB-003 | Implémenter database/queries.rs | 💻 | 🔴 | ⏳ | 6h |
| DB-004 | Créer système migrations DB | 💻 | 🔴 | ⏳ | 4h |
| DB-005 | Configurer SQLite optimisations (WAL, PRAGMA) | 🔧 | 🔴 | ⏳ | 2h |
| DB-006 | Implémenter connexion pool | 💻 | 🟠 | ⏳ | 3h |
| DB-007 | Tests unitaires database module | 🧪 | 🔴 | ⏳ | 4h |
| DB-008 | Benchmark SQLite performance | 📊 | 🟠 | ⏳ | 2h |

### 👁️ Module Watchdog

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| W-001 | Implémenter FileWatcher (notify-rs) | 💻 | 🔴 | ⏳ | 6h |
| W-002 | Implémenter EventDebouncer | 💻 | 🔴 | ⏳ | 4h |
| W-003 | Implémenter ExclusionFilter | 💻 | 🔴 | ⏳ | 5h |
| W-004 | Gérer événements Created/Deleted | 💻 | 🔴 | ⏳ | 4h |
| W-005 | Gérer événements Renamed/Moved | 💻 | 🔴 | ⏳ | 5h |
| W-006 | Gérer événements Modified (hash check) | 💻 | 🔴 | ⏳ | 4h |
| W-007 | Implémenter file_id → path mapping | 💻 | 🔴 | ⏳ | 3h |
| W-008 | Queue événements (batch processing) | 💻 | 🟠 | ⏳ | 4h |
| W-009 | Tests unitaires watchdog | 🧪 | 🔴 | ⏳ | 6h |
| W-010 | Tests intégration (créer 1000 fichiers) | 🧪 | 🟠 | ⏳ | 3h |
| W-011 | Gérer démarrage/arrêt propre watchdog | 💻 | 🔴 | ⏳ | 2h |

### 📇 Module Indexer

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| I-001 | Implémenter metadata extraction | 💻 | 🔴 | ⏳ | 4h |
| I-002 | Implémenter file walker (walkdir) | 💻 | 🔴 | ⏳ | 3h |
| I-003 | Implémenter hashing (blake3) | 💻 | 🔴 | ⏳ | 2h |
| I-004 | Intégrer Tantivy index | 💻 | 🔴 | ⏳ | 8h |
| I-005 | Configurer Tantivy schema | 🔧 | 🔴 | ⏳ | 3h |
| I-006 | Implémenter indexation parallèle (rayon) | 💻 | 🟠 | ⏳ | 6h |
| I-007 | Implémenter progression tracking | 💻 | 🔴 | ⏳ | 4h |
| I-008 | Implémenter pause/resume indexation | 💻 | 🟠 | ⏳ | 5h |
| I-009 | Gérer erreurs indexation (log, skip) | 💻 | 🔴 | ⏳ | 3h |
| I-010 | Implémenter indexation incrémentale | 💻 | 🔴 | ⏳ | 6h |
| I-011 | Optimiser vitesse (1000 files/min) | 🔧 | 🔴 | ⏳ | 4h |
| I-012 | Tests unitaires indexer | 🧪 | 🔴 | ⏳ | 6h |
| I-013 | Benchmark indexation 100k fichiers | 📊 | 🔴 | ⏳ | 3h |

### 🔍 Module Search Engine (basique)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| SE-001 | Implémenter SearchEngine struct | 💻 | 🔴 | ⏳ | 3h |
| SE-002 | Implémenter recherche rapide (Tantivy) | 💻 | 🔴 | ⏳ | 5h |
| SE-003 | Implémenter fuzzy matching | 💻 | 🔴 | ⏳ | 4h |
| SE-004 | Implémenter filtres (extension, date, taille) | 💻 | 🔴 | ⏳ | 5h |
| SE-005 | Implémenter ranking/scoring | 💻 | 🟠 | ⏳ | 4h |
| SE-006 | Implémenter snippet generation | 💻 | 🟠 | ⏳ | 3h |
| SE-007 | Optimiser <100ms (100k fichiers) | 🔧 | 🔴 | ⏳ | 4h |
| SE-008 | Tests unitaires search engine | 🧪 | 🔴 | ⏳ | 5h |
| SE-009 | Benchmark vitesse recherche | 📊 | 🔴 | ⏳ | 2h |

### 🎨 Interface Frontend MVP

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| UI-001 | Créer layout principal | 🎨 | 🔴 | ⏳ | 4h |
| UI-002 | Créer SearchBar component | 🎨 | 🔴 | ⏳ | 3h |
| UI-003 | Créer SearchResults component | 🎨 | 🔴 | ⏳ | 5h |
| UI-004 | Créer SearchFilters component | 🎨 | 🔴 | ⏳ | 4h |
| UI-005 | Créer FilePreview component | 🎨 | 🟠 | ⏳ | 5h |
| UI-006 | Créer ProgressBar indexation | 🎨 | 🔴 | ⏳ | 3h |
| UI-007 | Implémenter raccourci global Ctrl+Shift+F | 💻 | 🔴 | ⏳ | 3h |
| UI-008 | Implémenter debounce recherche (300ms) | 💻 | 🔴 | ⏳ | 2h |
| UI-009 | Gérer états loading/error | 🎨 | 🔴 | ⏳ | 3h |
| UI-010 | Créer page Configuration | 🎨 | 🔴 | ⏳ | 6h |
| UI-011 | Créer FolderTree component | 🎨 | 🔴 | ⏳ | 8h |
| UI-012 | Créer ExclusionRules component | 🎨 | 🔴 | ⏳ | 5h |
| UI-013 | Tests composants (Vitest + Testing Library) | 🧪 | 🔴 | ⏳ | 8h |

### 🔌 Tauri Commands (IPC)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| T-001 | Implémenter command search_files | 💻 | 🔴 | ⏳ | 3h |
| T-002 | Implémenter command start_indexing | 💻 | 🔴 | ⏳ | 4h |
| T-003 | Implémenter command get_indexing_progress | 💻 | 🔴 | ⏳ | 2h |
| T-004 | Implémenter command pause/resume_indexing | 💻 | 🟠 | ⏳ | 3h |
| T-005 | Implémenter command get_config | 💻 | 🔴 | ⏳ | 2h |
| T-006 | Implémenter command update_watched_folders | 💻 | 🔴 | ⏳ | 4h |
| T-007 | Implémenter command open_file | 💻 | 🔴 | ⏳ | 2h |
| T-008 | Implémenter command open_file_location | 💻 | 🔴 | ⏳ | 2h |
| T-009 | Implémenter events (indexing_progress, file_changed) | 💻 | 🔴 | ⏳ | 4h |
| T-010 | Tests IPC commands | 🧪 | 🔴 | ⏳ | 4h |

### 🔧 Configuration & Settings

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| C-001 | Implémenter AppConfig struct | 💻 | 🔴 | ⏳ | 2h |
| C-002 | Implémenter sauvegarde config.json | 💻 | 🔴 | ⏳ | 3h |
| C-003 | Implémenter chargement config au démarrage | 💻 | 🔴 | ⏳ | 2h |
| C-004 | Implémenter validation config | 💻 | 🔴 | ⏳ | 3h |
| C-005 | Gérer chemins AppData Windows | 💻 | 🔴 | ⏳ | 2h |
| C-006 | Implémenter defaults config (première install) | 💻 | 🔴 | ⏳ | 2h |
| C-007 | Tests config persistence | 🧪 | 🔴 | ⏳ | 3h |

### 🚀 Fonctionnalités système

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| SYS-001 | Implémenter démarrage au boot (optionnel) | 💻 | 🟠 | ⏳ | 4h |
| SYS-002 | Implémenter mode Tray (icône système) | 💻 | 🟠 | ⏳ | 5h |
| SYS-003 | Implémenter indexation différée (scheduler) | 💻 | 🔴 | ⏳ | 6h |
| SYS-004 | Configurer heure indexation (ex: 2h du matin) | 💻 | 🔴 | ⏳ | 4h |
| SYS-005 | Mode "sleep" (dors en tray, se réveille à l'heure) | 💻 | 🔴 | ⏳ | 5h |
| SYS-006 | Détection inactivité machine (pas indexer si usage actif) | 💻 | 🟠 | ⏳ | 4h |
| SYS-007 | Notification système (indexation terminée) | 💻 | 🟠 | ⏳ | 2h |
| SYS-008 | Gestion permissions UAC (élévation si besoin) | 💻 | 🟠 | ⏳ | 4h |

### 🧪 Tests Phase 1

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| TS-001 | Tests E2E : Installation → Config → Indexation | 🧪 | 🔴 | ⏳ | 6h |
| TS-002 | Tests E2E : Première recherche | 🧪 | 🔴 | ⏳ | 3h |
| TS-003 | Tests E2E : Exclusions granulaires | 🧪 | 🔴 | ⏳ | 4h |
| TS-004 | Tests intégration : Watchdog + Indexation | 🧪 | 🔴 | ⏳ | 4h |
| TS-005 | Tests load : 100k fichiers | 🧪 | 🔴 | ⏳ | 4h |
| TS-006 | Benchmark performance complet | 📊 | 🔴 | ⏳ | 6h |
| TS-007 | Tests regression suite | 🧪 | 🟠 | ⏳ | 4h |

### ✅ Validation Phase 1

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| V-101 | Review code complet | ✅ | 🔴 | ⏳ | 8h |
| V-102 | Validation perf (recherche <100ms) | ✅ | 🔴 | ⏳ | 2h |
| V-103 | Validation perf (indexation >500 files/min) | ✅ | 🔴 | ⏳ | 2h |
| V-104 | Validation UX (3 utilisateurs alpha) | ✅ | 🔴 | ⏳ | 4h |
| V-105 | Fix bugs critiques alpha | 💻 | 🔴 | ⏳ | 16h |
| V-106 | Approval lancement Phase 2 | ✅ | 🔴 | ⏳ | 1h |

---

## Phase 2 : OCR + Contenu (Semaines 9-12)

### 📄 Module Content Extractor (basique)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| CE-001 | Implémenter extraction TXT/MD/LOG | 💻 | 🔴 | ⏳ | 3h |
| CE-002 | Implémenter extraction PDF texte (pdf-extract) | 💻 | 🔴 | ⏳ | 5h |
| CE-003 | Implémenter extraction DOCX (docx-rs) | 💻 | 🔴 | ⏳ | 4h |
| CE-004 | Implémenter extraction XLSX (calamine) | 💻 | 🟠 | ⏳ | 4h |
| CE-005 | Implémenter fallback PDF (lopdf) | 💻 | 🟠 | ⏳ | 4h |
| CE-006 | Implémenter détection type fichier | 💻 | 🔴 | ⏳ | 3h |
| CE-007 | Gérer erreurs extraction (skip, log) | 💻 | 🔴 | ⏳ | 2h |
| CE-008 | Tests extraction formats | 🧪 | 🔴 | ⏳ | 5h |

### 🔍 Recherche full-text (SQLite FTS5)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| FTS-001 | Créer table content + content_fts | 💻 | 🔴 | ⏳ | 3h |
| FTS-002 | Implémenter triggers sync FTS | 💻 | 🔴 | ⏳ | 2h |
| FTS-003 | Implémenter stockage contenu extrait | 💻 | 🔴 | ⏳ | 3h |
| FTS-004 | Implémenter search_content command | 💻 | 🔴 | ⏳ | 4h |
| FTS-005 | Configurer tokenizer français | 🔧 | 🔴 | ⏳ | 2h |
| FTS-006 | Optimiser requêtes FTS5 | 🔧 | 🟠 | ⏳ | 3h |
| FTS-007 | Tests recherche full-text | 🧪 | 🔴 | ⏳ | 4h |
| FTS-008 | Benchmark perf FTS5 | 📊 | 🔴 | ⏳ | 2h |

### 👁️ Module OCR

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| OCR-001 | Intégrer leptess (Tesseract binding) | 💻 | 🔴 | ⏳ | 4h |
| OCR-002 | Télécharger tessdata fra+eng (best) | 🔧 | 🔴 | ⏳ | 1h |
| OCR-003 | Implémenter OcrEngine struct | 💻 | 🔴 | ⏳ | 4h |
| OCR-004 | Implémenter extract_text_from_image | 💻 | 🔴 | ⏳ | 5h |
| OCR-005 | Implémenter extract_from_pdf (pages) | 💻 | 🔴 | ⏳ | 6h |
| OCR-006 | Implémenter pdf_has_text_layer (détection) | 💻 | 🔴 | ⏳ | 3h |
| OCR-007 | Implémenter preprocessing (Leptonica) | 💻 | 🟠 | ⏳ | 6h |
| OCR-008 | Implémenter should_ocr (config rules) | 💻 | 🔴 | ⏳ | 3h |
| OCR-009 | Implémenter queue async OCR | 💻 | 🔴 | ⏳ | 5h |
| OCR-010 | Implémenter cache OCR (hash → skip si déjà fait) | 💻 | 🟠 | ⏳ | 4h |
| OCR-011 | Optimiser vitesse <5s/page | 🔧 | 🔴 | ⏳ | 6h |
| OCR-012 | Implémenter batch parallel OCR | 💻 | 🟠 | ⏳ | 5h |
| OCR-013 | Tests OCR précision (>95% FR) | 🧪 | 🔴 | ⏳ | 4h |
| OCR-014 | Benchmark vitesse OCR | 📊 | 🔴 | ⏳ | 2h |

### 🎨 Interface OCR

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| UI-OCR-001 | Créer OcrSettings component | 🎨 | 🔴 | ⏳ | 4h |
| UI-OCR-002 | Configurer langues OCR (UI) | 🎨 | 🔴 | ⏳ | 3h |
| UI-OCR-003 | Configurer types fichiers OCR | 🎨 | 🔴 | ⏳ | 3h |
| UI-OCR-004 | Configurer dossiers OCR | 🎨 | 🔴 | ⏳ | 3h |
| UI-OCR-005 | Afficher progression OCR distincte | 🎨 | 🟠 | ⏳ | 4h |
| UI-OCR-006 | Tests UI OCR config | 🧪 | 🔴 | ⏳ | 3h |

### 🔌 Tauri Commands OCR

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| T-OCR-001 | Implémenter command update_ocr_config | 💻 | 🔴 | ⏳ | 2h |
| T-OCR-002 | Implémenter command search_content | 💻 | 🔴 | ⏳ | 3h |
| T-OCR-003 | Ajouter phase OCR dans indexing_progress event | 💻 | 🔴 | ⏳ | 2h |

### 🧪 Tests Phase 2

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| TS-101 | Tests E2E : Config OCR → Indexation PDF scanné | 🧪 | 🔴 | ⏳ | 4h |
| TS-102 | Tests E2E : Recherche dans contenu OCR | 🧪 | 🔴 | ⏳ | 3h |
| TS-103 | Tests intégration : OCR + FTS5 | 🧪 | 🔴 | ⏳ | 4h |
| TS-104 | Tests perf : OCR 1000 pages | 🧪 | 🔴 | ⏳ | 3h |
| TS-105 | Validation précision OCR corpus admin | 🧪 | 🔴 | ⏳ | 6h |

### ✅ Validation Phase 2

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| V-201 | Review code OCR + extraction | ✅ | 🔴 | ⏳ | 6h |
| V-202 | Validation perf OCR (<5s/page) | ✅ | 🔴 | ⏳ | 2h |
| V-203 | Validation précision OCR (>95%) | ✅ | 🔴 | ⏳ | 3h |
| V-204 | Validation UX recherche contenu | ✅ | 🔴 | ⏳ | 3h |
| V-205 | Fix bugs Phase 2 | 💻 | 🔴 | ⏳ | 12h |
| V-206 | Approval lancement Phase 3 | ✅ | 🔴 | ⏳ | 1h |

---

## Phase 3 : IA Assist Me (Semaines 13-17)

### 🧪 POC LEANN (critique)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| POC-001 | Créer corpus test 10k documents | 🧪 | 🔴 | ⏳ | 4h |
| POC-002 | Implémenter wrapper LEANN basique | 💻 | 🔴 | ⏳ | 6h |
| POC-003 | Implémenter wrapper FAISS (comparaison) | 💻 | 🔴 | ⏳ | 4h |
| POC-004 | Benchmark taille index (LEANN vs FAISS) | 📊 | 🔴 | ⏳ | 3h |
| POC-005 | Benchmark vitesse recherche | 📊 | 🔴 | ⏳ | 3h |
| POC-006 | Benchmark recall@10 (précision) | 📊 | 🔴 | ⏳ | 4h |
| POC-007 | Rédiger rapport POC LEANN | 📝 | 🔴 | ⏳ | 4h |
| POC-008 | Décision finale : LEANN ou FAISS | ✅ | 🔴 | ⏳ | 2h |

### 🤖 Module AI Engine (Embeddings)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| AI-001 | Télécharger modèle all-MiniLM-L6-v2 | 🔧 | 🔴 | ⏳ | 1h |
| AI-002 | Intégrer Candle (ML framework Rust) | 💻 | 🔴 | ⏳ | 6h |
| AI-003 | Implémenter EmbeddingModel struct | 💻 | 🔴 | ⏳ | 8h |
| AI-004 | Implémenter encode (text → vector) | 💻 | 🔴 | ⏳ | 6h |
| AI-005 | Implémenter tokenize/detokenize | 💻 | 🔴 | ⏳ | 4h |
| AI-006 | Implémenter mean pooling | 💻 | 🔴 | ⏳ | 3h |
| AI-007 | Implémenter L2 normalization | 💻 | 🔴 | ⏳ | 2h |
| AI-008 | Optimiser vitesse <50ms/doc | 🔧 | 🔴 | ⏳ | 4h |
| AI-009 | Tests embeddings qualité | 🧪 | 🔴 | ⏳ | 4h |
| AI-010 | Benchmark génération embeddings | 📊 | 🔴 | ⏳ | 2h |

### 🗄️ Module Vector DB (LEANN ou FAISS)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| VDB-001 | Implémenter LeannIndex struct (si LEANN choisi) | 💻 | 🔴 | ⏳ | 8h |
| VDB-002 | Implémenter add_document (chunking) | 💻 | 🔴 | ⏳ | 6h |
| VDB-003 | Implémenter search (top-k similarité) | 💻 | 🔴 | ⏳ | 6h |
| VDB-004 | Implémenter split_into_chunks (500 tokens, overlap) | 💻 | 🔴 | ⏳ | 4h |
| VDB-005 | Implémenter persistence vector DB | 💻 | 🔴 | ⏳ | 5h |
| VDB-006 | Optimiser mémoire (memory-mapped) | 🔧 | 🟠 | ⏳ | 4h |
| VDB-007 | Tests vector search précision | 🧪 | 🔴 | ⏳ | 4h |
| VDB-008 | Benchmark vitesse recherche vecteurs | 📊 | 🔴 | ⏳ | 2h |

### 🔗 Intégration Embeddings dans Indexation

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| EMB-001 | Ajouter génération embeddings dans indexer | 💻 | 🔴 | ⏳ | 5h |
| EMB-002 | Implémenter batch embeddings (100 docs) | 💻 | 🟠 | ⏳ | 4h |
| EMB-003 | Stocker embeddings metadata DB (table embeddings) | 💻 | 🔴 | ⏳ | 3h |
| EMB-004 | Implémenter file_id → vector mapping | 💻 | 🔴 | ⏳ | 3h |
| EMB-005 | Gérer mise à jour embeddings (fichier modifié) | 💻 | 🔴 | ⏳ | 4h |
| EMB-006 | Gérer suppression embeddings (fichier supprimé) | 💻 | 🔴 | ⏳ | 2h |
| EMB-007 | Implémenter mode "embeddings uniquement" (réindexation) | 💻 | 🟠 | ⏳ | 4h |
| EMB-008 | Option : Activer/désactiver embeddings | 💻 | 🔴 | ⏳ | 2h |
| EMB-009 | Tests intégration embeddings + indexation | 🧪 | 🔴 | ⏳ | 5h |

### 🧠 Module Assist Me

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| AM-001 | Implémenter AssistMeEngine struct | 💻 | 🔴 | ⏳ | 4h |
| AM-002 | Implémenter answer_question (sans LLM) | 💻 | 🔴 | ⏳ | 6h |
| AM-003 | Implémenter récupération top-k sources | 💻 | 🔴 | ⏳ | 4h |
| AM-004 | Implémenter build_context (sources → texte) | 💻 | 🔴 | ⏳ | 3h |
| AM-005 | Implémenter format_sources_only (sans LLM) | 💻 | 🔴 | ⏳ | 3h |
| AM-006 | Implémenter Source struct (chemin, snippet, page) | 💻 | 🔴 | ⏳ | 2h |
| AM-007 | Gérer sources fichiers + emails (unifiées) | 💻 | 🔴 | ⏳ | 4h |
| AM-008 | Implémenter confidence scoring | 💻 | 🟠 | ⏳ | 3h |
| AM-009 | Tests Assist Me précision | 🧪 | 🔴 | ⏳ | 6h |
| AM-010 | Benchmark latency <3s | 📊 | 🔴 | ⏳ | 2h |

### 🎨 Interface Assist Me

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| UI-AM-001 | Créer ChatInterface component | 🎨 | 🔴 | ⏳ | 8h |
| UI-AM-002 | Créer SourceCard component (cliquable) | 🎨 | 🔴 | ⏳ | 5h |
| UI-AM-003 | Créer ResponseView component | 🎨 | 🔴 | ⏳ | 5h |
| UI-AM-004 | Implémenter historique questions | 🎨 | 🟠 | ⏳ | 4h |
| UI-AM-005 | Implémenter export résultats (MD/PDF) | 💻 | 🟠 | ⏳ | 5h |
| UI-AM-006 | Gérer liens cliquables (fichier, page, email) | 💻 | 🔴 | ⏳ | 4h |
| UI-AM-007 | Implémenter preview fichier au survol | 🎨 | 🟠 | ⏳ | 5h |
| UI-AM-008 | Gérer états loading/streaming | 🎨 | 🔴 | ⏳ | 3h |
| UI-AM-009 | Tests UI Assist Me | 🧪 | 🔴 | ⏳ | 5h |

### 🔌 Tauri Commands Assist Me

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| T-AM-001 | Implémenter command assist_me_query | 💻 | 🔴 | ⏳ | 4h |
| T-AM-002 | Implémenter command get_file_preview | 💻 | 🟠 | ⏳ | 3h |
| T-AM-003 | Implémenter command export_assist_results | 💻 | 🟠 | ⏳ | 3h |

### 🤖 LLM Local (optionnel)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| LLM-001 | Intégrer llama.cpp (binding Rust) | 💻 | 🟢 | ⏳ | 6h |
| LLM-002 | Télécharger Llama 3.2 1B GGUF | 🔧 | 🟢 | ⏳ | 1h |
| LLM-003 | Implémenter LocalLLM struct | 💻 | 🟢 | ⏳ | 5h |
| LLM-004 | Implémenter generate (prompt → réponse) | 💻 | 🟢 | ⏳ | 4h |
| LLM-005 | Créer prompt template RAG | 💻 | 🟢 | ⏳ | 3h |
| LLM-006 | Implémenter answer_question (avec LLM) | 💻 | 🟢 | ⏳ | 4h |
| LLM-007 | Option UI : Activer/désactiver LLM | 🎨 | 🟢 | ⏳ | 2h |
| LLM-008 | Tests qualité réponses LLM | 🧪 | 🟢 | ⏳ | 6h |
| LLM-009 | Benchmark latency avec LLM (<10s) | 📊 | 🟢 | ⏳ | 2h |

### 🧪 Tests Phase 3

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| TS-201 | Tests E2E : Assist Me question simple | 🧪 | 🔴 | ⏳ | 3h |
| TS-202 | Tests E2E : Clic source → ouvre fichier | 🧪 | 🔴 | ⏳ | 3h |
| TS-203 | Tests intégration : Embeddings + Vector search | 🧪 | 🔴 | ⏳ | 4h |
| TS-204 | Tests précision : 50 questions corpus admin | 🧪 | 🔴 | ⏳ | 8h |
| TS-205 | Tests perf : Génération 100k embeddings | 🧪 | 🔴 | ⏳ | 4h |
| TS-206 | Tests A/B : Avec vs sans LLM | 🧪 | 🟢 | ⏳ | 6h |

### ✅ Validation Phase 3

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| V-301 | Review code AI + embeddings | ✅ | 🔴 | ⏳ | 8h |
| V-302 | Validation POC LEANN décision finale | ✅ | 🔴 | ⏳ | 2h |
| V-303 | Validation perf recherche sémantique (<3s) | ✅ | 🔴 | ⏳ | 2h |
| V-304 | Validation précision Assist Me (>80% satisfaisant) | ✅ | 🔴 | ⏳ | 4h |
| V-305 | Validation UX Assist Me (5 utilisateurs) | ✅ | 🔴 | ⏳ | 4h |
| V-306 | Fix bugs Phase 3 | 💻 | 🔴 | ⏳ | 16h |
| V-307 | Approval lancement Phase 4 | ✅ | 🔴 | ⏳ | 1h |

---

## Phase 4 : Emails (Semaines 18-22)

### 📧 Module Email Parser (Outlook PST)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| EM-PST-001 | Rechercher/tester lib PST (libpff vs alternatives) | 🧪 | 🔴 | ⏳ | 6h |
| EM-PST-002 | Implémenter wrapper libpff (FFI si nécessaire) | 💻 | 🔴 | ⏳ | 12h |
| EM-PST-003 | Implémenter parse_pst (PST → Vec<Email>) | 💻 | 🔴 | ⏳ | 8h |
| EM-PST-004 | Extraire métadonnées email (from, to, subject, date) | 💻 | 🔴 | ⏳ | 4h |
| EM-PST-005 | Extraire body (text + html) | 💻 | 🔴 | ⏳ | 4h |
| EM-PST-006 | Extraire pièces jointes | 💻 | 🔴 | ⏳ | 5h |
| EM-PST-007 | Gérer threading conversations | 💻 | 🟠 | ⏳ | 6h |
| EM-PST-008 | Tests parsing PST corpus | 🧪 | 🔴 | ⏳ | 4h |

### 📧 Module Email Parser (MAPI Windows)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| EM-MAPI-001 | Implémenter read_outlook_mapi (Windows API) | 💻 | 🔴 | ⏳ | 10h |
| EM-MAPI-002 | Gérer profil Outlook actif | 💻 | 🔴 | ⏳ | 4h |
| EM-MAPI-003 | Extraire emails via MAPI | 💻 | 🔴 | ⏳ | 6h |
| EM-MAPI-004 | Extraire pièces jointes via MAPI | 💻 | 🔴 | ⏳ | 4h |
| EM-MAPI-005 | Gérer permissions MAPI | 💻 | 🔴 | ⏳ | 3h |
| EM-MAPI-006 | Tests MAPI avec Outlook installé | 🧪 | 🔴 | ⏳ | 4h |

### 📧 Module Email Parser (Thunderbird MBOX)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| EM-MBOX-001 | Implémenter parse_mbox (mailparse) | 💻 | 🔴 | ⏳ | 6h |
| EM-MBOX-002 | Détecter auto profil Thunderbird (AppData) | 💻 | 🔴 | ⏳ | 3h |
| EM-MBOX-003 | Parser emails MBOX format | 💻 | 🔴 | ⏳ | 5h |
| EM-MBOX-004 | Extraire pièces jointes MBOX | 💻 | 🔴 | ⏳ | 4h |
| EM-MBOX-005 | Gérer encodages email (UTF-8, ISO-8859-1) | 💻 | 🔴 | ⏳ | 3h |
| EM-MBOX-006 | Tests parsing MBOX corpus | 🧪 | 🔴 | ⏳ | 3h |

### 📧 Module Email Parser (IMAP)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| EM-IMAP-001 | Intégrer async-imap | 💻 | 🟠 | ⏳ | 4h |
| EM-IMAP-002 | Implémenter connexion IMAP | 💻 | 🟠 | ⏳ | 5h |
| EM-IMAP-003 | Implémenter fetch emails IMAP | 💻 | 🟠 | ⏳ | 6h |
| EM-IMAP-004 | Implémenter synchro incrémentale IMAP | 💻 | 🟠 | ⏳ | 6h |
| EM-IMAP-005 | Gérer authentification (OAuth2 Exchange) | 💻 | 🟠 | ⏳ | 8h |
| EM-IMAP-006 | Stocker credentials chiffrés (DPAPI Windows) | 💻 | 🟠 | ⏳ | 4h |
| EM-IMAP-007 | Tests IMAP avec serveur test | 🧪 | 🟠 | ⏳ | 4h |

### 💾 Database Emails

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| DB-EM-001 | Créer tables emails + attachments | 💻 | 🔴 | ⏳ | 3h |
| DB-EM-002 | Créer index emails (from, date, subject) | 💻 | 🔴 | ⏳ | 2h |
| DB-EM-003 | Créer FTS5 emails (body + subject) | 💻 | 🔴 | ⏳ | 3h |
| DB-EM-004 | Implémenter stockage pièces jointes (cache local) | 💻 | 🔴 | ⏳ | 4h |
| DB-EM-005 | Lier attachments → files (extraction contenu) | 💻 | 🔴 | ⏳ | 3h |
| DB-EM-006 | Tests DB emails | 🧪 | 🔴 | ⏳ | 3h |

### 🔍 Recherche Emails

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| SE-EM-001 | Implémenter search_emails command | 💻 | 🔴 | ⏳ | 5h |
| SE-EM-002 | Intégrer emails dans recherche sémantique | 💻 | 🔴 | ⏳ | 6h |
| SE-EM-003 | Implémenter filtres emails (date, from, to, folder) | 💻 | 🔴 | ⏳ | 4h |
| SE-EM-004 | Implémenter recherche pièces jointes | 💻 | 🔴 | ⏳ | 4h |
| SE-EM-005 | Implémenter ranking emails vs fichiers | 💻 | 🟠 | ⏳ | 3h |
| SE-EM-006 | Tests recherche emails | 🧪 | 🔴 | ⏳ | 4h |

### 🎨 Interface Emails

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| UI-EM-001 | Créer EmailSourceConfig component | 🎨 | 🔴 | ⏳ | 6h |
| UI-EM-002 | Créer EmailResult component | 🎨 | 🔴 | ⏳ | 5h |
| UI-EM-003 | Afficher emails dans SearchResults (mixte) | 🎨 | 🔴 | ⏳ | 4h |
| UI-EM-004 | Afficher pièces jointes email | 🎨 | 🔴 | ⏳ | 4h |
| UI-EM-005 | Lien "Ouvrir dans Outlook/Thunderbird" | 💻 | 🔴 | ⏳ | 5h |
| UI-EM-006 | Afficher threading conversations | 🎨 | 🟠 | ⏳ | 6h |
| UI-EM-007 | Tests UI emails | 🧪 | 🔴 | ⏳ | 4h |

### 🔌 Tauri Commands Emails

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| T-EM-001 | Implémenter command index_emails | 💻 | 🔴 | ⏳ | 4h |
| T-EM-002 | Implémenter command search_emails | 💻 | 🔴 | ⏳ | 3h |
| T-EM-003 | Implémenter command open_email | 💻 | 🔴 | ⏳ | 5h |
| T-EM-004 | Event email_indexing_progress | 💻 | 🔴 | ⏳ | 2h |

### 🧪 Tests Phase 4

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| TS-301 | Tests E2E : Config source email → Indexation | 🧪 | 🔴 | ⏳ | 4h |
| TS-302 | Tests E2E : Recherche email + fichier mixte | 🧪 | 🔴 | ⏳ | 3h |
| TS-303 | Tests E2E : Assist Me avec emails | 🧪 | 🔴 | ⏳ | 3h |
| TS-304 | Tests intégration : PST + MBOX + IMAP | 🧪 | 🔴 | ⏳ | 6h |
| TS-305 | Tests perf : Indexation 10k emails | 🧪 | 🔴 | ⏳ | 3h |
| TS-306 | Tests corpus : 50k emails réels | 🧪 | 🟠 | ⏳ | 6h |

### ✅ Validation Phase 4

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| V-401 | Review code emails parsing | ✅ | 🔴 | ⏳ | 8h |
| V-402 | Validation parsing PST/MBOX/IMAP | ✅ | 🔴 | ⏳ | 4h |
| V-403 | Validation recherche unifiée fichiers+emails | ✅ | 🔴 | ⏳ | 3h |
| V-404 | Validation UX emails (5 utilisateurs) | ✅ | 🔴 | ⏳ | 4h |
| V-405 | Fix bugs Phase 4 | 💻 | 🔴 | ⏳ | 16h |
| V-406 | Approval lancement Phase 5 | ✅ | 🔴 | ⏳ | 1h |

---

## Phase 5 : Production (Semaines 23-25)

### 🚀 Optimisation Performance

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| PERF-001 | Profiling complet (CPU, mémoire) | 📊 | 🔴 | ⏳ | 6h |
| PERF-002 | Optimiser hotspots identifiés | 💻 | 🔴 | ⏳ | 12h |
| PERF-003 | Réduire empreinte mémoire idle (<500MB) | 🔧 | 🔴 | ⏳ | 8h |
| PERF-004 | Optimiser démarrage app (<3s) | 🔧 | 🔴 | ⏳ | 6h |
| PERF-005 | Tests support 1M+ fichiers | 🧪 | 🔴 | ⏳ | 8h |
| PERF-006 | Optimiser UI (60fps, pas de freeze) | 🔧 | 🔴 | ⏳ | 6h |
| PERF-007 | Benchmark final complet | 📊 | 🔴 | ⏳ | 4h |

### 📦 Packaging & Distribution

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| PKG-001 | Configurer WiX (MSI installer) | 🔧 | 🔴 | ⏳ | 6h |
| PKG-002 | Créer icônes app (multi-résolutions) | 🎨 | 🔴 | ⏳ | 4h |
| PKG-003 | Configurer signing certificat Windows | 🔧 | 🔴 | ⏳ | 4h |
| PKG-004 | Implémenter silent install (GPO compatible) | 🔧 | 🔴 | ⏳ | 4h |
| PKG-005 | Tests installation/désinstallation | 🧪 | 🔴 | ⏳ | 4h |
| PKG-006 | Créer installateur offline (inclut tout) | 🔧 | 🔴 | ⏳ | 3h |
| PKG-007 | Créer installateur online (télécharge components) | 🔧 | 🟠 | ⏳ | 6h |

### 🔄 Auto-Update

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| UPD-001 | Configurer Tauri updater | 🔧 | 🔴 | ⏳ | 4h |
| UPD-002 | Setup serveur updates (CDN ou GitHub Releases) | 🔧 | 🔴 | ⏳ | 4h |
| UPD-003 | Générer keypair signing updates | 🔧 | 🔴 | ⏳ | 1h |
| UPD-004 | Implémenter check updates au démarrage | 💻 | 🔴 | ⏳ | 3h |
| UPD-005 | UI notification update disponible | 🎨 | 🔴 | ⏳ | 3h |
| UPD-006 | Tests auto-update flow | 🧪 | 🔴 | ⏳ | 4h |

### 📝 Documentation Utilisateur

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| DOC-USER-001 | Rédiger USER_GUIDE.md | 📝 | 🔴 | ⏳ | 8h |
| DOC-USER-002 | Créer vidéos tutoriels (3-5 min) | 📝 | 🟠 | ⏳ | 12h |
| DOC-USER-003 | Créer FAQ | 📝 | 🔴 | ⏳ | 4h |
| DOC-USER-004 | Créer TROUBLESHOOTING.md | 📝 | 🔴 | ⏳ | 6h |
| DOC-USER-005 | Screenshots interface (guide visuel) | 📝 | 🔴 | ⏳ | 4h |

### 📝 Documentation Admin IT

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| DOC-ADMIN-001 | Rédiger ADMIN_GUIDE.md | 📝 | 🔴 | ⏳ | 6h |
| DOC-ADMIN-002 | Documenter déploiement GPO | 📝 | 🔴 | ⏳ | 4h |
| DOC-ADMIN-003 | Documenter config enterprise | 📝 | 🔴 | ⏳ | 4h |
| DOC-ADMIN-004 | Créer template config.json administrations | 📝 | 🔴 | ⏳ | 3h |
| DOC-ADMIN-005 | Documenter prérequis système | 📝 | 🔴 | ⏳ | 2h |

### 📝 Documentation Projet

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| DOC-PROJ-001 | Créer CHANGELOG.md | 📝 | 🔴 | ⏳ | 3h |
| DOC-PROJ-002 | Finaliser README.md | 📝 | 🔴 | ⏳ | 2h |
| DOC-PROJ-003 | Rédiger RELEASE_NOTES v1.0 | 📝 | 🔴 | ⏳ | 3h |
| DOC-PROJ-004 | Créer site web projet (optionnel) | 📝 | 🟢 | ⏳ | 16h |

### 🧪 Tests Beta

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| BETA-001 | Recruter 20-50 beta testers (agents admin) | ✅ | 🔴 | ⏳ | 8h |
| BETA-002 | Préparer package beta + questionnaire | 🔧 | 🔴 | ⏳ | 4h |
| BETA-003 | Lancer beta 4 semaines | ✅ | 🔴 | ⏳ | - |
| BETA-004 | Collecter feedback hebdomadaire | 📊 | 🔴 | ⏳ | 8h |
| BETA-005 | Analyser bugs/suggestions beta | 📊 | 🔴 | ⏳ | 6h |
| BETA-006 | Implémenter fixes critiques beta | 💻 | 🔴 | ⏳ | 24h |
| BETA-007 | Release candidate (RC1) | 🚀 | 🔴 | ⏳ | - |
| BETA-008 | Tests finaux RC1 | 🧪 | 🔴 | ⏳ | 8h |

### 🚀 Release v1.0

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| REL-001 | Freeze code (code freeze) | ✅ | 🔴 | ⏳ | - |
| REL-002 | Tests régression complets | 🧪 | 🔴 | ⏳ | 12h |
| REL-003 | Build final MSI | 🚀 | 🔴 | ⏳ | 2h |
| REL-004 | Signer MSI | 🚀 | 🔴 | ⏳ | 1h |
| REL-005 | Upload GitHub Release | 🚀 | 🔴 | ⏳ | 1h |
| REL-006 | Activer auto-update endpoint | 🚀 | 🔴 | ⏳ | 1h |
| REL-007 | Annonce release (blog, réseaux) | 📝 | 🟠 | ⏳ | 4h |
| REL-008 | Monitoring crashes post-release | 📊 | 🔴 | ⏳ | - |

### ✅ Validation Phase 5

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| V-501 | Review final code complet | ✅ | 🔴 | ⏳ | 12h |
| V-502 | Validation perf finale (tous benchmarks) | ✅ | 🔴 | ⏳ | 6h |
| V-503 | Validation sécurité audit | ✅ | 🔴 | ⏳ | 8h |
| V-504 | Validation UX finale (10 utilisateurs) | ✅ | 🔴 | ⏳ | 8h |
| V-505 | Approval release v1.0 | ✅ | 🔴 | ⏳ | 2h |

---

## Tâches transverses

### 🔧 Maintenance Index (IMPORTANT)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| MNT-001 | Implémenter nettoyage index (fichiers supprimés) | 💻 | 🔴 | ⏳ | 5h |
| MNT-002 | Implémenter réindexation fichier modifié | 💻 | 🔴 | ⏳ | 4h |
| MNT-003 | Implémenter mise à jour embeddings (fichier changé) | 💻 | 🔴 | ⏳ | 5h |
| MNT-004 | Implémenter suppression embeddings orphelins | 💻 | 🔴 | ⏳ | 3h |
| MNT-005 | Commande "Nettoyer index" (UI) | 🎨 | 🔴 | ⏳ | 3h |
| MNT-006 | Commande "Réindexer tout" (UI) | 🎨 | 🔴 | ⏳ | 3h |
| MNT-007 | Commande "Optimiser base de données" (VACUUM) | 💻 | 🟠 | ⏳ | 2h |
| MNT-008 | Détection corruption index (vérification intégrité) | 💻 | 🟠 | ⏳ | 4h |
| MNT-009 | Réparation auto index corrompu | 💻 | 🟠 | ⏳ | 5h |
| MNT-010 | Statistiques index (nb fichiers, taille, dernière màj) | 💻 | 🟠 | ⏳ | 3h |

### 🕐 Planificateur tâches (NOUVEAU)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| SCHED-001 | Implémenter TaskScheduler struct | 💻 | 🔴 | ⏳ | 4h |
| SCHED-002 | Implémenter planification heure fixe (ex: 2h du matin) | 💻 | 🔴 | ⏳ | 5h |
| SCHED-003 | Implémenter mode "sleep" en tray | 💻 | 🔴 | ⏳ | 4h |
| SCHED-004 | Vérifier machine allumée à l'heure prévue | 💻 | 🔴 | ⏳ | 3h |
| SCHED-005 | Si éteinte : indexer au prochain démarrage | 💻 | 🔴 | ⏳ | 3h |
| SCHED-006 | Pause auto si utilisateur actif | 💻 | 🟠 | ⏳ | 4h |
| SCHED-007 | UI : Configurer horaire indexation | 🎨 | 🔴 | ⏳ | 4h |
| SCHED-008 | UI : Activer/désactiver indexation auto | 🎨 | 🔴 | ⏳ | 2h |
| SCHED-009 | Notification tray : "Indexation démarrera à 2h" | 💻 | 🟠 | ⏳ | 2h |
| SCHED-010 | Tests planificateur | 🧪 | 🔴 | ⏳ | 4h |

### 📊 Telemetry & Analytics

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| TEL-001 | Implémenter LocalTelemetry (privacy-first) | 💻 | 🟠 | ⏳ | 4h |
| TEL-002 | Collecter métriques usage (recherches, indexation) | 💻 | 🟠 | ⏳ | 3h |
| TEL-003 | Implémenter export telemetry (support) | 💻 | 🟠 | ⏳ | 2h |
| TEL-004 | UI : Opt-in telemetry | 🎨 | 🟠 | ⏳ | 2h |
| TEL-005 | UI : Voir données collectées | 🎨 | 🟠 | ⏳ | 3h |
| TEL-006 | Dashboard stats local (optionnel) | 🎨 | 🟢 | ⏳ | 8h |

### 🛡️ Sécurité

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| SEC-001 | Audit sécurité code (cargo-audit) | 🧪 | 🔴 | ⏳ | 4h |
| SEC-002 | Implémenter chiffrement DB (optionnel SQLCipher) | 💻 | 🟠 | ⏳ | 8h |
| SEC-003 | Chiffrement credentials emails (DPAPI Windows) | 💻 | 🔴 | ⏳ | 4h |
| SEC-004 | Validation input (chemins, queries) | 💻 | 🔴 | ⏳ | 4h |
| SEC-005 | Sandboxing Tauri frontend | 🔧 | 🔴 | ⏳ | 2h |
| SEC-006 | Logs accès fichiers sensibles | 💻 | 🟠 | ⏳ | 3h |
| SEC-007 | Respect ACL Windows | 💻 | 🔴 | ⏳ | 4h |
| SEC-008 | Tests sécurité (penetration testing) | 🧪 | 🟠 | ⏳ | 12h |

### 🌐 Internationalisation (i18n)

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| I18N-001 | Setup i18n framework (frontend) | 🔧 | 🟢 | ⏳ | 4h |
| I18N-002 | Extraire strings français → fichiers traduction | 📝 | 🟢 | ⏳ | 6h |
| I18N-003 | Traduction anglais | 📝 | 🟢 | ⏳ | 8h |
| I18N-004 | Tests UI multilingue | 🧪 | 🟢 | ⏳ | 3h |

---

## Backlog futur (Post v1.0)

### 🔵 Fonctionnalités avancées

| # | Tâche | Type | Priorité | Status | Durée |
|---|-------|------|----------|--------|-------|
| FUT-001 | Support réseau partagé (`\\Serveur\`) | 💻 | 🔵 | ⏳ | 24h |
| FUT-002 | Synchronisation index multi-PC | 💻 | 🔵 | ⏳ | 40h |
| FUT-003 | Application mobile compagnon (iOS/Android) | 💻 | 🔵 | ⏳ | 200h |
| FUT-004 | Plugin Outlook (recherche intégrée) | 💻 | 🔵 | ⏳ | 60h |
| FUT-005 | Support macOS (Tauri cross-platform) | 💻 | 🔵 | ⏳ | 80h |
| FUT-006 | Support Linux | 💻 | 🔵 | ⏳ | 60h |
| FUT-007 | OCR Azure premium (optionnel payant) | 💻 | 🔵 | ⏳ | 24h |
| FUT-008 | Support GPU CUDA (embeddings 10x plus rapide) | 💻 | 🔵 | ⏳ | 40h |
| FUT-009 | Recherche vocale (speech-to-text) | 💻 | 🔵 | ⏳ | 60h |
| FUT-010 | Export/import index (backup/restore) | 💻 | 🔵 | ⏳ | 16h |
| FUT-011 | API REST (intégration tierce) | 💻 | 🔵 | ⏳ | 40h |
| FUT-012 | Plugins système (extensibilité) | 💻 | 🔵 | ⏳ | 80h |

---

## Récapitulatif par phase

| Phase | Tâches | Priorité 🔴 | Priorité 🟠 | Priorité 🟢 | Durée estimée |
|-------|--------|-------------|-------------|-------------|---------------|
| **Phase 0** | 15 | 9 | 6 | 0 | ~2 semaines |
| **Phase 1** | 85 | 65 | 18 | 2 | ~6 semaines |
| **Phase 2** | 35 | 28 | 6 | 1 | ~4 semaines |
| **Phase 3** | 55 | 38 | 8 | 9 | ~5 semaines |
| **Phase 4** | 50 | 40 | 8 | 2 | ~5 semaines |
| **Phase 5** | 45 | 38 | 5 | 2 | ~3 semaines |
| **Transverses** | 40 | 25 | 13 | 2 | ~4 semaines |
| **TOTAL** | **325** | **243** | **64** | **18** | **~29 semaines** |

---

## Prochaines actions immédiates

### Semaine 1-2 (Documentation - EN COURS)
- [x] Créer tous les documents specs
- [ ] Validation documentation avec équipe
- [ ] Approval budget/ressources

### Semaine 3 (Setup + POC)
- [ ] Setup projet Tauri + React
- [ ] CI/CD GitHub Actions
- [ ] **POC LEANN** (critique)
- [ ] Premières structures Rust

### Semaine 4 (MVP Core)
- [ ] Watchdog basique fonctionnel
- [ ] Indexation métadonnées
- [ ] Database SQLite
- [ ] Recherche basique Tantivy

---

**Document version :** 1.0
**Dernière mise à jour :** 2025-11-12
**Total tâches :** 325
**Durée totale estimée :** ~29 semaines (7 mois)
