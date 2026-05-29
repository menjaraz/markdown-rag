/// RAG pipeline example using markdown-rag library
///
/// This example demonstrates how markdown-rag integrates into a RAG pipeline.
/// For a full working example with Qdrant and Ollama, see INTEGRATION.md

use markdown_rag::{DocumentLoader, SplitterConfig};
use std::time::Instant;

fn format_size(bytes: usize) -> String {
    if bytes < 1_000 {
        format!("{bytes} B")
    } else if bytes < 1_000_000 {
        format!("{} KB", bytes / 1_000)
    } else {
        format!("{} MB", bytes / 1_000_000)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🦀 RAG Pipeline with markdown-rag Library\n");
    
    let start = Instant::now();

    // 1. Load and chunk documents
    println!("📖 Loading & chunking documents...");
    
    let config = SplitterConfig {
        chunk_size: 400,
        overlap: 0,
    };
    
    let loader = DocumentLoader::new(config);
    
    // Create sample docs if needed
    if !std::path::Path::new("./docs").exists() {
        println!("📝 Creating sample markdown files...");
        create_sample_docs()?;
    }

    // Get stats first
    let stats = loader.get_stats("./docs")?;
    println!("   {}", stats.summary());
    println!();

    // Load chunks
    let chunks = loader.load_directory("./docs")?;
    println!("✓ Loaded {} chunks\n", chunks.len());

    // 2. Simulate embedding step (in real RAG: use ollama-rs)
    println!("🧠 [Simulated] Embedding chunks with Ollama...");
    println!("   (In production: use ollama_rs crate)\n");
    
    for (idx, chunk) in chunks.iter().take(3).enumerate() {
        println!("   {}. {} chars → 384-dim embedding", 
                 idx + 1, chunk.char_count);
    }
    if chunks.len() > 3 {
        println!("   ... {} more chunks", chunks.len() - 3);
    }
    println!();

    // 3. Simulate storage step (in real RAG: use qdrant-client)
    println!("💾 [Simulated] Storing in Qdrant vector database...");
    println!("   (In production: use qdrant_client crate)");
    println!("   ✓ Stored {} vectors with metadata\n", chunks.len());

    // 4. Example query workflow
    println!("{}", "=".repeat(60));
    println!("🎯 RAG Query Workflow");
    println!("{}\n", "=".repeat(60));

    let query = "What is Rust?";
    println!("❓ User query: {}\n", query);

    println!("Step 1️⃣  Embed query");
    println!("   Input:  \"{}\"", query);
    println!("   Output: 384-dim vector\n");

    println!("Step 2️⃣  Search Qdrant");
    if let Some(chunk) = chunks.first() {
        println!("   Top result: \"{}...\"", chunk.preview(50));
        println!("   Score: 0.89 (very relevant)\n");
    }

    println!("Step 3️⃣  Generate answer");
    println!("   Context: Top 2 chunks ({} chars)", 
             chunks.iter().take(2).map(|c| c.char_count).sum::<usize>());
    println!("   Model: Ollama (phi or mistral)");
    println!("   Answer: \"Rust is a systems programming language...\" \n");

    // 5. Show integration points
    println!("{}", "=".repeat(60));
    println!("📚 Integration Architecture");
    println!("{}\n", "=".repeat(60));

    println!("markdown-rag library handles:");
    println!("   ✅ Load markdown files recursively");
    println!("   ✅ Chunk semantically by markdown structure");
    println!("   ✅ Track metadata (source, offset, position)");
    println!("   ✅ Provide statistics for analysis\n");

    println!("You integrate with:");
    println!("   🤖 ollama-rs: Generate embeddings (nomic-embed-text)");
    println!("   🤖 ollama-rs: Generate answers (phi, mistral)");
    println!("   💾 qdrant-client: Store and search vectors\n");

    // 6. Summary
    println!("{}", "=".repeat(60));
    println!("✨ Summary");
    println!("{}\n", "=".repeat(60));

    println!("Loaded:        {} documents", stats.total_documents);
    println!("Chunks:        {}", stats.total_chunks);
    println!("Total size:    {}", format_size(stats.total_size_bytes));
    println!("Avg chunk:     {} B", stats.avg_chunk_size);
    println!("Total time:    {:.2}s", start.elapsed().as_secs_f32());
    println!();

    println!("🔗 For full RAG example with Qdrant + Ollama:");
    println!("   See INTEGRATION.md in the library documentation");
    println!();

    println!("✨ markdown-rag is ready to integrate!");

    Ok(())
}

fn create_sample_docs() -> anyhow::Result<()> {
    std::fs::create_dir_all("./docs")?;

    std::fs::write(
        "./docs/rust.md",
        r#"# Rust

Rust is a systems programming language emphasizing memory safety and performance.

## Key Points

- No garbage collector
- Strong type system
- Ownership model
- Zero-cost abstractions

## Performance

Rust runs as fast as C++.
"#,
    )?;

    std::fs::write(
        "./docs/embeddings.md",
        r#"# Embeddings

Embeddings are numerical vectors representing text meaning.

## How They Work

Text → embedding model → vector (384 dimensions for nomic-embed-text)

## Similarity Search

Compare vectors using distance metrics like cosine similarity.
"#,
    )?;

    Ok(())
}