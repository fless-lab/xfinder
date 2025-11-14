# Architecture Dual-Mode : Classique vs Assist Me
**xfinder - Coexistence recherche classique + IA sémantique**

---

## 🎯 Vue d'ensemble

xfinder va avoir **2 MODES COMPLÉMENTAIRES** qui coexistent :

### **Mode 1 : Recherche Classique** ⚡ (déjà implémenté)
- **Usage** : Chercher un fichier par nom, extension, date
- **Technologie** : Tantivy (n-grams) + SQLite
- **Performance** : <100ms
- **Toujours actif** : C'est le mode par défaut

### **Mode 2 : Assist Me** 🤖 (à implémenter)
- **Usage** : Poser des questions en langage naturel
- **Technologie** : LEANN (embeddings) + Candle
- **Performance** : <3s
- **Activable/désactivable** : L'utilisateur choisit

**Les deux modes utilisent la MÊME base de données SQLite !**

---

## 🎨 UI - Comment basculer entre les modes ?

### **Option A : Onglets (RECOMMANDÉ)** ✅

```
┌─────────────────────────────────────────────────────────────┐
│  xfinder                                      [_][□][X]     │
├─────────────────────────────────────────────────────────────┤
│  [🔍 Recherche] [🤖 Assist Me]                    [⚙️]      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  MODE RECHERCHE CLASSIQUE                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ 🔍 contrat dupont                                    │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
│  Filtres : Type [Tous ▼] Date [Tous ▼] Taille [Tous ▼]    │
│                                                              │
│  Résultats (234) :                                          │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ 📄 Contrat_Dupont_2024.pdf                           │  │
│  │    C:\Users\Admin\Documents\Contrats\                │  │
│  │    Modifié : 15/03/2024 - 2.3 MB                     │  │
│  │    [Ouvrir] [Dossier]                                │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │ 📄 Contrat_Dupont_Marie_Signature.pdf                │  │
│  │    C:\Users\Admin\Bureau\À_traiter\                  │  │
│  │    Modifié : 20/03/2024 - 890 KB                     │  │
│  │    [Ouvrir] [Dossier]                                │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

**Clic sur onglet "🤖 Assist Me" :**

```
┌─────────────────────────────────────────────────────────────┐
│  xfinder                                      [_][□][X]     │
├─────────────────────────────────────────────────────────────┤
│  [🔍 Recherche] [🤖 Assist Me]                    [⚙️]      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  MODE ASSIST ME (IA Sémantique)                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ 💬 Posez votre question...                           │  │
│  └──────────────────────────────────────────────────────┘  │
│  [🔍 Rechercher]                                            │
│                                                              │
│  ── Historique ──────────────────────────────────────────   │
│  • Quels sont les budgets formation 2024 ?                  │
│  • Retrouve les échanges avec Marie sur RGPD                │
│                                                              │
│  ── Suggestions ─────────────────────────────────────────   │
│  💡 "Trouve mes factures EDF 2024"                          │
│  💡 "Contrats signés ce mois"                               │
│  💡 "Emails avec pièces jointes importantes"                │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

**Après avoir posé une question :**

