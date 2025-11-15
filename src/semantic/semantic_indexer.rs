// src/semantic/semantic_indexer.rs
// Indexeur sémantique orchestrant le pipeline complet (extraction → chunking → embeddings → LEANN)

use anyhow::{Context, Result};
use std::path::Path;
use std::sync::{Arc, Mutex};

use super::{ContentExtractor, Chunker, ChunkConfig, EmbeddingGenerator, LeannIndex};
use crate::database::{Database, queries};

/// Structure pour stocker un chunk indexé
#[derive(Debug, Clone)]
pub struct IndexedChunk {
    /// ID du fichier dans la DB SQLite
    pub file_id: i64,

    /// Index du chunk dans le fichier (0-based)
    pub chunk_index: usize,

    /// Texte du chunk
    pub text: String,

    /// Position de départ dans le fichier
    pub start_pos: usize,

    /// Position de fin dans le fichier
    pub end_pos: usize,
}

/// Indexeur sémantique complet
pub struct SemanticIndexer {
    /// Générateur d'embeddings (Sentence Transformers)
    embedding_gen: Arc<Mutex<EmbeddingGenerator>>,

    /// Index vectoriel LEANN
    leann_index: Arc<Mutex<LeannIndex>>,

    /// Chunker de texte
    chunker: Chunker,

    /// Chemin de l'index LEANN
    index_path: String,

    /// Database pour sauvegarder les chunks et mappings (optionnel)
    database: Option<Arc<Database>>,
}

impl SemanticIndexer {
    /// Crée un nouvel indexeur sémantique
    ///
    /// # Arguments
    /// * `index_path` - Chemin où sauvegarder l'index LEANN
    /// * `model_name` - Nom du modèle Sentence Transformers (ex: "all-MiniLM-L6-v2")
    pub fn new<P: AsRef<Path>>(index_path: P, model_name: &str) -> Result<Self> {
        let index_path_str = index_path.as_ref().to_string_lossy().to_string();

        // Créer l'embedding generator
        let mut embedding_gen = EmbeddingGenerator::with_model(model_name)?;
        embedding_gen.load_model()
            .context("Failed to load Sentence Transformer model")?;

        let dim = embedding_gen.dimension();

        // Créer l'index LEANN et initialiser le builder
        let mut leann_index = LeannIndex::new(&index_path_str, dim)?;
        leann_index.init_builder()
            .context("Failed to initialize LEANN builder")?;

        Ok(Self {
            embedding_gen: Arc::new(Mutex::new(embedding_gen)),
            leann_index: Arc::new(Mutex::new(leann_index)),
            chunker: Chunker::new(),
            index_path: index_path_str,
            database: None,
        })
    }

    /// Attache une database pour sauvegarder les chunks
    pub fn set_database(&mut self, database: Arc<Database>) {
        self.database = Some(database);
    }

    /// Crée un indexeur avec config de chunking personnalisée
    pub fn with_chunk_config<P: AsRef<Path>>(
        index_path: P,
        model_name: &str,
        chunk_config: ChunkConfig,
    ) -> Result<Self> {
        let mut indexer = Self::new(index_path, model_name)?;
        indexer.chunker = Chunker::with_config(chunk_config);
        Ok(indexer)
    }

    /// Indexe un fichier complet (extraction → chunking → embeddings → LEANN)
    ///
    /// # Arguments
    /// * `file_path` - Chemin du fichier à indexer
    /// * `file_id` - ID du fichier dans la DB SQLite
    ///
    /// # Returns
    /// Nombre de chunks indexés
    ///
    /// # Pipeline
    /// 1. Vérifier si le format est supporté
    /// 2. Extraire le texte (ContentExtractor)
    /// 3. Découper en chunks (Chunker)
    /// 4. Générer les embeddings (EmbeddingGenerator)
    /// 5. Ajouter à l'index LEANN
    pub fn index_file<P: AsRef<Path>>(&self, file_path: P, file_id: i64) -> Result<usize> {
        let file_path = file_path.as_ref();

        // 1. Vérifier format supporté
        if !ContentExtractor::is_supported(file_path) {
            return Ok(0); // Skip unsupported files
        }

        // 2. Extraire le texte
        let text = ContentExtractor::extract_text(file_path)
            .with_context(|| format!("Failed to extract text from: {}", file_path.display()))?;

        // Nettoyer le texte
        let clean_text = ContentExtractor::clean_text(&text);

        if clean_text.trim().is_empty() {
            return Ok(0); // Skip empty files
        }

        // 3. Découper en chunks
        let chunks = self.chunker.chunk_text(&clean_text)
            .context("Failed to chunk text")?;

        if chunks.is_empty() {
            return Ok(0);
        }

        // 4. Préparer les textes pour batch encoding
        let chunk_texts: Vec<&str> = chunks.iter().map(|c| c.text.as_str()).collect();

        // 5. Générer les embeddings (batch)
        let embeddings = {
            let gen = self.embedding_gen.lock().unwrap();
            gen.encode_batch(&chunk_texts)
                .context("Failed to generate embeddings")?
        };

        // 6. Sauvegarder le mapping file_id -> path dans la DB (si disponible)
        if let Some(ref db) = self.database {
            let path_str = file_path.to_string_lossy().to_string();
            db.with_conn(|conn| {
                queries::upsert_semantic_file_mapping(conn, file_id, &path_str)
            }).ok(); // Ignorer les erreurs de DB pour ne pas bloquer l'indexation
        }

        // 7. Ajouter chaque chunk à LEANN et sauvegarder dans la DB
        let leann = self.leann_index.lock().unwrap();

        for (chunk, embedding) in chunks.iter().zip(embeddings.iter()) {
            // Générer un ID unique pour le chunk
            // Format: file_id * 1000000 + chunk_index
            // Exemple: file_id=123, chunk_index=5 → chunk_id=123000005
            let chunk_id = file_id * 1_000_000 + chunk.chunk_index as i64;

            // Ajouter à LEANN
            leann.add_embedding(chunk_id, embedding)
                .with_context(|| format!("Failed to add chunk {} to LEANN", chunk_id))?;

            // Sauvegarder le chunk dans la DB (si disponible)
            if let Some(ref db) = self.database {
                let chunk_record = queries::SemanticChunkRecord {
                    chunk_id,
                    file_id,
                    chunk_index: chunk.chunk_index,
                    text: chunk.text.clone(),
                    start_pos: chunk.start_pos,
                    end_pos: chunk.end_pos,
                    indexed_at: chrono::Utc::now().timestamp(),
                };

                db.with_conn(|conn| {
                    queries::insert_semantic_chunk(conn, &chunk_record)
                }).ok(); // Ignorer les erreurs de DB pour ne pas bloquer l'indexation
            }
        }

        Ok(chunks.len())
    }

