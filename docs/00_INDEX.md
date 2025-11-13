# xfinder - Index Documentation

**Documentation complète du projet xfinder**

---

## Navigation rapide

| Document | Contenu | Audience | Statut |
|----------|---------|----------|--------|
| **[README.md](../README.md)** | Vue d'ensemble projet | Tous | ✅ À jour (2025-11-13) |
| **[LICENSE](../LICENSE)** | Licence MIT | Tous | ✅ À jour |
| **[QUICKSTART.md](../QUICKSTART.md)** | Démarrage rapide développeur | Dev | ⚠️ Mise à jour recommandée |
| **[GIT_WORKFLOW.md](../GIT_WORKFLOW.md)** | Guide Git commit réguliers | Dev | ✅ À jour |
| **[01_PRD](01_PRD_Product_Requirements.md)** | Spécifications produit | PM, Dev, Sponsors | ✅ À jour |
| **[03_Decisions](03_Decisions_Techniques.md)** | Choix techniques | Dev, Architects | ✅ À jour (egui) |
| **[05_Tests](05_Plan_Tests_Metriques.md)** | Tests & métriques | QA, Dev | ⚠️ Ajouter tests SQLite |
| **[06_Backlog](06_Backlog_Complet.md)** | 325 tâches détaillées | PM, Dev | ⚠️ Mettre à jour progression |
| **[07_Securite](07_Architecture_Securite.md)** | Modèle menaces & sécurité | Dev, Security | ✅ À jour |
| **[08_Architecture_egui](08_Architecture_Finale_egui.md)** | **Architecture FINALE** | Dev | ⚠️ Ajouter SQLite + Config |

**Total documentation active : ~200 pages**

---

## ⚠️ Note importante

### Architecture finale : **egui** (pas Tauri)

**Décision :** Application native Rust pure avec egui (comme spotlight_windows)

**Documents obsolètes (archive uniquement) :**
- ~~02_Architecture_Technique.md~~ (version Tauri)
- ~~04_API_Schemas.md~~ (Tauri IPC - pas applicable egui)

**→ Référence actuelle : `08_Architecture_Finale_egui.md`**

---

## État actuel du projet

### Phase 1 : Core Search - EN COURS ✨

**Version actuelle :** 0.1.0
**Dernière mise à jour :** 2025-11-13

#### ✅ Fonctionnalités implémentées

**Recherche & Indexation:**
- ✅ Tantivy full-text search (n-grams 2-20)
- ✅ Indexation >10,000 fichiers/sec
- ✅ FileScanner avec exclusions
- ✅ Recherche instantanée (<100ms)
- ✅ Filtres avancés (type, date, taille)
- ✅ Tri multi-critères

**SQLite Integration:**
- ✅ Base de données embarquée (WAL mode)
- ✅ Tables: files, search_history, error_log, config
- ✅ Batch inserts (1000 fichiers/transaction)
- ✅ Synchronisation Tantivy ↔ SQLite
- ✅ Modal statistiques (total, par extension, top searches)

**Surveillance Temps Réel:**
- ✅ Watchdog avec notify-rs
- ✅ Sync automatique Tantivy + SQLite
- ✅ Gestion Created, Modified, Removed, Renamed
- ✅ Respect des exclusions en temps réel

**Configuration:**
- ✅ Persistance TOML (~/.xfinder_index/config.toml)
- ✅ Auto-save sur tous changements
- ✅ Defaults intelligents
- ✅ Sections: scan_paths, exclusions, indexing, ui

**Interface:**
- ✅ UI egui complète
- ✅ Sidebar avec contrôles
- ✅ Top panel avec actions
- ✅ Prévisualisation (texte, images, audio, PDF)
- ✅ Modal Paramètres (onglets Exclusions/Général)
- ✅ Modal Statistiques

#### 🔨 En développement

**Phase 1 - Reste à faire:**
- ⏳ System Tray (icône système + auto-start)
- ⏳ Scheduler (indexation planifiée 2h AM)

**Phase 2+ (Futur):**
- 📋 OCR (Tesseract)
- 📋 Semantic Search (embeddings + LEANN)
- 📋 Email Integration (PST/MBOX)

---

## Par rôle

### Développeur

**Commence par :**
1. [README.md](../README.md) - Vue d'ensemble
2. [08_Architecture_egui](08_Architecture_Finale_egui.md) - Architecture détaillée
3. [GIT_WORKFLOW.md](../GIT_WORKFLOW.md) - Conventions commit

**Référence technique :**
- [01_PRD](01_PRD_Product_Requirements.md) - Features complètes
- [03_Decisions](03_Decisions_Techniques.md) - Choix tech
- [07_Securite](07_Architecture_Securite.md) - Best practices

**Développement :**
```bash
# Clone et build
git clone https://github.com/fless-lab/xfinder.git
cd xfinder
cargo build --release

# Lancer
cargo run --release
```

---

## Structure projet actuelle

