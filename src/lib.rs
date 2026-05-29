//! # Markdown RAG
//!
//! A semantic markdown document loader and chunker for RAG pipelines.
//!
//! ## Features
//!
//! - **Semantic chunking**: Respects markdown structure (headers, paragraphs)
//! - **Configurable size**: Set chunk size, overlap, and boundaries
//! - **Batch processing**: Load entire directories efficiently
//! - **Metadata tracking**: Preserves source path and chunk positions
//! - **Stream mode**: Process large document sets without loading all in memory
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use markdown_rag::{DocumentLoader, SplitterConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = SplitterConfig {
//!         chunk_size: 600,
//!         overlap: 0,
//!     };
//!
//!     let loader = DocumentLoader::new(config);
//!     let chunks = loader.load_directory("./docs").await?;
//!
//!     for chunk in chunks {
//!         println!("{}: {} chars", chunk.source, chunk.content.len());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Modes
//!
//! - **Batch mode**: Load all documents at once (`load_directory`)
//! - **Stream mode**: Process one at a time for memory efficiency (`load_documents_streaming`)

mod document;
mod error;
mod loader;
mod splitter;

pub use document::{ChunkedDocument, Document};
pub use error::{Error, Result};
pub use loader::DocumentLoader;
pub use splitter::{MarkdownSplitter, SplitterConfig};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = SplitterConfig::default();
        assert_eq!(config.chunk_size, 600);
        assert_eq!(config.overlap, 0);
    }

    #[test]
    fn test_config_custom() {
        let config = SplitterConfig {
            chunk_size: 400,
            overlap: 50,
        };
        assert_eq!(config.chunk_size, 400);
        assert_eq!(config.overlap, 50);
    }
}
