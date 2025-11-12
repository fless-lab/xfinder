# 🚀 QUICKSTART - Démarrer xfinder de ZÉRO

**Guide ultra-simple pour commencer sans se perdre**

---

## 📋 Checklist avant de commencer

### 1. Installer les outils (30 min)

```bash
# === Rust ===
# Télécharger depuis : https://rustup.rs/
# Ou en ligne de commande :
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Vérifier installation
rustc --version
cargo --version

# === Git ===
# Télécharger depuis : https://git-scm.com/
git --version

# === Tesseract (OCR) - Pour plus tard ===
# Télécharger depuis : https://github.com/UB-Mannheim/tesseract/wiki
# Installer dans C:\Program Files\Tesseract-OCR

# === Optionnel : VS Code ===
# Télécharger depuis : https://code.visualstudio.com/
# Extensions recommandées :
# - rust-analyzer
# - CodeLLDB (debug)
```

---

## 📁 Étape 1 : Créer la structure du projet (5 min)

```bash
# Créer dossier projet
cd D:\DataLab
mkdir xfinder
cd xfinder

# Initialiser Git
git init
git add .
git commit -m "Initial commit - Documentation"

# Créer projet Rust
cargo new . --name xfinder

# Ta structure actuelle :
# xfinder/
# ├── docs/           (déjà créé - toute la doc)
# ├── src/
# │   └── main.rs     (créé par cargo)
# ├── Cargo.toml      (créé par cargo)
# └── README.md       (déjà créé)
```

---

## 📝 Étape 2 : Cargo.toml minimal (commencer simple)

Remplace le contenu de `Cargo.toml` par ça :

```toml
[package]
name = "xfinder"
version = "0.1.0"
edition = "2021"

[dependencies]
# === UI (on commence avec egui simple) ===
eframe = "0.27"
egui = "0.27"

# === Search (Tantivy - comme spotlight_windows) ===
tantivy = "0.22"

# === Database ===
rusqlite = { version = "0.32", features = ["bundled"] }

# === Filesystem ===
walkdir = "2.4"

# === Utils ===
anyhow = "1.0"
```

**C'est tout pour commencer !** On ajoutera le reste plus tard.

---

## 💻 Étape 3 : Hello World egui (10 min)

Remplace `src/main.rs` par ça :

```rust
// src/main.rs
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "xfinder",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    search_query: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            search_query: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔍 xfinder - Recherche intelligente");

            ui.add_space(20.0);

            // Barre de recherche
            ui.horizontal(|ui| {
                ui.label("Rechercher :");
                ui.text_edit_singleline(&mut self.search_query);
            });

            ui.add_space(10.0);

            // Affiche ce que tu tapes (pour tester)
            if !self.search_query.is_empty() {
                ui.label(format!("Tu cherches : {}", self.search_query));
            }
        });
    }
}
```

**Teste que ça marche :**

```bash
cargo run
```

✅ **Si une fenêtre s'ouvre avec une barre de recherche = SUCCESS !** 🎉

---

## 📚 Étape 4 : Structure progressive (ne pas créer tout d'un coup)

**IMPORTANT : On va créer les modules AU FUR ET À MESURE, pas tous maintenant !**

```
xfinder/
├── src/
│   ├── main.rs              ✅ FAIT (hello world)
│   │
│   ├── app.rs               ⏭️ PROCHAINE ÉTAPE
│   │   (État global app)
│   │
│   ├── search/              ⏭️ SEMAINE 1-2
│   │   ├── mod.rs
│   │   └── tantivy_index.rs (recherche basique)
│   │
│   ├── database/            ⏭️ SEMAINE 2
│   │   ├── mod.rs
│   │   └── sqlite.rs
│   │
│   ├── indexer/             ⏭️ SEMAINE 3
│   │   └── ...
│   │
│   └── ...                  ⏭️ PLUS TARD
│
├── Cargo.toml               ✅ FAIT (minimal)
└── docs/                    ✅ FAIT (toute la doc)
```

**On va créer 1 module à la fois, tester, puis passer au suivant.**

---

## 🎯 Plan des 4 premières semaines (étape par étape)

