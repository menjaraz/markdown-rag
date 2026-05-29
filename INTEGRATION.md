# Integration Guide: Using `markdown-rag` in Your RAG Pipeline

This guide shows how to integrate the `markdown-rag` library into your Qdrant RAG pipeline.

## 1. Add Dependency

Update your `Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
markdown-rag = { path = "../markdown-rag" }  # Local path
# OR from crates.io (when published):
# markdown-rag = "0.1"
```

## 2. Replace Old Splitting Code

### Before (Inline)
```rust
use text_splitter::MarkdownSplitter;
use walkdir::WalkDir;

// Load documents manually
for entry in WalkDir::new("./docs")
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
{
    let content = fs::read_to_string(path)?;
    let splitter = MarkdownSplitter::new(600);
    let chunks: Vec<&str> = splitter.chunks(&content).collect();
    // ... process chunks ...
}
```

### After (Using Library)
```rust
use markdown_rag::{DocumentLoader, SplitterConfig};

let config = SplitterConfig::default();  // or custom
let loader = DocumentLoader::new(config);
let chunks = loader.load_directory("./docs")?;

for chunk in chunks {
    // ... process chunk ...
}
```

## 3. RAG Pipeline Integration Examples

### Standard Pipeline (Batch)

```rust
use markdown_rag::{DocumentLoader, SplitterConfig};
use qdrant_client::Qdrant;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load and chunk documents
    let loader = DocumentLoader::new(SplitterConfig::default());
    let chunks = loader.load_directory("./docs")?;
    println!("✓ Loaded {} chunks", chunks.len());

    // 2. Connect to Qdrant and Ollama
    let qdrant = Qdrant::from_url("http://localhost:6334").build()?;
    let ollama = Ollama::default();

    // 3. Embed and store
    for chunk in chunks {
        let embedding = ollama
            .generate_embeddings(GenerateEmbeddingsRequest::new(
                "nomic-embed-text".to_string(),
                chunk.content.clone().into(),
            ))
            .await?;

        let point = PointStruct::new(
            chunk.chunk_index as u64,
            embedding.embeddings[0].clone(),
            [
                ("source", chunk.source.into()),
                ("text", chunk.content.into()),
            ]
            .into_iter()
            .collect(),
        );

        qdrant.upsert_points("knowledge_base", vec![point]).await?;
    }

    Ok(())
}
```

### Low-Resource Pipeline (Streaming, CPU-Only)

```rust
use markdown_rag::{DocumentLoader, SplitterConfig};

#[tokio::main]
async fn main() -> Result<()> {
    // Use small device config
    let config = SplitterConfig {
        chunk_size: 350,
        overlap: 0,
    };

    let loader = DocumentLoader::new(config);
    let qdrant = Qdrant::from_url("http://localhost:6334").build()?;
    let ollama = Ollama::default();

    let mut point_id = 1u64;

    // Stream: Process one chunk at a time
    loader.load_directory_streaming("./docs", |chunk| {
        // Don't store in memory, process immediately
        
        // Embed
        let embedding = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                ollama
                    .generate_embeddings(GenerateEmbeddingsRequest::new(
                        "nomic-embed-text".to_string(),
                        chunk.content.clone().into(),
                    ))
                    .await
            })
        })?;

        // Store
        let point = PointStruct::new(
            point_id,
            embedding.embeddings[0].clone(),
            [("source", chunk.source.into()), ("text", chunk.content.into())]
                .into_iter()
                .collect(),
        );

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                qdrant.upsert_points("knowledge_base", vec![point]).await
            })
        })?;

        point_id += 1;
        Ok(())
    })?;

    Ok(())
}
```

## 4. Configuration Strategies

### For Low-Resource (CPU-Only, 8 GB RAM)
```rust
let config = SplitterConfig {
    chunk_size: 350,  // Smaller chunks
    overlap: 0,
};
let loader = DocumentLoader::new(config);
```

### For Standard Machine (16GB+ RAM)
```rust
let config = SplitterConfig {
    chunk_size: 800,  // Larger chunks preserve context
    overlap: 100,     // Overlap helps continuity
};
let loader = DocumentLoader::new(config);
```

### Using Presets
```rust
// CPU-only friendly
let splitter = MarkdownSplitter::for_small_devices()?;

// Preserve context (larger)
let splitter = MarkdownSplitter::for_context_preservation()?;

// Default (standard)
let splitter = MarkdownSplitter::standard()?;
```

## 5. Analyzing Before Embedding

```rust
use markdown_rag::DocumentLoader;

let loader = DocumentLoader::new(SplitterConfig::default());

// Get statistics first
let stats = loader.get_stats("./docs")?;
println!("{}", stats.summary());

// Decide strategy based on size
if stats.total_chunks > 1000 {
    println!("⚠️  Large document set. Consider streaming mode.");
}

// Load only if reasonable
if stats.total_size_bytes < 100_000_000 {  // < 100MB
    let chunks = loader.load_directory("./docs")?;
    // Process normally
}
```

## 6. Testing Integration

Create a test to verify the pipeline:

```rust
#[tokio::test]
async fn test_rag_pipeline() -> Result<()> {
    // Create sample docs
    std::fs::create_dir_all("./test_docs")?;
    std::fs::write("./test_docs/test.md", "# Test\n\nContent here.")?;

    // Load
    let loader = DocumentLoader::new(SplitterConfig::default());
    let chunks = loader.load_directory("./test_docs")?;
    
    assert!(!chunks.is_empty());
    assert_eq!(chunks[0].source, "test.md");

    // Cleanup
    std::fs::remove_file("./test_docs/test.md")?;
    std::fs::remove_dir("./test_docs")?;

    Ok(())
}
```

## 7. Benefits of Using the Library

| Aspect | Inline Code | Library |
|--------|-------------|---------|
| **Lines of code** | 30-50 | 5-10 |
| **Error handling** | Manual | Built-in |
| **Reusability** | One project | Multiple projects |
| **Testing** | You test | Library tested |
| **Documentation** | Inline comments | Full docs |
| **Performance analysis** | Manual stats | `get_stats()` method |
| **Streaming support** | Custom impl | Built-in |

## 8. Upgrading from Inline

Migration checklist:

- [ ] Add `markdown-rag` to `Cargo.toml`
- [ ] Replace `walkdir` + `text-splitter` code with `DocumentLoader`
- [ ] Update chunk structure usage (if needed)
- [ ] Run tests to verify same behavior
- [ ] Commit and deploy

## 9. Troubleshooting

### Issue: "No chunks generated"
```rust
// Chunk size too large relative to document size
let config = SplitterConfig {
    chunk_size: 300,  // Reduce from 600
    overlap: 0,
};
```

### Issue: Too many chunks
```rust
// Chunk size too small
let config = SplitterConfig {
    chunk_size: 1000,  // Increase from 600
    overlap: 0,
};
```

### Issue: Out of memory
```rust
// Switch to streaming
loader.load_directory_streaming("./docs", |chunk| {
    // Process immediately
    Ok(())
})?;
```

## 10. Publishing the Library

When ready to publish:

```bash
cd markdown-rag
cargo test
cargo publish
```

Then use in other projects:

```toml
[dependencies]
markdown-rag = "0.1"
```

---

**Next**: Check the `examples/` directory for working code samples.
