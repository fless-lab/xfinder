# xfinder - Jarvis Administratif

> **Assistant de recherche intelligent pour administrations Windows**
> Recherche ultrarapide + IA conversationnelle + Emails intégrés

---

## Vision

xfinder permet aux agents administratifs de retrouver **instantanément** fichiers et emails via :
- 🔍 **Recherche ultrarapide** : Trouve par nom en <100ms
- 🧠 **IA sémantique** : Comprend "budgets formation 2024"
- 💬 **Mode Assist Me** : Répond à vos questions avec sources vérifiables
- 📧 **Emails intégrés** : Recherche unifiée fichiers + Outlook/Thunderbird
- 👁️ **OCR intelligent** : Indexe PDF scannés et images
- ⚡ **Temps réel** : Indexation automatique des nouveaux fichiers

---

## Fonctionnalités principales

### 1. Recherche rapide
- Recherche instantanée par nom (<100ms pour 100k fichiers)
- Fuzzy matching : "cntrat dpon" trouve "Contrat Dupont"
- Filtres avancés : extension, date, taille, dossier
- Raccourci global : `Ctrl+Shift+F`

### 2. Recherche intelligente (IA)
- Recherche sémantique : comprend le sens, pas juste les mots
- Mode "Assist Me" : posez des questions en français
- Sources vérifiables : chaque réponse cite les fichiers/emails
- Index ultra-compact : LEANN (97% plus léger que solutions classiques)

### 3. OCR automatique
- Détection auto PDF scannés
- Extraction texte images (JPG, PNG, TIFF)
- Configurable par dossier/type fichier
- Support français + anglais

### 4. Emails
- Indexation Outlook (PST/MAPI)
- Indexation Thunderbird (MBOX)
- Support IMAP/Exchange
- Recherche pièces jointes

### 5. Surveillance temps réel
- Watchdog automatique : détecte nouveaux fichiers
- Mise à jour index en temps réel
- Gère déplacements/renommages intelligemment

---

## Stack technique

| Composant | Technologie | Pourquoi |
|-----------|-------------|----------|
| **Application** | Tauri 2.0 | Léger (10MB), sécurisé, rapide |
| **Backend** | Rust | Performance, sécurité mémoire |
| **Frontend** | React + TypeScript | Interface moderne, maintenable |
| **Recherche rapide** | Tantivy | Lucene-like en Rust |
| **Recherche contenu** | SQLite FTS5 | Full-text natif, simple |
| **IA/Embeddings** | LEANN + all-MiniLM-L6-v2 | Index compact, rapide |
| **OCR** | Tesseract 5 | Référence industrie, offline |
| **Watchdog** | notify-rs | Surveillance filesystem |
| **Email parsing** | mailparse + libpff | PST/MBOX support |

**Taille totale :** ~120MB (app 10MB + OCR 30MB + modèle IA 80MB)

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   FRONTEND (React)                      │
│  Interface recherche + Configuration + Assist Me        │
└────────────────────┬────────────────────────────────────┘
                     │ IPC Tauri