```
┌─────────────────────────────────────────────────────────────┐
│  xfinder                                      [_][□][X]     │
├─────────────────────────────────────────────────────────────┤
│  [🔍 Recherche] [🤖 Assist Me]                    [⚙️]      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  💬 Quels sont les budgets formation validés en 2024 ?      │
│                                                              │
│  🤖 J'ai trouvé 3 budgets formation validés en 2024 :       │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ #1 - Budget "Compétences numériques" - 45 000€      │  │
│  │                                                       │  │
│  │ "... formation aux outils numériques pour l'ensemble │  │
│  │ du personnel administratif. Budget total: 45000€     │  │
│  │ validé en comité du 12 janvier 2024 ..."            │  │
│  │                                                       │  │
│  │ 📄 Budget_Formation_2024.pdf (page 3)                │  │
│  │ 📁 C:\Users\Admin\Documents\RH\                      │  │
│  │ Pertinence: 94%                                       │  │
│  │ [Ouvrir fichier] [Ouvrir dossier]                    │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │ #2 - Budget "Management interculturel" - 28 500€    │  │
│  │                                                       │  │
│  │ "... sessions de formation au management inter-      │  │
│  │ culturel programmées pour Q2 2024. Montant           │  │
│  │ approuvé: 28500 euros TTC ..."                       │  │
│  │                                                       │  │
│  │ 📄 Formation_Q1_2024.docx (section 2.3)              │  │
│  │ 📁 \\Serveur\RH\Validations\                         │  │
│  │ Pertinence: 88%                                       │  │
│  │ [Ouvrir fichier] [Ouvrir dossier]                    │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │ #3 - Budget "Cybersécurité agents" - 67 000€        │  │
│  │                                                       │  │
│  │ 📧 Email de DG - 22/04/2024                          │  │
│  │ "RE: Validation budget cyber"                        │  │
│  │ 📎 Devis_Formation_Cyber.xlsx                        │  │
│  │ Pertinence: 85%                                       │  │
│  │ [Ouvrir email] [Ouvrir PJ]                           │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
│  💰 Total : 140 500€                                        │
│  [Copier résultats] [Exporter PDF] [Nouvelle question]     │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

---

### **Code UI - Implémentation egui**

```rust
// src/app.rs

pub enum AppMode {
    ClassicSearch,
    AssistMe,
}

pub struct XFinderApp {
    // ... champs existants

    pub current_mode: AppMode,

    // Mode classique (déjà existant)
    pub search_query: String,
    pub search_results: Vec<SearchResult>,

    // Mode Assist Me (nouveau)
    pub assist_me_query: String,
    pub assist_me_conversation: Vec<QAPair>, // Historique
    pub assist_me_sources: Vec<Source>,
    pub assist_me_loading: bool,
}

impl eframe::App for XFinderApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Lazy init
        self.lazy_init();

        // Top panel avec onglets
        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("xfinder");

                ui.separator();

                // Onglets
                if ui.selectable_label(
                    matches!(self.current_mode, AppMode::ClassicSearch),
                    "🔍 Recherche"
                ).clicked() {
                    self.current_mode = AppMode::ClassicSearch;
                }

                if ui.selectable_label(
                    matches!(self.current_mode, AppMode::AssistMe),
                    "🤖 Assist Me"
                ).clicked() {
                    self.current_mode = AppMode::AssistMe;

                    // Init Assist Me si pas déjà fait
                    if self.assist_me_engine.is_none() {
                        self.init_assist_me();
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙️").clicked() {
                        self.show_settings_modal = true;
                    }
                });
            });
        });

        // Panel central selon le mode
        match self.current_mode {
            AppMode::ClassicSearch => {
                render_classic_search_ui(self, ctx);
            }
            AppMode::AssistMe => {
                render_assist_me_ui(self, ctx);
            }
        }

        // ... reste (modals, side panel, etc.)
    }
}

// src/ui/assist_me_ui.rs

