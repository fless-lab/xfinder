# PRD - xfinder "Jarvis Administratif"
**Product Requirements Document**

---

## Executive Summary

### Vision
xfinder est un assistant de recherche intelligent pour administrations Windows, permettant aux agents administratifs de retrouver instantanément fichiers et emails via recherche classique ou IA conversationnelle.

### Objectifs métier
- **Gain productivité** : Réduire de 80% le temps de recherche documentaire
- **Adoption facile** : Installation en 5 min, configuration en 10 min
- **Confidentialité** : 100% local, aucune donnée cloud
- **Scalabilité** : Gérer 500k+ fichiers sur PC standard

### Cible utilisateurs
- **Primaire** : Agents administratifs (mairies, ministères, organismes publics)
- **Secondaire** : PME secteur tertiaire, professions libérales

---

## Problèmes utilisateurs

### Problème 1 : Recherche Windows inefficace
**Situation actuelle :**
- "Je cherche le contrat Dupont depuis 20 min"
- Recherche Windows limitée au nom de fichier
- Pas de recherche dans contenu PDF/emails

**Impact :**
- 30-40 min/jour perdues en recherche
- Frustration, perte de productivité

### Problème 2 : Emails perdus
**Situation actuelle :**
- "C'était dans un email de Marie il y a 3 mois..."
- Recherche Outlook lente, pas sémantique
- Pièces jointes invisibles à la recherche

**Impact :**
- Demandes en doublon
- Retards administratifs

### Problème 3 : Documents éparpillés
**Situation actuelle :**
- Fichiers sur C:\, réseau, emails, clés USB
- Pas de vue unifiée

**Impact :**
- Informations dupliquées
- Risque oubli documents importants

---

## Solution xfinder

### Vue d'ensemble
Une application desktop Windows (10MB) installable en 2 clics, offrant :

1. **Recherche ultra-rapide** : Trouve fichiers par nom en <100ms
2. **Recherche intelligente** : "Trouve les budgets formation 2024" → comprend et trouve
3. **Mode Assist Me** : Répond à des questions avec sources vérifiables
4. **Surveillance auto** : Indexe nouveaux fichiers en temps réel
5. **Configuration fine** : Choix précis dossiers/exclusions

### Différenciateurs
| Critère | Recherche Windows | Everything | xfinder |
|---------|------------------|-----------|---------|
| Vitesse nom fichier | ⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| Recherche contenu | ⭐⭐ | ❌ | ⭐⭐⭐⭐⭐ |
| Recherche sémantique | ❌ | ❌ | ⭐⭐⭐⭐⭐ |
| Emails intégrés | ❌ | ❌ | ⭐⭐⭐⭐⭐ |
| Mode questions/réponses | ❌ | ❌ | ⭐⭐⭐⭐⭐ |
| OCR images | ❌ | ❌ | ⭐⭐⭐⭐ |
| Taille install | - | 2MB | 10MB |
| Confidentialité | ✅ | ✅ | ✅ |

---

## Features détaillées

### F1 : Configuration initiale
**Priorité : MUST HAVE (MVP)**

**Description :**
Interface graphique permettant de sélectionner les dossiers à surveiller.

**User flow :**
1. Première ouverture → Assistant configuration
2. Arborescence fichiers Windows affichée
3. Utilisateur coche dossiers (défaut : `C:\Users\[nom]`)
4. Exclusions : `.tmp`, `node_modules`, etc.
5. Configuration OCR : ON/OFF par type
6. Bouton "Démarrer indexation"

**Critères acceptation :**
- [x] Arborescence navigable
- [x] Sélection multiple dossiers
- [x] Exclusions par extension/pattern
- [x] Sauvegarde config JSON
- [x] Bouton "Indexer maintenant"

