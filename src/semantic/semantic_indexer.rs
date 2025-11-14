// src/semantic/semantic_indexer.rs
// Indexeur sémantique orchestrant le pipeline complet (extraction → chunking → embeddings → LEANN)

use anyhow::{Context, Result};
use std::path::Path;
use std::sync::{Arc, Mutex};

use super::{ContentExtractor, Chunker, ChunkConfig, EmbeddingGenerator, LeannIndex};

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
        })
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

        // 6. Ajouter chaque chunk à LEANN
        let leann = self.leann_index.lock().unwrap();

        for (chunk, embedding) in chunks.iter().zip(embeddings.iter()) {
            // Générer un ID unique pour le chunk
            // Format: file_id * 1000000 + chunk_index
            // Exemple: file_id=123, chunk_index=5 → chunk_id=123000005
            let chunk_id = file_id * 1_000_000 + chunk.chunk_index as i64;

            leann.add_embedding(chunk_id, embedding)
                .with_context(|| format!("Failed to add chunk {} to LEANN", chunk_id))?;
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
}