pub fn render_assist_me_ui(app: &mut XFinderApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("🤖 Assist Me - Recherche Intelligente");
            ui.add_space(10.0);

            // Input question
            let response = ui.add_sized(
                [600.0, 40.0],
                egui::TextEdit::singleline(&mut app.assist_me_query)
                    .hint_text("💬 Posez votre question...")
                    .font(egui::FontId::proportional(16.0))
            );

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                app.process_assist_me_query();
            }

            ui.add_space(10.0);

            if ui.button("🔍 Rechercher").clicked() {
                app.process_assist_me_query();
            }
        });

        ui.separator();

        // Affichage résultats
        if app.assist_me_loading {
            ui.vertical_centered(|ui| {
                ui.spinner();
                ui.label("🔍 Recherche sémantique en cours...");
            });
        } else if !app.assist_me_sources.is_empty() {
            // Afficher sources
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(10.0);
                ui.label(format!("📚 {} sources pertinentes trouvées", app.assist_me_sources.len()));
                ui.add_space(10.0);

                for (idx, source) in app.assist_me_sources.iter().enumerate() {
                    render_source_card(ui, idx + 1, source);
                    ui.add_space(10.0);
                }
            });
        } else if app.assist_me_query.is_empty() {
            // État vide : suggestions
            render_suggestions(ui);
        } else {
            // Aucun résultat
            ui.vertical_centered(|ui| {
                ui.label("❌ Aucune source trouvée pour cette question");
                ui.label("💡 Essayez de reformuler ou d'utiliser d'autres mots-clés");
            });
        }
    });
}

fn render_suggestions(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        ui.heading("💡 Exemples de questions");
        ui.add_space(10.0);

        let suggestions = vec![
            "Trouve mes factures EDF de 2024",
            "Quels sont les contrats signés ce mois ?",
            "Emails avec pièces jointes importantes",
            "Documents RGPD modifiés récemment",
            "Budget formation validé en janvier",
        ];

        for suggestion in suggestions {
            if ui.button(format!("💬 {}", suggestion)).clicked() {
                // TODO: Remplir query avec suggestion
            }
            ui.add_space(5.0);
        }
    });
}
```

---

## 🏗️ Architecture d'indexation HYBRIDE

### **Principe : Indexation en 2 PHASES séparées**

```
NOUVEAU FICHIER DÉTECTÉ
    ↓
┌─────────────────────────────────────────────┐
│ PHASE 1 : INDEXATION RAPIDE (toujours)     │ ⚡ <1s
│ ─────────────────────────────────────────── │
│ 1. Extraire métadonnées (nom, taille, date)│
│ 2. Indexer Tantivy (nom + extension)       │
│ 3. Insérer SQLite (metadata)               │
│                                              │
│ ✅ Fichier immédiatement searchable         │
│    en mode Recherche Classique              │
└─────────────────────────────────────────────┘
    ↓
    │ (si Assist Me activé)
    ↓
┌─────────────────────────────────────────────┐
│ PHASE 2 : INDEXATION IA (optionnelle)      │ 🐌 1-5s/fichier
│ ─────────────────────────────────────────── │
│ 1. Extraire contenu texte (PDF, DOCX)      │
│ 2. Chunking intelligent (500 tokens)       │
│ 3. Générer embeddings (Candle)             │
│ 4. Indexer LEANN (vector search)           │
│ 5. Stocker chunks dans SQLite              │
│                                              │
│ ✅ Fichier searchable en mode Assist Me     │
└─────────────────────────────────────────────┘
```

### **Avantages de cette approche :**

✅ **Mode classique toujours rapide** : Pas d'attente embeddings
✅ **Assist Me optionnel** : L'utilisateur active si besoin
✅ **Background progressif** : Embeddings calculés sans bloquer
✅ **Graceful degradation** : Assist Me marche même si indexation IA incomplète

---

## ⚙️ Détails d'implémentation

### **1. Configuration utilisateur (Settings)**

```rust
// src/config/mod.rs

pub struct AssistMeConfig {
    pub enabled: bool,                // Activer Assist Me ?
    pub auto_index_new_files: bool,   // Auto-indexer nouveaux fichiers ?
    pub batch_size: usize,            // Nb fichiers à indexer par batch
    pub model_path: String,           // Chemin modèle embeddings
}

