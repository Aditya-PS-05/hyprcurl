//! Core Curl wrapper implementation

use crate::error::{check_code, CurlError, Result};
use crate::types::{Browser, CurlOpt, HttpVersion};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// Main Curl handle wrapper
pub struct Curl {
    handle: *mut curl_sys::CURL,
    headers: *mut curl_sys::curl_slist,
    error_buffer: [c_char; curl_sys::CURL_ERROR_SIZE],
    // Store strings to keep them alive for libcurl
    stored_strings: Vec<CString>,
    stored_url: Option<CString>,
    stored_postfields: Option<CString>,
    stored_headers: Vec<CString>, // Keep header strings alive
}

impl Curl {
    /// Create a new Curl instance
    pub fn new() -> Result<Self> {
        // Initialize curl globally first
        crate::init_curl();

        let handle = unsafe { curl_sys::curl_easy_init() };
        if handle.is_null() {
            return Err(CurlError::InitError);
        }

        let mut curl = Curl {
            handle,
            headers: ptr::null_mut(),
            error_buffer: [0; curl_sys::CURL_ERROR_SIZE],
            stored_strings: Vec::new(),
            stored_url: None,
            stored_postfields: None,
            stored_headers: Vec::new(),
        };

        // Set error buffer
        unsafe {
            curl_sys::curl_easy_setopt(
                curl.handle,
                curl_sys::CURLOPT_ERRORBUFFER,
                curl.error_buffer.as_ptr(),
            );
        }

        // Disable signals (important for multi-threading)
        curl.setopt_long(CurlOpt::NoSignal, 1)?;

        // Set follow redirects
        curl.setopt_long(CurlOpt::FollowLocation, 1)?;

        // Set user agent to avoid default issues
        curl.setopt_str(CurlOpt::UserAgent, "curl-cffi-rs/0.1.0")?;

        // Set secure SSL defaults (like curl-cffi Python)
        curl.set_ssl_verify(None)?; // Enable SSL verification with default CA

        // Set follow redirects
        curl.setopt_long(CurlOpt::FollowLocation, 1)?;

        // Set user agent to avoid default issues
        curl.setopt_str(CurlOpt::UserAgent, "curl-cffi-rs/0.1.0")?;

        Ok(curl)
    }

    /// Set URL to fetch
    pub fn set_url(&mut self, url: &str) -> Result<()> {
        let c_url = CString::new(url).map_err(|_| CurlError::InvalidUrl(url.to_string()))?;
        unsafe {
            let code =
                curl_sys::curl_easy_setopt(self.handle, curl_sys::CURLOPT_URL, c_url.as_ptr());
            check_code(code)?;
        }
        // Store the URL to keep it alive
        self.stored_url = Some(c_url);
        Ok(())
    }

    /// Set a long option
    pub fn setopt_long(&mut self, option: CurlOpt, value: i64) -> Result<()> {
        unsafe {
            let code = curl_sys::curl_easy_setopt(self.handle, option.to_raw(), value);
            check_code(code)?;
        }
        Ok(())
    }

    /// Set a string option
    pub fn setopt_str(&mut self, option: CurlOpt, value: &str) -> Result<()> {
        let c_value = CString::new(value)
            .map_err(|_| CurlError::InvalidOption(format!("Invalid string: {}", value)))?;

        unsafe {
            let code = curl_sys::curl_easy_setopt(self.handle, option.to_raw(), c_value.as_ptr());
            check_code(code)?;
        }

        // Store the string to keep it alive (libcurl doesn't copy it)
        // Special handling for POSTFIELDS which needs its own storage
        if option == CurlOpt::PostFields {
            self.stored_postfields = Some(c_value);
        } else {
            self.stored_strings.push(c_value);
        }

        Ok(())
    }

    /// Add a header
    pub fn add_header(&mut self, header: &str) -> Result<()> {
        let c_header = CString::new(header)
            .map_err(|_| CurlError::InvalidOption(format!("Invalid header: {}", header)))?;

        unsafe {
            self.headers = curl_sys::curl_slist_append(self.headers, c_header.as_ptr());
            if self.headers.is_null() {
                return Err(CurlError::Other("Failed to add header".to_string()));
            }
        }

        // Store the header string to keep it alive
        self.stored_headers.push(c_header);
        Ok(())
    }

