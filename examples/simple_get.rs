//! Simple GET request example

use hyprcurl::{get, Request};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simple GET Request Examples ===\n");

    // Method 1: Using convenience function
    println!("1. Using convenience function:");
    let response = get("https://httpbin.org/get")?;
    println!("Response length: {} bytes\n", response.len());

    // Method 2: Using Request builder
    println!("2. Using Request builder:");
    let response = Request::get("https://httpbin.org/get").send()?;

    let body = String::from_utf8_lossy(&response);
    println!("Response:\n{}\n", body);

    Ok(())
}