impl Default for AssistMeConfig {
    fn default() -> Self {
        Self {
            enabled: false,           // Désactivé par défaut
            auto_index_new_files: true,
            batch_size: 10,
            model_path: "models/all-MiniLM-L6-v2".to_string(),
        }
    }
}
```

**UI Settings :**

```
┌─────────────────────────────────────────┐
│ Paramètres - Assist Me                 │
├─────────────────────────────────────────┤
│                                          │
│ ☑ Activer la recherche sémantique      │
│                                          │
│ Options :                                │
│  ☑ Indexer automatiquement nouveaux     │
│    fichiers (en arrière-plan)           │
│                                          │
│  Batch : [10] fichiers simultanés       │
│                                          │
│ État :                                   │
│  📊 12,450 / 15,000 fichiers indexés    │
│  [Indexer tous maintenant] [Pause]      │
│                                          │
│ Stockage :                               │
│  💾 Index LEANN : 8.2 MB                │
│  💾 Chunks DB : 45.3 MB                 │
│                                          │
└─────────────────────────────────────────┘
```

---

### **2. Pipeline d'indexation**

```rust
// src/app.rs

impl XFinderApp {
    /// Indexe un fichier (appelé par watchdog ou scan manuel)
    pub fn index_file(&mut self, file_path: &Path) -> Result<()> {
        // ═══════════════════════════════════════════════
        // PHASE 1 : INDEXATION RAPIDE (TOUJOURS)
        // ═══════════════════════════════════════════════

        // 1. Métadonnées
        let metadata = std::fs::metadata(file_path)?;
        let filename = file_path.file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        // 2. Tantivy index (nom fichier)
        if let Some(ref index) = self.search_index {
            index.add_file(&file_path.to_string_lossy(), &filename)?;
        }

        // 3. SQLite (metadata)
        if let Some(ref db) = self.database {
            db.insert_file(
                &file_path.to_string_lossy(),
                &filename,
                metadata.len(),
                metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs() as i64,
            )?;
        }

        // ✅ PHASE 1 TERMINÉE - Fichier searchable en mode classique

        // ═══════════════════════════════════════════════
        // PHASE 2 : INDEXATION IA (SI ACTIVÉ)
        // ═══════════════════════════════════════════════

        if self.config.assist_me.enabled && self.config.assist_me.auto_index_new_files {
            // Ajouter à la queue d'indexation background
            self.enqueue_semantic_indexing(file_path.to_path_buf());
        }

        Ok(())
    }

    /// Queue pour indexation sémantique background
    fn enqueue_semantic_indexing(&mut self, file_path: PathBuf) {
        // Envoyer dans channel background
        if let Some(ref tx) = self.semantic_indexing_tx {
            let _ = tx.send(file_path);
        }
    }
}
```

---

### **3. Thread background pour embeddings**

```rust
// src/app.rs - dans start_indexing() ou au démarrage

// Créer thread background pour indexation sémantique
let (semantic_tx, semantic_rx) = unbounded::<PathBuf>();
self.semantic_indexing_tx = Some(semantic_tx);

let semantic_indexer = self.semantic_indexer.clone(); // Arc<SemanticIndexer>
let batch_size = self.config.assist_me.batch_size;

std::thread::spawn(move || {
    let mut batch = Vec::new();

    loop {
        // Recevoir fichiers à indexer
        match semantic_rx.recv_timeout(Duration::from_secs(5)) {
            Ok(file_path) => {
                batch.push(file_path);

                // Si batch plein, traiter
                if batch.len() >= batch_size {
                    process_semantic_batch(&semantic_indexer, &batch);
                    batch.clear();
                }
            }
            Err(_) => {
                // Timeout : traiter batch partiel si non vide
                if !batch.is_empty() {
                    process_semantic_batch(&semantic_indexer, &batch);
                    batch.clear();
                }
            }
        }
    }
});

fn process_semantic_batch(indexer: &SemanticIndexer, files: &[PathBuf]) {
    for file_path in files {
        // Extract → Chunk → Embed → LEANN
        if let Err(e) = indexer.index_file_semantic(file_path) {
            eprintln!("Semantic indexing error for {:?}: {}", file_path, e);
        }
    }
}
```

---

### **4. Watchdog - Comportement**

```rust
// src/search/file_watcher.rs

