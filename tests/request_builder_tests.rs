//! Tests for the Request builder API

use hyprcurl::{Browser, Request};

#[test]
fn test_request_builder_simple_get() {
    let response = Request::get("https://httpbin.org/get")
        .send()
        .expect("Simple GET should succeed");

    assert!(!response.is_empty());
}

#[test]
fn test_request_builder_get_with_impersonate() {
    let response = Request::get("https://httpbin.org/user-agent")
        .impersonate(Browser::ChromeLatest)
        .send()
        .expect("GET with impersonate should succeed");

    let body = String::from_utf8_lossy(&response);
    assert!(body.contains("Chrome"));
}

#[test]
#[ignore] // Requires proxy
fn test_request_builder_get_with_proxy() {
    let response = Request::get("https://httpbin.org/ip")
        .proxies("http://localhost:3128")
        .send();

    if let Ok(resp) = response {
        assert!(!resp.is_empty());
    }
}

#[test]
#[ignore] // Requires proxy
fn test_request_builder_full_chain() {
    let response = Request::get("https://httpbin.org/headers")
        .impersonate(Browser::ChromeLatest)
        .proxies("http://localhost:3128")
        .send();

    if let Ok(resp) = response {
        let body = String::from_utf8_lossy(&resp);
        assert!(body.contains("Chrome"));
    }
}

#[test]
fn test_request_builder_post_simple() {
    let data = r#"{"test": "data"}"#;
    let response = Request::post("https://httpbin.org/post", data)
        .send()
        .expect("POST should succeed");

    let body = String::from_utf8_lossy(&response);
    assert!(body.contains("test"));
    assert!(body.contains("data"));
}

#[test]
fn test_request_builder_post_with_impersonate() {
    let data = r#"{"message": "hello"}"#;
    let response = Request::post("https://httpbin.org/post", data)
        .impersonate(Browser::FirefoxLatest)
        .send()
        .expect("POST with impersonate should succeed");

    let body = String::from_utf8_lossy(&response);
    assert!(body.contains("hello"));
    assert!(body.contains("Firefox"));
}

#[test]
#[ignore] // Requires proxy
fn test_request_builder_post_with_proxy() {
    let data = r#"{"key": "value"}"#;
    let response = Request::post("https://httpbin.org/post", data)
        .proxies("http://localhost:3128")
        .send();

    if let Ok(resp) = response {
        let body = String::from_utf8_lossy(&resp);
        assert!(body.contains("key"));
    }
}

#[test]
#[ignore] // Requires proxy
fn test_request_builder_post_full_chain() {
    let data = r#"{"name": "curl-cffi-rs"}"#;
    let response = Request::post("https://httpbin.org/post", data)
        .impersonate(Browser::ChromeLatest)
        .proxies("http://localhost:3128")
        .send();

    if let Ok(resp) = response {
        let body = String::from_utf8_lossy(&resp);
        assert!(body.contains("curl-cffi-rs"));
        assert!(body.contains("Chrome"));
    }
}

#[test]
fn test_builder_with_different_browsers() {
    let browsers = vec![
        Browser::ChromeLatest,
        Browser::Chrome { version: 110 },
        Browser::FirefoxLatest,
        Browser::Firefox { version: 121 },
        Browser::SafariLatest,
        Browser::EdgeLatest,
    ];

    for browser in browsers {
        let response = Request::get("https://httpbin.org/user-agent")
            .impersonate(browser)
            .send()
            .expect("Request should succeed for all browsers");

        assert!(!response.is_empty(), "Response should not be empty");
    }
}

#[test]
fn test_builder_url_types() {
    // Test that Into<String> works for both &str and String
    let url_str = "https://httpbin.org/get";
    let url_string = String::from("https://httpbin.org/get");

    let response1 = Request::get(url_str).send().expect("&str URL should work");
    let response2 = Request::get(url_string)
        .send()
        .expect("String URL should work");

    assert!(!response1.is_empty());
    assert!(!response2.is_empty());
}

#[test]
fn test_builder_data_types() {
    let data_str = r#"{"test": "value"}"#;
    let data_string = String::from(r#"{"test": "value"}"#);

    let response1 = Request::post("https://httpbin.org/post", data_str)
        .send()
        .expect("&str data should work");

    let response2 = Request::post("https://httpbin.org/post", data_string)
        .send()
        .expect("String data should work");

    assert!(!response1.is_empty());
    assert!(!response2.is_empty());
}

#[test]
fn test_builder_proxy_types() {
    // Test that Into<String> works for proxies parameter
    let proxy_str = "http://localhost:3128";
    let proxy_string = String::from("http://localhost:3128");

    let _req1 = Request::get("https://httpbin.org/get").proxies(proxy_str);
    let _req2 = Request::get("https://httpbin.org/get").proxies(proxy_string);

    // Just check that it compiles and builds correctly
    // This test passes if compilation succeeds, proving Into<String> works for both types
}
