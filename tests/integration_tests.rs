//! Integration tests for hyprcurl
//!
//! These tests make actual HTTP requests to httpbin.org

use hyprcurl::{get, post, Browser, Curl, Request};

#[test]
fn test_simple_get_request() {
    let response = get("https://httpbin.org/get").expect("GET request should succeed");

    assert!(!response.is_empty(), "Response should not be empty");

    let body = String::from_utf8_lossy(&response);
    assert!(body.contains("httpbin"), "Response should contain 'httpbin'");
}

#[test]
fn test_get_with_request_builder() {
    let response = Request::get("https://httpbin.org/get")
        .send()
        .expect("GET request should succeed");

    assert!(!response.is_empty(), "Response should not be empty");
}

#[test]
fn test_simple_post_request() {
    let data = r#"{"test": "value"}"#;
    let response = post("https://httpbin.org/post", data).expect("POST request should succeed");

    assert!(!response.is_empty(), "Response should not be empty");

    let body = String::from_utf8_lossy(&response);
    assert!(body.contains("test"), "Response should contain posted data");
}

#[test]
fn test_post_with_request_builder() {
    let data = r#"{"message": "hello"}"#;
    let response = Request::post("https://httpbin.org/post", data)
        .send()
        .expect("POST request should succeed");

    let body = String::from_utf8_lossy(&response);
    assert!(
        body.contains("hello"),
        "Response should contain posted message"
    );
}

#[test]
fn test_get_with_browser_impersonation() {
    let response = Request::get("https://httpbin.org/user-agent")
        .impersonate(Browser::ChromeLatest)
        .send()
        .expect("Request with browser impersonation should succeed");

    let body = String::from_utf8_lossy(&response);
    assert!(
        body.contains("Chrome"),
        "Response should show Chrome user agent"
    );
}

#[test]
fn test_get_with_firefox_impersonation() {
    let response = Request::get("https://httpbin.org/user-agent")
        .impersonate(Browser::FirefoxLatest)
        .send()
        .expect("Request with Firefox impersonation should succeed");

    let body = String::from_utf8_lossy(&response);
    assert!(
        body.contains("Firefox"),
        "Response should show Firefox user agent"
    );
}

#[test]
fn test_get_with_safari_impersonation() {
    let response = Request::get("https://httpbin.org/user-agent")
        .impersonate(Browser::SafariLatest)
        .send()
        .expect("Request with Safari impersonation should succeed");

    let body = String::from_utf8_lossy(&response);
    assert!(
        body.contains("Safari"),
        "Response should show Safari user agent"
    );
}

#[test]
fn test_custom_headers() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    curl.set_url("https://httpbin.org/headers")
        .expect("Failed to set URL");
    curl.add_header("X-Custom-Header: test-value")
        .expect("Failed to add header");

    let mut buffer = Vec::new();
    curl.perform(&mut buffer)
        .expect("Request should succeed");

    let body = String::from_utf8_lossy(&buffer);
    assert!(
        body.contains("X-Custom-Header"),
        "Response should show custom header"
    );
    assert!(
        body.contains("test-value"),
        "Response should show custom header value"
    );
}

#[test]
fn test_response_code() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    curl.set_url("https://httpbin.org/status/200")
        .expect("Failed to set URL");

    let mut buffer = Vec::new();
    curl.perform(&mut buffer)
        .expect("Request should succeed");

    let status = curl
        .response_code()
        .expect("Should be able to get response code");
    assert_eq!(status, 200, "Status code should be 200");
}

#[test]
fn test_404_response() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    curl.set_url("https://httpbin.org/status/404")
        .expect("Failed to set URL");

    let mut buffer = Vec::new();
    curl.perform(&mut buffer)
        .expect("Request should succeed");

    let status = curl
        .response_code()
        .expect("Should be able to get response code");
    assert_eq!(status, 404, "Status code should be 404");
}

#[test]
fn test_redirect_following() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    curl.set_url("https://httpbin.org/redirect/3")
        .expect("Failed to set URL");

    let mut buffer = Vec::new();
    curl.perform(&mut buffer)
        .expect("Request should succeed");

    let effective_url = curl
        .effective_url()
        .expect("Should be able to get effective URL");
    assert!(
        effective_url.contains("/get"),
        "Should have followed redirects to /get"
    );
}

#[test]
fn test_response_timing() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    curl.set_url("https://httpbin.org/delay/1")
        .expect("Failed to set URL");

    let mut buffer = Vec::new();
    curl.perform(&mut buffer)
        .expect("Request should succeed");

    let total_time = curl
        .total_time()
        .expect("Should be able to get total time");
    assert!(
        total_time >= 1.0,
        "Total time should be at least 1 second for delay/1"
    );
}

#[test]
#[ignore] // Ignore by default as it requires a proxy
fn test_http_proxy() {
    let response = Request::get("https://httpbin.org/ip")
        .proxies("http://localhost:3128")
        .send();

    // This will fail if no proxy is running
    if let Ok(resp) = response {
        assert!(!resp.is_empty(), "Response should not be empty");
    }
}

#[test]
#[ignore] // Ignore by default as it requires a proxy
fn test_socks_proxy() {
    let response = Request::get("https://httpbin.org/ip")
        .proxies("socks5://localhost:1080")
        .send();

    // This will fail if no proxy is running
    if let Ok(resp) = response {
        assert!(!resp.is_empty(), "Response should not be empty");
    }
}

#[test]
fn test_handle_reuse() {
    // First request
    let response1 = Request::get("https://httpbin.org/get")
        .send()
        .expect("First request should succeed");

    // Second request (separate Request instance)
    let response2 = Request::get("https://httpbin.org/headers")
        .send()
        .expect("Second request should succeed");

    assert!(!response1.is_empty(), "First response should not be empty");
    assert!(!response2.is_empty(), "Second response should not be empty");
}

#[test]
fn test_post_with_browser_and_json() {
    let data = r#"{"name": "curl-cffi-rs", "version": "0.1.0"}"#;

    let response = Request::post("https://httpbin.org/post", data)
        .impersonate(Browser::ChromeLatest)
        .send()
        .expect("POST with browser impersonation should succeed");

    let body = String::from_utf8_lossy(&response);
    assert!(
        body.contains("curl-cffi-rs"),
        "Response should contain posted data"
    );
    assert!(
        body.contains("Chrome"),
        "Response should show Chrome user agent"
    );
}

#[test]
fn test_timeout_works() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    curl.set_url("https://httpbin.org/delay/10")
        .expect("Failed to set URL");

    // Set 1 second timeout for a 10 second delay - should timeout
    curl.setopt_long(hyprcurl::CurlOpt::Timeout, 1)
        .expect("Failed to set timeout");

    let mut buffer = Vec::new();
    let result = curl.perform(&mut buffer);

    // Request should fail with timeout
    assert!(
        result.is_err(),
        "Request should fail due to timeout"
    );
}
