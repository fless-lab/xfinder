// src/semantic/leann_wrapper.rs
// Wrapper PyO3 pour LEANN (Low-storage vector index)

use anyhow::{Context, Result};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};
use std::path::Path;

/// Wrapper Rust pour l'index LEANN (Python)
pub struct LeannIndex {
    /// Instance Python du builder LEANN
    py_builder: Option<PyObject>,

    /// Instance Python du searcher LEANN
    py_searcher: Option<PyObject>,

    /// Chemin de l'index sur disque
    index_path: String,

    /// Dimension des embeddings (384 pour all-MiniLM-L6-v2)
    dim: usize,
}

impl LeannIndex {
    /// Crée un nouvel index LEANN vide
    ///
    /// # Arguments
    /// * `index_path` - Chemin où sauvegarder l'index
    /// * `dim` - Dimension des embeddings (384 pour all-MiniLM-L6-v2)
    pub fn new<P: AsRef<Path>>(index_path: P, dim: usize) -> Result<Self> {
        let index_path_str = index_path.as_ref().to_string_lossy().to_string();

        // Vérifier que Python est initialisé
        pyo3::prepare_freethreaded_python();

        Ok(Self {
            py_builder: None,
            py_searcher: None,
            index_path: index_path_str,
            dim,
        })
    }

    /// Initialise le builder LEANN
    ///
    /// # Errors
    /// Retourne une erreur si LEANN n'est pas installé ou si l'init échoue
    pub fn init_builder(&mut self) -> Result<()> {
        Python::with_gil(|py| {
            // Importer le module LEANN
            let leann_module = py
                .import_bound("leann")
                .context("LEANN module not found. Install with: pip install leann")?;

            // Créer le builder LEANN
            // Équivalent Python: builder = leann.Builder(dim=384)
            let builder = leann_module
                .getattr("Builder")?
                .call1((self.dim,))?;

            self.py_builder = Some(builder.into());
            Ok(())
        })
    }

    /// Ajoute un embedding à l'index
    ///
    /// # Arguments
    /// * `doc_id` - ID du document (file_id dans SQLite)
    /// * `embedding` - Vecteur d'embedding (dimension = self.dim)
    ///
    /// # Errors
    /// Retourne une erreur si le builder n'est pas initialisé ou si l'ajout échoue
    pub fn add_embedding(&self, doc_id: i64, embedding: &[f32]) -> Result<()> {
        if embedding.len() != self.dim {
            anyhow::bail!(
                "Embedding dimension mismatch: expected {}, got {}",
                self.dim,
                embedding.len()
            );
        }

        Python::with_gil(|py| {
            let builder = self
                .py_builder
                .as_ref()
                .context("Builder not initialized. Call init_builder() first")?
                .as_ref(py);

            // Convertir Vec<f32> en PyList
            let py_embedding = PyList::new_bound(py, embedding);

            // builder.add(doc_id, embedding)
            builder.call_method1("add", (doc_id, py_embedding))?;

            Ok(())
        })
    }

    /// Construit l'index LEANN et le sauvegarde sur disque
    ///
    /// # Errors
    /// Retourne une erreur si le builder n'est pas initialisé ou si la construction échoue
    pub fn build(&mut self) -> Result<()> {
        Python::with_gil(|py| {
            let builder = self
                .py_builder
                .as_ref()
                .context("Builder not initialized")?
                .as_ref(py);

            // builder.build(index_path)
            builder.call_method1("build", (&self.index_path,))?;

            Ok(())
        })
    }

    /// Charge un index LEANN depuis le disque
    ///
    /// # Errors
    /// Retourne une erreur si le chargement échoue
    pub fn load(&mut self) -> Result<()> {
        Python::with_gil(|py| {
            // Importer le module LEANN
            let leann_module = py
                .import_bound("leann")
                .context("LEANN module not found")?;

            // Créer le searcher LEANN
            // Équivalent Python: searcher = leann.Searcher(index_path)
            let searcher = leann_module
                .getattr("Searcher")?
                .call1((&self.index_path,))?;

            self.py_searcher = Some(searcher.into());
            Ok(())
        })
    }

    /// Recherche les k voisins les plus proches d'un embedding
    ///
    /// # Arguments
    /// * `query_embedding` - Vecteur d'embedding de la question
    /// * `k` - Nombre de résultats à retourner
    ///
    /// # Returns
    /// Vec de (doc_id, distance) triés par distance croissante
    ///
    /// # Errors
    /// Retourne une erreur si le searcher n'est pas initialisé ou si la recherche échoue
    pub fn search(&self, query_embedding: &[f32], k: usize) -> Result<Vec<(i64, f32)>> {
        if query_embedding.len() != self.dim {
            anyhow::bail!(
                "Query embedding dimension mismatch: expected {}, got {}",
                self.dim,
                query_embedding.len()
            );
        }

        Python::with_gil(|py| {
            let searcher = self
                .py_searcher
                .as_ref()
                .context("Searcher not initialized. Call load() first")?
                .as_ref(py);

            // Convertir Vec<f32> en PyList
            let py_query = PyList::new_bound(py, query_embedding);

            // results = searcher.search(query_embedding, k)
            // results est une liste de tuples (doc_id, distance)
            let results: Bound<PyList> = searcher
                .call_method1("search", (py_query, k))?
                .extract()?;

            // Convertir PyList[(int, float)] en Vec<(i64, f32)>
            let mut rust_results = Vec::new();
            for item in results.iter() {
                let tuple: Bound<PyTuple> = item.extract()?;
                let doc_id: i64 = tuple.get_item(0)?.extract()?;
                let distance: f32 = tuple.get_item(1)?.extract()?;
                rust_results.push((doc_id, distance));
            }

            Ok(rust_results)
        })
    }

    /// Retourne le nombre de documents indexés
    ///
    /// # Errors
    /// Retourne une erreur si le searcher n'est pas initialisé
    pub fn count(&self) -> Result<usize> {
        Python::with_gil(|py| {
            let searcher = self
                .py_searcher
                .as_ref()
                .context("Searcher not initialized")?
                .as_ref(py);

            let count: usize = searcher.call_method0("count")?.extract()?;
            Ok(count)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    #[ignore] // Ignorer par défaut (nécessite LEANN installé)
    fn test_leann_wrapper_basic() {
        let dir = tempdir().unwrap();
        let index_path = dir.path().join("test_leann_index");

        let mut index = LeannIndex::new(&index_path, 384).unwrap();
        index.init_builder().unwrap();

        // Ajouter quelques embeddings de test
        let embedding1 = vec![0.5; 384];
        let embedding2 = vec![0.8; 384];

        index.add_embedding(1, &embedding1).unwrap();
        index.add_embedding(2, &embedding2).unwrap();

        // Construire l'index
        index.build().unwrap();

        // Charger et rechercher
        index.load().unwrap();

        let query = vec![0.6; 384];
        let results = index.search(&query, 2).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(index.count().unwrap(), 2);
    }
}