### Semaine 1 : Recherche basique (Tantivy)

**Objectif :** Pouvoir chercher des fichiers par nom

```
Jour 1-2 : Setup Tantivy index
  → Créer src/search/mod.rs
  → Indexer 100 fichiers test
  → Afficher résultats dans UI

Jour 3-4 : Améliorer UI recherche
  → Liste résultats scrollable
  → Navigation clavier (↑↓)
  → Ouvrir fichier au clic

Jour 5 : Tests
  → Vérifier que ça marche sur 1000 fichiers
  → Mesurer vitesse (<100ms ?)
```

**Livrable semaine 1 :** App qui cherche des fichiers par nom ✅

---

### Semaine 2 : Base de données + métadonnées

**Objectif :** Stocker infos fichiers dans SQLite

```
Jour 1-2 : Setup SQLite
  → Créer src/database/mod.rs
  → Table files (id, path, name, size, date)
  → Insert/Select basique

Jour 3-4 : Lier Tantivy + SQLite
  → Indexation stocke dans les 2
  → Recherche retourne données complètes

Jour 5 : UI résultats enrichis
  → Afficher taille, date modif
  → Icônes par type fichier
```

**Livrable semaine 2 :** Résultats avec métadonnées ✅

---

### Semaine 3 : Watchdog (auto-indexation)

**Objectif :** Détecter nouveaux fichiers automatiquement

```
Jour 1-3 : Module watchdog
  → Créer src/watchdog/mod.rs
  → Détecter ajout/suppression/déplacement
  → Mettre à jour index

Jour 4-5 : Tests robustesse
  → Copier 1000 fichiers → vérifie indexation
  → Déplacer fichier → vérifie màj chemin
```

**Livrable semaine 3 :** Index se met à jour tout seul ✅

---

### Semaine 4 : Configuration + UI settings

**Objectif :** Choisir dossiers à surveiller

```
Jour 1-2 : Config TOML
  → Créer src/config/mod.rs
  → Lire/écrire config.toml

Jour 3-4 : UI configuration
  → Fenêtre settings (egui)
  → Sélectionner dossiers

Jour 5 : Démo complète
  → Config → Indexation → Recherche → Résultats
```

**Livrable semaine 4 :** MVP fonctionnel de bout en bout ✅

---

## 📖 Ressources pour apprendre

### 1. **Rust (si tu débutes)**

```bash
# Tutorial officiel (excellent)
https://doc.rust-lang.org/book/

# Rust by Example (apprendre en codant)
https://doc.rust-lang.org/rust-by-example/

# Pour xfinder, tu as besoin surtout de :
- Chapitre 1-10 : Bases
- Chapitre 13 : Iterators (important pour parcourir fichiers)
- Chapitre 15 : Smart Pointers (Arc, Mutex)
- Chapitre 16 : Concurrency (tokio pour async)
```

### 2. **egui (UI)**

```bash
# Documentation officielle
https://docs.rs/egui/latest/egui/

# Exemples (TRÈS UTILE - copie/colle)
https://github.com/emilk/egui/tree/master/examples

# Pour xfinder, regarde surtout :
- hello_world : Base
- text_edit : Barre recherche
- scrolling : Liste résultats
- custom_window : Fenêtre config
```

### 3. **Tantivy (recherche)**

```bash
# Tutoriel officiel
https://github.com/quickwit-oss/tantivy

# Guide complet
https://docs.rs/tantivy/latest/tantivy/

# Exemple de base (COMMENCE PAR ÇA)
https://github.com/quickwit-oss/tantivy/blob/main/examples/basic_search.rs
```

### 4. **SQLite (base de données)**

```bash
# rusqlite docs
https://docs.rs/rusqlite/latest/rusqlite/

# Tutorial simple
https://github.com/rusqlite/rusqlite#usage

# Exemple de base
https://github.com/rusqlite/rusqlite/blob/master/examples/person.rs
```

### 5. **Inspiration spotlight_windows**

```bash
# Ton projet actuel
https://github.com/fless-lab/spotlight_windows

# Regarde surtout :
- src/search/ : Comment utiliser Tantivy
- src/ui/ : Structure UI egui
- src/indexer/ : Parcourir fichiers
```

