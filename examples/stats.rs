//! Example: Document statistics and analysis

use markdown_rag::{DocumentLoader, SplitterConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Statistics Example\n");

    let config = SplitterConfig::default();
    let loader = DocumentLoader::new(config);

    // Get detailed statistics
    let stats = loader.get_stats("./docs")?;

    println!("📈 Document Statistics");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Documents:      {}", stats.total_documents);
    println!("  Chunks:         {}", stats.total_chunks);
    println!("  Total size:     {} MB", stats.total_size_bytes / 1_000_000);
    println!("  Total words:    {}", stats.total_words);
    println!("  Total lines:    {}", stats.total_lines);
    println!("  Avg chunk size: {} B", stats.avg_chunk_size);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Summary line
    println!("📌 {}\n", stats.summary());

    // Estimate reading/embedding time
    let est_reading_mins = stats.total_words / 200;
    println!("⏱️  Estimated reading time: ~{} mins", est_reading_mins);
    println!(
        "⏱️  Estimated embedding time (1 char/100ms): ~{}s",
        stats.total_size_bytes / 100
    );

    // Compare chunk sizes
    println!("\n🔍 Chunk Size Analysis:");
    println!("  Current chunk size: 600");
    println!("  Small device equivalent (350): ~{} chunks", 
             (stats.total_chunks * 600) / 350);
    println!("  Large context (1000): ~{} chunks",
             (stats.total_chunks * 600) / 1000);

    Ok(())
}
