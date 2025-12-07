//! Browser impersonation example
//!
//! This example demonstrates how to impersonate different browsers
//! to avoid bot detection.

use hyprcurl::{Browser, Request};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Browser Impersonation Examples ===\n");

    // Test with Chrome (latest)
    println!("1. Impersonating Chrome (latest):");
    let response = Request::get("https://httpbin.org/user-agent")
        .impersonate(Browser::ChromeLatest)
        .send()?;

    println!("Response: {}\n", String::from_utf8_lossy(&response));

    // Test with specific Chrome version
    println!("2. Impersonating Chrome 110:");
    let response = Request::get("https://httpbin.org/user-agent")
        .impersonate(Browser::Chrome { version: 110 })
        .send()?;

    println!("Response: {}\n", String::from_utf8_lossy(&response));

    // Test with Firefox
    println!("3. Impersonating Firefox (latest):");
    let response = Request::get("https://httpbin.org/user-agent")
        .impersonate(Browser::FirefoxLatest)
        .send()?;

    println!("Response: {}\n", String::from_utf8_lossy(&response));

    // Test with Safari
    println!("4. Impersonating Safari (latest):");
    let response = Request::get("https://httpbin.org/user-agent")
        .impersonate(Browser::SafariLatest)
        .send()?;

    println!("Response: {}\n", String::from_utf8_lossy(&response));

    // Real-world example: TLS fingerprint test
    println!("5. Testing TLS fingerprint with Chrome:");
    let response = Request::get("https://tls.browserleaks.com/json")
        .impersonate(Browser::ChromeLatest)
        .send()?;

    println!("TLS Fingerprint Response (first 500 chars):");
    let body = String::from_utf8_lossy(&response);
    println!("{}\n", body.chars().take(500).collect::<String>());

    Ok(())
}
