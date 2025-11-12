# Instructions Claude - xfinder

**Date : 2025-11-12**
**Projet : xfinder - Recherche intelligente Windows**
**Architecture : Pure Rust + egui (pas Tauri)**

---

## Règles ABSOLUES

### 1. Commits Git
- **Committer RÉGULIÈREMENT** (toutes les heures ou dès qu'une feature marche)
- **Format commit** : `type: description` (ex: `feat: add tantivy search`)
- **Types** : `feat`, `fix`, `refactor`, `docs`, `test`, `wip`, `chore`
- **JAMAIS mentionner Claude/AI dans les commits** - auteur = développeur solo uniquement
- **PAS d'émojis dans les commits**

### 2. Code
- **Architecture TDD** : Écrire les tests en même temps que le code
- **PAS de mocks** : Tests avec vraies données
- **PAS d'émojis dans le code** (sauf UI si vraiment nécessaire)
- **Code professionnel et propre**
- **Commentaires en français** (code en anglais standard)

### 3. Documentation
- **Cohérence critique** : Si modification importante, mettre à jour tous les docs concernés
- **Référence architecture** : `docs/08_Architecture_Finale_egui.md`
- **Plan implémentation** : `IMPLEMENTATION_PLAN.md` (suivre étape par étape)

---

## État actuel du projet

### Phase : 0 (Setup) - TERMINÉ ✅
- [x] Cargo.toml créé (dépendances minimales)
- [x] src/main.rs avec Hello World egui
- [x] Tests basiques (TDD)
- [x] Compilation OK (`cargo test` + `cargo build --release`)
- [x] Premier commit effectué

### Prochaine étape : Semaine 1 - Indexation Tantivy
**Fichier** : `IMPLEMENTATION_PLAN.md` - Section "Semaine 1"
**Objectif** : Recherche basique de fichiers par nom

### Tâches Semaine 1
1. [⏭️] Tâche 1.1 : Créer structure `src/search/`
2. [⏭️] Tâche 1.2 : Setup Tantivy index basique
3. [⏭️] Tâche 1.3 : Indexer 3 fichiers test
4. [⏭️] Tâche 1.4 : Indexer dossier réel (Downloads)
5. [⏭️] Tâche 1.5 : UI résultats scrollables

---

## Stack Technique FINALE

```
UI          : egui 0.27 (immediate mode, natif Rust)
Windowing   : winit (inclus avec egui)
Rendering   : wgpu (GPU-accelerated)
Search      : Tantivy 0.22 (comme spotlight_windows)
Database    : SQLite (rusqlite 0.32)
Filesystem  : walkdir 2.4
OCR         : Tesseract (plus tard - Semaine 9+)
IA          : Candle + LEANN (plus tard - Semaine 13+)
```

---

## Commandes quotidiennes

### Tests
```bash
cargo test              # Lancer tous les tests
cargo test -- --nocapture  # Voir outputs
```

### Build
```bash
cargo build             # Debug (rapide)
cargo build --release   # Optimisé (lent mais petit)
cargo run               # Lancer l'app
```

### Git
```bash
git status              # Voir changements
git add .               # Tout ajouter
git commit -m "feat: description"  # Commit
git log --oneline -10   # Historique
```

---

## Structure actuelle

```
xfinder/
├── .claude/
│   └── instructions.md    ← CE FICHIER
├── .git/
├── docs/                  ← Documentation complète (9 fichiers)
│   └── 08_Architecture_Finale_egui.md  ← RÉFÉRENCE
├── src/
│   └── main.rs            ← Hello World egui + tests
├── target/                ← Build artifacts (gitignored)
├── Cargo.toml             ← Dépendances
├── Cargo.lock             ← Lock versions
├── .gitignore
├── IMPLEMENTATION_PLAN.md ← PLAN ÉTAPE PAR ÉTAPE
├── QUICKSTART.md
├── GIT_WORKFLOW.md
└── README.md
```

---

## Références importantes

### Inspiration
- **spotlight_windows** : https://github.com/fless-lab/spotlight_windows
  - Regarde `src/search/` pour Tantivy
  - Regarde `src/ui/` pour structure egui
  - **NE PAS copier directement** - s'inspirer seulement

### Documentation externe
- Tantivy : https://docs.rs/tantivy/latest/tantivy/
- egui : https://docs.rs/egui/latest/egui/
- rusqlite : https://docs.rs/rusqlite/latest/rusqlite/

---

## Méthodologie TDD

### Cycle Red-Green-Refactor
1. **Red** : Écrire test qui échoue
2. **Green** : Écrire code minimal qui passe
3. **Refactor** : Améliorer code
4. **Commit** : Sauvegarder

### Exemple
```rust
// 1. RED - Test échoue
#[test]
fn test_search_index_creation() {
    let index = SearchIndex::new("test_index");
    assert!(index.is_ok());
}

// 2. GREEN - Code minimal
pub struct SearchIndex {}
impl SearchIndex {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self {})
    }
}

// 3. REFACTOR - Améliorer
// (Ajouter vraie logique Tantivy)

// 4. COMMIT
git commit -m "test: add search index creation test"
```

---

## Décisions importantes

### Pourquoi egui et pas Tauri ?
- **Performance** : Startup <500ms (vs 1s Tauri)
- **Mémoire** : ~50MB (vs 80MB Tauri)
- **Natif** : 100% Rust, pas de WebView
- **Taille** : Binary ~8MB (vs 120MB+ Tauri)

### Obsolète (NE PAS UTILISER)
- ❌ `docs/02_Architecture_Technique.md` (version Tauri)
- ❌ `docs/04_API_Schemas.md` (IPC Tauri)

---

## Checklist avant chaque commit

- [ ] Tests passent : `cargo test`
- [ ] Code compile : `cargo build`
- [ ] Pas d'émojis dans code (sauf UI)
- [ ] Commentaires en français
- [ ] Message commit professionnel (pas de mention Claude)
- [ ] Format : `type: description`

---

## Notes de session

### 2025-11-12 - Setup initial
- Créé Cargo.toml avec dépendances minimales
- Créé src/main.rs avec Hello World egui
- Ajouté tests TDD basiques
- Commits effectués :
  - `docs: finalisation documentation complète`
  - `feat: hello world egui avec tests TDD`

### Prochaine session
→ Commencer Semaine 1 - Tâche 1.1 : Créer `src/search/mod.rs`
→ Suivre `IMPLEMENTATION_PLAN.md` étape par étape

---

**Dernière mise à jour** : 2025-11-12
**Prochaine action** : Semaine 1 - Tantivy indexation
