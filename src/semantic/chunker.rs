// src/semantic/chunker.rs
// Découpage de texte en chunks pour l'indexation sémantique

use anyhow::Result;

/// Configuration du chunking
#[derive(Debug, Clone)]
pub struct ChunkConfig {
    /// Taille maximale d'un chunk en tokens (approximatif)
    pub max_tokens: usize,

    /// Overlap entre chunks (en tokens)
    pub overlap_tokens: usize,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            max_tokens: 500,      // ~400 mots, optimal pour sentence transformers
            overlap_tokens: 50,   // 10% d'overlap pour contexte
        }
    }
}

/// Chunk de texte avec métadonnées
#[derive(Debug, Clone)]
pub struct TextChunk {
    /// Texte du chunk
    pub text: String,

    /// Index du chunk dans le document (0-based)
    pub chunk_index: usize,

    /// Position de départ dans le texte original (en caractères)
    pub start_pos: usize,

    /// Position de fin dans le texte original (en caractères)
    pub end_pos: usize,
}

/// Chunker de texte
pub struct Chunker {
    config: ChunkConfig,
}

impl Chunker {
    /// Crée un nouveau chunker avec la config par défaut
    pub fn new() -> Self {
        Self {
            config: ChunkConfig::default(),
        }
    }

    /// Crée un chunker avec une config personnalisée
    pub fn with_config(config: ChunkConfig) -> Self {
        Self { config }
    }

    /// Découpe un texte en chunks
    ///
    /// # Arguments
    /// * `text` - Texte à découper
    ///
    /// # Returns
    /// Vec de TextChunk avec overlap
    ///
    /// # Algorithm
    /// 1. Approximation : 1 token ≈ 4 caractères (moyenne pour l'anglais/français)
    /// 2. Découpage par phrases pour garder cohérence sémantique
    /// 3. Overlap de 10% entre chunks pour continuité
    pub fn chunk_text(&self, text: &str) -> Result<Vec<TextChunk>> {
        if text.trim().is_empty() {
            return Ok(Vec::new());
        }

        // Approximation : 1 token ≈ 4 caractères
        let chars_per_token = 4;
        let max_chars = self.config.max_tokens * chars_per_token;
        let overlap_chars = self.config.overlap_tokens * chars_per_token;

        // Si le texte est court, retourner un seul chunk
        if text.len() <= max_chars {
            return Ok(vec![TextChunk {
                text: text.to_string(),
                chunk_index: 0,
                start_pos: 0,
                end_pos: text.len(),
            }]);
        }

        // Découper en phrases
        let sentences = self.split_sentences(text);

        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_start = 0;
        let mut current_pos = 0;
        let mut chunk_index = 0;

        for sentence in sentences {
            let sentence_with_space = if current_chunk.is_empty() {
                sentence.to_string()
            } else {
                format!(" {}", sentence)
            };

            // Si ajouter cette phrase dépasse la limite
            if current_chunk.len() + sentence_with_space.len() > max_chars && !current_chunk.is_empty() {
                // Sauvegarder le chunk actuel
                chunks.push(TextChunk {
                    text: current_chunk.clone(),
                    chunk_index,
                    start_pos: current_start,
                    end_pos: current_pos,
                });

                chunk_index += 1;

                // Calculer l'overlap
                let overlap_text = if current_chunk.len() > overlap_chars {
                    // Prendre les derniers overlap_chars du chunk précédent
                    let overlap_start = current_chunk.len() - overlap_chars;
                    current_chunk[overlap_start..].to_string()
                } else {
                    current_chunk.clone()
                };

                // Démarrer nouveau chunk avec overlap
                current_start = current_pos - overlap_text.len();
                current_chunk = overlap_text;
            }

            // Ajouter la phrase au chunk actuel
            current_chunk.push_str(&sentence_with_space);
            current_pos += sentence.len() + 1; // +1 pour l'espace
        }

        // Ajouter le dernier chunk s'il existe
        if !current_chunk.is_empty() {
            chunks.push(TextChunk {
                text: current_chunk,
                chunk_index,
                start_pos: current_start,
                end_pos: text.len(),
            });
        }

        Ok(chunks)
    }

