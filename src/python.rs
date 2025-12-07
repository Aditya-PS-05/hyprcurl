//! Python bindings using PyO3

use crate::curl::Curl as RustCurl;
use crate::error::CurlError;
use crate::types::{Browser, HttpVersion};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Convert CurlError to PyErr
impl From<CurlError> for PyErr {
    fn from(err: CurlError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}

/// Python wrapper for Curl
#[pyclass]
pub struct Curl {
    inner: RustCurl,
}

#[pymethods]
impl Curl {
    /// Create a new Curl instance
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Curl {
            inner: RustCurl::new()?,
        })
    }

    /// Set URL
    fn set_url(&mut self, url: &str) -> PyResult<()> {
        self.inner.set_url(url)?;
        Ok(())
    }

    /// Add header
    fn add_header(&mut self, header: &str) -> PyResult<()> {
        self.inner.add_header(header)?;
        Ok(())
    }

    /// Set timeout
    fn set_timeout(&mut self, timeout: i64) -> PyResult<()> {
        self.inner
            .setopt_long(crate::types::CurlOpt::Timeout, timeout)?;
        Ok(())
    }

    /// Set user agent
    fn set_user_agent(&mut self, ua: &str) -> PyResult<()> {
        self.inner
            .setopt_str(crate::types::CurlOpt::UserAgent, ua)?;
        Ok(())
    }

    /// Impersonate browser
    fn impersonate(&mut self, target: &str, default_headers: bool) -> PyResult<()> {
        self.inner.impersonate(target, default_headers)?;
        Ok(())
    }

    /// Set proxy
    fn set_proxy(&mut self, proxy: &str) -> PyResult<()> {
        self.inner.set_proxy(proxy)?;
        Ok(())
    }

    /// Set POST data
    fn set_post_data(&mut self, data: &str) -> PyResult<()> {
        self.inner.set_post_data(data)?;
        Ok(())
    }

    /// Perform request
    fn perform<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let mut buffer = Vec::new();

        // Release GIL during blocking I/O
        py.allow_threads(|| self.inner.perform(&mut buffer))?;

        Ok(PyBytes::new_bound(py, &buffer))
    }

    /// Get response code
    fn response_code(&self) -> PyResult<i64> {
        Ok(self.inner.response_code()?)
    }

    /// Get total time
    fn total_time(&self) -> PyResult<f64> {
        Ok(self.inner.total_time()?)
    }

    /// Get effective URL
    fn effective_url(&self) -> PyResult<String> {
        Ok(self.inner.effective_url()?)
    }

    /// Reset handle
    fn reset(&mut self) {
        self.inner.reset();
    }

    /// Get version
    #[staticmethod]
    fn version() -> String {
        crate::version()
    }
}

/// Helper function to parse browser string
fn parse_browser(impersonate: &str) -> PyResult<Browser> {
    let browser = match impersonate.to_lowercase().as_str() {
        "chrome" => Browser::ChromeLatest,
        "firefox" => Browser::FirefoxLatest,
        "safari" => Browser::SafariLatest,
        "edge" => Browser::EdgeLatest,
        s if s.starts_with("chrome") => {
            let version: u32 = s[6..].parse().map_err(|_| {
                PyException::new_err(format!("Invalid browser version: {}", impersonate))
            })?;
            Browser::Chrome { version }
        }
        s if s.starts_with("firefox") => {
            let version: u32 = s[7..].parse().map_err(|_| {
                PyException::new_err(format!("Invalid browser version: {}", impersonate))
            })?;
            Browser::Firefox { version }
        }
        s if s.starts_with("edge") => {
            let version: u32 = s[4..].parse().map_err(|_| {
                PyException::new_err(format!("Invalid browser version: {}", impersonate))
            })?;
            Browser::Edge { version }
        }
        _ => {
            return Err(PyException::new_err(format!(
                "Unknown browser: {}",
                impersonate
            )))
        }
    };
    Ok(browser)
}

/// Quick GET request (Python curl_cffi compatible)
///
/// # Arguments
/// * `url` - The URL to fetch
/// * `impersonate` - Optional browser to impersonate (e.g., "chrome", "chrome110", "firefox")
/// * `proxies` - Optional proxy URL (e.g., "http://localhost:3128", "socks5://localhost:1080")
#[pyfunction]
#[pyo3(signature = (url, impersonate=None, proxies=None))]
fn get(url: &str, impersonate: Option<&str>, proxies: Option<&str>) -> PyResult<Vec<u8>> {
    let mut curl = RustCurl::new()?;

    if let Some(browser_str) = impersonate {
        let browser = parse_browser(browser_str)?;
        curl.set_browser_impersonation(browser)?;
    }

    if let Some(proxy) = proxies {
        curl.set_proxy(proxy)?;
    }

    curl.set_url(url)?;
    let mut buffer = Vec::new();
    curl.perform(&mut buffer)?;
    Ok(buffer)
}

/// POST request (Python curl_cffi compatible)
///
/// # Arguments
/// * `url` - The URL to post to
/// * `data` - The data to POST
/// * `impersonate` - Optional browser to impersonate
/// * `proxies` - Optional proxy URL
#[pyfunction]
#[pyo3(signature = (url, data, impersonate=None, proxies=None))]
fn post(
    url: &str,
    data: &str,
    impersonate: Option<&str>,
    proxies: Option<&str>,
) -> PyResult<Vec<u8>> {
    let mut curl = RustCurl::new()?;

    if let Some(browser_str) = impersonate {
        let browser = parse_browser(browser_str)?;
        curl.set_browser_impersonation(browser)?;
    }

    if let Some(proxy) = proxies {
        curl.set_proxy(proxy)?;
    }

    curl.set_url(url)?;
    curl.set_post_data(data)?;
    let mut buffer = Vec::new();
    curl.perform(&mut buffer)?;
    Ok(buffer)
}

/// Initialize the Python module
#[pymodule]
fn hyprcurl(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Curl>()?;
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_function(wrap_pyfunction!(post, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_curl_new() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|_py| {
            let curl = Curl::new();
            assert!(curl.is_ok());
        });
    }
}
