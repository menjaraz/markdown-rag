# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-05-29

### Added
- `DocumentLoader` with batch (`load_directory`) and streaming (`load_directory_streaming`) modes
- `MarkdownSplitter` with semantic chunking via `text-splitter` crate
- `SplitterConfig` with validation (min chunk size 100, overlap < chunk size)
- Preset configurations: `for_small_devices()` (350 chars), `for_context_preservation()` (1000 chars, 100 overlap), `standard()` (600 chars)
- `ChunkedDocument` with metadata: source path, byte offset, chunk index, char/word counts
- `LoaderStats` with human-readable `summary()` and adaptive size formatting (B/KB/MB)
- Four examples: `basic`, `streaming`, `stats`, `rag-integration`

[0.1.0]: https://github.com/menjaraz/markdown-rag/releases/tag/v0.1.0
