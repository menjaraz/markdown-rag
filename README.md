# Markdown RAG

A semantic markdown document loader and chunker for RAG pipelines, optimized for resource-constrained environments.

[![Crates.io](https://img.shields.io/crates/v/markdown-rag.svg)](https://crates.io/crates/markdown-rag)
[![License](https://img.shields.io/crates/l/markdown-rag.svg)](https://github.com/menjaraz/markdown-rag#license)

## Features

✨ **Semantic Chunking**: Respects markdown structure (headers, paragraphs, code blocks)
⚡ **Configurable**: Chunk size, overlap, and preset profiles
📚 **Batch & Stream**: Load all at once or process one file at a time
📊 **Statistics**: Get detailed stats about your documents
🔍 **Metadata**: Track source paths, byte offsets, word/char counts
🎯 **CPU-Only Optimized**: Presets for low-resource machines (350 chars/chunk)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
markdown-rag = "0.1"
```

## Quick Start

### Basic Usage

```rust
use markdown_rag::{DocumentLoader, SplitterConfig};

fn main() -> Result<()> {
    // Use default config (600 char chunks)
    let loader = DocumentLoader::new(SplitterConfig::default());
    
    // Load all markdown files from a directory
    let chunks = loader.load_directory("./docs")?;
    
    for chunk in chunks {
        println!(
            "[{}] {} chars, {} words",
            chunk.source, chunk.char_count, chunk.word_count
        );
        println!("Preview: {}\n", chunk.preview(100));
    }
    
    Ok(())
}
```

### Custom Configuration

```rust
use markdown_rag::{DocumentLoader, SplitterConfig};

let config = SplitterConfig {
    chunk_size: 400,    // Smaller chunks for CPU-only environments
    overlap: 50,        // 50 char overlap between chunks
};

let loader = DocumentLoader::new(config);
let chunks = loader.load_directory("./docs")?;
```

### Stream Mode (Memory Efficient)

```rust
use markdown_rag::DocumentLoader;

let loader = DocumentLoader::new(SplitterConfig::default());

loader.load_directory_streaming("./docs", |chunk| {
    // Process each chunk as it's created
    println!("Processing: {}", chunk.source);
    Ok(())
})?;
```

### Preset Configurations

```rust
use markdown_rag::MarkdownSplitter;

// For small devices (300 chars)
let splitter = MarkdownSplitter::for_small_devices()?;

// For context preservation (1000 chars)
let splitter = MarkdownSplitter::for_context_preservation()?;

// Custom
let splitter = MarkdownSplitter::standard()?;
```

## API

### `DocumentLoader`

Main API for loading and chunking documents.

```rust
pub struct DocumentLoader {
    pub fn new(config: SplitterConfig) -> Self
    pub fn load_directory(&self, dir: &str) -> Result<Vec<ChunkedDocument>>
    pub fn load_directory_streaming<F>(&self, dir: &str, callback: F) -> Result<()>
    pub fn load_file(&self, path: &str) -> Result<Vec<ChunkedDocument>>
    pub fn load_raw_directory(&self, dir: &str) -> Result<Vec<Document>>
    pub fn get_stats(&self, dir: &str) -> Result<LoaderStats>
    pub fn count_markdown_files(&self, dir: &str) -> Result<usize>
}
```

### `MarkdownSplitter`

Low-level semantic chunking.

```rust
pub struct MarkdownSplitter {
    pub fn new(config: SplitterConfig) -> Result<Self>
    pub fn split(&self, source: &str, content: &str) -> Result<Vec<ChunkedDocument>>
    pub fn split_batch(&self, docs: &[(String, String)]) -> Result<Vec<ChunkedDocument>>
    pub fn for_small_devices() -> Result<Self>
    pub fn for_context_preservation() -> Result<Self>
}
```

### `ChunkedDocument`

Represents a semantic chunk.

```rust
pub struct ChunkedDocument {
    pub source: String,           // Source file path
    pub content: String,          // Chunk text
    pub chunk_index: usize,       // Position in document
    pub byte_offset: usize,       // Position in source file
    pub char_count: usize,        // Length in characters
    pub word_count: usize,        // Approximate word count
}

impl ChunkedDocument {
    pub fn preview(&self, max_chars: usize) -> String
    pub fn is_small(&self, threshold: usize) -> bool
    pub fn reading_time_secs(&self) -> u32
}
```

## Examples

### Complete RAG Integration

```rust
use markdown_rag::{DocumentLoader, SplitterConfig};
use qdrant_client::Qdrant;

#[tokio::main]
async fn main() -> Result<()> {
    // Load and chunk documents
    let config = SplitterConfig::default();
    let loader = DocumentLoader::new(config);
    let chunks = loader.load_directory("./docs")?;
    
    println!("Loaded {} chunks", chunks.len());
    
    // Connect to Qdrant
    let qdrant = Qdrant::from_url("http://localhost:6334").build()?;
    
    // Process each chunk with embeddings
    for chunk in chunks {
        // Generate embedding
        let embedding = embed(&chunk.content).await?;
        
        // Store in Qdrant
        let point = create_point(&chunk, embedding)?;
        qdrant.upsert_points(collection_name, vec![point]).await?;
    }
    
    Ok(())
}
```

### Statistics and Analysis

```rust
let loader = DocumentLoader::new(SplitterConfig::default());
let stats = loader.get_stats("./docs")?;

println!("{}", stats.summary());
// 📊 Documents: 5 | Chunks: 32 | Size: 2MB | Avg chunk: 64KB | Words: 15000
```

### Stream Processing (CPU-Only, 8 GB RAM)

```rust
let config = SplitterConfig {
    chunk_size: 350,
    overlap: 0,
};

let loader = DocumentLoader::new(config);

loader.load_directory_streaming("./docs", |chunk| {
    // Process immediately, don't store in memory
    println!(
        "Chunk {}: {} chars, {}s to read",
        chunk.chunk_index,
        chunk.char_count,
        chunk.reading_time_secs()
    );
    Ok(())
})?;
```

## Configuration Presets

### Standard RAG (600 chars)
```rust
SplitterConfig::default()
// chunk_size: 600, overlap: 0
```

### Low-Resource / CPU-Only (350 chars)
```rust
MarkdownSplitter::for_small_devices()?
// chunk_size: 350, overlap: 0
```

### Context Preservation (1000 chars + overlap)
```rust
MarkdownSplitter::for_context_preservation()?
// chunk_size: 1000, overlap: 100
```

## Performance

On a low-resource machine (CPU-only, 8 GB RAM):

- **Loading 10 documents (~100KB)**: ~500ms
- **Chunking 100 documents (~1MB)**: ~2s
- **Peak memory**: <200MB
- **Stream mode**: Constant ~50MB

## Error Handling

```rust
use markdown_rag::error::Error;

match loader.load_directory("./docs") {
    Ok(chunks) => println!("Loaded {} chunks", chunks.len()),
    Err(Error::Io(e)) => eprintln!("File error: {}", e),
    Err(Error::EmptyDocument { path }) => eprintln!("Empty file: {}", path),
    Err(Error::InvalidConfig { reason }) => eprintln!("Bad config: {}", reason),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Testing

Run tests:

```bash
cargo test
```

With output:

```bash
cargo test -- --nocapture
```

## Benchmarks

Compare chunk sizes:

```rust
use markdown_rag::{MarkdownSplitter, SplitterConfig};

let content = std::fs::read_to_string("large_doc.md")?;

for chunk_size in [300, 400, 600, 1000] {
    let config = SplitterConfig {
        chunk_size,
        overlap: 0,
    };
    let splitter = MarkdownSplitter::new(config)?;
    let chunks = splitter.split("doc.md", &content)?;
    println!(
        "chunk_size={}: {} chunks (avg {}B)",
        chunk_size,
        chunks.len(),
        chunks.iter().map(|c| c.char_count).sum::<usize>() / chunks.len()
    );
}
```

## Contributing

Contributions welcome! Areas:

- [ ] Token-based chunking (not just chars)
- [ ] Custom splitter strategies
- [ ] Overlap implementation
- [ ] More preset profiles
- [ ] Parallel loading

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## See Also

- [text-splitter](https://crates.io/crates/text-splitter) - Underlying markdown parsing
- [qdrant-client](https://crates.io/crates/qdrant-client) - Vector database
- [ollama-rs](https://crates.io/crates/ollama-rs) - Local LLM embeddings
