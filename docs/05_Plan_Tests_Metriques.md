# Plan de Tests & Métriques - xfinder
**Testing Strategy & Quality Metrics**

---

## Table des matières

1. [Stratégie de tests](#stratégie-de-tests)
2. [Tests unitaires](#tests-unitaires)
3. [Tests d'intégration](#tests-dintégration)
4. [Tests end-to-end](#tests-end-to-end)
5. [Tests de performance](#tests-de-performance)
6. [Tests utilisateurs](#tests-utilisateurs)
7. [Métriques qualité](#métriques-qualité)
8. [Benchmarks attendus](#benchmarks-attendus)

---

## Stratégie de tests

### Pyramide de tests

```
           ╱╲
          ╱E2E╲           ~10 tests (critiques)
         ╱──────╲
        ╱ Intég ╲         ~50 tests (modules)
       ╱──────────╲
      ╱  Unitaires ╲      ~200 tests (fonctions)
     ╱──────────────╲
```

### Couverture cible

| Type | Couverture | Priorité |
|------|------------|----------|
| **Unitaires** | >80% | Critique |
| **Intégration** | >60% | Haute |
| **E2E** | Parcours critiques | Haute |
| **Performance** | Benchmarks MVP | Critique |

### Outils

| Langage | Framework | Coverage | CI |
|---------|-----------|----------|-----|
| **Rust** | cargo test | tarpaulin | GitHub Actions |
| **TypeScript** | Vitest | vitest coverage | GitHub Actions |
| **E2E** | Playwright | - | GitHub Actions |

---

## Tests unitaires

### Backend Rust

#### Module Watchdog

```rust
// src/modules/watchdog/tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_detect_file_creation() {
        let temp_dir = TempDir::new().unwrap();
        let mut watcher = FileWatcher::new().unwrap();
        watcher.watch(temp_dir.path().to_path_buf()).unwrap();

        // Crée fichier
        let test_file = temp_dir.path().join("test.txt");
        std::fs::write(&test_file, "content").unwrap();

        // Attend event
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Vérifie event reçu
        // TODO: Ajouter receiver dans struct pour tests
    }

    #[tokio::test]
    async fn test_debouncing() {
        // Vérifie que 100 modifications rapides → 1 seul event
        let temp_dir = TempDir::new().unwrap();
        let mut watcher = FileWatcher::new().unwrap();

        for i in 0..100 {
            let file = temp_dir.path().join(format!("file_{}.txt", i));
            std::fs::write(file, "test").unwrap();
        }

        tokio::time::sleep(Duration::from_secs(1)).await;

        // Assert: Événements groupés en batch
    }

    #[test]
    fn test_exclusion_filter() {
        let filter = ExclusionFilter::new(vec![
            ExclusionRule::Extension(["tmp", "log"].iter().map(|s| s.to_string()).collect()),
        ]);

        assert!(!filter.should_index(&Path::new("test.tmp"), &metadata));
        assert!(filter.should_index(&Path::new("test.txt"), &metadata));
    }
}
```

#### Module Content Extractor

```rust
// src/modules/content_extractor/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_text_extraction() {
        let pdf_path = "tests/fixtures/sample.pdf";
        let result = extract_pdf_text(Path::new(pdf_path)).unwrap();

        assert!(!result.is_empty());
        assert!(result[0].text.contains("expected content"));
    }

    #[test]
    fn test_docx_extraction() {
        let docx_path = "tests/fixtures/sample.docx";
        let text = extract_docx_text(Path::new(docx_path)).unwrap();

        assert!(text.contains("Document Title"));
    }

    #[test]
    fn test_pdf_has_text_layer_detection() {
        assert!(pdf_has_text_layer(Path::new("tests/fixtures/text.pdf")).unwrap());
        assert!(!pdf_has_text_layer(Path::new("tests/fixtures/scanned.pdf")).unwrap());
    }

    #[test]
    fn test_ocr_should_process() {
        let config = OcrConfig {
            enabled: true,
            file_types: vec!["pdf".to_string()],
            min_size_kb: 100,
            ..Default::default()
        };

        let ocr_engine = OcrEngine::new(config).unwrap();

        let file = FileMetadata {
            extension: "pdf".to_string(),
            size: 200_000, // 200 KB
            ..Default::default()
        };

        assert!(ocr_engine.should_ocr(&file));
    }
}
```

#### Module Search Engine

```rust
// src/modules/search_engine/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_fast_mode() {
        let engine = create_test_search_engine().await;

        // Index fichiers tests
        engine.index_file("test.pdf", "content").await.unwrap();
        engine.index_file("document.docx", "text").await.unwrap();

        let results = engine.search(
            "test",
            SearchMode::Fast,
            SearchFilters::default(),
        ).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].filename, "test.pdf");
    }

    #[tokio::test]
    async fn test_fuzzy_matching() {
        let engine = create_test_search_engine().await;
        engine.index_file("contrat_dupont.pdf", "").await.unwrap();

        // Typo : "cntrat dpon"
        let results = engine.search("cntrat dpon", SearchMode::Fast, SearchFilters::default())
            .await.unwrap();

        assert!(!results.is_empty(), "Fuzzy matching devrait trouver le fichier");
    }

    #[tokio::test]
    async fn test_filters_combination() {
        let engine = create_test_search_engine().await;

        // Index fichiers avec dates différentes
        // ...

        let filters = SearchFilters {
            extensions: Some(vec![".pdf".to_string()]),
            date_after: Some(Utc::now().timestamp() - 86400),
            size_min: Some(1000),
            ..Default::default()
        };

        let results = engine.search("", SearchMode::Fast, filters).await.unwrap();

        // Vérifie tous les résultats matchent filtres
        for result in results {
            assert_eq!(result.extension, ".pdf");
            assert!(result.size >= 1000);
        }
    }
}
```

#### Module AI Engine (LEANN)

```rust
// src/modules/ai_engine/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_embedding_generation() {
        let model = EmbeddingModel::load("all-MiniLM-L6-v2").unwrap();

        let text = "Ceci est un test d'embedding en français.";
        let embedding = model.encode(text).unwrap();

        assert_eq!(embedding.len(), 384); // Dimension attendue
        assert!(embedding.iter().all(|&x| x.is_finite()));
    }

    #[tokio::test]
    async fn test_semantic_similarity() {
        let model = EmbeddingModel::load("all-MiniLM-L6-v2").unwrap();

        let emb1 = model.encode("budget formation").unwrap();
        let emb2 = model.encode("coûts formation professionnelle").unwrap();
        let emb3 = model.encode("recette cuisine").unwrap();

        let sim_12 = cosine_similarity(&emb1, &emb2);
        let sim_13 = cosine_similarity(&emb1, &emb3);

        assert!(sim_12 > sim_13, "Textes similaires devraient avoir score plus élevé");
        assert!(sim_12 > 0.5, "Similarité sémantique attendue");
    }

    #[tokio::test]
    async fn test_leann_index_search() {
        let mut index = LeannIndex::new("tests/temp_index").unwrap();

        // Ajoute documents
        index.add_document("doc1", "Formation professionnelle budget 2024").unwrap();
        index.add_document("doc2", "Recette tarte aux pommes").unwrap();
        index.add_document("doc3", "Coûts formation agents administratifs").unwrap();

        // Recherche
        let results = index.search("budget formation", 5).unwrap();

        assert_eq!(results[0].file_id, "doc1");
        assert!(results[0].score > results[1].score);
    }
}
```

### Frontend TypeScript

```typescript
// src/components/Search/SearchBar.test.tsx

import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { SearchBar } from './SearchBar';

describe('SearchBar', () => {
  it('renders search input', () => {
    render(<SearchBar onSearch={vi.fn()} />);
    expect(screen.getByPlaceholderText('Rechercher...')).toBeInTheDocument();
  });

  it('calls onSearch when typing', async () => {
    const onSearch = vi.fn();
    render(<SearchBar onSearch={onSearch} />);

    const input = screen.getByPlaceholderText('Rechercher...');
    fireEvent.change(input, { target: { value: 'test query' } });

    // Debounce 300ms
    await new Promise(r => setTimeout(r, 350));

    expect(onSearch).toHaveBeenCalledWith('test query');
  });

  it('displays loading state', () => {
    render(<SearchBar onSearch={vi.fn()} loading={true} />);
    expect(screen.getByText('Recherche...')).toBeInTheDocument();
  });
});
```

```typescript
// src/api/tauri.test.ts

import { describe, it, expect, vi } from 'vitest';
import { searchFiles } from './tauri';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/tauri', () => ({
  invoke: vi.fn(),
}));

describe('Tauri API', () => {
  it('searchFiles calls correct command', async () => {
    const { invoke } = await import('@tauri-apps/api/tauri');

    (invoke as any).mockResolvedValue([
      { id: '1', filename: 'test.pdf', path: '/test.pdf', score: 0.9 },
    ]);

    const results = await searchFiles('test', {});

    expect(invoke).toHaveBeenCalledWith('search_files', {
      query: 'test',
      filters: {},
      limit: 100,
    });

    expect(results).toHaveLength(1);
    expect(results[0].filename).toBe('test.pdf');
  });
});
```

---

## Tests d'intégration

### Indexation complète

```rust
// tests/integration/indexing.rs

#[tokio::test]
async fn test_full_indexing_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Crée corpus test
    create_test_corpus(&temp_dir);

    // Initialise app
    let app = AppState::new_for_testing().await;

    // Démarre indexation
    let options = IndexingOptions {
        paths: vec![temp_dir.path().to_path_buf()],
        mode: IndexingMode::Full,
        enable_ocr: false,
        enable_embeddings: true,
    };

    app.indexer.lock().await.index_folders(options).await.unwrap();

    // Vérifie résultats
    let stats = app.get_statistics().await.unwrap();
    assert_eq!(stats.total_files_indexed, 10);

    // Teste recherche
    let results = app.search_engine.lock().await
        .search("test", SearchMode::Fast, SearchFilters::default())
        .await.unwrap();

    assert!(!results.is_empty());
}

fn create_test_corpus(dir: &TempDir) {
    std::fs::write(dir.path().join("doc1.txt"), "Test content").unwrap();
    std::fs::write(dir.path().join("doc2.pdf"), include_bytes!("fixtures/sample.pdf")).unwrap();
    // ...
}
```

### Watchdog + Indexation temps réel

```rust
#[tokio::test]
async fn test_watchdog_auto_indexing() {
    let temp_dir = TempDir::new().unwrap();
    let app = AppState::new_for_testing().await;

    // Démarre watchdog
    app.start_watching(temp_dir.path()).await.unwrap();

    // Crée nouveau fichier
    let new_file = temp_dir.path().join("new.txt");
    std::fs::write(&new_file, "New content").unwrap();

    // Attend indexation
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Vérifie fichier indexé
    let results = app.search_engine.lock().await
        .search("new", SearchMode::Fast, SearchFilters::default())
        .await.unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].filename, "new.txt");
}
```

### Email parsing + Recherche

```rust
#[tokio::test]
async fn test_email_indexing_and_search() {
    let app = AppState::new_for_testing().await;

    // Index emails test
    let mbox_path = "tests/fixtures/sample.mbox";
    let source = EmailSource {
        source_type: EmailSourceType::Thunderbird,
        path: Some(PathBuf::from(mbox_path)),
        ..Default::default()
    };

    app.email_parser.lock().await
        .index_source(source).await.unwrap();

    // Recherche dans emails
    let results = app.search_engine.lock().await
        .search_emails("budget", SearchFilters::default())
        .await.unwrap();

    assert!(!results.is_empty());
    assert!(results[0].subject.contains("budget"));
}
```

---

## Tests end-to-end

### Parcours critique 1 : Installation → Première recherche

```typescript
// e2e/first-use.spec.ts

import { test, expect } from '@playwright/test';

test('Installation et première recherche', async ({ page }) => {
  // Lance app
  await page.goto('tauri://localhost');

  // Écran accueil assistant config
  await expect(page.getByText('Bienvenue dans xfinder')).toBeVisible();

  // Sélectionne dossier Documents
  await page.getByRole('checkbox', { name: 'Documents' }).check();
  await page.getByRole('button', { name: 'Démarrer indexation' }).click();

  // Attend fin indexation (timeout 5 min)
  await expect(page.getByText('Indexation terminée')).toBeVisible({ timeout: 300000 });

  // Effectue recherche
  await page.getByPlaceholder('Rechercher...').fill('test');

  // Vérifie résultats
  await expect(page.getByTestId('search-result')).toHaveCount.greaterThan(0);
});
```

### Parcours critique 2 : Mode Assist Me

```typescript
// e2e/assist-me.spec.ts

test('Mode Assist Me avec sources', async ({ page }) => {
  await page.goto('tauri://localhost');

  // Active mode Assist Me
  await page.getByRole('button', { name: 'Assist Me' }).click();

  // Pose question
  await page.getByPlaceholder('Posez votre question...').fill(
    'Quels sont les budgets formation validés ?'
  );
  await page.keyboard.press('Enter');

  // Attend réponse
  await expect(page.getByTestId('assist-response')).toBeVisible({ timeout: 10000 });

  // Vérifie sources cliquables
  const sources = page.getByTestId('source-link');
  await expect(sources).toHaveCount.greaterThan(0);

  // Clique source → ouvre fichier
  await sources.first().click();

  // TODO: Vérifier fichier ouvert (compliqué cross-platform)
});
```

### Parcours critique 3 : Configuration exclusions

```typescript
// e2e/exclusions.spec.ts

test('Configuration exclusions granulaires', async ({ page }) => {
  await page.goto('tauri://localhost');

  // Ouvre paramètres
  await page.getByRole('button', { name: 'Paramètres' }).click();

  // Va dans dossiers surveillés
  await page.getByRole('tab', { name: 'Dossiers' }).click();

  // Développe arborescence
  await page.getByText('Documents').click();

  // Exclut sous-dossier
  await page.getByText('Archives').rightClick();
  await page.getByRole('menuitem', { name: 'Exclure ce dossier' }).click();

  // Sauvegarde
  await page.getByRole('button', { name: 'Sauvegarder' }).click();

  // Vérifie persistence
  await page.reload();
  await page.getByRole('button', { name: 'Paramètres' }).click();
  await expect(page.getByText('Archives').locator('..').getByTestId('excluded-badge')).toBeVisible();
});
```

---

## Tests de performance

### Benchmarks Rust (criterion)

```rust
// benches/search_benchmark.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_search_speed(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let engine = rt.block_on(create_test_engine_with_10k_files());

    c.bench_function("search_fast_10k", |b| {
        b.to_async(&rt).iter(|| async {
            engine.search(
                black_box("test query"),
                SearchMode::Fast,
                SearchFilters::default(),
            ).await
        });
    });
}

fn bench_indexing_speed(c: &mut Criterion) {
    let mut group = c.benchmark_group("indexing");

    for file_count in [100, 1000, 10000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(file_count),
            &file_count,
            |b, &count| {
                b.iter(|| {
                    // Index N fichiers
                    index_n_files(black_box(count))
                });
            },
        );
    }

    group.finish();
}

fn bench_ocr_speed(c: &mut Criterion) {
    let ocr_engine = OcrEngine::new(OcrConfig::default()).unwrap();

    c.bench_function("ocr_a4_page_300dpi", |b| {
        b.iter(|| {
            ocr_engine.extract_text_from_image(
                black_box(Path::new("tests/fixtures/a4_page.jpg"))
            )
        });
    });
}

criterion_group!(benches, bench_search_speed, bench_indexing_speed, bench_ocr_speed);
criterion_main!(benches);
```

### Benchmarks cibles

| Test | Objectif | Acceptable | Critique |
|------|----------|------------|----------|
| **Recherche nom (10k fichiers)** | <50ms | <100ms | >200ms |
| **Recherche nom (100k fichiers)** | <100ms | <200ms | >500ms |
| **Recherche sémantique** | <2s | <5s | >10s |
| **Indexation vitesse** | >1000 files/min | >500 files/min | <200 files/min |
| **OCR page A4** | <5s | <10s | >20s |
| **Génération embedding** | <50ms | <100ms | >200ms |
| **Démarrage app** | <2s | <5s | >10s |
| **Mémoire idle** | <300MB | <500MB | >1GB |
| **Mémoire indexation** | <1.5GB | <2GB | >3GB |

### Script de test de charge

```bash
#!/bin/bash
# tests/load_test.sh

echo "=== xfinder Load Test ==="

# 1. Crée corpus test
echo "Création corpus 100k fichiers..."
python scripts/generate_corpus.py --count 100000 --output ./test_corpus

# 2. Démarre profiling
echo "Démarrage profiling..."
cargo build --release --features profiling

# 3. Lance indexation
echo "Indexation 100k fichiers..."
time ./target/release/xfinder index ./test_corpus

# 4. Mesures
echo "Mesures performance..."

# Vitesse indexation
DURATION=$(grep "Indexation terminée" logs/xfinder.log | tail -1 | awk '{print $NF}')
FILES_PER_MIN=$(echo "scale=2; 100000 / ($DURATION / 60)" | bc)
echo "Vitesse: $FILES_PER_MIN fichiers/min"

# Taille index
INDEX_SIZE=$(du -sh ./data/index.db | awk '{print $1}')
echo "Taille index: $INDEX_SIZE"

# Mémoire peak
PEAK_MEM=$(grep "Peak memory" logs/xfinder.log | tail -1 | awk '{print $NF}')
echo "Mémoire peak: $PEAK_MEM"

# 5. Tests recherche
echo "Tests recherche..."

for i in {1..100}; do
  time ./target/release/xfinder search "test query $i" >> search_times.log
done

AVG_SEARCH=$(awk '{sum+=$1; count++} END {print sum/count}' search_times.log)
echo "Temps recherche moyen: ${AVG_SEARCH}ms"

# 6. Génère rapport
python scripts/generate_report.py
```

---

## Tests utilisateurs

### Alpha testing (interne)

**Participants :** 5 employés équipe + 2 externes

**Scénarios :**
1. Installation sans aide
2. Configuration dossiers perso
3. 10 recherches réalistes
4. Mode Assist Me (3 questions)
5. Feedback questionnaire

**Métriques :**
- Temps installation (cible <5 min)
- Taux succès recherche (cible >80%)
- SUS score (System Usability Scale, cible >70)

### Beta testing (pilotes)

**Participants :** 20-50 agents administratifs volontaires

**Durée :** 4 semaines

**Métriques collectées :**
- Nombre recherches/jour
- Temps moyen recherche
- Taux trouvaille (avec feedback utilisateur)
- Bugs/crashes reportés
- NPS (Net Promoter Score)

**Questionnaire beta :**
```
1. xfinder répond-il à vos besoins de recherche ?
   [ ] Totalement [ ] Partiellement [ ] Pas vraiment

2. Quelle fonctionnalité utilisez-vous le plus ?
   [ ] Recherche rapide [ ] Recherche contenu [ ] Assist Me [ ] Emails

3. Qu'aimeriez-vous améliorer en priorité ?
   [Texte libre]

4. Recommanderiez-vous xfinder à un collègue ? (NPS)
   [0-10]

5. Bugs rencontrés :
   [Texte libre]
```

---

## Métriques qualité

### Code quality

| Métrique | Outil | Cible |
|----------|-------|-------|
| **Couverture tests** | tarpaulin, vitest | >80% |
| **Complexité cyclomatique** | cargo-geiger | <10 par fonction |
| **Warnings** | clippy | 0 |
| **Vulnérabilités** | cargo-audit | 0 critique |
| **Duplications** | jscpd | <3% |
| **Documentation** | rustdoc | >90% publics |

### CI/CD Checks

```yaml
# .github/workflows/ci.yml

name: CI

on: [push, pull_request]

jobs:
  test-rust:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable

      - name: Run tests
        run: cargo test --all-features

      - name: Run clippy
        run: cargo clippy -- -D warnings

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Security audit
        run: cargo audit

      - name: Coverage
        run: |
          cargo install tarpaulin
          cargo tarpaulin --out Xml

      - name: Upload coverage
        uses: codecov/codecov-action@v3

  test-frontend:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Install deps
        run: npm ci

      - name: Run tests
        run: npm test

      - name: Lint
        run: npm run lint

      - name: Type check
        run: npm run type-check

  benchmark:
    runs-on: windows-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v3

      - name: Run benchmarks
        run: cargo bench

      - name: Store results
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: target/criterion/report/index.html

  e2e:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3

      - name: Build app
        run: npm run tauri build

      - name: Run E2E tests
        run: npx playwright test

      - name: Upload test results
        uses: actions/upload-artifact@v3
        if: failure()
        with:
          name: playwright-report
          path: playwright-report/
```

---

## Benchmarks attendus

### Résultats référence (matériel cible)

**Config test :**
- CPU : Intel i5-10400 (6 cores)
- RAM : 16GB DDR4
- SSD : 500GB NVMe
- OS : Windows 11 Pro

**Corpus test :**
- 100,000 fichiers
- Mix : 60% Office, 30% PDF, 10% TXT
- Taille totale : ~50GB
- 20,000 PDF scannés (pour OCR)

### Résultats MVP attendus

| Opération | Résultat | Status |
|-----------|----------|--------|
| **Indexation initiale** | 45 min (2222 files/min) | ✅ Acceptable |
| **Taille index SQLite** | 1.2GB (2.4% corpus) | ✅ Excellent |
| **Taille index LEANN** | 450MB | ✅ Excellent |
| **Recherche nom (100k)** | 82ms avg | ✅ Excellent |
| **Recherche contenu** | 245ms avg | ✅ Bon |
| **Recherche sémantique** | 1.8s avg | ✅ Bon |
| **OCR page A4** | 4.2s avg | ✅ Excellent |
| **Embedding génération** | 38ms avg | ✅ Excellent |
| **Démarrage app** | 1.4s | ✅ Excellent |
| **Mémoire idle** | 280MB | ✅ Excellent |
| **Mémoire indexation peak** | 1.8GB | ✅ Acceptable |
| **CPU usage idle** | 0.5% | ✅ Excellent |
| **CPU usage indexation** | 75% avg | ✅ Bon |

### Critères de réussite MVP

**DOIT (bloquant) :**
- ✅ Recherche <100ms (10k fichiers)
- ✅ Indexation >500 files/min
- ✅ Mémoire idle <500MB
- ✅ 0 crashes sur tests standards
- ✅ Couverture tests >70%

**DEVRAIT (important) :**
- ✅ Recherche sémantique <3s
- ✅ OCR <10s/page
- ✅ Taille index <5% corpus
- ✅ Démarrage <3s

**PEUT (nice to have) :**
- ⚪ Recherche <50ms (10k fichiers)
- ⚪ Indexation >1000 files/min
- ⚪ Mémoire idle <300MB

---

## Dashboard métriques

### Grafana/Prometheus (optionnel production)

```yaml
# prometheus.yml (si déploiement entreprise)

scrape_configs:
  - job_name: 'xfinder'
    static_configs:
      - targets: ['localhost:9090']

# Métriques exposées
metrics:
  - xfinder_search_duration_seconds
  - xfinder_indexing_files_total
  - xfinder_indexing_errors_total
  - xfinder_memory_usage_bytes
  - xfinder_active_users_total
```

### Telemetry locale (privacy-first)

```rust
// Stocké localement, jamais envoyé
pub struct LocalTelemetry {
    pub metrics: HashMap<String, Metric>,
}

pub enum Metric {
    Counter { value: u64 },
    Histogram { buckets: Vec<f64> },
    Gauge { value: f64 },
}

// Exemple usage
telemetry.record("search_duration_ms", 85.0);
telemetry.increment("searches_total");

// Export volontaire pour support
telemetry.export_for_support("support_data.json");
```

---

## Checklist pré-release

### MVP Release Criteria

**Fonctionnel :**
- [ ] Indexation 100k fichiers sans crash
- [ ] Recherche nom fonctionne (fuzzy)
- [ ] Recherche contenu fonctionne
- [ ] Watchdog détecte changements
- [ ] Configuration dossiers persistée
- [ ] Exclusions appliquées correctement

**Performance :**
- [ ] Recherche <100ms (10k fichiers)
- [ ] Indexation >500 files/min
- [ ] Mémoire <500MB idle

**Qualité :**
- [ ] 0 crashes sur tests E2E
- [ ] Couverture tests >70%
- [ ] 0 warnings clippy
- [ ] 0 vulnérabilités critiques

**UX :**
- [ ] Installation <5 min
- [ ] Interface responsive
- [ ] Raccourci global fonctionne
- [ ] Progression indexation visible

**Documentation :**
- [ ] README complet
- [ ] Guide utilisateur
- [ ] Guide installation IT
- [ ] CHANGELOG

---

**Document version :** 1.0
**Dernière mise à jour :** 2025-11-12
**Prochaine revue :** Après alpha tests (semaine 8)
