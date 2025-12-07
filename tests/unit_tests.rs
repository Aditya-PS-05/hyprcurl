//! Unit tests for curl-cffi-rs

use hyprcurl::{Browser, Curl, CurlOpt};

#[test]
fn test_curl_initialization() {
    let curl = Curl::new();
    assert!(curl.is_ok(), "Curl initialization should succeed");
}

#[test]
fn test_version() {
    let version = hyprcurl::version();
    assert!(
        version.contains("curl") || version.contains("libcurl"),
        "Version string should contain 'curl' or 'libcurl'"
    );
}

#[test]
fn test_set_url() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    let result = curl.set_url("https://httpbin.org/get");
    assert!(result.is_ok(), "Setting URL should succeed");
}

#[test]
fn test_invalid_url() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    // URL with null byte should fail
    let result = curl.set_url("https://example.com\0/path");
    assert!(result.is_err(), "Invalid URL should return error");
}

#[test]
fn test_add_header() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    let result = curl.add_header("User-Agent: TestBot/1.0");
    assert!(result.is_ok(), "Adding header should succeed");

    let result = curl.add_header("Accept: application/json");
    assert!(result.is_ok(), "Adding multiple headers should succeed");
}

#[test]
fn test_set_timeout() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    let result = curl.setopt_long(CurlOpt::Timeout, 30);
    assert!(result.is_ok(), "Setting timeout should succeed");
}

#[test]
fn test_set_proxy() {
    let mut curl = Curl::new().expect("Failed to initialize curl");

    let result = curl.set_proxy("http://localhost:3128");
    assert!(result.is_ok(), "Setting HTTP proxy should succeed");

    let mut curl2 = Curl::new().expect("Failed to initialize curl");
    let result = curl2.set_proxy("socks5://localhost:1080");
    assert!(result.is_ok(), "Setting SOCKS5 proxy should succeed");
}

#[test]
fn test_set_proxy_with_auth() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    let result = curl.set_proxy_with_auth("http://localhost:3128", "user", "pass");
    assert!(result.is_ok(), "Setting proxy with auth should succeed");
}

#[test]
fn test_browser_impersonation() {
    let mut curl = Curl::new().expect("Failed to initialize curl");

    let result = curl.set_browser_impersonation(Browser::ChromeLatest);
    assert!(
        result.is_ok(),
        "Setting Chrome Latest impersonation should succeed"
    );

    let mut curl2 = Curl::new().expect("Failed to initialize curl");
    let result = curl2.set_browser_impersonation(Browser::Chrome { version: 110 });
    assert!(
        result.is_ok(),
        "Setting Chrome 110 impersonation should succeed"
    );

    let mut curl3 = Curl::new().expect("Failed to initialize curl");
    let result = curl3.set_browser_impersonation(Browser::FirefoxLatest);
    assert!(
        result.is_ok(),
        "Setting Firefox Latest impersonation should succeed"
    );

    let mut curl4 = Curl::new().expect("Failed to initialize curl");
    let result = curl4.set_browser_impersonation(Browser::SafariLatest);
    assert!(
        result.is_ok(),
        "Setting Safari Latest impersonation should succeed"
    );
}

#[test]
fn test_browser_user_agents() {
    let chrome = Browser::ChromeLatest.user_agent();
    assert!(chrome.contains("Chrome"), "Chrome UA should contain 'Chrome'");

    let firefox = Browser::FirefoxLatest.user_agent();
    assert!(
        firefox.contains("Firefox"),
        "Firefox UA should contain 'Firefox'"
    );

    let safari = Browser::SafariLatest.user_agent();
    assert!(safari.contains("Safari"), "Safari UA should contain 'Safari'");

    let edge = Browser::EdgeLatest.user_agent();
    assert!(edge.contains("Edg"), "Edge UA should contain 'Edg'");
}

#[test]
fn test_browser_tls_ciphers() {
    let chrome_ciphers = Browser::ChromeLatest.tls_ciphers();
    assert!(
        chrome_ciphers.contains("TLS_AES_128_GCM_SHA256"),
        "Chrome should have TLS_AES_128_GCM_SHA256"
    );

    let firefox_ciphers = Browser::FirefoxLatest.tls_ciphers();
    assert!(
        firefox_ciphers.contains("TLS_CHACHA20_POLY1305_SHA256"),
        "Firefox should have TLS_CHACHA20_POLY1305_SHA256"
    );
}

#[test]
fn test_browser_tls_curves() {
    let chrome_curves = Browser::ChromeLatest.tls_curves();
    assert!(
        chrome_curves.contains("X25519"),
        "Chrome should support X25519 curve"
    );
    assert!(
        chrome_curves.contains("X25519Kyber768Draft00"),
        "Chrome should support Kyber"
    );

    let firefox_curves = Browser::FirefoxLatest.tls_curves();
    assert!(
        firefox_curves.contains("X25519"),
        "Firefox should support X25519 curve"
    );
    assert!(
        !firefox_curves.contains("Kyber"),
        "Firefox should not have Kyber in curves"
    );
}

#[test]
fn test_post_data() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    let result = curl.set_post_data(r#"{"key": "value"}"#);
    assert!(result.is_ok(), "Setting POST data should succeed");
}

#[test]
fn test_reset() {
    let mut curl = Curl::new().expect("Failed to initialize curl");
    curl.set_url("https://httpbin.org/get")
        .expect("Failed to set URL");
    curl.add_header("X-Test: value").expect("Failed to add header");

    // Reset should clear everything
    curl.reset();

    // After reset, we should be able to set new URL
    let result = curl.set_url("https://httpbin.org/post");
    assert!(result.is_ok(), "Should be able to set URL after reset");
}