┌────────────────────▼────────────────────────────────────┐
│              BACKEND (Rust)                             │
│                                                          │
│  Watchdog → Indexer → Content Extractor (OCR)          │
│  Search Engine ← Tantivy + SQLite FTS5 + LEANN         │
│  Email Parser → Outlook/Thunderbird/IMAP                │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│              STORAGE (local)                            │
│  index.db (SQLite) + vectors.db (LEANN) + content.db   │
└─────────────────────────────────────────────────────────┘
```

---

## Roadmap

### ✅ Phase 0 : Documentation (Actuelle - Semaine 1-2)
- [x] Spécifications produit (PRD)
- [x] Architecture technique
- [x] Décisions techniques
- [x] Schémas API/DB
- [x] Plan de tests

### 🚧 Phase 1 : MVP Indexation (Semaines 3-8)
- [ ] Setup Tauri + React
- [ ] Watchdog filesystem
- [ ] Indexation fichiers (métadonnées + contenu)
- [ ] Recherche rapide (Tantivy)
- [ ] Interface basique
- [ ] Configuration dossiers/exclusions

**Livrable :** Recherche fichiers fonctionnelle

### 📅 Phase 2 : OCR + Contenu (Semaines 9-12)
- [ ] Intégration Tesseract
- [ ] Détection auto PDF scannés
- [ ] Config OCR par dossier
- [ ] Recherche full-text (SQLite FTS5)

**Livrable :** Recherche dans contenu + OCR

### 📅 Phase 3 : IA Assist Me (Semaines 13-17)
- [ ] POC LEANN (benchmark vs FAISS)
- [ ] Génération embeddings
- [ ] Recherche sémantique
- [ ] Interface conversationnelle
- [ ] Citations sources

**Livrable :** Mode questions/réponses intelligent

### 📅 Phase 4 : Emails (Semaines 18-22)
- [ ] Parser Outlook PST/MAPI
- [ ] Parser Thunderbird MBOX
- [ ] Support IMAP
- [ ] Indexation pièces jointes
- [ ] Recherche unifiée

**Livrable :** Recherche fichiers + emails

### 📅 Phase 5 : Production (Semaines 23-25)
- [ ] Optimisation performance
- [ ] Installateur MSI
- [ ] Auto-update
- [ ] Documentation utilisateur
- [ ] Tests beta

**Livrable :** Version production déployable

---

## Documentation

| Document | Description |
|----------|-------------|
| [01_PRD_Product_Requirements.md](docs/01_PRD_Product_Requirements.md) | Spécifications produit complètes |
| [02_Architecture_Technique.md](docs/02_Architecture_Technique.md) | Architecture détaillée + code samples |
| [03_Decisions_Techniques.md](docs/03_Decisions_Techniques.md) | Choix techno et justifications |
| [04_API_Schemas.md](docs/04_API_Schemas.md) | API Tauri + schémas DB |
| [05_Plan_Tests_Metriques.md](docs/05_Plan_Tests_Metriques.md) | Stratégie tests + benchmarks |

---

## Quick Start (futur)

```bash
# Installation
Download xfinder-setup.msi
Double-click → Install

# Première utilisation
1. Lance xfinder
2. Sélectionne dossiers à surveiller
3. Démarre indexation
4. Recherche ! (Ctrl+Shift+F)
```

---

## Performances cibles

| Métrique | Objectif |
|----------|----------|
| Recherche (100k fichiers) | <100ms |
| Indexation | >1000 fichiers/min |
| OCR page A4 | <5s |
| Recherche sémantique | <3s |
| Taille index | <5% corpus |
| Mémoire idle | <500MB |
| Démarrage app | <3s |

---

## Questions ouvertes

### Fonctionnelles
1. **Recherche réseau** : Surveiller serveurs partagés `\\Serveur\` ?
2. **Langues** : Multilingue ou français prioritaire ?
3. **LLM** : Mode Assist Me avec génération ou juste citations ?

### Techniques
4. **LEANN** : Valider performance vs FAISS (POC semaine 3-4)
5. **PST parsing** : MAPI ou libpff en priorité ?
6. **GPU** : Support CUDA pour embeddings ? (+500MB mais 10x vitesse)

### Business
7. **Pricing** : Gratuit admin publiques, payant privé ?
8. **Support** : Communauté ou support dédié ?

---

## Contribution

Projet en phase de documentation. Code à venir Phase 1 (semaine 3).

---

## Licence

À définir (probablement GPL-3.0 ou Apache-2.0)

---

## Contact

Projet pour administrations françaises.

**Status :** 📋 Phase documentation (semaine 1-2)
**Prochaine étape :** POC LEANN + Setup Tauri (semaine 3)

---

**Généré le :** 2025-11-12
**Version doc :** 1.0