**Mockup :**
```
┌─────────────────────────────────────────┐
│ xfinder - Configuration                 │
├─────────────────────────────────────────┤
│ Dossiers surveillés :                   │
│                                          │
│ 📁 C:\Users\Admin\                      │
│  ├─ ☑ Documents       [Complet]        │
│  ├─ ☐ Downloads       [Ignoré]         │
│  ├─ ☑ Bureau          [Complet]        │
│  └─ ☑ OneDrive        [Métadonnées]    │
│                                          │
│ Exclusions :                             │
│  Extensions : [.tmp .log .cache]        │
│  Dossiers   : [node_modules .git]       │
│                                          │
│ OCR :                                    │
│  ☑ Activer pour PDF scannés             │
│  ☑ Activer pour images (JPG, PNG)      │
│  Taille min : [500 KB]                  │
│                                          │
│        [Annuler]  [Démarrer indexation] │
└─────────────────────────────────────────┘
```

---

### F2 : Indexation automatique (Watchdog)
**Priorité : MUST HAVE (MVP)**

**Description :**
Surveillance temps réel du système de fichiers pour maintenir l'index à jour.

**Comportements :**

| Événement | Détection | Action |
|-----------|-----------|--------|
| Nouveau fichier | Ajout dans dossier surveillé | Indexation auto (métadonnées + contenu + embedding) |
| Suppression | Fichier retiré définitivement | Suppression index + embedding |
| Déplacement | Fichier déplacé | Mise à jour chemin, garde embedding |
| Renommage | Nom changé | Mise à jour nom, garde embedding |
| Modification | Contenu modifié | Vérif hash → si changé : réindexation |

**Optimisations :**
- **Debounce 500ms** : Attend fin modifications avant indexation
- **Queue batch** : Groupe événements multiples (copie dossier)
- **Priorité** : Index rapide immédiat, embeddings en background

**Critères acceptation :**
- [x] Détecte ajout fichier en <1s
- [x] Détecte suppression en <1s
- [x] Détecte déplacement (pas réindexation complète)
- [x] Gère 1000 fichiers copiés simultanément (batch)
- [x] Background, pas de freeze UI

---

### F3 : Recherche rapide (nom/métadonnées)
**Priorité : MUST HAVE (MVP)**

**Description :**
Recherche instantanée par nom, extension, date, taille.

**Interface :**
```
┌─────────────────────────────────────────┐
│ 🔍 Rechercher...          [⚙️] [≡]     │
├─────────────────────────────────────────┤
│ [contrat dupont                      ]  │
│                                          │
│ Filtres :                                │
│  Type : [Tous ▼] Date : [Dernier mois ▼]│
│  Taille : [Tous ▼] Dossier : [Tous ▼] │
└─────────────────────────────────────────┘

Résultats (234 trouvés) :

📄 Contrat_Dupont_2024.pdf
   C:\Users\Admin\Documents\Contrats\
   Modifié : 15/03/2024 - 2.3 MB

📄 Contrat_Dupont_Marie_Signature.pdf
   C:\Users\Admin\Bureau\À_traiter\
   Modifié : 20/03/2024 - 890 KB
```

**Fonctionnalités :**
- Recherche incrémentale (résultats pendant frappe)
- Fuzzy matching : "cntrat dpon" trouve "Contrat Dupont"
- Filtres cumulatifs
- Tri : pertinence, date, taille, nom
- Actions : Ouvrir, Ouvrir dossier, Copier chemin

**Performance cible :**
- <100ms pour 100k fichiers
- <500ms pour 1M fichiers

**Critères acceptation :**
- [x] Résultats en <100ms
- [x] Fuzzy matching fonctionne
- [x] Filtres combinables
- [x] Clic ouvre fichier
- [x] Raccourci global (Ctrl+Shift+F)

---

### F4 : Extraction contenu intelligente
**Priorité : MUST HAVE (Phase 2)**

**Description :**
Extraction automatique du texte des fichiers pour recherche full-text.

**Types supportés :**

| Format | Méthode | Librairie | Vitesse |
|--------|---------|-----------|---------|
| `.txt .md .log` | Lecture directe | std::fs | ⚡⚡⚡⚡⚡ |
| `.pdf` (texte) | Extraction texte | pdf-extract | ⚡⚡⚡⚡ |
| `.pdf` (scanné) | OCR | leptess (Tesseract) | ⚡⚡ |
| `.docx .odt` | Unzip + XML | docx-rs | ⚡⚡⚡⚡ |
| `.xlsx` | Cellules | calamine | ⚡⚡⚡⚡ |
| `.jpg .png` | OCR optionnel | leptess | ⚡⚡ |
| `.eml .msg` | Parser email | mailparse | ⚡⚡⚡⚡ |

