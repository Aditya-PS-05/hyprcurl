//! Comprehensive example showing all features

use hyprcurl::{Browser, Curl, Request};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Comprehensive curl-cffi-rs Examples ===\n");

    // 1. Simple requests
    println!("1. SIMPLE REQUESTS");
    println!("-----------------");
    simple_requests()?;
    println!();

    // 2. Browser impersonation
    println!("2. BROWSER IMPERSONATION");
    println!("------------------------");
    browser_impersonation()?;
    println!();

    // 3. Low-level API
    println!("3. LOW-LEVEL API");
    println!("----------------");
    low_level_api()?;
    println!();

    // 4. Response inspection
    println!("4. RESPONSE INSPECTION");
    println!("----------------------");
    response_inspection()?;
    println!();

    Ok(())
}

fn simple_requests() -> Result<(), Box<dyn std::error::Error>> {
    // Simple GET
    let response = Request::get("https://httpbin.org/get").send()?;
    println!("GET response: {} bytes", response.len());

    // Simple POST
    let response = Request::post("https://httpbin.org/post", r#"{"key": "value"}"#).send()?;
    println!("POST response: {} bytes", response.len());

    Ok(())
}

fn browser_impersonation() -> Result<(), Box<dyn std::error::Error>> {
    // Chrome (latest)
    let response = Request::get("https://httpbin.org/headers")
        .impersonate(Browser::ChromeLatest)
        .send()?;
    println!("Chrome impersonation: {} bytes", response.len());

    // Specific Chrome version
    let response = Request::get("https://httpbin.org/headers")
        .impersonate(Browser::Chrome { version: 110 })
        .send()?;
    println!("Chrome 110: {} bytes", response.len());

    // Firefox
    let response = Request::get("https://httpbin.org/headers")
        .impersonate(Browser::FirefoxLatest)
        .send()?;
    println!("Firefox: {} bytes", response.len());

    // Safari
    let response = Request::get("https://httpbin.org/headers")
        .impersonate(Browser::SafariLatest)
        .send()?;
    println!("Safari: {} bytes", response.len());

    Ok(())
}

fn low_level_api() -> Result<(), Box<dyn std::error::Error>> {
    // Using Curl directly for advanced control
    let mut curl = Curl::new()?;

    // Set URL
    curl.set_url("https://httpbin.org/get")?;

    // Add custom headers
    curl.add_header("X-Custom-Header: test")?;
    curl.add_header("Accept: application/json")?;

    // Set browser impersonation
    curl.set_browser_impersonation(Browser::ChromeLatest)?;

    // Set timeout
    curl.setopt_long(hyprcurl::CurlOpt::Timeout, 30)?;

    // Perform request
    let mut buffer = Vec::new();
    curl.perform(&mut buffer)?;

    println!("Low-level API response: {} bytes", buffer.len());

    // Get response info
    let status = curl.response_code()?;
    let time = curl.total_time()?;

    println!("Status: {}, Time: {:.2}s", status, time);

    Ok(())
}

fn response_inspection() -> Result<(), Box<dyn std::error::Error>> {
    let mut curl = Curl::new()?;
    curl.set_url("https://httpbin.org/redirect/3")?; // Will redirect 3 times

    let mut buffer = Vec::new();
    curl.perform(&mut buffer)?;

    // Get response metadata
    let status_code = curl.response_code()?;
    let total_time = curl.total_time()?;
    let effective_url = curl.effective_url()?;

    println!("Status Code: {}", status_code);
    println!("Total Time: {:.3}s", total_time);
    println!("Effective URL: {}", effective_url);
    println!("Body Length: {} bytes", buffer.len());

    Ok(())
}
