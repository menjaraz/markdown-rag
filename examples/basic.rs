//! Basic example: Load and chunk markdown documents

use markdown_rag::{DocumentLoader, SplitterConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📚 Markdown RAG Example\n");

    // Create loader with custom config
    let config = SplitterConfig {
        chunk_size: 600,
        overlap: 0,
    };

    let loader = DocumentLoader::new(config);

    // Load all markdown files
    println!("📖 Loading documents from ./docs...");
    let chunks = loader.load_directory("./docs")?;

    if chunks.is_empty() {
        println!("No markdown files found. Skipping.\n");
        return Ok(());
    }

    println!("✓ Loaded {} chunks\n", chunks.len());

    // Display statistics
    println!("📊 Statistics:");
    println!("   Total chunks: {}", chunks.len());
    println!(
        "   Total characters: {}",
        chunks.iter().map(|c| c.char_count).sum::<usize>()
    );
    println!(
        "   Total words: {}",
        chunks.iter().map(|c| c.word_count).sum::<usize>()
    );
    println!(
        "   Avg chunk size: {}B\n",
        chunks.iter().map(|c| c.char_count).sum::<usize>() / chunks.len()
    );

    // Show sample chunks
    println!("📄 Sample chunks:");
    for chunk in chunks.iter().take(3) {
        println!("\n  Source: {}", chunk.source);
        println!("  Chunk: {}/{}", chunk.chunk_index + 1, chunks.len());
        println!("  Size: {} chars, {} words", chunk.char_count, chunk.word_count);
        println!("  Preview: {}", chunk.preview(80));
    }

    println!("\n✨ Done!");
    Ok(())
}