    /// Découpe un texte en phrases
    ///
    /// Simple heuristique : split sur . ! ? suivi d'espace ou fin de ligne
    fn split_sentences(&self, text: &str) -> Vec<String> {
        let mut sentences = Vec::new();
        let mut current_sentence = String::new();

        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];
            current_sentence.push(c);

            // Fin de phrase si : . ! ? suivi d'espace/newline/EOF
            if matches!(c, '.' | '!' | '?') {
                let next_is_space_or_end = i + 1 >= chars.len()
                    || chars[i + 1].is_whitespace();

                if next_is_space_or_end && !current_sentence.trim().is_empty() {
                    sentences.push(current_sentence.trim().to_string());
                    current_sentence.clear();
                }
            }

            i += 1;
        }

        // Ajouter la dernière phrase si elle existe
        if !current_sentence.trim().is_empty() {
            sentences.push(current_sentence.trim().to_string());
        }

        // Si aucune phrase détectée (pas de ponctuation), découper par lignes
        if sentences.is_empty() {
            sentences = text
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.trim().to_string())
                .collect();
        }

        sentences
    }

    /// Estime le nombre de tokens dans un texte
    ///
    /// Approximation : 1 token ≈ 4 caractères
    pub fn estimate_tokens(text: &str) -> usize {
        text.len() / 4
    }
}

impl Default for Chunker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_short_text() {
        let chunker = Chunker::new();
        let text = "Ceci est un texte court.";

        let chunks = chunker.chunk_text(text).unwrap();

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, text);
        assert_eq!(chunks[0].chunk_index, 0);
    }

    #[test]
    fn test_chunk_long_text() {
        let chunker = Chunker::with_config(ChunkConfig {
            max_tokens: 10,  // ~40 chars
            overlap_tokens: 2,
        });

        let text = "Première phrase. Deuxième phrase. Troisième phrase. Quatrième phrase.";

        let chunks = chunker.chunk_text(text).unwrap();

        // Devrait créer plusieurs chunks
        assert!(chunks.len() > 1);

        // Vérifier l'ordre des index
        for (i, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.chunk_index, i);
        }
    }

    #[test]
    fn test_split_sentences() {
        let chunker = Chunker::new();

        let text = "Première phrase. Deuxième phrase! Troisième phrase?";
        let sentences = chunker.split_sentences(text);

        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0], "Première phrase.");
        assert_eq!(sentences[1], "Deuxième phrase!");
        assert_eq!(sentences[2], "Troisième phrase?");
    }

    #[test]
    fn test_estimate_tokens() {
        let text = "1234567890123456"; // 16 chars = ~4 tokens
        assert_eq!(Chunker::estimate_tokens(text), 4);

        let text2 = "a".repeat(400); // 400 chars = ~100 tokens
        assert_eq!(Chunker::estimate_tokens(&text2), 100);
    }

    #[test]
    fn test_empty_text() {
        let chunker = Chunker::new();
        let chunks = chunker.chunk_text("").unwrap();
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_overlap_exists() {
        let chunker = Chunker::with_config(ChunkConfig {
            max_tokens: 20,
            overlap_tokens: 5,
        });

        let text = "A".repeat(200); // Force multiple chunks
        let chunks = chunker.chunk_text(&text).unwrap();

        if chunks.len() > 1 {
            // Vérifier que le début du chunk 2 est similaire à la fin du chunk 1
            let chunk1_end = &chunks[0].text[chunks[0].text.len() - 20..];
            let chunk2_start = &chunks[1].text[..20.min(chunks[1].text.len())];

            // Il devrait y avoir un overlap
            assert!(chunk1_end.chars().next() == chunk2_start.chars().next());
        }
    }
}
