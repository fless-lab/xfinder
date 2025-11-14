// src/semantic/mod.rs
// Module de recherche sémantique (LEANN + embeddings)

mod leann_wrapper;
mod content_extractor;

pub use leann_wrapper::LeannIndex;
pub use content_extractor::ContentExtractor;
