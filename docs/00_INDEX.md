# xfinder - Index Documentation

**Documentation complète du projet xfinder**

---

## Navigation rapide

| Document | Contenu | Audience | Pages |
|----------|---------|----------|-------|
| **[README.md](../README.md)** | Vue d'ensemble projet | Tous | 4 |
| **[01_PRD](01_PRD_Product_Requirements.md)** | Spécifications produit | PM, Dev, Sponsors | 35 |
| **[02_Architecture](02_Architecture_Technique.md)** | Architecture technique | Dev, Tech Lead | 45 |
| **[03_Decisions](03_Decisions_Techniques.md)** | Choix techniques | Dev, Architects | 30 |
| **[04_API](04_API_Schemas.md)** | API & schémas données | Dev Frontend/Backend | 40 |
| **[05_Tests](05_Plan_Tests_Metriques.md)** | Tests & métriques | QA, Dev | 25 |

**Total :** ~180 pages de documentation

---

## Par rôle

### Product Manager / Sponsors
**À lire en priorité :**
1. [README.md](../README.md) - Vision globale
2. [01_PRD](01_PRD_Product_Requirements.md) - Features détaillées
   - Executive Summary
   - Problèmes utilisateurs
   - Roadmap
   - Personas
   - Métriques succès

**Optionnel :**
- [03_Decisions](03_Decisions_Techniques.md) - Comprendre choix stack

---

### Développeur Backend (Rust)
**À lire en priorité :**
1. [02_Architecture](02_Architecture_Technique.md) - Architecture complète
   - Modules Rust
   - Watchdog
   - Indexer
   - Content Extractor (OCR)
   - AI Engine (LEANN)
   - Email Parser
2. [04_API](04_API_Schemas.md) - Schémas DB + Tauri commands
3. [05_Tests](05_Plan_Tests_Metriques.md) - Tests unitaires + benchmarks

**Référence :**
- [03_Decisions](03_Decisions_Techniques.md) - Pourquoi Tauri, Tesseract, etc.

---

### Développeur Frontend (React)
**À lire en priorité :**
1. [04_API](04_API_Schemas.md) - Tauri IPC + Types TypeScript
   - Search API
   - Config API
   - Events
2. [02_Architecture](02_Architecture_Technique.md) - Structure frontend
3. [05_Tests](05_Plan_Tests_Metriques.md) - Tests composants

**Référence :**
- [01_PRD](01_PRD_Product_Requirements.md) - Mockups interface

---

### QA / Testeur
**À lire en priorité :**
1. [05_Tests](05_Plan_Tests_Metriques.md) - Plan complet
   - Tests E2E
   - Tests performance
   - Tests utilisateurs
   - Benchmarks
2. [01_PRD](01_PRD_Product_Requirements.md) - Features à valider

---

### DevOps / IT
**À lire en priorité :**
1. [02_Architecture](02_Architecture_Technique.md)
   - Déploiement
   - Configuration
2. [03_Decisions](03_Decisions_Techniques.md) - MSI vs MSIX
3. [01_PRD](01_PRD_Product_Requirements.md) - Contraintes système

---

## Par phase projet

### Phase 0 : Documentation ✅ (Actuelle)
**Docs créés :**
- ✅ README.md
- ✅ 01_PRD_Product_Requirements.md
- ✅ 02_Architecture_Technique.md
- ✅ 03_Decisions_Techniques.md
- ✅ 04_API_Schemas.md
- ✅ 05_Plan_Tests_Metriques.md

**Prochaine étape :** POC LEANN (semaine 3)

---

### Phase 1 : MVP Indexation (Semaines 3-8)
**Docs à consulter :**
- 02_Architecture - Modules Watchdog, Indexer
- 04_API - Schéma DB files
- 05_Tests - Tests unitaires indexation

**Docs à créer :**
- [ ] CONTRIBUTING.md - Guide contribution
- [ ] SETUP.md - Setup environnement dev

---

### Phase 2 : OCR (Semaines 9-12)
**Docs à consulter :**
- 02_Architecture - Module Content Extractor
- 03_Decisions - Choix Tesseract
- 05_Tests - Benchmarks OCR

**Docs à créer :**
- [ ] OCR_GUIDE.md - Guide configuration OCR

---

### Phase 3 : IA Assist Me (Semaines 13-17)
**Docs à consulter :**
- 02_Architecture - Module AI Engine
- 03_Decisions - LEANN vs FAISS
- 04_API - AssistMe API

**Docs à créer :**
- [ ] LEANN_POC.md - Résultats POC
- [ ] AI_TUNING.md - Guide tuning modèle

---

### Phase 4 : Emails (Semaines 18-22)
**Docs à consulter :**
- 02_Architecture - Module Email Parser
- 03_Decisions - PST strategy
- 04_API - Email schemas

**Docs à créer :**
- [ ] EMAIL_SUPPORT.md - Formats supportés

---

### Phase 5 : Production (Semaines 23-25)
**Docs à créer :**
- [ ] USER_GUIDE.md - Guide utilisateur
- [ ] ADMIN_GUIDE.md - Guide administrateur IT
- [ ] TROUBLESHOOTING.md - Guide dépannage
- [ ] CHANGELOG.md - Historique versions

---

## Structure complète projet (future)

