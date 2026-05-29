//! Document loader from file system

use crate::document::{ChunkedDocument, Document};
use crate::error::{Error, Result};
use crate::splitter::{MarkdownSplitter, SplitterConfig};
use walkdir::WalkDir;

/// Document loader with batching and streaming options
pub struct DocumentLoader {
    config: SplitterConfig,
}

impl DocumentLoader {
    /// Create a new document loader
    pub fn new(config: SplitterConfig) -> Self {
        Self { config }
    }

    /// Load all markdown documents from a directory
    ///
    /// This loads and chunks all documents at once into memory.
    ///
    /// # Arguments
    ///
    /// * `dir` - Directory path containing .md files
    ///
    /// # Returns
    ///
    /// Vector of all chunks
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let loader = DocumentLoader::new(SplitterConfig::default());
    /// let chunks = loader.load_directory("./docs")?;
    /// println!("Loaded {} chunks", chunks.len());
    /// ```
    pub fn load_directory(&self, dir: &str) -> Result<Vec<ChunkedDocument>> {
        let splitter = MarkdownSplitter::new(self.config)?;
        let mut all_chunks = Vec::new();

        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        {
            let path = entry.path();
            let content = std::fs::read_to_string(path).map_err(Error::Io)?;

            let rel_path = path
                .strip_prefix(dir)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            let chunks = splitter.split(&rel_path, &content)?;
            all_chunks.extend(chunks);
        }

        Ok(all_chunks)
    }

    /// Load documents with streaming (memory-efficient)
    ///
    /// Processes one file at a time. Call the callback for each chunk.
    ///
    /// # Arguments
    ///
    /// * `dir` - Directory path
    /// * `callback` - Function called for each chunk
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// loader.load_directory_streaming("./docs", |chunk| {
    ///     println!("Processing: {} chars", chunk.char_count);
    /// })?;
    /// ```
    pub fn load_directory_streaming<F>(&self, dir: &str, mut callback: F) -> Result<()>
    where
        F: FnMut(ChunkedDocument) -> Result<()>,
    {
        let splitter = MarkdownSplitter::new(self.config)?;

        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        {
            let path = entry.path();
            let content = std::fs::read_to_string(path)?;

            let rel_path = path
                .strip_prefix(dir)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            let chunks = splitter.split(&rel_path, &content)?;
            for chunk in chunks {
                callback(chunk)?;
            }
        }

        Ok(())
    }

    /// Load a single markdown file
    pub fn load_file(&self, path: &str) -> Result<Vec<ChunkedDocument>> {
        let content = std::fs::read_to_string(path)?;
        let splitter = MarkdownSplitter::new(self.config)?;
        splitter.split(path, &content)
    }

    /// Load raw documents without chunking
    ///
    /// Useful for inspecting or preprocessing documents.
    pub fn load_raw_directory(&self, dir: &str) -> Result<Vec<Document>> {
        let mut documents = Vec::new();

        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        {
            let path = entry.path();
            let content = std::fs::read_to_string(path)?;

            let rel_path = path
                .strip_prefix(dir)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            documents.push(Document::new(rel_path, content));
        }

        Ok(documents)
    }

    /// Get loader configuration
    pub fn config(&self) -> SplitterConfig {
        self.config
    }

    /// Count markdown files in a directory (non-recursive at root level)
    pub fn count_markdown_files(&self, dir: &str) -> Result<usize> {
        let count = WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
            .count();

        Ok(count)
    }

    /// Get statistics about loaded documents
    pub fn get_stats(&self, dir: &str) -> Result<LoaderStats> {
        let raw_docs = self.load_raw_directory(dir)?;

        let total_size_bytes: usize = raw_docs.iter().map(|d| d.size_bytes).sum();
        let total_docs = raw_docs.len();
        let total_words: usize = raw_docs.iter().map(|d| d.word_count()).sum();
        let total_lines: usize = raw_docs.iter().map(|d| d.line_count()).sum();

        // Now chunk them to get chunk stats
        let chunks = self.load_directory(dir)?;

        Ok(LoaderStats {
            total_documents: total_docs,
            total_chunks: chunks.len(),
            total_size_bytes,
            total_words,
            total_lines,
            avg_chunk_size: if chunks.is_empty() {
                0
            } else {
                total_size_bytes / chunks.len()
            },
        })
    }
}

/// Statistics about loaded documents
#[derive(Debug, Clone)]
pub struct LoaderStats {
    /// Number of source documents
    pub total_documents: usize,
    /// Number of chunks generated
    pub total_chunks: usize,
    /// Total size in bytes
    pub total_size_bytes: usize,
    /// Total word count
    pub total_words: usize,
    /// Total line count
    pub total_lines: usize,
    /// Average chunk size in bytes
    pub avg_chunk_size: usize,
}

impl LoaderStats {
    /// Format stats as human-readable string
    pub fn summary(&self) -> String {
        format!(
            "📊 Documents: {} | Chunks: {} | Size: {}MB | Avg chunk: {}B | Words: {}",
            self.total_documents,
            self.total_chunks,
            self.total_size_bytes / 1_000_000,
            self.avg_chunk_size,
            self.total_words,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_creation() {
        let loader = DocumentLoader::new(SplitterConfig::default());
        assert_eq!(loader.config().chunk_size, 600);
    }
}
