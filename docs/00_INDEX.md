# xfinder - Index Documentation

**Documentation complète du projet xfinder**

---

## Navigation rapide

| Document | Contenu | Audience | Statut |
|----------|---------|----------|--------|
| **[README.md](../README.md)** | Vue d'ensemble projet | Tous | ✅ À jour |
| **[QUICKSTART.md](../QUICKSTART.md)** | Démarrage rapide développeur | Dev | ✅ À jour |
| **[GIT_WORKFLOW.md](../GIT_WORKFLOW.md)** | Guide Git commit réguliers | Dev | ✅ À jour |
| **[01_PRD](01_PRD_Product_Requirements.md)** | Spécifications produit | PM, Dev, Sponsors | ✅ À jour |
| **[03_Decisions](03_Decisions_Techniques.md)** | Choix techniques | Dev, Architects | ⚠️ Voir note |
| **[05_Tests](05_Plan_Tests_Metriques.md)** | Tests & métriques | QA, Dev | ✅ À jour |
| **[06_Backlog](06_Backlog_Complet.md)** | 325 tâches détaillées | PM, Dev | ⚠️ Voir note |
| **[07_Securite](07_Architecture_Securite.md)** | Modèle menaces & sécurité | Dev, Security | ✅ À jour |
| **[08_Architecture_egui](08_Architecture_Finale_egui.md)** | **Architecture FINALE** | Dev | ✅ **RÉFÉRENCE** |

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

## Par rôle

### Développeur (toi)

**Commence par :**
1. [QUICKSTART.md](../QUICKSTART.md) - Démarrage zéro
2. [08_Architecture_egui](08_Architecture_Finale_egui.md) - Structure code
3. [GIT_WORKFLOW.md](../GIT_WORKFLOW.md) - Commit réguliers

**Référence :**
- [01_PRD](01_PRD_Product_Requirements.md) - Features à implémenter
- [06_Backlog](06_Backlog_Complet.md) - Tâches détaillées
- [07_Securite](07_Architecture_Securite.md) - Best practices sécurité

---

## Par phase projet

### Phase actuelle : Setup (Semaine 1)

**Docs à lire :**
- ✅ [QUICKSTART.md](../QUICKSTART.md)
- ✅ [08_Architecture_egui](08_Architecture_Finale_egui.md)
- ✅ [GIT_WORKFLOW.md](../GIT_WORKFLOW.md)

**Action :**
```bash
# 1. Hello World egui
cargo run

# 2. Commit
git add .
git commit -m "feat: hello world egui fonctionne"
```

---

### Phase 1 : MVP (Semaines 1-4)

**Focus :** Recherche basique Tantivy + egui

**Docs utiles :**
- [08_Architecture_egui](08_Architecture_Finale_egui.md) - Modules search/
- [06_Backlog](06_Backlog_Complet.md) - Tâches Semaine 1-4

---

### Phases 2-5 : Features avancées

**Référence :**
- [01_PRD](01_PRD_Product_Requirements.md) - Specs complètes
- [06_Backlog](06_Backlog_Complet.md) - Roadmap 25 semaines

---

## Structure projet finale

```
xfinder/
├── README.md                    ✅ Vue d'ensemble
├── QUICKSTART.md                ✅ Démarrage rapide
├── GIT_WORKFLOW.md              ✅ Guide Git
├── Cargo.toml                   ⏭️ À créer
│
├── src/
│   ├── main.rs                  ⏭️ Hello World egui
│   ├── app.rs                   ⏭️ État app
│   ├── ui/                      ⏭️ Interface egui
│   ├── search/                  ⏭️ Tantivy
│   ├── database/                ⏭️ SQLite
│   └── ...
│
└── docs/
    ├── 00_INDEX.md              ✅ Ce fichier
    ├── 01_PRD...                ✅ Specs
    ├── 03_Decisions...          ⚠️ Note egui
    ├── 05_Tests...              ✅ Tests
    ├── 06_Backlog...            ⚠️ Note tâches egui
    ├── 07_Securite...           ✅ Sécurité
    └── 08_Architecture_egui...  ✅ RÉFÉRENCE FINALE
```

---

## Stack technique FINALE

| Composant | Technologie | Justification |
|-----------|-------------|---------------|
| **UI** | **egui** | Natif, rapide, Rust pur |
| **Windowing** | winit | Inclus avec egui |
| **Rendering** | wgpu | GPU-accelerated |
| **Search** | Tantivy | Prouvé (spotlight_windows) |
| **Database** | SQLite | Léger, fiable |
| **OCR** | Tesseract | Best-in-class offline |
| **IA** | Candle + LEANN | Rust ML |
| **Email** | libpff + mailparse | PST + MBOX |

**Taille : ~8MB base + 110MB (OCR+IA) = 118MB**

---

## Glossaire

| Terme | Définition |
|-------|------------|
| **egui** | Framework UI natif Rust immediate mode |
| **Tantivy** | Moteur recherche full-text (Lucene-like) |
| **LEANN** | Vector DB ultra-compact (97% réduction) |
| **Watchdog** | Surveillance filesystem temps réel |
| **OCR** | Optical Character Recognition |
| **Embedding** | Représentation vectorielle texte (IA) |

---

## Changelog documentation

| Date | Version | Changements |
|------|---------|-------------|
| 2025-11-12 | 1.0 | Documentation complète initiale |
| 2025-11-12 | 1.1 | **Migration Tauri → egui** (décision finale) |

---

## Prochaines étapes

### Documentation
1. ✅ Index mis à jour (ce fichier)
2. ⏭️ Mise à jour 03_Decisions (note egui)
3. ⏭️ Mise à jour 06_Backlog (tâches egui)

### Projet
1. ⏭️ Hello World egui (QUICKSTART.md)
2. ⏭️ Tantivy recherche basique (Semaine 1)
3. ⏭️ SQLite + métadonnées (Semaine 2)

---

**Index version :** 1.1
**Dernière mise à jour :** 2025-11-12
**Architecture actuelle :** egui natif Rust