**Détection automatique besoin OCR :**
```rust
fn needs_ocr(file: &File) -> bool {
    match file.extension {
        "pdf" => !pdf_has_text_layer(file),
        "jpg"|"png"|"tiff" => {
            config.ocr_enabled &&
            file.size > config.min_size_kb &&
            is_in_watched_folder(file)
        },
        _ => false
    }
}
```

**Critères acceptation :**
- [x] PDF texte extrait correctement (accents FR)
- [x] PDF scanné détecté automatiquement
- [x] OCR activable par dossier/type
- [x] Progression visible (1500 fichiers à traiter)
- [x] Contenu stocké SQLite (full-text search)

---

### F5 : OCR intelligent
**Priorité : SHOULD HAVE (Phase 2)**

**Description :**
Extraction texte des images et PDF scannés via OCR.

**Exigences :**
1. **Lightweight** : Binaire <50MB
2. **Précision** : >95% sur docs administratifs français
3. **Vitesse** : <5s par page A4 (300 DPI)
4. **Offline** : Aucune connexion requise
5. **Langues** : Français + Anglais

**Options évaluées :**

| Solution | Taille | Vitesse | Précision FR | Intégration Rust |
|----------|--------|---------|--------------|------------------|
| **Tesseract 5.x** | 30MB | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | leptess (binding) |
| PaddleOCR | 8MB | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ONNX Runtime |
| EasyOCR | 150MB | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | Python (non viable) |
| Windows OCR API | 0MB | ⭐⭐⭐ | ⭐⭐⭐⭐ | Windows 10+ uniquement |

**✅ RECOMMANDATION : Tesseract 5.x**

**Justification :**
- Référence industrie (Google)
- Excellent support français (tessdata_best)
- Binding Rust mature (`leptess`)
- Taille raisonnable (30MB avec langues)
- Offline complet

**Implémentation :**
```rust
// Cargo.toml
leptess = "0.14"

// ocr.rs
use leptess::{LepTess, Variable};

fn ocr_image(path: &Path) -> Result<String> {
    let mut lt = LepTess::new(None, "fra+eng")?;
    lt.set_image(path)?;
    lt.set_variable(Variable::TesseditPagesegMode, "3")?; // Auto
    lt.get_utf8_text()
}
```

**Configuration utilisateur :**
```
☑ Activer OCR
  Langues : [☑] Français [☑] Anglais [ ] Allemand

  Appliquer à :
    [☑] PDF scannés (détection auto)
    [☑] Images dans Documents/ (>500KB)
    [ ] Toutes les images

  Performance :
    Threads : [4] (recommandé : nb CPU)
    Priorité : ⚪ Basse ⚫ Normale ⚪ Haute
```

**Optimisations :**
- **Preprocessing** : Deskew, denoise, binarization (Leptonica)
- **Batch** : Traite plusieurs pages en parallèle
- **Cache** : Hash image → si déjà traité, skip
- **Resume** : Reprend indexation après fermeture

**Critères acceptation :**
- [x] Reconnaissance français >95% précision
- [x] <5s par page A4 (CPU moyen i5)
- [x] Détecte automatiquement PDF scannés
- [x] Configurable par dossier
- [x] Progression affichée
- [x] Interruptible/reprennable

---

### F6 : Mode "Assist Me" (IA conversationnelle)
**Priorité : MUST HAVE (Phase 3)**

**Description :**
Répond à des questions en langage naturel avec sources vérifiables.

**Exemples d'usage :**

**Exemple 1 : Recherche documentaire**
```
Utilisateur : "Quels sont les budgets formation validés en 2024 ?"

xfinder :
D'après mes recherches, 3 budgets formation validés en 2024 :

1. "Compétences numériques" - 45 000€
   📄 Budget_Formation_2024.pdf (page 3)
   📁 C:\Users\Admin\Documents\RH\

2. "Management interculturel" - 28 500€
   📄 Formation_Q1_2024.docx (section 2.3)
   📁 \\Serveur\RH\Validations\

3. "Cybersécurité agents" - 67 000€
   📧 Email DG - 22/04/2024 "RE: Validation budget cyber"
   📎 Devis_Formation_Cyber.xlsx

Total : 140 500€

[Cliquez sur les sources pour ouvrir]
```

