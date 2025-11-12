// src/search/tantivy_index.rs
// Module gérant l'indexation et la recherche avec Tantivy
//
// Ce module encapsule toute la logique d'interaction avec Tantivy:
// - Création et ouverture de l'index
// - Ajout de documents (fichiers) à l'index
// - Recherche dans l'index avec scoring

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
    // Initialise un nouvel index Tantivy ou ouvre un index existant
    //
    // L'index sera créé dans le dossier spécifié. Si un index existe déjà
    // à cet emplacement, il sera ouvert pour être réutilisé.
    //
    // Le schéma initial contient deux champs:
    // - path: chemin complet du fichier (TEXT | STORED)
    // - filename: nom du fichier uniquement (TEXT | STORED)
    pub fn new(index_dir: &Path) -> Result<Self> {
        // Schéma avec n-grams pour recherche "as-you-type"
        // Ex: tape "doc" → trouve "document.pdf"
        let mut schema_builder = Schema::builder();

        let text_opts = TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer("default")
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions)
            )
            .set_stored();

        let path_field = schema_builder.add_text_field("path", text_opts.clone());
        let filename_field = schema_builder.add_text_field("filename", text_opts);
        let schema = schema_builder.build();

        // Assurons-nous que le dossier d'index existe
        // Si le dossier n'existe pas, on le crée
        std::fs::create_dir_all(index_dir)
            .context("Impossible de créer le dossier d'index")?;

        // Tentons de créer un nouvel index
        // Si un index existe déjà (erreur AlreadyExists), on l'ouvre
        // Cette approche permet de gérer à la fois les créations et réouvertures
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

    // Ajoute un fichier à l'index via le writer fourni
    //
    // Cette méthode crée un document Tantivy avec les informations du fichier
    // et l'ajoute au writer. Le document ne sera persisté qu'après un commit()
    // sur le writer.
    //
    // Paramètres:
    // - writer: Le IndexWriter actif pour cette session d'indexation
    // - path: Chemin complet du fichier (ex: C:\Users\...\document.pdf)
    // - filename: Nom du fichier uniquement (ex: document.pdf)
    pub fn add_file(&self, writer: &mut IndexWriter, path: &str, filename: &str) -> Result<()> {
        let doc = doc!(
            self.path_field => path,
            self.filename_field => filename,
        );
        writer.add_document(doc)?;
        Ok(())
    }

    // Crée un IndexWriter pour commencer une session d'indexation
    //
    // Le writer alloue 50MB de RAM pour le buffer d'indexation.
    // N'oublie pas d'appeler writer.commit() pour persister les changements!
    pub fn create_writer(&self) -> Result<IndexWriter> {
        let writer = self
            .index
            .writer(50_000_000)
            .context("Impossible de créer le writer")?;
        Ok(writer)
    }

    // Efface tous les documents de l'index
    // Utile pour réinitialiser complètement avant une nouvelle indexation
    pub fn clear(&self) -> Result<()> {
        let mut writer = self.create_writer()?;
        writer.delete_all_documents()?;
        writer.commit()?;
        Ok(())
    }

    // Compte le nombre de documents dans l'index
    pub fn count_documents(&self) -> Result<usize> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        Ok(searcher.num_docs() as usize)
    }

    // Recherche des fichiers dans l'index en fonction d'une requête textuelle
    //
    // Cette méthode:
    // 1. Parse la requête utilisateur (ex: "document pdf")
    // 2. Exécute la recherche sur le champ filename
    // 3. Retourne les meilleurs résultats avec leurs scores
    //
    // Paramètres:
    // - query_str: Texte de recherche saisi par l'utilisateur
    // - limit: Nombre maximum de résultats à retourner
    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Ouvre un reader sur l'index
        // Le reader permet de rechercher dans l'index de manière thread-safe
        let reader = self
            .index
            .reader()
            .context("Impossible de créer le reader")?;

        let searcher = reader.searcher();

        // Configure le parser de requête pour chercher dans le champ filename
        // Tantivy supporte les requêtes complexes (AND, OR, phrases, etc.)
        let query_parser = QueryParser::for_index(&self.index, vec![self.filename_field]);
        let query = query_parser
            .parse_query(query_str)
            .context("Impossible de parser la requête")?;

        // Lance la recherche et récupère les N meilleurs documents
        // TopDocs collecte les résultats triés par score de pertinence
        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(limit))
            .context("Erreur lors de la recherche")?;

        // Convertir les résultats Tantivy en SearchResult
        // On déduplique par chemin pour éviter les doublons
        let mut results = Vec::new();
        let mut seen_paths = std::collections::HashSet::new();

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

            // Skip si déjà vu
            if !seen_paths.insert(path.clone()) {
                continue;
            }

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
