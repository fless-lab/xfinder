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
use tantivy::query::{QueryParser, TermQuery};
use tantivy::schema::*;
use tantivy::tokenizer::{NgramTokenizer, LowerCaser, TextAnalyzer};
use tantivy::{doc, Index, IndexWriter, TantivyDocument, Term};

use super::SearchResult;

// Options de recherche avancée
#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub exact_match: bool,
    pub case_sensitive: bool,
    pub search_in_filename: bool,
    pub search_in_path: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            exact_match: false,
            case_sensitive: false,
            search_in_filename: true,
            search_in_path: true,
        }
    }
}

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
        std::fs::create_dir_all(index_dir)
            .context("Impossible de créer le dossier d'index")?;

        // Essayer d'ouvrir un index existant d'abord
        let (index, schema, path_field, filename_field) = if index_dir.join("meta.json").exists() {
            // Index existe - ouvrir et récupérer son schéma
            let index = Index::open_in_dir(index_dir)
                .context("Impossible d'ouvrir l'index existant")?;
            let schema = index.schema();
            let path_field = schema.get_field("path")
                .context("Champ 'path' introuvable dans le schéma")?;
            let filename_field = schema.get_field("filename")
                .context("Champ 'filename' introuvable dans le schéma")?;
            (index, schema, path_field, filename_field)
        } else {
            // Créer un nouvel index avec schéma n-gram
            let mut schema_builder = Schema::builder();

            let text_opts = TextOptions::default()
                .set_indexing_options(
                    TextFieldIndexing::default()
                        .set_tokenizer("ngram3")
                        .set_index_option(IndexRecordOption::WithFreqsAndPositions)
                )
                .set_stored();

            let path_field = schema_builder.add_text_field("path", text_opts.clone());
            let filename_field = schema_builder.add_text_field("filename", text_opts);
            let schema = schema_builder.build();

            let index = Index::create_in_dir(index_dir, schema.clone())
                .context("Impossible de créer l'index")?;

            (index, schema, path_field, filename_field)
        };

        // CRITIQUE: Enregistrer le tokenizer n-gram À CHAQUE FOIS
        // Même si on ouvre un index existant, le tokenizer doit être enregistré
        // car il n'est pas persisté sur disque
        //
        // N-grams 2-20: équilibre optimal vitesse/flexibilité
        // - Fragments: ".m", "log", "pdf" (2-5 chars)
        // - Mots: "readme", "document" (6-10 chars)
        // - Requêtes typiques: "presentation" (12 chars), "configuration" (13 chars)
        // - Limite à 20 car les utilisateurs tapent rarement plus de 20 chars
        // - Pour chercher des noms complets longs: utiliser l'option "Match exact"
        let ngram_tokenizer = TextAnalyzer::builder(
            NgramTokenizer::new(2, 20, false).unwrap()
        )
        .filter(LowerCaser)
        .build();

        index.tokenizers().register("ngram3", ngram_tokenizer);

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

    // Supprime COMPLÈTEMENT l'index (dossier + schéma + tout)
    // Nécessaire pour changer le tokenizer
    pub fn delete_completely(index_dir: &std::path::Path) -> Result<()> {
        if index_dir.exists() {
            std::fs::remove_dir_all(index_dir)?;
        }
        Ok(())
    }

    // Compte le nombre de documents dans l'index
    pub fn count_documents(&self) -> Result<usize> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        Ok(searcher.num_docs() as usize)
    }

    // Supprime un fichier de l'index par son chemin
    pub fn delete_file_by_path(&self, file_path: &str) -> Result<()> {
        let mut writer = self.create_writer()?;
        let term = Term::from_field_text(self.path_field, file_path);
        writer.delete_term(term);
        writer.commit()?;
        Ok(())
    }

    // Met à jour le chemin d'un fichier (pour les déplacements)
    // Supprime l'ancien chemin et ajoute le nouveau
    pub fn update_file_path(&self, old_path: &str, new_path: &str, filename: &str) -> Result<()> {
        let mut writer = self.create_writer()?;

        // Supprimer l'ancien
        let term = Term::from_field_text(self.path_field, old_path);
        writer.delete_term(term);

        // Ajouter le nouveau
        self.add_file(&mut writer, new_path, filename)?;

        writer.commit()?;
        Ok(())
    }

    // Met à jour un fichier existant (pour les modifications)
    // Garde le même path mais rafraîchit les métadonnées
    pub fn update_file(&self, path: &str, filename: &str) -> Result<()> {
        let mut writer = self.create_writer()?;

        // Supprimer l'ancien
        let term = Term::from_field_text(self.path_field, path);
        writer.delete_term(term);

        // Ré-ajouter avec nouvelles métadonnées
        self.add_file(&mut writer, path, filename)?;

        writer.commit()?;
        Ok(())
    }

    // Recherche ultra-flexible: marche avec n'importe quel fragment
    // Ex: ".m" trouve ".md", "log" trouve "CHANGELOG.md", "ops" trouve "DataOps.pdf"
    // Avec n-grams 2-20, supporte les requêtes typiques (mots jusqu'à 20 chars)
    // Pour les noms complets très longs: utiliser l'option "Match exact"
    //
    // Options disponibles:
    // - exact_match: recherche exacte sans n-grams
    // - case_sensitive: respecter la casse
    // - search_in_filename/search_in_path: limiter la zone de recherche
    pub fn search(&self, query_str: &str, limit: usize, options: SearchOptions) -> Result<Vec<SearchResult>> {
        let reader = self
            .index
            .reader()
            .context("Impossible de créer le reader")?;

        let searcher = reader.searcher();

        // Préparer la requête selon les options
        let clean_query = if options.case_sensitive {
            query_str.trim().to_string()
        } else {
            query_str.trim().to_lowercase()
        };

        // Déterminer les champs à rechercher
        let mut search_fields = Vec::new();
        if options.search_in_filename {
            search_fields.push(self.filename_field);
        }
        if options.search_in_path {
            search_fields.push(self.path_field);
        }

        // Si aucun champ sélectionné, chercher dans les deux par défaut
        if search_fields.is_empty() {
            search_fields.push(self.filename_field);
            search_fields.push(self.path_field);
        }

        // Construire la requête selon le mode (exact ou flexible)
        let query: Box<dyn tantivy::query::Query> = if options.exact_match {
            // Mode exact: utiliser TermQuery pour chaque champ
            use tantivy::query::BooleanQuery;
            use tantivy::query::Occur;

            let term_queries: Vec<(Occur, Box<dyn tantivy::query::Query>)> = search_fields
                .iter()
                .map(|field| {
                    let term = Term::from_field_text(*field, &clean_query);
                    (Occur::Should, Box::new(TermQuery::new(term, IndexRecordOption::Basic)) as Box<dyn tantivy::query::Query>)
                })
                .collect();

            Box::new(BooleanQuery::new(term_queries))
        } else {
            // Mode flexible: utiliser QueryParser avec n-grams
            let query_parser = QueryParser::for_index(&self.index, search_fields);
            Box::new(query_parser
                .parse_query(&clean_query)
                .context("Impossible de parser la requête")?)
        };

        // Lance la recherche et récupère les N meilleurs documents
        // TopDocs collecte les résultats triés par score de pertinence
        let top_docs = searcher
            .search(&*query, &TopDocs::with_limit(limit))
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

        // Rechercher "txt" avec options par défaut
        let results = index.search("txt", 10, SearchOptions::default()).unwrap();
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

        let results = index.search("nonexistent_file_xyz", 10, SearchOptions::default()).unwrap();
        assert_eq!(results.len(), 0);

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