**Exemple 2 : Recherche email**
```
Utilisateur : "Retrouve les échanges avec Marie sur le projet RGPD"

xfinder :
7 conversations trouvées (mars-mai 2024) :

📧 "Kick-off RGPD" - Marie Durand - 15/03/2024
   → Lancement projet, planification initiale
   📎 Planning_RGPD.xlsx

📧 "RE: Cartographie données sensibles" - 22/03/2024
   → Discussion conformité Article 30

📧 "Urgent: Breach notification" - 05/04/2024
   → Incident sécurité client Dupont
   📎 Rapport_Incident_Confidentiel.pdf

[Afficher tout]
```

**Architecture technique :**

```
Question utilisateur
    ↓
[Embedding question] (all-MiniLM-L6-v2)
    ↓
[Recherche LEANN] → Top 20 documents pertinents
    ↓
[Récupération contexte]
    - Chunks texte (500 tokens)
    - Métadonnées (chemin, page, date, auteur)
    - Type source (fichier/email)
    ↓
[Agrégation résultats]
Option A: Sans LLM
    → Affiche extraits + métadonnées

Option B: Avec LLM local (Llama 3.2 1B)
    → Génère réponse synthétique
    → Cite sources
    ↓
[Formatage UI]
    → Liens cliquables
    → Icônes type (📄📧📎)
    → Preview au survol
```

**Fonctionnalités UI :**
- Liens cliquables ouvrent fichier/email
- Highlight passage pertinent
- Preview document au survol
- Export résultats (markdown/PDF)
- Historique questions (persisté)

**Critères acceptation :**
- [x] Répond en <3s (sans LLM) / <10s (avec LLM)
- [x] Top-5 sources pertinentes à 80%+
- [x] Liens ouvrent bon fichier/email
- [x] Fonctionne offline
- [x] Historique navigable
- [x] Export résultats

---

### F7 : Indexation emails
**Priorité : MUST HAVE (Phase 4)**

**Description :**
Indexation complète des emails Outlook/Thunderbird/IMAP avec pièces jointes.

**Sources supportées :**

| Source | Format | Méthode accès | Librairie |
|--------|--------|---------------|-----------|
| Outlook | .pst, .ost | Lecture fichier OU API MAPI | libpff-rs |
| Thunderbird | .mbox | Lecture fichier texte | mailparse |
| IMAP | Serveur distant | Connexion réseau | async-imap |

**Données indexées :**
```rust
struct Email {
    id: String,
    subject: String,
    from: String,
    to: Vec<String>,
    cc: Vec<String>,
    date: DateTime,
    body_text: String,
    body_html: String,
    attachments: Vec<Attachment>,
    folder: String,  // Inbox, Sent, Archives
    thread_id: String,
}

struct Attachment {
    filename: String,
    size: u64,
    mime_type: String,
    extracted_path: PathBuf,  // Cache local
    content_text: Option<String>,  // Si PDF/DOCX
}
```

**Fonctionnalités :**
- Indexation initiale (historique complet)
- Synchronisation incrémentale (nouveaux emails)
- Extraction pièces jointes (cache local)
- Indexation contenu PJ (PDF, DOCX)
- Threading emails (conversations)

**Configuration :**
```
Sources emails :

☑ Outlook
  Fichier PST : [C:\Users\Admin\Documents\Outlook.pst] [Parcourir]
  OU
  ☑ Utiliser profil actif (MAPI)

☑ Thunderbird
  Profil : [C:\Users\Admin\AppData\Roaming\Thunderbird\...] [Auto-detect]

☐ IMAP
  Serveur : [imap.example.com] Port : [993]
  Email : [admin@example.com]
  ⚠️ Mot de passe stocké chiffré localement

Indexer :
  [☑] Boîte réception  [☑] Envoyés  [☑] Archives
  [ ] Corbeille  [ ] Spam

Pièces jointes :
  [☑] Extraire et indexer contenu
  Types : [☑] PDF [☑] DOCX [☑] XLSX [ ] Images
```

