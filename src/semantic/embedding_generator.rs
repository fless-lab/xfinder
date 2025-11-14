// src/semantic/embedding_generator.rs
// Génération d'embeddings via Sentence Transformers (Python + PyO3)

use anyhow::{Context, Result};
use pyo3::prelude::*;
use pyo3::types::PyList;

/// Générateur d'embeddings utilisant Sentence Transformers
pub struct EmbeddingGenerator {
    /// Instance Python du modèle Sentence Transformer
    py_model: Option<PyObject>,

    /// Nom du modèle (ex: "all-MiniLM-L6-v2")
    model_name: String,

    /// Dimension des embeddings produits
    dim: usize,
}

impl EmbeddingGenerator {
    /// Crée un nouveau générateur avec le modèle par défaut
    ///
    /// Modèle par défaut : all-MiniLM-L6-v2
    /// - Dimension: 384
    /// - Performance: rapide (~3k sentences/sec sur CPU)
    /// - Qualité: excellente pour retrieval
    pub fn new() -> Result<Self> {
        Self::with_model("all-MiniLM-L6-v2")
    }

    /// Crée un générateur avec un modèle spécifique
    ///
    /// # Arguments
    /// * `model_name` - Nom du modèle Sentence Transformers
    ///
    /// # Examples
    /// - "all-MiniLM-L6-v2" (384 dim, rapide, recommandé)
    /// - "all-mpnet-base-v2" (768 dim, plus précis mais plus lent)
    /// - "paraphrase-multilingual-MiniLM-L12-v2" (384 dim, multilingue)
    pub fn with_model(model_name: &str) -> Result<Self> {
        // Déterminer la dimension selon le modèle
        let dim = match model_name {
            "all-MiniLM-L6-v2" => 384,
            "paraphrase-multilingual-MiniLM-L12-v2" => 384,
            "all-mpnet-base-v2" => 768,
            "all-distilroberta-v1" => 768,
            _ => 384, // Default dimension
        };

        pyo3::prepare_freethreaded_python();

        Ok(Self {
            py_model: None,
            model_name: model_name.to_string(),
            dim,
        })
    }

    /// Charge le modèle Sentence Transformers
    ///
    /// # Errors
    /// Retourne une erreur si sentence-transformers n'est pas installé
    /// ou si le modèle ne peut pas être chargé
    pub fn load_model(&mut self) -> Result<()> {
        Python::with_gil(|py| {
            // Importer SentenceTransformer
            let st_module = py
                .import_bound("sentence_transformers")
                .context(
                    "sentence-transformers not found. Install with: pip install sentence-transformers"
                )?;

            // Charger le modèle
            // model = SentenceTransformer('all-MiniLM-L6-v2')
            let model = st_module
                .getattr("SentenceTransformer")?
                .call1((&self.model_name,))
                .with_context(|| format!("Failed to load model: {}", self.model_name))?;

            self.py_model = Some(model.into());

            Ok(())
        })
    }

    /// Génère un embedding pour un seul texte
    ///
    /// # Arguments
    /// * `text` - Texte à encoder
    ///
    /// # Returns
    /// Vecteur d'embedding (dimension = self.dim)
    ///
    /// # Errors
    /// Retourne une erreur si le modèle n'est pas chargé ou si l'encoding échoue
    pub fn encode(&self, text: &str) -> Result<Vec<f32>> {
        let results = self.encode_batch(&[text])?;
        results.into_iter().next()
            .context("No embedding returned")
    }

