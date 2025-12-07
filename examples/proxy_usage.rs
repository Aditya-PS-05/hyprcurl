//! Proxy usage example
//!
//! This example shows how to use HTTP and SOCKS proxies

use hyprcurl::{Browser, Request};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Proxy Usage Examples ===\n");

    // Example 1: HTTP proxy
    println!("1. Using HTTP proxy:");
    println!("Note: This will fail if proxy is not running");
    match Request::get("https://httpbin.org/ip")
        .proxies("http://localhost:3128")
        .send()
    {
        Ok(response) => {
            println!("Response: {}\n", String::from_utf8_lossy(&response));
        }
        Err(e) => {
            println!("Failed (expected if no proxy): {}\n", e);
        }
    }

    // Example 2: SOCKS5 proxy
    println!("2. Using SOCKS5 proxy:");
    println!("Note: This will fail if proxy is not running");
    match Request::get("https://httpbin.org/ip")
        .proxies("socks5://localhost:1080")
        .send()
    {
        Ok(response) => {
            println!("Response: {}\n", String::from_utf8_lossy(&response));
        }
        Err(e) => {
            println!("Failed (expected if no proxy): {}\n", e);
        }
    }

    // Example 3: Proxy with browser impersonation
    println!("3. Proxy + Browser impersonation:");
    match Request::get("https://httpbin.org/headers")
        .impersonate(Browser::ChromeLatest)
        .proxies("http://localhost:3128")
        .send()
    {
        Ok(response) => {
            println!("Response: {}\n", String::from_utf8_lossy(&response));
        }
        Err(e) => {
            println!("Failed (expected if no proxy): {}\n", e);
        }
    }

    println!("Note: To test proxies, run:");
    println!("  HTTP proxy:  docker run -p 3128:3128 -d datadog/squid");
    println!("  SOCKS proxy: ssh -D 1080 -N user@server");

    Ok(())
}