impl FileWatcher {
    pub fn apply_events_batch(
        &self,
        index: &SearchIndex,
        database: Option<&Arc<Database>>,
        semantic_tx: Option<&Sender<PathBuf>>, // ← NOUVEAU
        // ... autres params
    ) -> Result<usize> {
        let events = self.poll_events();

        for event in events {
            match event {
                FileEvent::Created(path) => {
                    // ═══════════════════════════════════════
                    // PHASE 1 : Indexation rapide (immédiate)
                    // ═══════════════════════════════════════

                    // Tantivy
                    index.add_file(&path, &filename)?;

                    // SQLite metadata
                    if let Some(db) = database {
                        db.insert_file(&path, &filename, size, modified)?;
                    }

                    // ═══════════════════════════════════════
                    // PHASE 2 : Queue sémantique (background)
                    // ═══════════════════════════════════════

                    if let Some(tx) = semantic_tx {
                        // Envoyer dans queue background
                        let _ = tx.send(PathBuf::from(&path));
                    }
                }

                FileEvent::Modified(path) => {
                    // Vérifier hash
                    let new_hash = hash_file_fast(&path)?;

                    if hash_changed {
                        // Mettre à jour Tantivy + SQLite
                        index.update_file(&path, &filename)?;
                        db.update_file_hash(&path, new_hash)?;

                        // Re-queue pour embeddings si activé
                        if let Some(tx) = semantic_tx {
                            let _ = tx.send(PathBuf::from(&path));
                        }
                    }
                }

                FileEvent::Removed(path) => {
                    // Supprimer de Tantivy + SQLite
                    index.remove_file(&path)?;
                    db.delete_file(&path)?;

                    // Supprimer embeddings LEANN
                    // Note: LEANN ne supporte pas delete direct
                    // → Marquer comme deleted dans DB, rebuild LEANN périodiquement
                }
            }
        }

        Ok(updated_count)
    }
}
```

---

## 📊 Comparaison des 2 modes

| Aspect | Mode Classique 🔍 | Mode Assist Me 🤖 |
|--------|-------------------|-------------------|
| **Usage** | Chercher fichier par nom | Poser question sémantique |
| **Query** | "contrat dupont" | "Quels sont les budgets 2024 ?" |
| **Technologie** | Tantivy + n-grams | LEANN + embeddings |
| **Indexation** | Rapide (~100ms/fichier) | Lente (1-5s/fichier) |
| **Recherche** | <100ms | <3s |
| **Stockage** | ~1 MB pour 10k fichiers | ~10 MB pour 10k fichiers |
| **Précision** | Exacte (fuzzy match) | Sémantique (sens) |
| **Offline** | ✅ 100% | ✅ 100% |
| **Toujours actif** | ✅ Oui | ❌ Optionnel |

---

## 🎯 Réponses à tes questions

### **1. Mode classique reste ?**
✅ **OUI !** Le mode classique reste **TOUJOURS ACTIF** et par défaut.
- C'est l'onglet "🔍 Recherche"
- Aucun changement pour utilisateurs qui veulent juste chercher par nom

### **2. UI pour basculer ?**
✅ **Onglets en haut** (comme Chrome, VS Code)
- Onglet 1 : "🔍 Recherche" (mode classique)
- Onglet 2 : "🤖 Assist Me" (mode IA)
- Clic pour basculer instantanément

### **3. Interface chat IA ?**
✅ **Oui, mais simplifié** :
- **PAS** de conversation (pas de "dialogue")
- **OUI** question → sources avec extraits
- Format : Input question → Liste sources (comme Google)
- Chaque source est cliquable (ouvre le fichier)

### **4. Indexation - comment ça se passe ?**
✅ **Indexation en 2 PHASES** :
- **Phase 1 (rapide)** : Toujours faite, fichier immédiatement searchable en mode classique
- **Phase 2 (lente)** : Optionnelle, en background, pour Assist Me

### **5. Watchdog - comportement ?**
✅ **Watchdog intelligent** :
- Nouveau fichier détecté → Indexation rapide immédiate (Tantivy)
- Si Assist Me activé → Queue background pour embeddings
- Pas de blocage, tout est async

### **6. Indexation simultanée ?**
✅ **OUI, les deux en parallèle** :
- Thread 1 : Indexation rapide (Tantivy + SQLite)
- Thread 2 : Indexation sémantique (embeddings + LEANN)
- Thread 3 : Watchdog (détection événements)
- Thread 4 : UI (egui)

**Total : 4 threads non-bloquants**

---

## 🚀 Flow utilisateur complet

### **Scénario 1 : Utilisateur classique (ne veut pas IA)**

```
1. Lance xfinder
2. Reste sur onglet "🔍 Recherche" (défaut)
3. Tape "contrat dupont"
4. Résultats instantanés (<100ms)
5. Clic "Ouvrir" → PDF s'ouvre

   ✅ Aucune différence avec avant !
   ✅ Pas de slowdown dû à Assist Me
