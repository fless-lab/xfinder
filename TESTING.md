# Guide de Test - xfinder

**Comment tester au fur et à mesure qu'on avance**

---

## Phase 0 : Setup (MAINTENANT)

### Test 1 : Vérifier que l'app se lance
```bash
cd D:\DataLab\xfinder
cargo run
```

**Résultat attendu** :
- Une fenêtre s'ouvre (800x600)
- Titre : "xfinder - Recherche intelligente"
- Une barre de recherche visible
- Si tu tapes du texte, il s'affiche en dessous

**Pour fermer** : Clique sur X ou Alt+F4

---

### Test 2 : Vérifier les tests unitaires
```bash
cargo test
```

**Résultat attendu** :
```
running 2 tests
test tests::test_app_creation ... ok
test tests::test_search_query_update ... ok

test result: ok. 2 passed
```

---

### Test 3 : Vérifier la compilation optimisée
```bash
cargo build --release
```

**Résultat attendu** :
- Compilation réussie
- Binaire créé : `target/release/xfinder.exe`

**Lancer la version optimisée** :
```bash
.\target\release\xfinder.exe
```

---

## Semaine 1 : Indexation Tantivy (PROCHAINE ÉTAPE)

### Test 4 : Indexer 3 fichiers test
```bash
cargo test test_index_files
```

**Résultat attendu** :
- 3 fichiers ajoutés à l'index
- Recherche "test" retourne résultats

---

### Test 5 : Recherche dans l'UI
```bash
cargo run
```

**Actions** :
1. Tape "test" dans la barre
2. Vérifie que des résultats apparaissent
3. Clique sur un résultat → le fichier s'ouvre

---

### Test 6 : Indexer dossier réel (Downloads)
```bash
cargo run
```

**Actions** :
1. Config → Ajouter dossier `C:\Users\TON_USER\Downloads`
2. Attendre indexation (barre de progression)
3. Rechercher un fichier que tu sais être dans Downloads
4. Vérifie qu'il apparaît dans les résultats

---

## Semaine 2 : Base de données (PLUS TARD)

### Test 7 : Métadonnées affichées
```bash
cargo run
```

**Actions** :
1. Recherche un fichier
2. Vérifie que les infos s'affichent :
   - Nom complet
   - Taille (Ko/Mo)
   - Date de modification
   - Type de fichier

---

## Semaine 3 : Watchdog (PLUS TARD)

### Test 8 : Détection ajout fichier
```bash
cargo run
```

**Actions** :
1. Laisse l'app ouverte
2. Crée un nouveau fichier dans Downloads : `test_watchdog.txt`
3. Attends 1-2 secondes
4. Recherche "watchdog" dans l'app
5. Vérifie que le fichier apparaît

---

### Test 9 : Détection déplacement fichier
```bash
cargo run
```

**Actions** :
1. Déplace `test_watchdog.txt` dans un sous-dossier
2. Recherche "watchdog"
3. Vérifie que le nouveau chemin est correct

---

## Commandes utiles au quotidien

### Compilation rapide (debug)
```bash
cargo build
```
- Temps : ~5-10s
- Binaire : `target/debug/xfinder.exe`
- Taille : ~200MB (avec debug info)

### Compilation optimisée (release)
```bash
cargo build --release
```
- Temps : ~5min (première fois), puis ~30s
- Binaire : `target/release/xfinder.exe`
- Taille : ~8MB (strippé)

### Lancer l'app
```bash
cargo run              # Version debug
cargo run --release    # Version optimisée
```

### Tests
```bash
cargo test                      # Tous les tests
cargo test test_search          # Tests contenant "search"
cargo test -- --nocapture       # Voir les println! dans tests
cargo test -- --test-threads=1  # Tests séquentiels (si besoin)
```

### Nettoyage
```bash
cargo clean  # Supprime target/ (~2GB)
```

---

## Debug si ça marche pas

### Problème : Fenêtre ne s'ouvre pas
```bash
# Vérifie les erreurs
cargo run 2>&1 | more

# Essaye version software renderer
# (Ajoute dans Cargo.toml : eframe = { version = "0.27", default-features = false })
```

### Problème : Compilation échoue
```bash
# Nettoie et rebuild
cargo clean
cargo build

# Vérifie version Rust
rustc --version
# Doit être >= 1.70

# Update Rust
rustup update
```

### Problème : Tests échouent
```bash
# Voir détails
cargo test -- --nocapture

# Lancer 1 seul test
cargo test test_app_creation
```

---

## Vérifier les performances

### Taille binaire
```bash
ls -lh target/release/xfinder.exe
# Attendu : ~8MB
```

### Temps recherche (Semaine 1+)
```bash
cargo run --release
# Dans l'app, tape une recherche
# En bas de la fenêtre : "Résultats en 45ms" (doit être <100ms)
```

### Utilisation mémoire (Semaine 2+)
- Ouvre Task Manager (Ctrl+Shift+Esc)
- Trouve "xfinder.exe"
- RAM doit être <100MB au repos

---

## Étapes de test par semaine

| Semaine | Feature | Test manuel | Résultat attendu |
|---------|---------|-------------|------------------|
| **0** | Setup | `cargo run` | Fenêtre + barre recherche |
| **1** | Tantivy | Recherche "test" | Résultats apparaissent |
| **2** | SQLite | Voir taille fichier | "1.5 Mo" affiché |
| **3** | Watchdog | Créer fichier | Apparaît auto dans index |
| **4** | Config | Choisir dossier | Indexation démarre |

---

## Questions fréquentes

### Q : Combien de temps prend la compilation ?
**R** :
- Première fois : ~5-10 min (télécharge dépendances)
- Ensuite (debug) : ~5-10s
- Release : ~30s-1min

### Q : Dois-je fermer l'app entre les tests ?
**R** : Oui, sinon `cargo run` donnera une erreur "déjà en cours"

### Q : Les tests modifient mes fichiers ?
**R** : Non, les tests créent des fichiers temporaires dans `tmp/` (nettoyés automatiquement)

---

**Dernière mise à jour** : 2025-11-12
**Prochaine étape de test** : Semaine 1 - Test 4 (indexation Tantivy)