**Critères acceptation :**
- [x] Indexe 10k emails en <5 min
- [x] Détecte nouveaux emails en <30s
- [x] Extrait PJ automatiquement
- [x] Recherche dans corps + PJ
- [x] Ouvre email dans client natif (Outlook/Thunderbird)
- [x] Threading conversations

---

### F8 : Exclusions granulaires
**Priorité : SHOULD HAVE (MVP)**

**Description :**
Configuration fine des dossiers/fichiers à surveiller ou exclure.

**Types d'exclusions :**

1. **Par dossier**
   - Clic droit dossier → "Exclure"
   - Héritage : enfants exclus par défaut
   - Override possible : "Inclure malgré parent exclu"

2. **Par extension**
   - Liste globale : `.tmp, .log, .cache, .bak`
   - Ajout rapide : clic droit fichier → "Ignorer type .xyz"

3. **Par pattern**
   - Glob/regex : `*_backup.*, node_modules, .git`
   - Présets : "Fichiers dev", "Fichiers système"

4. **Par taille**
   - "Ignorer fichiers > 500 MB"
   - "Ignorer fichiers < 10 KB"

5. **Par nom**
   - "Ignorer ce fichier précis"
   - Utile pour exceptions

**Modes surveillance :**
- **Complet** : Métadonnées + contenu + embeddings
- **Métadonnées** : Juste nom/date/taille (rapide)
- **Exclu** : Ignoré complètement

**Interface arborescence :**
```
📁 C:\Users\Admin\Documents  [⚙️ Complet]
  ├─ 📂 Projets              [⚙️ Complet]
  │   ├─ 📂 Projet_A         [⚙️ Complet]
  │   └─ 📂 Archives_Old     [❌ Exclu]
  ├─ 📂 Photos_Vacances      [❌ Exclu]
  └─ 📄 *.tmp                [❌ Extension exclue]

Clic droit sur "Archives_Old" :
  ✓ Surveiller en mode complet
    Surveiller métadonnées uniquement
    Exclure ce dossier
  ────────────
    Exclure fichiers > 100MB ici
    Ajouter pattern exclusion...
```

**Règles priorité :**
1. Exclusion spécifique (fichier/dossier) > Pattern
2. Pattern > Extension globale
3. Enfant > Parent (override possible)

**Critères acceptation :**
- [x] Arborescence interactive
- [x] Exclusion immédiate (pas réindexation)
- [x] Override règles parent
- [x] Patterns regex fonctionnent
- [x] Prévisualisation exclusions (nb fichiers affectés)

---

## Personas

### Persona 1 : Marie, Assistante administrative (mairie)
**Profil :**
- 45 ans, 15 ans ancienneté
- Utilise : Windows 10, Outlook, Word/Excel
- Compétences tech : Moyennes
- Frustrations : Perd du temps à chercher documents, Outlook lent

**Besoin xfinder :**
- Retrouver rapidement délibérations/arrêtés
- Chercher emails avec élus
- Interface simple, pas technique

**Scénario :**
> "Le maire me demande le dernier arrêté sur les travaux Rue de la Paix. Je ne me souviens plus du numéro ni de la date exacte. J'ouvre xfinder, je tape 'arrêté travaux paix' → trouve en 2 secondes. Je clique, le PDF s'ouvre. Gain : 15 minutes."

---

### Persona 2 : Thomas, Responsable RH (ministère)
**Profil :**
- 35 ans, tech-savvy
- Utilise : Windows 11, Exchange, SharePoint
- Compétences tech : Avancées
- Frustrations : Trop de sources (serveur, mails, SharePoint), recherche pas unifiée

**Besoin xfinder :**
- Vue unifiée fichiers serveur + emails
- Recherche sémantique (concepts pas mots-clés)
- Mode "Assist Me" pour synthèses

**Scénario :**
> "Je dois préparer un rapport sur les formations 2024. J'active 'Assist Me', je demande 'Liste toutes les formations validées Q1 2024 avec budgets'. xfinder agrège infos de 12 documents + 8 emails, me présente tableau synthétique avec sources. Gain : 2 heures."

---

