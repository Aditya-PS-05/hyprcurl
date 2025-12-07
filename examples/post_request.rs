//! POST request example

use hyprcurl::{post, Browser, Request};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== POST Request Examples ===\n");

    // Method 1: Simple POST using convenience function
    println!("1. Simple POST with convenience function:");
    let data = r#"{"name": "curl-cffi-rs", "type": "library"}"#;
    let response = post("https://httpbin.org/post", data)?;

    println!("Response length: {} bytes\n", response.len());

    // Method 2: POST with Request builder
    println!("2. POST with Request builder:");
    let data = r#"{"message": "Hello from Rust!"}"#;
    let response = Request::post("https://httpbin.org/post", data).send()?;

    let body = String::from_utf8_lossy(&response);
    println!("Response (first 500 chars):\n{}\n", body.chars().take(500).collect::<String>());

    // Method 3: POST with browser impersonation
    println!("3. POST with browser impersonation:");
    let data = r#"{"browser": "chrome", "version": "latest"}"#;
    let response = Request::post("https://httpbin.org/post", data)
        .impersonate(Browser::ChromeLatest)
        .send()?;

    println!("Response length: {} bytes\n", response.len());

    // Method 4: POST with impersonation and proxy (will fail without proxy)
    println!("4. POST with impersonation + proxy:");
    let data = r#"{"test": "proxy"}"#;
    match Request::post("https://httpbin.org/post", data)
        .impersonate(Browser::ChromeLatest)
        .proxies("http://localhost:3128")
        .send()
    {
        Ok(response) => {
            println!("Response length: {} bytes\n", response.len());
        }
        Err(e) => {
            println!("Failed (expected without proxy): {}\n", e);
        }
    }

    Ok(())
}