    /// Set HTTP version
    pub fn set_http_version(&mut self, version: HttpVersion) -> Result<()> {
        self.setopt_long(CurlOpt::HttpVersion, version.to_curl())
    }

    /// Set POST data from a string
    pub fn set_post_data(&mut self, data: &str) -> Result<()> {
        self.setopt_str(CurlOpt::PostFields, data)?;
        // Enable POST
        self.setopt_long(CurlOpt::CustomRequest, 0)?; // Reset method if it was set
        Ok(())
    }

    /// Set POST data from bytes
    pub fn set_post_bytes(&mut self, data: &[u8]) -> Result<()> {
        // Store the data
        let c_data = CString::new(data)
            .map_err(|_| CurlError::InvalidOption("POST data contains null byte".to_string()))?;

        unsafe {
            let code = curl_sys::curl_easy_setopt(
                self.handle,
                curl_sys::CURLOPT_POSTFIELDS,
                c_data.as_ptr(),
            );
            check_code(code)?;

            // Set the size explicitly
            let size_code = curl_sys::curl_easy_setopt(
                self.handle,
                curl_sys::CURLOPT_POSTFIELDSIZE,
                data.len() as i64,
            );
            check_code(size_code)?;
        }

        self.stored_postfields = Some(c_data);
        Ok(())
    }

    /// Set HTTP/SOCKS proxy
    ///
    /// Supports: http://, https://, socks4://, socks4a://, socks5://, socks5h://
    ///
    /// # Examples
    /// ```no_run
    /// # use hyprcurl::Curl;
    /// let mut curl = Curl::new().unwrap();
    /// curl.set_proxy("http://localhost:3128").unwrap();
    /// curl.set_proxy("socks5://localhost:1080").unwrap();
    /// ```
    pub fn set_proxy(&mut self, proxy: &str) -> Result<()> {
        self.setopt_str(CurlOpt::Proxy, proxy)
    }

    /// Set proxy with authentication
    ///
    /// # Examples
    /// ```no_run
    /// # use hyprcurl::Curl;
    /// let mut curl = Curl::new().unwrap();
    /// curl.set_proxy_with_auth("http://localhost:3128", "user", "pass").unwrap();
    /// ```
    pub fn set_proxy_with_auth(
        &mut self,
        proxy: &str,
        username: &str,
        password: &str,
    ) -> Result<()> {
        self.set_proxy(proxy)?;
        let auth = format!("{}:{}", username, password);
        self.setopt_str(CurlOpt::ProxyUserPwd, &auth)
    }

    /// Impersonate a browser
    ///
    /// Note: This requires libcurl-impersonate
    pub fn impersonate(&mut self, target: &str, _default_headers: bool) -> Result<()> {
        let c_target = CString::new(target)
            .map_err(|_| CurlError::InvalidOption(format!("Invalid target: {}", target)))?;

        // This is a custom function from libcurl-impersonate
        // We'll need to bind it separately or use dlsym
        unsafe {
            // For now, just set user agent as a placeholder
            // In real implementation, would call curl_easy_impersonate
            let code = curl_sys::curl_easy_setopt(
                self.handle,
                curl_sys::CURLOPT_USERAGENT,
                c_target.as_ptr(),
            );
            check_code(code)?;
        }
        Ok(())
    }