### Persona 3 : Sophie, Archiviste (organisme public)
**Profil :**
- 52 ans, experte métier
- Utilise : Windows 10, Thunderbird, LibreOffice
- Compétences tech : Faibles
- Frustrations : Archives papier numérisées (PDF scannés), pas searchables

**Besoin xfinder :**
- OCR automatique sur PDF scannés
- Recherche dans archives historiques
- Configuration simple

**Scénario :**
> "On me demande un rapport de 1998 numérisé (PDF image). Avant : impossible à trouver sauf feuilleter. Avec xfinder + OCR : je tape 'rapport 1998 budget infrastructure', trouve en 3 secondes. Révolutionnaire."

---

## Métriques de succès

### Métriques produit

| Métrique | Objectif | Mesure |
|----------|----------|--------|
| **Temps recherche moyen** | <10s | Analytics in-app |
| **Taux trouvaille** | >90% | "Avez-vous trouvé ce que vous cherchiez ?" |
| **Adoption utilisateurs** | >80% après 1 mois | Nb licences actives |
| **Fréquence usage** | >5 recherches/jour | Telemetry |
| **NPS** | >50 | Enquête trimestrielle |

### Métriques techniques

| Métrique | Objectif | Mesure |
|----------|----------|--------|
| **Vitesse indexation** | 1000 fichiers/min | Benchmark |
| **Temps réponse recherche** | <100ms (nom), <3s (IA) | Logs perf |
| **Empreinte mémoire** | <500MB (idle), <2GB (indexation) | Profiling |
| **Taille index** | <5% taille corpus | Ratio index/corpus |
| **Crash rate** | <0.1% sessions | Telemetry |

### Métriques business

| Métrique | Objectif | Mesure |
|----------|----------|--------|
| **Gain temps/agent/mois** | 10+ heures | Enquête + analytics |
| **ROI** | 3x sur 1 an | (Gain temps × taux horaire) / coût licence |
| **Taux renouvellement** | >90% | Subscriptions |
| **Recommandation** | >70% | "Recommanderiez-vous à collègues ?" |

---

## Roadmap priorisée

### Phase 1 : MVP Indexation (6 semaines)
**Objectif : Recherche fichiers fonctionnelle**

**Semaines 1-2 : Infrastructure**
- Setup projet Tauri
- Architecture backend Rust
- UI basique (barre recherche)
- Config dossiers surveillés

**Semaines 3-4 : Indexation**
- Watchdog filesystem
- Index SQLite métadonnées
- Extraction contenu basique (TXT, PDF texte, DOCX)
- Recherche nom/extension/date

**Semaines 5-6 : Polish MVP**
- Gestion exclusions
- Interface résultats
- Installateur Windows
- Tests utilisateurs pilotes

**Livrables :**
- ✅ Exe Windows installable
- ✅ Recherche instantanée fichiers
- ✅ Watchdog temps réel
- ✅ Config dossiers/exclusions

---

### Phase 2 : Contenu + OCR (4 semaines)
**Objectif : Recherche full-text**

**Semaines 7-8 : Extraction**
- Support formats additionnels (XLSX, RTF)
- Détection PDF scannés
- FTS5 SQLite (full-text search)

**Semaines 9-10 : OCR**
- Intégration Tesseract
- Config OCR par dossier/type
- Optimisation performance

**Livrables :**
- ✅ Recherche dans contenu
- ✅ OCR PDF scannés/images
- ✅ Performance optimisée

---

### Phase 3 : IA Assist Me (5 semaines)
**Objectif : Recherche sémantique + Q&A**

**Semaines 11-12 : LEANN**
- Intégration LEANN
- Génération embeddings
- Index vectoriel

**Semaines 13-14 : Assist Me**
- Interface conversationnelle
- Mapping sources (file_id → path)
- Citations cliquables

**Semaine 15 : LLM optionnel**
- Intégration Llama 3.2 1B
- Génération réponses
- Tests A/B (avec/sans LLM)

**Livrables :**
- ✅ Recherche sémantique
- ✅ Mode questions/réponses
- ✅ Sources vérifiables

---

### Phase 4 : Emails (5 semaines)
**Objectif : Recherche unifiée fichiers + emails**

**Semaines 16-17 : Outlook**
- Parser PST/OST
- API MAPI
- Indexation emails