---

## 🔍 Recherches importantes à faire

### Avant de coder chaque module, cherche :

#### Pour Tantivy (Semaine 1)
```
Google/GitHub :
1. "tantivy rust tutorial"
2. "tantivy index files example"
3. "tantivy search performance"
4. spotlight_windows/src/search/ (ton code)
```

#### Pour SQLite (Semaine 2)
```
1. "rusqlite tutorial"
2. "rusqlite create table example"
3. "sqlite best practices rust"
```

#### Pour Watchdog (Semaine 3)
```
1. "notify-rs tutorial"
2. "rust file system watcher"
3. "detect file changes rust"
```

#### Pour egui UI (Semaine 4)
```
1. "egui window example"
2. "egui file picker"
3. "egui custom widget"
```

---

## 🆘 Si tu bloques

### Problème : Cargo build échoue

```bash
# Nettoie et rebuild
cargo clean
cargo build

# Vérifie version Rust à jour
rustup update
```

### Problème : egui fenêtre ne s'ouvre pas

```bash
# Vérifie drivers GPU
# Essaye version software renderer :
# Dans Cargo.toml, change :
eframe = { version = "0.27", default-features = false, features = ["default_fonts"] }
```

### Problème : "Je ne sais pas par où commencer"

**→ Commence par le Hello World egui (étape 3)**
**→ Puis cherche "tantivy basic example" sur Google**
**→ Copie/colle, adapte, teste**
**→ Répète jusqu'à ce que ça marche**

---

## 📅 Planning réaliste (par toi-même)

| Semaine | Focus | Objectif mesurable |
|---------|-------|-------------------|
| **1** | Tantivy recherche basique | Chercher 1000 fichiers <100ms |
| **2** | SQLite + métadonnées | Afficher taille/date résultats |
| **3** | Watchdog auto-update | Indexation temps réel fonctionne |
| **4** | Config + UI settings | Config dossiers surveillés |
| **5-8** | Peaufinage MVP | Tests, optimisations, bugs |
| **9+** | OCR, IA, Emails | Features avancées |

---

## ✅ Checklist démarrage AUJOURD'HUI

```bash
[ ] Installer Rust (rustup)
[ ] Installer Git
[ ] cd D:\DataLab\xfinder
[ ] Éditer Cargo.toml (copier version minimale ci-dessus)
[ ] Éditer src/main.rs (copier Hello World egui)
[ ] cargo run
[ ] ✅ Fenêtre s'ouvre ? SUCCESS !
```

**Une fois le Hello World qui marche, reviens me demander la suite !**

---

## 🎯 Prochaine étape (après Hello World)

Je te donnerai :
1. **Code Tantivy minimal** (indexer + chercher 100 fichiers)
2. **UI pour afficher résultats** (liste scrollable)
3. **Tests** (vérifier que ça marche)

**Une étape à la fois = pas de perte** 💪

---

## 💡 Conseils IMPORTANTS

### ✅ À FAIRE
- ✅ Coder 1 feature à la fois
- ✅ Tester immédiatement (cargo run)
- ✅ Commit Git souvent (`git commit -m "Feature X works"`)
- ✅ Chercher des exemples sur GitHub/Google
- ✅ Copier/coller du code (normal au début)
- ✅ Demander de l'aide si bloqué >30 min

### ❌ À ÉVITER
- ❌ Essayer de tout faire en même temps
- ❌ Coder sans tester
- ❌ Réinventer la roue (utilise les libs)
- ❌ Se décourager si erreur (NORMAL)
- ❌ Vouloir du code parfait dès le début

---

## 🚀 COMMENCE MAINTENANT !

```bash
# Étape 1 : Hello World egui (30 min max)
cd D:\DataLab\xfinder
# Édite Cargo.toml
# Édite src/main.rs
cargo run

# Étape 2 : Quand ça marche, reviens me dire !
# Je te donnerai le code Tantivy pour la suite
```

**GO GO GO ! 💪**

---

**Document version :** 1.0
**Dernière mise à jour :** 2025-11-12
**Pour : Démarrage solo sans se perdre**
