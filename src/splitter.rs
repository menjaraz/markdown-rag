//! Markdown semantic text splitter

use crate::document::ChunkedDocument;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use text_splitter::{Characters, MarkdownSplitter as TextSplitter};

/// Configuration for markdown splitting
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SplitterConfig {
    /// Target chunk size in characters
    pub chunk_size: usize,
    /// Overlap between chunks in characters (experimental, usually 0)
    pub overlap: usize,
}

impl SplitterConfig {
    /// Create a new configuration
    pub fn new(chunk_size: usize, overlap: usize) -> Self {
        Self { chunk_size, overlap }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.chunk_size < 100 {
            return Err(Error::InvalidConfig {
                reason: "chunk_size must be at least 100".to_string(),
            });
        }
        if self.overlap >= self.chunk_size {
            return Err(Error::InvalidConfig {
                reason: "overlap must be less than chunk_size".to_string(),
            });
        }
        Ok(())
    }
}

impl Default for SplitterConfig {
    fn default() -> Self {
        Self {
            chunk_size: 600,
            overlap: 0,
        }
    }
}

/// Markdown document splitter
pub struct MarkdownSplitter {
    config: SplitterConfig,
    splitter: TextSplitter<Characters>,
}

impl MarkdownSplitter {
    /// Create a new markdown splitter
    pub fn new(config: SplitterConfig) -> Result<Self> {
        config.validate()?;

        let splitter = TextSplitter::new(config.chunk_size);

        Ok(Self { config, splitter })
    }

    /// Split a document into semantic chunks
    ///
    /// # Arguments
    ///
    /// * `source` - Source file path
    /// * `content` - Document content
    ///
    /// # Returns
    ///
    /// Vector of chunked documents
    pub fn split(&self, source: &str, content: &str) -> Result<Vec<ChunkedDocument>> {
        if content.is_empty() {
            return Err(Error::EmptyDocument {
                path: source.to_string(),
            });
        }

        let chunks: Vec<&str> = self.splitter.chunks(content).collect();

        if chunks.is_empty() {
            return Err(Error::InvalidConfig {
                reason: format!(
                    "No chunks generated. Try reducing chunk_size from {}",
                    self.config.chunk_size
                ),
            });
        }

        let mut result = Vec::new();
        let mut byte_offset = 0;

        for (idx, chunk_text) in chunks.iter().enumerate() {
            let chunk = ChunkedDocument::new(
                source.to_string(),
                chunk_text.to_string(),
                idx,
                byte_offset,
            );

            byte_offset += chunk_text.len();
            result.push(chunk);
        }

        Ok(result)
    }

    /// Split multiple documents
    pub fn split_batch(&self, docs: &[(String, String)]) -> Result<Vec<ChunkedDocument>> {
        let mut all_chunks = Vec::new();

        for (source, content) in docs {
            let chunks = self.split(source, content)?;
            all_chunks.extend(chunks);
        }

        Ok(all_chunks)
    }

    /// Get configuration
    pub fn config(&self) -> SplitterConfig {
        self.config
    }

    /// Create a splitter with standard config (600 char chunks)
    pub fn standard() -> Result<Self> {
        Self::new(SplitterConfig::default())
    }

    /// Create a splitter optimized for small devices (300-400 char chunks)
    pub fn for_small_devices() -> Result<Self> {
        Self::new(SplitterConfig {
            chunk_size: 350,
            overlap: 0,
        })
    }

    /// Create a splitter optimized for context (800-1200 char chunks)
    pub fn for_context_preservation() -> Result<Self> {
        Self::new(SplitterConfig {
            chunk_size: 1000,
            overlap: 100,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let invalid = SplitterConfig {
            chunk_size: 50,
            overlap: 0,
        };
        assert!(invalid.validate().is_err());

        let invalid = SplitterConfig {
            chunk_size: 100,
            overlap: 100,
        };
        assert!(invalid.validate().is_err());

        let valid = SplitterConfig {
            chunk_size: 500,
            overlap: 50,
        };
        assert!(valid.validate().is_ok());
    }

    #[test]
    fn test_splitter_creation() {
        let splitter = MarkdownSplitter::new(SplitterConfig::default());
        assert!(splitter.is_ok());
    }

    #[test]
    fn test_splitter_split() {
        let splitter = MarkdownSplitter::standard().unwrap();
        let content = "# Header\n\nParagraph 1.\n\nParagraph 2.".to_string();
        let chunks = splitter.split("test.md", &content).unwrap();

        assert!(!chunks.is_empty());
        assert!(chunks[0].word_count > 0);
    }

    #[test]
    fn test_empty_document_error() {
        let splitter = MarkdownSplitter::standard().unwrap();
        let result = splitter.split("test.md", "");

        assert!(matches!(result, Err(Error::EmptyDocument { .. })));
    }

    #[test]
    fn test_preset_configs() {
        let small = MarkdownSplitter::for_small_devices();
        assert!(small.is_ok());
        assert_eq!(small.unwrap().config().chunk_size, 350);

        let context = MarkdownSplitter::for_context_preservation();
        assert!(context.is_ok());
        assert_eq!(context.unwrap().config().chunk_size, 1000);
    }

    #[test]
    fn test_batch_split() {
        let splitter = MarkdownSplitter::standard().unwrap();
        let docs = vec![
            ("doc1.md".to_string(), "First doc content".to_string()),
            ("doc2.md".to_string(), "Second doc content".to_string()),
        ];
        let chunks = splitter.split_batch(&docs).unwrap();

        assert!(chunks.len() >= 2);
        assert_eq!(chunks[0].source, "doc1.md");
        assert!(chunks.iter().any(|c| c.source == "doc2.md"));
    }
}
