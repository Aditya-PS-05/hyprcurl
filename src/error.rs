//! Error types for curl-cffi-rs

use thiserror::Error;

/// Result type alias for curl operations
pub type Result<T> = std::result::Result<T, CurlError>;

/// Errors that can occur when using curl
#[derive(Error, Debug)]
pub enum CurlError {
    /// Failed to initialize curl
    #[error("Failed to initialize curl")]
    InitError,

    /// Curl operation failed with error code
    #[error("Curl error {code}: {message}")]
    CurlCode {
        code: curl_sys::CURLcode,
        message: String,
    },

    /// Multi curl operation failed
    #[error("Curl multi error {code}: {message}")]
    MultiError {
        code: curl_sys::CURLMcode,
        message: String,
    },

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Invalid option value
    #[error("Invalid option value: {0}")]
    InvalidOption(String),

    /// WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    /// UTF-8 conversion error
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Handle is closed
    #[error("Cannot perform operation on closed handle")]
    HandleClosed,

    /// Other error
    #[error("{0}")]
    Other(String),
}

impl CurlError {
    /// Create a CurlError from a CURLcode
    pub fn from_curl_code(code: curl_sys::CURLcode) -> Self {
        let message = unsafe {
            let msg_ptr = curl_sys::curl_easy_strerror(code);
            std::ffi::CStr::from_ptr(msg_ptr)
                .to_string_lossy()
                .into_owned()
        };

        CurlError::CurlCode { code, message }
    }

    /// Get the curl error code if available
    pub fn code(&self) -> Option<u32> {
        match self {
            CurlError::CurlCode { code, .. } => u32::try_from(*code).ok(),
            CurlError::MultiError { code, .. } => Some(*code as u32),
            _ => None,
        }
    }
}

/// Check a CURLcode and convert to Result
pub(crate) fn check_code(code: curl_sys::CURLcode) -> Result<()> {
    if code == curl_sys::CURLE_OK {
        Ok(())
    } else {
        Err(CurlError::from_curl_code(code))
    }
}