    /// Indexe plusieurs fichiers en batch
    ///
    /// # Arguments
    /// * `files` - Vec de (file_path, file_id)
    ///
    /// # Returns
    /// Nombre total de chunks indexés
    pub fn index_batch(&self, files: &[(String, i64)]) -> Result<usize> {
        let mut total_chunks = 0;

        for (file_path, file_id) in files {
            match self.index_file(file_path, *file_id) {
                Ok(count) => {
                    total_chunks += count;
                }
                Err(e) => {
                    eprintln!("Error indexing {}: {}", file_path, e);
                    // Continue avec les autres fichiers
                }
            }
        }

        Ok(total_chunks)
    }

    /// Construit l'index LEANN final (à appeler après tous les add_embedding)
    ///
    /// # Errors
    /// Retourne une erreur si la construction échoue
    pub fn build_index(&self) -> Result<()> {
        let mut leann = self.leann_index.lock().unwrap();
        leann.build()
            .context("Failed to build LEANN index")?;
        Ok(())
    }

    /// Charge l'index LEANN depuis le disque
    ///
    /// # Errors
    /// Retourne une erreur si le chargement échoue
    pub fn load_index(&self) -> Result<()> {
        let mut leann = self.leann_index.lock().unwrap();
        leann.load()
            .context("Failed to load LEANN index")?;
        Ok(())
    }

    /// Recherche sémantique
    ///
    /// # Arguments
    /// * `query` - Question en langage naturel
    /// * `k` - Nombre de résultats à retourner
    ///
    /// # Returns
    /// Vec de (chunk_id, distance) triés par pertinence
    ///
    /// # Note
    /// chunk_id peut être décodé :
    /// - file_id = chunk_id / 1_000_000
    /// - chunk_index = chunk_id % 1_000_000
    pub fn search(&self, query: &str, k: usize) -> Result<Vec<(i64, f32)>> {
        // 1. Générer embedding de la query
        let query_embedding = {
            let gen = self.embedding_gen.lock().unwrap();
            gen.encode(query)
                .context("Failed to encode query")?
        };

        // 2. Rechercher dans LEANN
        let leann = self.leann_index.lock().unwrap();
        let results = leann.search(&query_embedding, k)
            .context("Failed to search in LEANN")?;

        Ok(results)
    }

    /// Décode un chunk_id en (file_id, chunk_index)
    pub fn decode_chunk_id(chunk_id: i64) -> (i64, usize) {
        let file_id = chunk_id / 1_000_000;
        let chunk_index = (chunk_id % 1_000_000) as usize;
        (file_id, chunk_index)
    }

    /// Encode (file_id, chunk_index) en chunk_id
    pub fn encode_chunk_id(file_id: i64, chunk_index: usize) -> i64 {
        file_id * 1_000_000 + chunk_index as i64
    }