    /// Perform the request and return response data
    pub fn perform(&mut self, buffer: &mut Vec<u8>) -> Result<()> {
        use std::os::raw::c_void;

        // Clear the buffer first
        buffer.clear();

        // Use a simple approach with a captured buffer
        unsafe {
            // Set write function using a closure that captures the buffer
            extern "C" fn write_func(
                ptr: *mut c_char,
                size: usize,
                nmemb: usize,
                userdata: *mut c_void,
            ) -> usize {
                let total_size = size * nmemb;
                if total_size == 0 || ptr.is_null() {
                    return 0;
                }

                unsafe {
                    let buffer = &mut *(userdata as *mut Vec<u8>);
                    let data = std::slice::from_raw_parts(ptr as *const u8, total_size);
                    buffer.extend_from_slice(data);
                }
                total_size
            }

            // Set write function
            let code = curl_sys::curl_easy_setopt(
                self.handle,
                curl_sys::CURLOPT_WRITEFUNCTION,
                write_func as *const c_void,
            );
            check_code(code)?;

            // Set write data
            let code = curl_sys::curl_easy_setopt(
                self.handle,
                curl_sys::CURLOPT_WRITEDATA,
                buffer as *mut Vec<u8> as *mut c_void,
            );
            check_code(code)?;

            // Apply headers if any
            if !self.headers.is_null() {
                let code = curl_sys::curl_easy_setopt(
                    self.handle,
                    curl_sys::CURLOPT_HTTPHEADER,
                    self.headers,
                );
                check_code(code)?;
            }

            // Perform the request
            let code = curl_sys::curl_easy_perform(self.handle);
            check_code(code)?;
        }

        Ok(())
    }

    pub fn perform_with_headers(
        &mut self,
        body: &mut Vec<u8>,
        headers: &mut Vec<String>,
    ) -> Result<()> {
        use std::os::raw::c_void;

        body.clear();
        headers.clear();

        unsafe {
            extern "C" fn header_func(
                ptr: *mut c_char,
                size: usize,
                nmemb: usize,
                userdata: *mut c_void,
            ) -> usize {
                let total_size = size * nmemb;
                let headers = unsafe { &mut *(userdata as *mut Vec<String>) };

                let header_line =
                    unsafe { std::slice::from_raw_parts(ptr as *const u8, total_size) };

                if let Ok(s) = std::str::from_utf8(header_line) {
                    let trimmed = s.trim();
                    if !trimmed.is_empty() {
                        headers.push(trimmed.to_string());
                    }
                }
                total_size
            }

            curl_sys::curl_easy_setopt(
                self.handle,
                curl_sys::CURLOPT_HEADERFUNCTION,
                header_func as *const c_void,
            );
            curl_sys::curl_easy_setopt(
                self.handle,
                curl_sys::CURLOPT_HEADERDATA,
                headers as *mut Vec<String> as *mut c_void,
            );

            self.perform(body)?;
        }

        Ok(())
    }

    /// Get response code
    pub fn response_code(&self) -> Result<i64> {
        let mut code: i64 = 0;
        unsafe {
            let ret = curl_sys::curl_easy_getinfo(
                self.handle,
                curl_sys::CURLINFO_RESPONSE_CODE,
                &mut code,
            );
            check_code(ret)?;
        }
        Ok(code)
    }

    /// Get total time
    pub fn total_time(&self) -> Result<f64> {
        let mut time: f64 = 0.0;
        unsafe {
            let ret =
                curl_sys::curl_easy_getinfo(self.handle, curl_sys::CURLINFO_TOTAL_TIME, &mut time);
            check_code(ret)?;
        }
        Ok(time)
    }

