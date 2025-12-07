//! # curl-cffi-rs
//!
//! A high-performance Rust implementation of curl_cffi with browser fingerprinting support.
//!
//! This crate provides a safe, ergonomic wrapper around libcurl with:
//! - Zero-cost abstractions
//! - Type-safe API
//! - Browser impersonation support
//! - Python bindings (via PyO3)
//!
//! ## Example
//!
//! ```no_run
//! use hyprcurl::{Request, Browser};
//!
//! // Simple GET
//! let response = Request::get("https://httpbin.org/get").send().unwrap();
//!
//! // With browser impersonation
//! let response = Request::get("https://tls.browserleaks.com/json")
//!     .impersonate(Browser::ChromeLatest)
//!     .send()
//!     .unwrap();
//!
//! // With proxy
//! let response = Request::get("https://httpbin.org/get")
//!     .impersonate(Browser::ChromeLatest)
//!     .proxies("http://localhost:3128")
//!     .send()
//!     .unwrap();
//! ```

pub mod curl;
pub mod error;
pub mod types;

#[cfg(feature = "python")]
pub mod python;

// Re-exports
pub use curl::Curl;
pub use error::{CurlError, Result};
pub use types::{Browser, CurlInfo, CurlOpt, HttpVersion};

// Version info
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Request builder for ergonomic API (Rust equivalent of Python curl_cffi)
///
/// # Examples
/// ```no_run
/// use hyprcurl::{Request, Browser};
///
/// // Simple GET
/// let response = Request::get("https://httpbin.org/get").send().unwrap();
///
/// // GET with browser impersonation
/// let response = Request::get("https://tls.browserleaks.com/json")
///     .impersonate(Browser::ChromeLatest)
///     .send()
///     .unwrap();
///
/// // GET with impersonate and proxy
/// let response = Request::get("https://httpbin.org/get")
///     .impersonate(Browser::Chrome { version: 110 })
///     .proxies("http://localhost:3128")
///     .send()
///     .unwrap();
///
/// // POST request
/// let response = Request::post("https://httpbin.org/post", r#"{"key": "value"}"#)
///     .impersonate(Browser::ChromeLatest)
///     .proxies("socks5://localhost:1080")
///     .send()
///     .unwrap();
/// ```
pub struct Request {
    url: String,
    data: Option<String>,
    impersonate: Option<Browser>,
    proxies: Option<String>,
}

impl Request {
    /// Create a GET request
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            data: None,
            impersonate: None,
            proxies: None,
        }
    }

    /// Create a POST request
    pub fn post(url: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            data: Some(data.into()),
            impersonate: None,
            proxies: None,
        }
    }

    /// Set browser impersonation (equivalent to Python's `impersonate="chrome"`)
    pub fn impersonate(mut self, browser: Browser) -> Self {
        self.impersonate = Some(browser);
        self
    }

    /// Set proxy (equivalent to Python's `proxies={"https": "..."}`)
    pub fn proxies(mut self, proxy: impl Into<String>) -> Self {
        self.proxies = Some(proxy.into());
        self
    }

    /// Execute the request and return response body
    pub fn send(self) -> Result<Vec<u8>> {
        let mut curl = Curl::new()?;

        if let Some(browser) = self.impersonate {
            curl.set_browser_impersonation(browser)?;
        }

        if let Some(proxy) = &self.proxies {
            curl.set_proxy(proxy)?;
        }

        curl.set_url(&self.url)?;

        if let Some(data) = &self.data {
            curl.set_post_data(data)?;
        }

        let mut buffer = Vec::new();
        curl.perform(&mut buffer)?;
        Ok(buffer)
    }
}

/// Quick GET request (convenience function for simple cases)
///
/// For advanced usage with impersonate/proxies, use: `Request::get(url).impersonate(...).send()`
///
/// # Examples
/// ```no_run
/// use hyprcurl::get;
///
/// let response = get("https://httpbin.org/get").unwrap();
/// println!("{}", String::from_utf8_lossy(&response));
/// ```
pub fn get(url: &str) -> Result<Vec<u8>> {
    Request::get(url).send()
}

/// Quick POST request (convenience function for simple cases)
///
/// For advanced usage with impersonate/proxies, use: `Request::post(url, data).impersonate(...).send()`
///
/// # Examples
/// ```no_run
/// use hyprcurl::post;
///
/// let data = r#"{"key": "value"}"#;
/// let response = post("https://httpbin.org/post", data).unwrap();
/// ```
pub fn post(url: &str, data: &str) -> Result<Vec<u8>> {
    Request::post(url, data).send()
}

// Initialize curl globally once
use std::sync::Once;
static INIT: Once = Once::new();

/// Initialize libcurl globally (called automatically)
pub(crate) fn init_curl() {
    INIT.call_once(|| unsafe {
        curl_sys::curl_global_init(curl_sys::CURL_GLOBAL_DEFAULT);
    });
}

/// Get libcurl version information
pub fn version() -> String {
    init_curl();
    unsafe {
        let version_ptr = curl_sys::curl_version();
        std::ffi::CStr::from_ptr(version_ptr)
            .to_string_lossy()
            .into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let ver = version();
        assert!(ver.contains("curl") || ver.contains("libcurl"));
    }
}
