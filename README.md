# Rust Web Scraper

A high-performance asynchronous web scraper built with Rust. This tool extracts article data from HTML pages and stores it in a structured SQLite database. It features a modular architecture and handles complex data types like author lists using JSON serialization.

## Features

* Asynchronous Processing: Built on the Tokio runtime for efficient I/O operations.
* Type-Safe Extraction: Uses the scraper crate with CSS selectors for robust data parsing.
* SQLx Integration: Compile-time verified SQL queries for SQLite.
* Flexible Storage: Handles multiple authors per article by storing vector data as JSON strings in the database.
* Modular Design: Clean separation between database management, HTML parsing, and the main execution flow.

## Tech Stack

* Language: Rust
* Runtime: Tokio
* HTTP Client: Reqwest
* HTML Parser: Scraper
* Database Driver: SQLx (with SQLite)
* Serialization: Serde

## Installation

1. Ensure you have the Rust toolchain installed (cargo, rustc).
2. Clone this repository.
3. Install the required system dependencies (if on Linux):
   ```bash
   sudo apt install libsqlite3-dev
