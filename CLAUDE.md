# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**markdown-rag** is a Rust library for semantic markdown document loading and chunking, optimized for RAG (Retrieval-Augmented Generation) pipelines. It respects markdown structure (headers, paragraphs, code blocks) when splitting documents into configurable chunks, with support for both batch and streaming modes. The library is designed to handle resource-constrained environments (e.g., Lenovo T460 with 8GB RAM).

**Key use case**: Preprocessing markdown documentation for vector databases (Qdrant) with embeddings (Ollama).

## Build & Development Commands

```bash
# Build the library
cargo build

# Build optimized release
cargo build --release

# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a single test
cargo test test_name -- --exact

# Run an example
cargo run --example basic
cargo run --example streaming
cargo run --example stats
cargo run --example rag-integration

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Generate documentation
cargo doc --open
```

## Architecture & Code Organization

### Module Structure

The library is organized into four core modules:

1. **`document.rs`** — Data structures for documents and chunks
   - `Document`: Raw markdown loaded from disk (path, content, size_bytes)
   - `ChunkedDocument`: Semantic chunk with metadata (source, content, chunk_index, byte_offset, char_count, word_count)
   - Methods: `preview()`, `is_small()`, `reading_time_secs()`

2. **`splitter.rs`** — Markdown semantic chunking
   - `SplitterConfig`: Configuration struct (chunk_size, overlap)
   - `MarkdownSplitter`: Core splitting logic using `text-splitter` crate
   - Validates config (chunk_size >= 100, overlap < chunk_size)
   - Provides preset configurations: `for_small_devices()` (350 chars), `for_context_preservation()` (1000 chars)
   - Key methods: `split()` (single doc), `split_batch()` (multiple docs)

3. **`loader.rs`** — Filesystem operations for documents
   - `DocumentLoader`: High-level API for loading markdown files from directories
   - Two primary modes:
     - **Batch**: `load_directory()` — loads all files into memory
     - **Streaming**: `load_directory_streaming()` — processes files one at a time with callback
   - Additional methods: `load_file()`, `load_raw_directory()`, `count_markdown_files()`, `get_stats()`
   - `LoaderStats`: Statistics struct with summary formatting

4. **`error.rs`** — Error types
   - Custom error enum using `thiserror` crate
   - Variants: `Io`, `EmptyDocument`, `InvalidPath`, `InvalidConfig`, `InvalidUtf8`, `Other`
   - Conversions from `String` and `&str` for ergonomics

### Data Flow

```
Raw Markdown Files
       ↓ (DocumentLoader::load_directory / load_directory_streaming)
     Document objects
       ↓ (MarkdownSplitter::split)
     ChunkedDocument objects
       ↓ (User code: embed, upsert to vector DB)
     Vector database
```

### Dependencies

- **text-splitter** (v0.14): Underlying semantic markdown splitting via `MarkdownSplitter<Characters>`
- **walkdir** (v2): Recursive directory traversal
- **anyhow** (v1.0): Error context
- **thiserror** (v1.0): Error macros
- **serde** (v1.0): Serialization framework
- **serde_json** (v1.0): JSON support
- **tokio** (dev-only): Async runtime for examples

### Configuration Presets & Performance Characteristics

The library provides three preset configurations optimized for different scenarios:

| Preset | Chunk Size | Overlap | Use Case | Performance (T460) |
|--------|-----------|---------|----------|-------------------|
| **Standard** | 600 chars | 0 | Default RAG | ~5-10 chunks/sec |
| **Small Devices** | 350 chars | 0 | T460, 8GB RAM | Fewer vectors, higher granularity |
| **Context Preservation** | 1000 chars | 100 | 16GB+ RAM | Better context, fewer vectors |

T460 (Intel i5-6200U, 8GB RAM) benchmarks:
- Loading 10 docs (~100KB): ~500ms
- Chunking 100 docs (~1MB): ~2s
- Peak memory (batch): <200MB
- Peak memory (stream): ~50MB (constant)

## Usage Patterns

### Pattern 1: Batch Loading (All-in-Memory)
```rust
let loader = DocumentLoader::new(SplitterConfig::default());
let chunks = loader.load_directory("./docs")?;
for chunk in chunks { /* process */ }
```
**When**: Small document sets (<100MB), parallel processing preferred

### Pattern 2: Streaming (Memory-Efficient)
```rust
loader.load_directory_streaming("./docs", |chunk| {
    // Process immediately, don't store in memory
    Ok(())
})?;
```
**When**: Large document sets (>100MB) or resource-constrained environments

### Pattern 3: Analysis First
```rust
let stats = loader.get_stats("./docs")?;
// Decide batch vs stream based on stats.total_chunks
```

## Testing Strategy

- **Unit tests**: In each module (e.g., `#[cfg(test)] mod tests`)
- **Config validation**: `SplitterConfig` validates chunk_size and overlap
- **Empty document handling**: Returns `Error::EmptyDocument`
- **Preset validation**: Tests that preset configs are correctly initialized
- **Integration examples**: In `examples/` directory (not formal tests, but runnable)

## Integration with RAG Pipelines

The library sits in Tier 1 of a three-tier RAG architecture:

1. **Tier 1 (markdown-rag)**: Document loading and semantic chunking
2. **Tier 2 (ollama-rs)**: Embedding generation
3. **Tier 3 (qdrant-client)**: Vector storage and similarity search

See `INTEGRATION.md` for complete RAG pipeline examples (batch and streaming variants).

## Key Design Decisions

1. **Character-based chunking** (not token-based): Simpler, deterministic, works across languages
2. **Semantic boundaries**: Respects markdown structure via `text-splitter` crate
3. **Metadata preservation**: Tracks source path, byte offset, chunk index for traceability
4. **Dual modes (batch/streaming)**: Flexibility for different memory and performance constraints
5. **Configuration validation**: Prevents invalid configurations early
6. **Error types**: Comprehensive error enum for granular error handling

## Extension Points

Future contributors can extend in these areas:

1. **Token-based chunking**: Implement token counting via LLM tokenizers (e.g., `tiktoken`)
2. **Custom splitter strategies**: Add domain-specific splitting logic
3. **Metadata extraction**: Extract headers, dates, language from documents
4. **Multi-format support**: Add PDF, HTML parsing (beyond markdown)
5. **Parallel processing**: Parallelize chunk generation (currently sequential)

## Common Tasks

### Adding a New Preset Configuration
1. Add a new method to `MarkdownSplitter` (e.g., `pub fn for_specialized_use_case()`)
2. Return `Self::new(SplitterConfig { chunk_size: X, overlap: Y })`
3. Add a test in `splitter.rs::tests`
4. Update `README.md` and `ARCHITECTURE.md`

### Modifying Error Handling
1. Update `error.rs` enum with new variant
2. Add conversion trait if needed (`impl From<X> for Error`)
3. Update error docstring
4. Update existing code that needs new error type

### Updating Dependencies
1. Modify `Cargo.toml`
2. Run `cargo check` to validate
3. Test with `cargo test`
4. Update documentation if API changes

## Files of Interest

- **`README.md`**: User-facing documentation and API reference
- **`ARCHITECTURE.md`**: Detailed architecture, module breakdown, and design patterns
- **`INTEGRATION.md`**: Complete RAG pipeline integration examples
- **`examples/`**: Working code samples (basic, streaming, stats, rag-integration)
- **`src/lib.rs`**: Public API exports and basic unit tests