```

### **Scénario 2 : Utilisateur avancé (veut IA)**

```
1. Lance xfinder
2. Va dans Settings → Coche "Activer Assist Me"
3. App lance indexation sémantique en background
   → Notification : "Indexation IA : 234/1000 fichiers"
4. Utilisateur peut déjà utiliser mode classique pendant ce temps
5. Une fois indexation terminée → Clic onglet "🤖 Assist Me"
6. Tape "Quels sont les budgets formation 2024 ?"
7. Résultats sémantiques (~2s)
8. Clic sur source → Fichier s'ouvre à la bonne page
```

### **Scénario 3 : Nouveau fichier ajouté (Assist Me activé)**

```
1. User copie "Budget_Formation_2025.pdf" dans dossier surveillé
2. Watchdog détecte (500ms)
3. PHASE 1 : Indexation rapide
   → Fichier immédiatement searchable en mode classique
4. PHASE 2 : Queue background
   → Embeddings générés en 2-3s
   → Fichier searchable en Assist Me après 3s

   ✅ Pas de freeze
   ✅ Double indexation automatique
```

---

## ⚡ Performance attendue

### **Indexation initiale (10,000 fichiers)**

| Phase | Temps | Bloque UI ? |
|-------|-------|-------------|
| Phase 1 (Tantivy) | ~2 min | ❌ Non (thread) |
| Phase 2 (Embeddings) | ~8 heures | ❌ Non (background) |

**Note :** Phase 2 peut tourner pendant plusieurs jours en background sans gêner !

### **Recherche**

| Mode | Latence | Qualité |
|------|---------|---------|
| Classique | <100ms | Exacte (nom) |
| Assist Me | <3s | Sémantique (sens) |

---

## 📝 Checklist implémentation

- [ ] Ajouter `AppMode` enum
- [ ] UI onglets (Recherche / Assist Me)
- [ ] Config `AssistMeConfig`
- [ ] Thread background indexation sémantique
- [ ] Queue channel pour nouveaux fichiers
- [ ] Watchdog envoie dans queue si Assist Me activé
- [ ] UI Settings pour activer/désactiver
- [ ] Progression indexation sémantique
- [ ] Tests dual-mode

---

**Ça répond à tes questions ?** 🎯

En gros :
- ✅ Mode classique reste intact et par défaut
- ✅ Assist Me = mode additionnel optionnel
- ✅ Onglets pour basculer
- ✅ Interface type chat simple (question → sources)
- ✅ Indexation en 2 phases (rapide + lente)
- ✅ Watchdog gère les deux automatiquement
- ✅ Tout en background, pas de freeze

**On commence ou tu as d'autres questions ?** 😊
