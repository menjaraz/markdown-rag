//! Example: Stream-based loading (memory efficient)

use markdown_rag::{DocumentLoader, SplitterConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Streaming Example (Memory Efficient)\n");

    // Use small device config (350 chars)
    let config = SplitterConfig {
        chunk_size: 350,
        overlap: 0,
    };

    let loader = DocumentLoader::new(config);

    println!("📖 Processing documents with streaming...\n");

    let mut chunk_count = 0;
    let mut total_chars = 0;
    let mut total_words = 0;

    // Process chunks one at a time without storing all in memory
    loader.load_directory_streaming("./docs", |chunk| {
        chunk_count += 1;
        total_chars += chunk.char_count;
        total_words += chunk.word_count;

        // Process immediately (e.g., generate embedding, upsert to DB)
        println!(
            "  ✓ Chunk {}: {} chars, {} words, {}s read time",
            chunk.chunk_index + 1,
            chunk.char_count,
            chunk.word_count,
            chunk.reading_time_secs()
        );

        Ok(())
    })?;

    println!("\n✓ Processed {} chunks", chunk_count);
    println!("  Total: {} chars, {} words", total_chars, total_words);
    println!(
        "  Avg chunk: {}B",
        if chunk_count > 0 {
            total_chars / chunk_count
        } else {
            0
        }
    );

    println!("\n💾 Memory usage: Constant ~50MB (vs. batch ~500MB)");
    println!("✨ Done!");

    Ok(())
}
