# Basic Usage

This chapter covers common patterns and configurations for curl-cffi-rs.

## Configuration Options

### Timeouts

Set timeouts to prevent requests from hanging:

```rust
use hyprcurl::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    let mut curl = Curl::new()?;
    curl.set_url("https://httpbin.org/delay/10")?;

    // Set connection timeout (30 seconds)
    curl.set_connect_timeout(30)?;

    // Set total timeout (60 seconds)
    curl.set_timeout(60)?;

    let mut response = Vec::new();
    curl.perform(&mut response)?;

    Ok(())
}
```

### Follow Redirects

Control how redirects are handled:

```rust
// Redirects are followed by default
let mut curl = Curl::new()?;

// Disable redirects
curl.set_follow_redirects(false)?;

// Set maximum number of redirects
curl.set_max_redirects(5)?;
```

### SSL/TLS Options

Configure SSL certificate verification:

```rust
// Verify SSL certificates (default)
curl.set_ssl_verify(Some(true))?;

// Disable SSL verification (not recommended for production!)
curl.set_ssl_verify(Some(false))?;

// Use custom CA bundle
curl.set_ca_cert("/path/to/cacert.pem")?;
```

## Working with Different HTTP Methods

### GET Request

```rust
let mut curl = Curl::new()?;
curl.set_url("https://api.example.com/users")?;
```

### POST Request

```rust
let mut curl = Curl::new()?;
curl.set_url("https://api.example.com/users")?;
curl.set_post_fields(r#"{"name": "John"}"#)?;
```

### PUT Request

```rust
use hyprcurl::types::CurlOpt;

let mut curl = Curl::new()?;
curl.set_url("https://api.example.com/users/1")?;
curl.set_custom_request("PUT")?;
curl.set_post_fields(r#"{"name": "Jane"}"#)?;
```

### DELETE Request

```rust
let mut curl = Curl::new()?;
curl.set_url("https://api.example.com/users/1")?;
curl.set_custom_request("DELETE")?;
```

## Response Handling

### Accessing Response Data

```rust
let mut curl = Curl::new()?;
curl.set_url("https://httpbin.org/json")?;

let mut response = Vec::new();
curl.perform(&mut response)?;

// Parse JSON response
let json: serde_json::Value = serde_json::from_slice(&response)?;
println!("{:#?}", json);
```

### Getting Response Metadata

```rust
curl.perform(&mut response)?;

// HTTP status code
let status = curl.get_response_code()?;

// Content type
let content_type = curl.get_content_type()?;

// Download size
let download_size = curl.get_download_size()?;

// Total time
let total_time = curl.get_total_time()?;

println!("Status: {}, Type: {:?}, Size: {} bytes, Time: {:.2}s",
         status, content_type, download_size, total_time);
```

## Reusing Curl Handles

Reuse the same Curl instance for multiple requests (connection pooling):

```rust
let mut curl = Curl::new()?;

// First request
curl.set_url("https://httpbin.org/get")?;
let mut response1 = Vec::new();
curl.perform(&mut response1)?;

// Second request (reuses connection)
curl.set_url("https://httpbin.org/headers")?;
let mut response2 = Vec::new();
curl.perform(&mut response2)?;
```

## Common Patterns

### API Client with Authentication

```rust
use hyprcurl::{Curl, CurlError};

struct ApiClient {
    base_url: String,
    api_key: String,
}

impl ApiClient {
    fn new(base_url: String, api_key: String) -> Self {
        Self { base_url, api_key }
    }

    fn get(&self, endpoint: &str) -> Result<Vec<u8>, CurlError> {
        let mut curl = Curl::new()?;
        let url = format!("{}{}", self.base_url, endpoint);
        curl.set_url(&url)?;

        // Add authentication header
        let auth_header = format!("Authorization: Bearer {}", self.api_key);
        curl.add_header(&auth_header)?;

        let mut response = Vec::new();
        curl.perform(&mut response)?;

        Ok(response)
    }
}
```

### Download File

```rust
use std::fs::File;
use std::io::Write;

fn download_file(url: &str, path: &str) -> Result<(), CurlError> {
    let mut curl = Curl::new()?;
    curl.set_url(url)?;

    let mut response = Vec::new();
    curl.perform(&mut response)?;

    let mut file = File::create(path)?;
    file.write_all(&response)?;

    Ok(())
}
```

## Next Steps

- Learn about [Making Requests](../guide/making-requests.md) in more detail
- Explore [Browser Impersonation](../guide/browser-impersonation.md)
- Check out [Error Handling](../guide/error-handling.md) best practices
