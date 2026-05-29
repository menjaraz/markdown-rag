//! Document and chunk data structures

use serde::{Deserialize, Serialize};

/// A raw markdown document loaded from disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// File path relative to the root directory
    pub path: String,
    /// Full file content
    pub content: String,
    /// File size in bytes
    pub size_bytes: usize,
}

impl Document {
    /// Create a new document
    pub fn new(path: String, content: String) -> Self {
        let size_bytes = content.len();
        Self {
            path,
            content,
            size_bytes,
        }
    }

    /// Check if document is empty
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Get line count
    pub fn line_count(&self) -> usize {
        self.content.lines().count()
    }

    /// Get approximate word count
    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }
}

/// A semantic chunk of a markdown document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkedDocument {
    /// Source file path
    pub source: String,
    /// Chunk content
    pub content: String,
    /// Chunk sequence number (0-indexed)
    pub chunk_index: usize,
    /// Byte offset in original document
    pub byte_offset: usize,
    /// Character count
    pub char_count: usize,
    /// Approximate word count
    pub word_count: usize,
}

impl ChunkedDocument {
    /// Create a new chunk
    pub fn new(
        source: String,
        content: String,
        chunk_index: usize,
        byte_offset: usize,
    ) -> Self {
        let char_count = content.len();
        let word_count = content.split_whitespace().count();

        Self {
            source,
            content,
            chunk_index,
            byte_offset,
            char_count,
            word_count,
        }
    }

    /// Get a preview of the chunk (first N characters)
    pub fn preview(&self, max_chars: usize) -> String {
        self.content
            .chars()
            .take(max_chars)
            .collect::<String>()
            + if self.char_count > max_chars { "..." } else { "" }
    }

    /// Check if chunk is too small (might be orphaned)
    pub fn is_small(&self, threshold: usize) -> bool {
        self.char_count < threshold
    }

    /// Estimate reading time in seconds (assuming 200 words per minute)
    pub fn reading_time_secs(&self) -> u32 {
        ((self.word_count as f32 / 200.0) * 60.0) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new("test.md".to_string(), "Hello\nWorld".to_string());
        assert_eq!(doc.path, "test.md");
        assert_eq!(doc.line_count(), 2);
        assert_eq!(doc.word_count(), 2);
        assert!(!doc.is_empty());
    }

    #[test]
    fn test_chunk_preview() {
        let chunk = ChunkedDocument::new(
            "test.md".to_string(),
            "This is a very long chunk that should be truncated".to_string(),
            0,
            0,
        );
        assert_eq!(chunk.preview(10), "This is a ...");
    }

    #[test]
    fn test_chunk_reading_time() {
        let chunk = ChunkedDocument::new(
            "test.md".to_string(),
            "word ".repeat(200), // 200 words
            0,
            0,
        );
        assert_eq!(chunk.reading_time_secs(), 60); // 1 minute
    }
}