```
xfinder/
├── README.md                    ✅ Vue d'ensemble
├── LICENSE                      ✅ MIT License
├── Cargo.toml                   ✅ Dépendances
│
├── src/
│   ├── main.rs                  ✅ Entry point
│   ├── app.rs                   ✅ État application
│   │
│   ├── ui/                      ✅ Interface egui
│   │   ├── mod.rs
│   │   ├── main_ui.rs           # Recherche & résultats
│   │   ├── side_panel.rs        # Contrôles latéraux
│   │   ├── top_panel.rs         # Actions principales
│   │   ├── preview_panel.rs     # Prévisualisation fichiers
│   │   ├── settings_modal.rs    # Paramètres (onglets)
│   │   ├── statistics_modal.rs  # Stats SQLite
│   │   └── icons.rs             # Icônes SVG
│   │
│   ├── search/                  ✅ Moteur recherche
│   │   ├── mod.rs
│   │   ├── tantivy_index.rs     # Index Tantivy
│   │   ├── scanner.rs           # Scan filesystem
│   │   └── file_watcher.rs      # Watchdog temps réel
│   │
│   ├── database/                ✅ SQLite
│   │   ├── mod.rs               # API publique
│   │   ├── schema.rs            # DDL + PRAGMAs
│   │   └── queries.rs           # CRUD operations
│   │
│   ├── config/                  ✅ Configuration
│   │   └── mod.rs               # TOML persistence
│   │
│   └── audio_player.rs          ✅ Prévisualisation audio
│
├── docs/                        ✅ Documentation
│   ├── 00_INDEX.md              ✅ Ce fichier
│   ├── 01_PRD...                ✅ Specs produit
│   ├── 03_Decisions...          ✅ Choix techniques
│   ├── 05_Tests...              ⚠️ Ajouter tests SQLite
│   ├── 06_Backlog...            ⚠️ Progression Phase 1
│   ├── 07_Securite...           ✅ Sécurité
│   └── 08_Architecture_egui...  ⚠️ Ajouter SQLite/Config
│
└── .xfinder_index/              ⏭️ Créé au runtime
    ├── tantivy_index/           # Index Tantivy
    ├── xfinder.db               # Base SQLite
    └── config.toml              # Configuration
```

---

## Stack technique actuelle

| Composant | Technologie | Version | Statut |
|-----------|-------------|---------|--------|
| **Language** | Rust | 1.70+ | ✅ |
| **UI** | egui | 0.27 | ✅ |
| **Rendering** | wgpu | (via eframe) | ✅ |
| **Windowing** | winit | (via eframe) | ✅ |
| **Search** | Tantivy | Latest | ✅ |
| **Database** | SQLite | 3.x (rusqlite) | ✅ |
| **Config** | TOML | 0.8 (serde) | ✅ |
| **File Watch** | notify-rs | Latest | ✅ |
| **Audio** | rodio | Latest | ✅ |
| **OCR** | Tesseract | - | ⏳ Phase 2 |
| **Embeddings** | Candle + LEANN | - | ⏳ Phase 3 |
| **Email** | libpff + mailparse | - | ⏳ Phase 4 |

**Taille binaire actuelle :** ~8MB (release mode)

---

## Performance mesurée

| Métrique | Cible | Actuel | Status |
|----------|-------|--------|--------|
| Indexation (SSD) | >1,000 files/min | >10,000 files/sec | ✅ 600x |
| Recherche (100k files) | <100ms | <100ms | ✅ |
| Mémoire (idle) | <100MB | ~50MB | ✅ |
| Démarrage | <500ms | <500ms | ✅ |
| SQLite batch insert | - | 1000 files/tx | ✅ |

---

## Glossaire

| Terme | Définition |
|-------|------------|
| **egui** | Framework UI natif Rust immediate mode |
| **Tantivy** | Moteur recherche full-text (Lucene-like) |
| **SQLite** | Base de données embarquée ACID |
| **WAL** | Write-Ahead Logging (mode SQLite non-bloquant) |
| **Watchdog** | Surveillance filesystem temps réel |
| **N-grams** | Tokenisation par sous-chaînes (2-20 chars) |
| **TOML** | Format config lisible (Tom's Obvious Minimal Language) |

---

## Changelog documentation

| Date | Version | Changements |
|------|---------|-------------|
| 2025-11-12 | 1.0 | Documentation complète initiale |
| 2025-11-12 | 1.1 | **Migration Tauri → egui** (décision finale) |
| 2025-11-13 | 1.2 | **Mise à jour Phase 1** - SQLite, Config, Stats |

---

## Prochaines étapes

### Documentation
1. ⏳ Mettre à jour 08_Architecture_egui (SQLite + Config)
2. ⏳ Mettre à jour 06_Backlog (progression Phase 1)
3. ⏳ Mettre à jour 05_Tests (tests SQLite)

### Projet (Phase 1 - Fin)
1. ⏳ System Tray (icône + auto-start Windows)
2. ⏳ Scheduler (indexation planifiée)
3. ✅ Tests end-to-end Phase 1

### Phase 2 (OCR)
1. ⏭️ Tesseract integration
2. ⏭️ Scanned PDF detection
3. ⏭️ Full-text content search

---

**Index version :** 1.2
**Dernière mise à jour :** 2025-11-13
**Architecture actuelle :** egui natif Rust
**Phase actuelle :** Phase 1 - Core Search (80% complété)