    /// Génère des embeddings pour plusieurs textes (batch)
    ///
    /// # Arguments
    /// * `texts` - Slice de textes à encoder
    ///
    /// # Returns
    /// Vec d'embeddings (chaque embedding est un Vec<f32>)
    ///
    /// # Errors
    /// Retourne une erreur si le modèle n'est pas chargé ou si l'encoding échoue
    ///
    /// # Performance
    /// Batch encoding est ~10x plus rapide que encoder 1 par 1
    pub fn encode_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        Python::with_gil(|py| {
            let model = self
                .py_model
                .as_ref()
                .context("Model not loaded. Call load_model() first")?
                .bind(py);

            // Convertir &[&str] en PyList
            let py_texts = PyList::new_bound(py, texts);

            // embeddings = model.encode(texts, convert_to_numpy=True)
            // Retourne un numpy array de shape (len(texts), dim)
            let embeddings = model
                .call_method1("encode", (py_texts,))?;

            // Convertir numpy array en Vec<Vec<f32>>
            let result = self.numpy_to_vec(py, &embeddings)?;

            Ok(result)
        })
    }

    /// Convertit un numpy array en Vec<Vec<f32>>
    fn numpy_to_vec(&self, py: Python, numpy_array: &Bound<PyAny>) -> Result<Vec<Vec<f32>>> {
        // Obtenir le shape
        let shape: Vec<usize> = numpy_array
            .getattr("shape")?
            .extract()?;

        if shape.len() != 2 {
            anyhow::bail!("Expected 2D numpy array, got shape: {:?}", shape);
        }

        let num_embeddings = shape[0];
        let embedding_dim = shape[1];

        if embedding_dim != self.dim {
            anyhow::bail!(
                "Embedding dimension mismatch: expected {}, got {}",
                self.dim,
                embedding_dim
            );
        }

        // Convertir en liste Python
        let py_list = numpy_array.call_method0("tolist")?;

        // Extraire en Vec<Vec<f32>>
        let mut result = Vec::with_capacity(num_embeddings);

        for i in 0..num_embeddings {
            let row = py_list.get_item(i)?;
            let embedding: Vec<f32> = row.extract()?;
            result.push(embedding);
        }

        Ok(result)
    }

    /// Retourne la dimension des embeddings
    pub fn dimension(&self) -> usize {
        self.dim
    }

    /// Retourne le nom du modèle
    pub fn model_name(&self) -> &str {
        &self.model_name
    }

    /// Vérifie si le modèle est chargé
    pub fn is_loaded(&self) -> bool {
        self.py_model.is_some()
    }
}

impl Default for EmbeddingGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create default EmbeddingGenerator")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Ignorer par défaut (nécessite sentence-transformers installé)
    fn test_embedding_generator_basic() {
        let mut generator = EmbeddingGenerator::new().unwrap();
        generator.load_model().unwrap();

        let text = "This is a test sentence.";
        let embedding = generator.encode(text).unwrap();

        assert_eq!(embedding.len(), 384); // all-MiniLM-L6-v2 = 384 dim
        assert!(embedding.iter().any(|&x| x != 0.0)); // Vérifier que ce n'est pas juste des zéros
    }

    #[test]
    #[ignore]
    fn test_batch_encoding() {
        let mut generator = EmbeddingGenerator::new().unwrap();
        generator.load_model().unwrap();

        let texts = vec![
            "First sentence.",
            "Second sentence.",
            "Third sentence.",
        ];

        let embeddings = generator.encode_batch(&texts).unwrap();

        assert_eq!(embeddings.len(), 3);
        assert_eq!(embeddings[0].len(), 384);
        assert_eq!(embeddings[1].len(), 384);
        assert_eq!(embeddings[2].len(), 384);

        // Les embeddings devraient être différents
        assert_ne!(embeddings[0], embeddings[1]);
    }

    #[test]
    fn test_dimension() {
        let generator = EmbeddingGenerator::new().unwrap();
        assert_eq!(generator.dimension(), 384);
    }

    #[test]
    fn test_model_name() {
        let generator = EmbeddingGenerator::with_model("all-mpnet-base-v2").unwrap();
        assert_eq!(generator.model_name(), "all-mpnet-base-v2");
        assert_eq!(generator.dimension(), 768);
    }

    #[test]
    fn test_is_loaded() {
        let generator = EmbeddingGenerator::new().unwrap();
        assert!(!generator.is_loaded());
    }
}
