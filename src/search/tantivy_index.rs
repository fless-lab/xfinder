// src/search/tantivy_index.rs
// Indexation et recherche avec Tantivy

use anyhow::{Context, Result};
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter, TantivyDocument};

use super::SearchResult;

pub struct SearchIndex {
    index: Index,
    schema: Schema,
    path_field: Field,
    filename_field: Field,
}

impl SearchIndex {
    /// Créer un nouvel index Tantivy
    pub fn new(index_dir: &Path) -> Result<Self> {
        // Schéma de base : chemin + nom fichier
        let mut schema_builder = Schema::builder();
        let path_field = schema_builder.add_text_field("path", TEXT | STORED);
        let filename_field = schema_builder.add_text_field("filename", TEXT | STORED);
        let schema = schema_builder.build();

        // Créer ou ouvrir l'index
        std::fs::create_dir_all(index_dir)
            .context("Impossible de créer le dossier d'index")?;

        let index = Index::create_in_dir(index_dir, schema.clone())
            .or_else(|_| Index::open_in_dir(index_dir))
            .context("Impossible de créer/ouvrir l'index Tantivy")?;

        Ok(Self {
            index,
            schema,
            path_field,
            filename_field,
        })
    }

    /// Ajouter un fichier à l'index
    pub fn add_file(&self, writer: &mut IndexWriter, path: &str, filename: &str) -> Result<()> {
        let doc = doc!(
            self.path_field => path,
            self.filename_field => filename,
        );
        writer.add_document(doc)?;
        Ok(())
    }

    /// Créer un writer pour indexation
    pub fn create_writer(&self) -> Result<IndexWriter> {
        let writer = self
            .index
            .writer(50_000_000)
            .context("Impossible de créer le writer")?;
        Ok(writer)
    }

    /// Rechercher dans l'index
    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let reader = self
            .index
            .reader()
            .context("Impossible de créer le reader")?;

        let searcher = reader.searcher();

        // Parser la requête sur le champ filename
        let query_parser = QueryParser::for_index(&self.index, vec![self.filename_field]);
        let query = query_parser
            .parse_query(query_str)
            .context("Impossible de parser la requête")?;

        // Chercher les top résultats
        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(limit))
            .context("Erreur lors de la recherche")?;

        // Convertir en SearchResult
        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
            let path = retrieved_doc
                .get_first(self.path_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let filename = retrieved_doc
                .get_first(self.filename_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            results.push(SearchResult::new(path, filename, score));
        }

        Ok(results)
    }
}

// Tests TDD
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_creation() {
        let temp_dir = std::env::temp_dir().join("xfinder_test_index_1");
        let _ = std::fs::remove_dir_all(&temp_dir);
        let index = SearchIndex::new(&temp_dir);
        assert!(index.is_ok());
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_add_and_search_file() {
        let temp_dir = std::env::temp_dir().join("xfinder_test_index_2");
        let _ = std::fs::remove_dir_all(&temp_dir);
        let index = SearchIndex::new(&temp_dir).unwrap();

        // Ajouter des fichiers test
        let mut writer = index.create_writer().unwrap();
        index
            .add_file(&mut writer, "C:\\test\\readme.txt", "readme.txt")
            .unwrap();
        index
            .add_file(&mut writer, "C:\\test\\document.pdf", "document.pdf")
            .unwrap();
        index
            .add_file(&mut writer, "C:\\test\\notes.txt", "notes.txt")
            .unwrap();
        writer.commit().unwrap();

        // Rechercher "txt"
        let results = index.search("txt", 10).unwrap();
        assert_eq!(results.len(), 2); // readme.txt + notes.txt

        // Vérifier qu'on trouve bien les fichiers
        let filenames: Vec<String> = results.iter().map(|r| r.filename.clone()).collect();
        assert!(filenames.contains(&"readme.txt".to_string()));
        assert!(filenames.contains(&"notes.txt".to_string()));

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_search_empty_query() {
        let temp_dir = std::env::temp_dir().join("xfinder_test_index_3");
        let _ = std::fs::remove_dir_all(&temp_dir);
        let index = SearchIndex::new(&temp_dir).unwrap();

        let results = index.search("nonexistent_file_xyz", 10).unwrap();
        assert_eq!(results.len(), 0);

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