    /// Get effective URL (after redirects)
    pub fn effective_url(&self) -> Result<String> {
        let mut url_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let ret = curl_sys::curl_easy_getinfo(
                self.handle,
                curl_sys::CURLINFO_EFFECTIVE_URL,
                &mut url_ptr,
            );
            check_code(ret)?;

            if url_ptr.is_null() {
                return Ok(String::new());
            }

            Ok(CStr::from_ptr(url_ptr).to_string_lossy().into_owned())
        }
    }

    /// Reset the handle to default state
    pub fn reset(&mut self) {
        unsafe {
            curl_sys::curl_easy_reset(self.handle);
        }
        self.cleanup_headers();
        // Clear stored strings
        self.stored_strings.clear();
        self.stored_url = None;
        self.stored_postfields = None;
        self.stored_headers.clear();
    }

    /// Cleanup headers
    fn cleanup_headers(&mut self) {
        if !self.headers.is_null() {
            unsafe {
                curl_sys::curl_slist_free_all(self.headers);
                // Reset headers in curl to prevent dangling pointer
                curl_sys::curl_easy_setopt(
                    self.handle,
                    curl_sys::CURLOPT_HTTPHEADER,
                    ptr::null::<curl_sys::curl_slist>(),
                );
            }
            self.headers = ptr::null_mut();
        }
        // Clear stored header strings
        self.stored_headers.clear();
    }

    /// Get raw handle (for advanced use)
    pub fn raw_handle(&self) -> *mut curl_sys::CURL {
        self.handle
    }

    /// Set SSL verification behavior
    ///
    /// # Arguments
    /// * `verify` - SSL verification option:
    ///   - `None` or `Some(true)`: Enable SSL verification (default, secure)
    ///   - `false`: Disable SSL verification (CURRENTLY DISABLED - causes segfault)
    ///   - `Some(path)`: Use custom CA certificate file
    ///
    /// # Note
    /// Disabling SSL verification (`Some(false)`) currently causes segfault due to
    /// libcurl compatibility issues. This is a known limitation.
    /// For testing with invalid certificates, use a custom CA file instead.
    pub fn set_ssl_verify(&mut self, verify: Option<bool>) -> Result<()> {
        match verify {
            None | Some(true) => {
                // Enable SSL verification (secure default)
                unsafe {
                    let code = curl_sys::curl_easy_setopt(
                        self.handle,
                        curl_sys::CURLOPT_SSL_VERIFYPEER,
                        1i64,
                    );
                    check_code(code)?;

                    let host_code = curl_sys::curl_easy_setopt(
                        self.handle,
                        curl_sys::CURLOPT_SSL_VERIFYHOST,
                        2i64, // 2 = strict hostname verification
                    );
                    check_code(host_code)?;
                }

                // Set default CA certificate if available
                if let Some(ca_path) = Self::get_default_ca_bundle() {
                    self.setopt_str(CurlOpt::CaInfo, &ca_path)?;
                }
            }
            Some(false) => {
                // DISABLED: Disabling SSL verification causes segfault
                // Return an error instead of crashing
                return Err(CurlError::Other(
                    "SSL verification cannot be disabled due to libcurl compatibility issues. \
                    Use a custom CA certificate file for testing instead."
                        .to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Set custom CA certificate file
    pub fn set_ca_cert_file(&mut self, ca_path: &str) -> Result<()> {
        self.setopt_str(CurlOpt::CaInfo, ca_path)
    }

    /// Set client certificate for authentication
    pub fn set_client_cert(&mut self, cert_path: &str, key_path: Option<&str>) -> Result<()> {
        self.setopt_str(CurlOpt::SslCert, cert_path)?;
        if let Some(key) = key_path {
            self.setopt_str(CurlOpt::SslKey, key)?;
        }
        Ok(())
    }

    /// Set browser impersonation to mimic specific browser fingerprints
    ///
    /// This method configures the curl handle to impersonate a specific browser
    /// by setting appropriate User-Agent, headers, TLS ciphers, and other options.
    ///
    /// # Arguments
    /// * `browser` - The browser type and version to impersonate
    ///
    /// # Examples
    /// ```
    /// use hyprcurl::curl::Curl;
    /// use hyprcurl::types::Browser;
    ///
    /// let mut curl = Curl::new().unwrap();
    /// // Impersonate Chrome 120
    /// curl.set_browser_impersonation(Browser::Chrome { version: 120 }).unwrap();
    /// ```
    pub fn set_browser_impersonation(&mut self, browser: Browser) -> Result<()> {
        // Set User-Agent
        let user_agent = browser.user_agent();
        self.setopt_str(CurlOpt::UserAgent, &user_agent)?;

        // Clear existing headers
        self.cleanup_headers();

        // Add browser-specific headers
        for (name, value) in browser.headers() {
            let header = format!("{}: {}", name, value);
            self.add_header(&header)?;
        }

        // Set TLS cipher suites (skip if not supported)
        // Temporarily disabled - may cause issues with some libcurl versions
        // let ciphers = browser.tls_ciphers();
        // if self.setopt_str(CurlOpt::SslCipherList, ciphers).is_err() {
        //     // Cipher list not supported, continue without it
        // }

        // Set TLS curves (skip if not supported)
        // Temporarily disabled - may cause issues with some libcurl versions
        // let curves = browser.tls_curves();
        // if self.setopt_str(CurlOpt::SslCurves, curves).is_err() {
        //     // Curves not supported, continue without it
        // }

        // Configure HTTP version (most modern browsers use HTTP/2)
        if self.set_http_version(HttpVersion::V2).is_err() {
            // HTTP/2 not supported, fall back to HTTP/1.1
            let _ = self.set_http_version(HttpVersion::V1_1);
        }

        // Set additional browser-like options
        self.setopt_long(CurlOpt::FollowLocation, 1)?; // Follow redirects like browsers
        self.setopt_long(CurlOpt::MaxRedirs, 10)?; // Reasonable redirect limit

        // Enable compression (browsers support this)
        self.setopt_str(CurlOpt::AcceptEncoding, "gzip, deflate, br")?;

        Ok(())
    }

    /// Get default CA certificate bundle path
    ///
    /// This follows the same logic as curl-cffi Python:
    /// 1. Check environment variables (REQUESTS_CA_BUNDLE, CURL_CA_BUNDLE)
    /// 2. Fall back to system-specific default paths
    /// 3. Use certifi-like bundled certificates if available
    fn get_default_ca_bundle() -> Option<String> {
        // Check environment variables first (like curl-cffi)
        if let Ok(ca_bundle) = std::env::var("REQUESTS_CA_BUNDLE") {
            if std::path::Path::new(&ca_bundle).exists() {
                return Some(ca_bundle);
            }
        }

        if let Ok(ca_bundle) = std::env::var("CURL_CA_BUNDLE") {
            if std::path::Path::new(&ca_bundle).exists() {
                return Some(ca_bundle);
            }
        }

        // Check SSL_CERT_FILE (OpenSSL style)
        if let Ok(cert_file) = std::env::var("SSL_CERT_FILE") {
            if std::path::Path::new(&cert_file).exists() {
                return Some(cert_file);
            }
        }

        // System-specific default paths
        #[cfg(target_os = "linux")]
        {
            let paths = [
                "/etc/ssl/certs/ca-certificates.crt",
                "/etc/ssl/certs/ca-bundle.crt",
                "/etc/pki/tls/certs/ca-bundle.crt",
                "/usr/share/ca-certificates/ca-certificates.crt",
            ];
            for path in &paths {
                if std::path::Path::new(path).exists() {
                    return Some(path.to_string());
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            let paths = [
                "/etc/ssl/cert.pem",
                "/usr/local/etc/openssl/cert.pem",
                "/opt/homebrew/etc/openssl@3/cert.pem",
            ];
            for path in &paths {
                if std::path::Path::new(path).exists() {
                    return Some(path.to_string());
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(program_files) = dirs::config_dir() {
                let cert_path = program_files.join("curl-ca-bundle.crt");
                if cert_path.exists() {
                    return Some(cert_path.to_string_lossy().to_string());
                }
            }
        }

        // No default CA bundle found
        None
    }
}

impl Drop for Curl {
    fn drop(&mut self) {
        self.cleanup_headers();
        if !self.handle.is_null() {
            unsafe {
                curl_sys::curl_easy_cleanup(self.handle);
            }
        }
    }
}

// Make sure Curl is Send (safe to send across threads)
unsafe impl Send for Curl {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_curl() {
        let curl = Curl::new();
        assert!(curl.is_ok());
    }

    #[test]
    fn test_set_url() {
        let mut curl = Curl::new().unwrap();
        assert!(curl.set_url("https://example.com").is_ok());
    }

    #[test]
    fn test_add_header() {
        let mut curl = Curl::new().unwrap();
        assert!(curl.add_header("User-Agent: test").is_ok());
    }
}
