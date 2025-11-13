# xfinder

**Moteur de recherche de fichiers ultra-rapide avec surveillance temps réel**

## Description

xfinder est une application de recherche de fichiers native pour Windows, conçue pour indexer et retrouver instantanément des fichiers parmi des milliers de documents. Interface moderne, performance maximale.

## Fonctionnalités

- **Recherche instantanée**: Résultats en <100ms sur 100k+ fichiers (Tantivy)
- **Surveillance temps réel**: Le watchdog met à jour l'index automatiquement
- **Métadonnées SQLite**: Statistiques et historique persistants
- **Filtres avancés**: Type, date, taille, extension
- **Configuration persistante**: Paramètres sauvegardés automatiquement (TOML)
- **Prévisualisation**: Texte, images, audio, PDF
- **Exclusions**: Extensions, patterns, dossiers personnalisables

## Installation

```bash
# Prérequis
rustc >= 1.70
cargo >= 1.70

# Clone et build
git clone https://github.com/your-org/xfinder.git
cd xfinder
cargo build --release

# Lancer
cargo run --release
```

L'exécutable se trouve dans `target/release/xfinder.exe` (~8MB)

## Utilisation

1. **Premier lancement**: Sélectionner les dossiers à indexer
2. **Indexation**: Cliquer "Nouvelle indexation" (ou "Refresh" pour mise à jour)
3. **Recherche**: Taper dans la barre de recherche
4. **Watchdog**: Activer la surveillance automatique dans la sidebar
5. **Paramètres**: Configurer exclusions et options via ⚙️

## Configuration

Fichier: `~/.xfinder_index/config.toml`

```toml
scan_paths = ["C:\\Users\\YourName\\Downloads"]

[exclusions]
extensions = [".tmp", ".log", ".cache"]
patterns = ["node_modules", ".git", "__pycache__"]
dirs = []

[indexing]
min_ngram_size = 2
max_ngram_size = 20
max_files_to_index = 100000
no_file_limit = false

[ui]
results_display_limit = 50
watchdog_enabled = false
```

## Technologies

| Composant | Tech |
|-----------|------|
| Language | Rust |
| UI | egui + wgpu |
| Indexation | Tantivy |
| Base de données | SQLite (WAL mode) |
| Surveillance | notify-rs |
| Config | TOML + serde |

## Performance

- **Indexation**: >10,000 fichiers/sec (SSD)
- **Recherche**: <100ms (P95)
- **Mémoire (idle)**: ~50MB
- **Démarrage**: <500ms

## Licence

MIT License - Voir [LICENSE](LICENSE)

## Status

**Version**: 0.1.0
**Phase**: Core Search (Phase 1)
**Dernière mise à jour**: 2025-11-13

---

Construit avec Rust 🦀
