# Installation

This chapter will guide you through installing curl-cffi-rs and its dependencies.

## Prerequisites

Before using curl-cffi-rs, you need:

- **Rust** - Install from [rustup.rs](https://rustup.rs/)
- **libcurl-impersonate** - The underlying C library (optional for basic usage)

## Adding to your project

Add curl-cffi-rs to your `Cargo.toml`:

```toml
[dependencies]
curl-cffi-rs = "0.1"
```

### Optional Features

curl-cffi-rs provides several optional features:

```toml
[dependencies]
curl-cffi-rs = { version = "0.1", features = ["async"] }
```

Available features:

- **async** - Enables async/await support with tokio
- **python** - Enables Python bindings via PyO3

### Example with all features

```toml
[dependencies]
curl-cffi-rs = { version = "0.1", features = ["async", "python"] }
```

## Installing libcurl-impersonate

For browser impersonation to work, you need libcurl-impersonate:

### Linux

```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# Clone and build libcurl-impersonate
git clone https://github.com/lwthiker/curl-impersonate
cd curl-impersonate
./configure
make
sudo make install
```

### macOS

```bash
brew install curl-impersonate
```

### Windows

Windows support is experimental. See the [libcurl-impersonate documentation](https://github.com/lwthiker/curl-impersonate) for details.

## Verifying Installation

Create a simple test program to verify everything works:

```rust
use hyprcurl::Curl;

fn main() {
    let curl = Curl::new().expect("Failed to initialize curl");
    println!("curl-cffi-rs is working! Version: {}", hyprcurl::version());
}
```

Run it:

```bash
cargo run
```

If you see the version information, you're ready to go!

## Next Steps

Now that you have curl-cffi-rs installed, continue to the [Quick Start](./quick-start.md) guide to make your first request.
