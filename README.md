# gitbook2text-rs

A fast Rust tool that reads GitBook links from a text file, appends `.md` to each URL, downloads all pages concurrently, and converts the content to clean plain text ready for Retrieval-Augmented Generation (RAG).

## Why

RAG works best with normalized, text-only corpora. GitBook sites are great sources, but the raw pages often need consistent fetching and cleanup. This tool automates that workflow end-to-end.

## Features

- Reads URLs from a plain `.txt` file (one per line)
- Automatically appends `.md` to each GitBook link
- Concurrent, rate-limited downloads with retries
- Saves original Markdown and normalized plain-text
- Deterministic file naming and idempotent runs
- Optional URL deduplication and domain allowlist

## How it works

1. Load URLs from `input/links.txt`.
2. Normalize each URL and append `.md`.
3. Fetch content with backoff + retry.
4. Write raw Markdown to `data/md/`.
5. Convert Markdown → plain text and write to `data/text/`.
6. Emit a manifest (`manifest.jsonl`) with URL, status, paths, and hash.

## Installation

```bash
# Rust 1.75+ recommended
rustup update
git clone https://github.com/<you>/<repo-name>.git
cd <repo-name>
cargo build --release
