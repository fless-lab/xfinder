// src/semantic/mod.rs
// Module de recherche sémantique (LEANN + embeddings)

mod leann_wrapper;
mod content_extractor;
mod chunker;
mod embedding_generator;

pub use leann_wrapper::LeannIndex;
pub use content_extractor::ContentExtractor;
pub use chunker::{Chunker, ChunkConfig, TextChunk};
pub use embedding_generator::EmbeddingGenerator;