```
xfinder/
├── README.md                          # ← Entrée principale
├── CHANGELOG.md                        # Historique versions
├── LICENSE                             # Licence
│
├── docs/                               # ← Documentation
│   ├── 00_INDEX.md                    # ← Ce fichier
│   ├── 01_PRD_Product_Requirements.md
│   ├── 02_Architecture_Technique.md
│   ├── 03_Decisions_Techniques.md
│   ├── 04_API_Schemas.md
│   ├── 05_Plan_Tests_Metriques.md
│   ├── CONTRIBUTING.md                 # Guide contribution
│   ├── SETUP.md                        # Setup dev
│   ├── USER_GUIDE.md                   # Guide utilisateur
│   └── ADMIN_GUIDE.md                  # Guide admin IT
│
├── src-tauri/                          # Backend Rust
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs
│   │   └── modules/
│   │       ├── watchdog/
│   │       ├── indexer/
│   │       ├── content_extractor/
│   │       ├── search_engine/
│   │       ├── ai_engine/
│   │       ├── email_parser/
│   │       └── database/
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src-ui/                             # Frontend React
│   ├── src/
│   │   ├── components/
│   │   ├── hooks/
│   │   ├── api/
│   │   └── types/
│   ├── package.json
│   └── vite.config.ts
│
├── tests/                              # Tests
│   ├── integration/
│   ├── e2e/
│   ├── fixtures/
│   └── benchmarks/
│
├── scripts/                            # Scripts utilitaires
│   ├── generate_corpus.py
│   ├── benchmark.sh
│   └── release.sh
│
└── .github/
    └── workflows/
        ├── ci.yml
        ├── release.yml
        └── benchmarks.yml
```

---

## Glossaire

| Terme | Définition |
|-------|------------|
| **Watchdog** | Surveillance temps réel filesystem |
| **Embedding** | Représentation vectorielle texte (pour IA) |
| **LEANN** | Vector DB ultra-compact (97% réduction) |
| **OCR** | Optical Character Recognition (reconnaissance texte images) |
| **FTS5** | SQLite Full-Text Search extension |
| **Tantivy** | Moteur recherche Rust (like Lucene) |
| **PST** | Format fichier Outlook (.pst) |
| **MBOX** | Format fichier Thunderbird (.mbox) |
| **IPC** | Inter-Process Communication (Tauri ↔ React) |
| **Fuzzy matching** | Recherche tolérante fautes frappe |
| **Assist Me** | Mode conversationnel IA |
| **RAG** | Retrieval-Augmented Generation (IA + sources) |

---

## Conventions documentation

### Format
- **Markdown** (.md) pour tout
- Headers ATX (`#`, `##`) pas Setext
- Code blocks avec langage : \`\`\`rust, \`\`\`typescript
- Tables pour comparaisons

### Structure document
1. Titre + description
2. Table matières (si >5 sections)
3. Contenu
4. Footer : version + date

### Mises à jour
- Incrémenter version (1.0 → 1.1)
- Ajouter date "Dernière mise à jour"
- Documenter changements majeurs

---

## Contribution documentation

### Ajouter document
1. Créer fichier `docs/NN_Nom_Document.md`
2. Ajouter entrée dans `00_INDEX.md`
3. Linker depuis documents pertinents

### Modifier document existant
1. Éditer fichier
2. Incrémenter version en footer
3. Ajouter ligne changelog si changement majeur

### Review
- Documentation reviewée comme code
- PR requises pour changements majeurs
- Typos : fix direct OK

---

## Ressources externes

### Références techniques
- [Tauri docs](https://tauri.app/v2/guides/)
- [Tantivy docs](https://docs.rs/tantivy/)
- [LEANN GitHub](https://github.com/yichuan-w/LEANN)
- [Tesseract docs](https://tesseract-ocr.github.io/)
- [all-MiniLM-L6-v2](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2)

### Inspiration produit
- [Everything](https://www.voidtools.com/) - Recherche rapide Windows
- [Recoll](https://www.lesbonscomptes.com/recoll/) - Recherche full-text
- [Alfred](https://www.alfredapp.com/) - Launcher macOS
- [Raycast](https://www.raycast.com/) - Launcher moderne

---

## Support

### Questions projet
- GitHub Issues : [À créer]
- Discussions : [À créer]

### Questions documentation
- Ouvrir issue avec tag `documentation`
- Proposer PR pour corrections

---

## Statistiques documentation

| Métrique | Valeur |
|----------|--------|
| **Documents totaux** | 6 |
| **Pages totales** | ~180 |
| **Mots totaux** | ~35,000 |
| **Langues** | Français |
| **Format** | Markdown |
| **Couverture** | 100% features MVP |

---

## Changelog documentation

| Date | Version | Changements |
|------|---------|-------------|
| 2025-11-12 | 1.0 | Création documentation complète initiale |

---

## Prochaines étapes

### Documentation
1. ✅ Documents base créés
2. ⏭️ CONTRIBUTING.md (Phase 1)
3. ⏭️ SETUP.md (Phase 1)
4. ⏭️ Résultats POC LEANN (Phase 3)
5. ⏭️ USER_GUIDE.md (Phase 5)

### Projet
1. ✅ Specs complètes
2. ⏭️ POC LEANN (semaine 3-4)
3. ⏭️ Setup Tauri projet (semaine 3)
4. ⏭️ Développement MVP (semaines 3-8)

---

**Index version :** 1.0
**Dernière mise à jour :** 2025-11-12
**Maintenance :** À mettre à jour chaque nouvelle phase
