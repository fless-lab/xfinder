# xfinder

**Advanced file search and retrieval system for Windows (will be extended to other OS) administrative environments**

## Overview

xfinder is a high-performance desktop search application designed for administrative users who need to locate files and information quickly across large document repositories. Built with Rust and native UI technologies, it provides enterprise-grade search capabilities in a lightweight package.

## Key Features

- **Fast Indexing**: Full-text search engine powered by Tantivy with sub-100ms query response time
- **Real-time Monitoring**: Automatic file system watching and index updates
- **Semantic Search**: AI-powered search understanding natural language queries
- **Email Integration**: Unified search across Outlook PST files, Thunderbird MBOX, and IMAP accounts
- **OCR Support**: Automatic text extraction from scanned PDFs and images (Tesseract 5)
- **Conversational Interface**: "Assist Me" mode providing contextual answers with verifiable sources

---

## Core Capabilities

### File Search
- Instant filename search with sub-100ms response for 100k+ files
- Fuzzy matching algorithm for typo-tolerant queries
- Advanced filtering by extension, date, size, and directory
- Global keyboard shortcut access (Ctrl+Shift+F)

### Content Indexing
- Full-text search across document contents (SQLite FTS5)
- Automatic detection and indexing of scanned PDFs
- OCR text extraction from images (JPEG, PNG, TIFF)
- Configurable by directory and file type
- Multi-language support (French and English priority)

### Semantic Search
- Natural language query understanding
- Vector-based similarity search using compact embeddings (LEANN)
- Conversational "Assist Me" mode with source attribution
- 97% smaller index size compared to traditional vector databases

### Email Search
- Outlook PST/MAPI integration
- Thunderbird MBOX parsing
- IMAP and Exchange server support
- Attachment indexing and search

### Real-time Updates
- File system monitoring via watchdog
- Automatic index updates on file creation, modification, and deletion
- Intelligent handling of file moves and renames
- Scheduled indexing with configurable intervals

---

## Technology Stack

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Language** | Rust | Memory safety, performance, concurrency |
| **UI Framework** | egui | Native, lightweight, GPU-accelerated |
| **Windowing** | winit | Cross-platform window management |
| **Rendering** | wgpu | Hardware-accelerated graphics |
| **Search Engine** | Tantivy | Lucene-like full-text search in Rust |
| **Database** | SQLite with FTS5 | Embedded, ACID-compliant, full-text capable |
| **Embeddings** | all-MiniLM-L6-v2 | Compact (80MB), multilingual, 384 dimensions |
| **Vector Database** | LEANN | Ultra-compact indices (97% size reduction) |
| **OCR** | Tesseract 5 | Industry standard, offline, multi-language |
| **File Monitoring** | notify-rs | Cross-platform filesystem events |
| **Email Parsing** | mailparse, libpff | PST and MBOX format support |

**Binary Size**: ~8MB base + 110MB (OCR + ML models) = 118MB total

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                 UI Layer (egui)                         │
│    Search Interface | Configuration | Assist Me Mode    │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│              Core Application (Rust)                    │
│                                                          │
│  File System Watchdog → Indexer → Content Extractor    │
│  Search Engine: Tantivy + SQLite FTS5 + LEANN          │
│  Email Parser: PST/MBOX/IMAP                            │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│              Storage Layer                              │
│  tantivy_index/ | metadata.db (SQLite) | vectors.leann │
└─────────────────────────────────────────────────────────┘
```

---

## Getting Started

### For Developers

```bash
# Prerequisites
rustc >= 1.70
cargo >= 1.70

# Clone and build
git clone https://github.com/fless-lab/xfinder.git
cd xfinder
cargo build --release

# Run tests
cargo test

# Launch application
cargo run
```

See [QUICKSTART.md](QUICKSTART.md) for detailed setup instructions.

### For End Users (Future)

```bash
# Installation
Download xfinder-setup.msi from releases
Run installer and follow prompts

# First Use
1. Launch xfinder
2. Select directories to monitor
3. Start indexing
4. Search using Ctrl+Shift+F
```

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Search query (100k files) | <100ms | P95 latency |
| Indexing throughput | >1000 files/min | Average on SSD |
| OCR processing (A4 page) | <5s | PaddleOCR/Tesseract standard quality |
| Semantic search | <3s | Including embedding generation |
| Index size overhead | <5% of corpus | Metadata + vectors |
| Memory footprint (idle) | <100MB | Application only |
| Cold start time | <500ms | To main window display |

---

## Design Decisions

### Language Priority
Multi-language support with French and English as primary targets. OCR and semantic search models selected for optimal French performance.

### Vector Database
LEANN selected for 97% index size reduction compared to FAISS. Proof-of-concept validation scheduled for Week 13-14.

### Email Parsing Strategy
- Primary: Windows MAPI API (requires Outlook installation)
- Fallback: libpff library for direct PST parsing
- Thunderbird: mailparse crate for MBOX files

### Network Drives
UNC path monitoring (`\\Server\Share`) supported via same watchdog mechanism as local drives.

### GPU Acceleration
Optional CUDA support for embedding generation provides 10x speed improvement at cost of 500MB additional dependencies. Disabled by default.

---

## Contributing

Project currently in active development. Contributions welcome after Phase 1 MVP completion.

## License

To be determined (likely GPL-3.0 or Apache-2.0)

## Project Status

**Current Phase**: Phase 1 - Core Search Implementation (Week 1)
**Last Updated**: 2025-11-12
**Version**: 0.1.0-alpha

---

Built with Rust for performance, security, and reliability.