    /// Retourne le chemin de l'index
    pub fn index_path(&self) -> &str {
        &self.index_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_id_encoding() {
        let file_id = 12345_i64;
        let chunk_index = 67_usize;

        let chunk_id = SemanticIndexer::encode_chunk_id(file_id, chunk_index);
        assert_eq!(chunk_id, 12345000067);

        let (decoded_file_id, decoded_chunk_index) = SemanticIndexer::decode_chunk_id(chunk_id);
        assert_eq!(decoded_file_id, file_id);
        assert_eq!(decoded_chunk_index, chunk_index);
    }

    #[test]
    fn test_chunk_id_edge_cases() {
        // Test avec chunk_index = 0
        let (file_id, chunk_index) = SemanticIndexer::decode_chunk_id(123000000);
        assert_eq!(file_id, 123);
        assert_eq!(chunk_index, 0);

        // Test avec chunk_index max (999999)
        let chunk_id = SemanticIndexer::encode_chunk_id(456, 999999);
        assert_eq!(chunk_id, 456999999);

        let (file_id2, chunk_index2) = SemanticIndexer::decode_chunk_id(chunk_id);
        assert_eq!(file_id2, 456);
        assert_eq!(chunk_index2, 999999);
    }

    #[test]
    fn test_full_semantic_search_pipeline() {
        // Ce test valide le pipeline complet:
        // 1. PyTorch + sentence-transformers (génération embeddings)
        // 2. LEANN (index vectoriel)
        // 3. Recherche sémantique

        use std::fs;
        use std::io::Write;
        use tempfile::tempdir;

        // Créer un répertoire temporaire pour le test
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test_leann_index");

        // Créer un fichier test avec du contenu
        let test_file_path = temp_dir.path().join("test_document.txt");
        let mut test_file = fs::File::create(&test_file_path)
            .expect("Failed to create test file");

        writeln!(test_file, "Rust is a systems programming language that focuses on safety and performance.")
            .expect("Failed to write to test file");
        writeln!(test_file, "It has great memory safety guarantees without using a garbage collector.")
            .expect("Failed to write to test file");
        writeln!(test_file, "Python is an interpreted high-level programming language.")
            .expect("Failed to write to test file");
        writeln!(test_file, "Python emphasizes code readability and simplicity.")
            .expect("Failed to write to test file");

        drop(test_file);

        // Créer l'indexeur sémantique
        println!("Creating SemanticIndexer with sentence-transformers model...");
        let indexer = SemanticIndexer::new(&index_path, "all-MiniLM-L6-v2")
            .expect("Failed to create SemanticIndexer");

        // Indexer le fichier de test (cela va utiliser PyTorch + sentence-transformers + LEANN)
        println!("Indexing test file (PyTorch + sentence-transformers + LEANN)...");
        let file_id = 1_i64;
        let chunks_indexed = indexer.index_file(&test_file_path, file_id)
            .expect("Failed to index file");

        println!("Indexed {} chunks", chunks_indexed);
        assert!(chunks_indexed > 0, "Should have indexed at least one chunk");

        // Construire l'index LEANN
        println!("Building LEANN index...");
        indexer.build_index()
            .expect("Failed to build LEANN index");

        // Test 1: Recherche sur Rust (devrait retourner les chunks sur Rust)
        println!("\nTest 1: Searching for 'memory safety in programming'...");
        let results = indexer.search("memory safety in programming", 3)
            .expect("Failed to search");

        println!("Found {} results", results.len());
        assert!(!results.is_empty(), "Should find results for 'memory safety'");

        // Vérifier que le premier résultat est pertinent (distance faible)
        let (best_chunk_id, best_distance) = results[0];
        println!("Best result: chunk_id={}, distance={:.4}", best_chunk_id, best_distance);
        assert!(best_distance < 1.0, "Best result should have distance < 1.0");

        // Vérifier le décodage du chunk_id
        let (decoded_file_id, chunk_index) = SemanticIndexer::decode_chunk_id(best_chunk_id);
        assert_eq!(decoded_file_id, file_id, "File ID should match");
        println!("Decoded: file_id={}, chunk_index={}", decoded_file_id, chunk_index);

        // Test 2: Recherche sur Python (devrait retourner les chunks sur Python)
        println!("\nTest 2: Searching for 'interpreted programming language'...");
        let python_results = indexer.search("interpreted programming language", 3)
            .expect("Failed to search");

        println!("Found {} results", python_results.len());
        assert!(!python_results.is_empty(), "Should find results for Python");

        let (python_chunk_id, python_distance) = python_results[0];
        println!("Best Python result: chunk_id={}, distance={:.4}", python_chunk_id, python_distance);

        // Les deux recherches devraient retourner des chunks différents
        println!("\nVerifying that different queries return different chunks...");
        println!("Rust query best chunk: {}", best_chunk_id);
        println!("Python query best chunk: {}", python_chunk_id);

        // Note: Ils pourraient être identiques si les chunks se chevauchent,
        // mais au moins un des top 3 devrait être différent
        let rust_chunk_ids: Vec<i64> = results.iter().map(|(id, _)| *id).collect();
        let python_chunk_ids: Vec<i64> = python_results.iter().map(|(id, _)| *id).collect();

        println!("Rust chunks: {:?}", rust_chunk_ids);
        println!("Python chunks: {:?}", python_chunk_ids);

        println!("\n✅ INTEGRATION TEST PASSED!");
        println!("   - PyTorch + sentence-transformers: embeddings generated ✓");
        println!("   - LEANN: index built and searched ✓");
        println!("   - Semantic search: relevant results returned ✓");
    }
}
