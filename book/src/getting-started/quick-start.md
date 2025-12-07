# Quick Start

Let's make your first HTTP request with curl-cffi-rs!

## Simple GET Request

Here's the simplest way to fetch a webpage:

```rust
use hyprcurl::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    // Create a new Curl instance
    let mut curl = Curl::new()?;

    // Set the URL to fetch
    curl.set_url("https://httpbin.org/get")?;

    // Create a buffer to store the response
    let mut response = Vec::new();

    // Perform the request
    curl.perform(&mut response)?;

    // Convert response to string and print
    let body = String::from_utf8_lossy(&response);
    println!("Response: {}", body);

    Ok(())
}
```

## POST Request with Data

Sending POST data is just as easy:

```rust
use hyprcurl::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    let mut curl = Curl::new()?;
    curl.set_url("https://httpbin.org/post")?;

    // Set POST data
    curl.set_post_fields(r#"{"name": "curl-cffi-rs", "type": "library"}"#)?;

    let mut response = Vec::new();
    curl.perform(&mut response)?;

    println!("{}", String::from_utf8_lossy(&response));
    Ok(())
}
```

## Adding Headers

Custom headers are common for API requests:

```rust
use hyprcurl::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    let mut curl = Curl::new()?;
    curl.set_url("https://httpbin.org/headers")?;

    // Add custom headers
    curl.add_header("Content-Type: application/json")?;
    curl.add_header("Authorization: Bearer YOUR_TOKEN")?;

    let mut response = Vec::new();
    curl.perform(&mut response)?;

    println!("{}", String::from_utf8_lossy(&response));
    Ok(())
}
```

## Browser Impersonation

The magic feature - impersonate a real browser:

```rust
use hyprcurl::{Curl, CurlError};
use hyprcurl::types::Browser;

fn main() -> Result<(), CurlError> {
    let mut curl = Curl::new()?;
    curl.set_url("https://tls.peet.ws/api/all")?;

    // Impersonate Chrome 120
    curl.impersonate(Browser::Chrome120)?;

    let mut response = Vec::new();
    curl.perform(&mut response)?;

    // This will show Chrome's TLS fingerprint!
    println!("{}", String::from_utf8_lossy(&response));
    Ok(())
}
```

## Getting Response Info

You can extract metadata about the response:

```rust
use hyprcurl::{Curl, CurlError};

fn main() -> Result<(), CurlError> {
    let mut curl = Curl::new()?;
    curl.set_url("https://httpbin.org/get")?;

    let mut response = Vec::new();
    curl.perform(&mut response)?;

    // Get response information
    let status_code = curl.get_response_code()?;
    let content_type = curl.get_content_type()?;

    println!("Status: {}", status_code);
    println!("Content-Type: {:?}", content_type);
    println!("Body length: {} bytes", response.len());

    Ok(())
}
```

## Error Handling

Always handle errors properly:

```rust
use hyprcurl::{Curl, CurlError};

fn fetch_url(url: &str) -> Result<String, CurlError> {
    let mut curl = Curl::new()?;
    curl.set_url(url)?;

    let mut response = Vec::new();
    curl.perform(&mut response)?;

    String::from_utf8(response)
        .map_err(|e| CurlError::Utf8Error(e.utf8_error()))
}

fn main() {
    match fetch_url("https://httpbin.org/get") {
        Ok(body) => println!("Success: {}", body),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Next Steps

You now know the basics! Continue to [Basic Usage](./basic-usage.md) to learn more about configuring requests, or jump to the [Guide](../guide/making-requests.md) section for specific use cases.