**Semaines 18-19 : Multi-sources**
- Thunderbird (MBOX)
- IMAP Exchange
- Extraction pièces jointes

**Semaine 20 : Intégration**
- Recherche unifiée
- Threading conversations
- Tests intégration

**Livrables :**
- ✅ Indexation Outlook/Thunderbird/IMAP
- ✅ Recherche emails + PJ
- ✅ Assist Me avec emails

---

### Phase 5 : Production (3 semaines)
**Objectif : Déploiement administration**

**Semaine 21 : Optimisation**
- Profiling perf
- Réduction empreinte mémoire
- Support 1M+ fichiers

**Semaine 22 : Déploiement**
- Installateur MSI
- Auto-update
- Guide administrateur

**Semaine 23 : Lancement**
- Documentation utilisateur
- Vidéos tutoriels
- Support pilotes

**Livrables :**
- ✅ Version production
- ✅ Installation silencieuse GPO
- ✅ Documentation complète

---

## Contraintes & Risques

### Contraintes techniques

| Contrainte | Impact | Mitigation |
|-----------|--------|------------|
| **Windows uniquement** | Limite marché | OK pour cible administration FR |
| **Ressources locales** | Perf variables selon PC | Optimisation, mode "léger" |
| **Formats propriétaires** | Parsing complexe (PST, DOCX) | Libs éprouvées (libpff, docx-rs) |
| **Taille embeddings** | Espace disque | LEANN (97% réduction) |

### Risques

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| **LEANN pas performant** | Moyenne | Élevé | POC rapide, fallback ChromaDB |
| **OCR trop lent** | Faible | Moyen | Mode optionnel, batch async |
| **Parsing PST échoue** | Moyenne | Élevé | Fallback API MAPI Windows |
| **Adoption utilisateurs faible** | Moyenne | Critique | Tests utilisateurs continus, UX simple |
| **Concurrence (Everything Pro)** | Faible | Moyen | Focus IA + emails (différenciation) |
| **Problèmes permissions Windows** | Élevée | Moyen | Élévation UAC si besoin, doc claire |

---

## Questions ouvertes (à valider)

### Fonctionnelles

1. **Recherche réseau** : Surveiller `\\Serveur\Partage` ?
   - ⚠️ Impact : Perf, permissions
   - 💡 Suggestion : V1 local, V2 réseau

2. **Langues** : Multilingue ou français uniquement ?
   - 🎯 Cible : Administrations FR → Français prioritaire
   - ➕ Bonus : Anglais (docs techniques)

3. **Mode LLM** : Obligatoire ou optionnel ?
   - 📊 Trade-off : UX vs complexité/taille
   - 💡 Suggestion : Optionnel (téléchargement séparé)

4. **Partage résultats** : Export, partage avec collègues ?
   - 🤔 Cas usage : "Envoyer recherche à collègue"
   - 💡 Features : Export markdown, lien xfinder://

### Techniques

5. **Stack frontend** : React, Vue ou Svelte ?
   - 📦 Taille bundle : Svelte (plus léger)
   - 👨‍💻 Ressources : React (plus de devs)

6. **Telemetry** : Analytics usage optionnelle ?
   - 🔒 Confidentialité : Problématique administration
   - 💡 Suggestion : Opt-in, anonymisé, local only

7. **Cloud sync** : Synchroniser index entre PCs ?
   - ⚠️ Complexité élevée
   - 💡 Suggestion : V2+, si demande forte

8. **Mobile** : Appli mobile compagnon ?
   - 🎯 Cas usage : Recherche depuis smartphone
   - 💡 Suggestion : Post-MVP si succès desktop

---

## Prochaines étapes

1. ✅ Validation PRD avec équipe/sponsors
2. ⏭️ Spécifications techniques détaillées
3. ⏭️ POC LEANN (valider promesses performance)
4. ⏭️ Mockups UI complets (Figma)
5. ⏭️ Architecture base de données
6. ⏭️ Setup projet Tauri
7. ⏭️ Sprint 1 : Watchdog + indexation basique

---

**Document version :** 1.0
**Dernière mise à jour :** 2025-11-12
**Auteurs :** Équipe xfinder
**Validateurs :** [À compléter]
