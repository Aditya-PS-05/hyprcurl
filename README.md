# hyprcurl

A high-performance Rust implementation of curl_cffi with browser fingerprinting support.

## Overview

Rust rewrite of [curl_cffi](https://github.com/yifeikong/curl_cffi), demonstrating:

## Features to be implemented

- Type-safe curl wrapper
- Synchronous API
- Asynchronous API (tokio)
- WebSocket support (placeholder)
- Python bindings via PyO3
- Browser fingerprinting support (via libcurl-impersonate)
- Zero-copy I/O where possible
- Comprehensive benchmarks

## Quick Start

### Rust Library

```rust
use hyprcurl::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    let mut curl = Curl::new()?;
    curl.set_url("https://httpbin.org/get")?;

    let mut response = Vec::new();
    curl.perform(&mut response)?;

    println!("Status: {}", curl.response_code()?);
    println!("Response: {}", String::from_utf8_lossy(&response));
    Ok(())
}
```

### Async Rust

```rust
use hyprcurl::{AsyncCurl, Curl};

#[tokio::main]
async fn main() {
    let async_curl = AsyncCurl::new().unwrap();

    let mut curl = Curl::new().unwrap();
    curl.set_url("https://httpbin.org/get").unwrap();

    let response = async_curl.perform(curl).await.unwrap();
    println!("Got {} bytes", response.len());
}
```

### Python Bindings

```python
import hyprcurl

# Simple request
response = hyprcurl.get("https://httpbin.org/get")
print(response)

# Advanced usage
curl = hyprcurl.Curl()
curl.set_url("https://httpbin.org/get")
curl.add_header("User-Agent: MyApp/1.0")
curl.impersonate("chrome110", True)

response = curl.perform()
print(f"Status: {curl.response_code()}")
print(f"Response: {response}")
```

## Installation

### Build from source

```bash
# Rust library
cargo build --release

# With async support
cargo build --release --features async

# With Python bindings
cargo build --release --features python
maturin develop --release
```

### Dependencies

- libcurl (or libcurl-impersonate for browser fingerprinting)
- Rust 1.70+
- Python 3.8+ (for Python bindings)

## Benchmarks

### Setup

1. Start the test server:
```bash
cd benchmarks
python server.py
```

2. Run Rust benchmarks:
```bash
cargo bench
```

3. Run Python vs Rust comparison:
```bash
python benchmarks/python_vs_rust_bench.py
```

## Documentation

This project includes comprehensive documentation in the form of a book (similar to the Rust Book).

### Building the Documentation

To build and view the documentation:

```bash
# Build the book
mdbook build book

# Serve the book locally and open in browser
mdbook serve book --open

# The book will be available at http://localhost:3000
```

### Installing mdBook

If you don't have mdBook installed:

```bash
cargo install mdbook
```

### API Documentation

To generate and view the API documentation:

```bash
# Generate API docs for the library
cargo doc --no-deps --open

# Generate docs with all features enabled
cargo doc --all-features --open
```

The documentation covers:
- Getting started guides
- Browser impersonation
- SSL/TLS configuration
- Async/await usage
- WebSocket support
- Python bindings
- Complete API reference
