# Browser Impersonation

One of the most powerful features of curl-cffi-rs is its ability to impersonate real web browsers at the TLS fingerprint level.

## Why Browser Impersonation?

Modern websites use sophisticated techniques to detect automated clients:

- **TLS Fingerprinting** - Analyzing SSL/TLS handshake characteristics
- **HTTP/2 Fingerprinting** - Examining HTTP/2 settings and priorities
- **JA3/JA4 Signatures** - Unique identifiers based on TLS parameters

curl-cffi-rs uses libcurl-impersonate to mimic real browsers, making your requests indistinguishable from a real browser at the network level.

## Supported Browsers

curl-cffi-rs supports impersonating various browsers:

```rust
use hyprcurl::types::Browser;

// Chrome versions
Browser::Chrome99
Browser::Chrome100
Browser::Chrome101
Browser::Chrome104
Browser::Chrome107
Browser::Chrome110
Browser::Chrome116
Browser::Chrome119
Browser::Chrome120
Browser::Chrome123
Browser::Chrome124
Browser::Chrome126
Browser::Chrome127
Browser::Chrome128
Browser::Chrome131

// Edge versions
Browser::Edge99
Browser::Edge101

// Safari versions
Browser::Safari15_3
Browser::Safari15_5
Browser::Safari17_0
Browser::Safari17_2_1
Browser::Safari18_0
Browser::SafariIOS17_2
Browser::SafariIOS17_4_1
Browser::SafariIOS18_1_1
```

## Basic Usage

### Impersonating Chrome

```rust
use hyprcurl::{Curl, CurlError};
use hyprcurl::types::Browser;

fn main() -> Result<(), CurlError> {
    let mut curl = Curl::new()?;

    // Impersonate Chrome 120
    curl.impersonate(Browser::Chrome120)?;

    curl.set_url("https://tls.peet.ws/api/all")?;

    let mut response = Vec::new();
    curl.perform(&mut response)?;

    println!("{}", String::from_utf8_lossy(&response));
    Ok(())
}
```

### Impersonating Safari

```rust
curl.impersonate(Browser::Safari17_0)?;
```

### Impersonating Edge

```rust
curl.impersonate(Browser::Edge101)?;
```

## Testing Your Fingerprint

You can verify that impersonation is working by checking your TLS fingerprint:

```rust
use hyprcurl::{Curl, types::Browser};

fn test_fingerprint(browser: Browser) -> Result<(), CurlError> {
    let mut curl = Curl::new()?;
    curl.impersonate(browser)?;
    curl.set_url("https://tls.peet.ws/api/all")?;

    let mut response = Vec::new();
    curl.perform(&mut response)?;

    let json: serde_json::Value = serde_json::from_slice(&response)?;

    println!("Browser: {:?}", browser);
    println!("JA3 Fingerprint: {}", json["tls"]["ja3"]);
    println!("JA4 Fingerprint: {}", json["tls"]["ja4"]);
    println!("User Agent: {}", json["http"]["headers"]["user-agent"]);

    Ok(())
}

fn main() {
    test_fingerprint(Browser::Chrome120).unwrap();
    test_fingerprint(Browser::Safari17_0).unwrap();
}
```

## Combining with Custom Headers

You can combine browser impersonation with custom headers:

```rust
let mut curl = Curl::new()?;

// Impersonate Chrome 120
curl.impersonate(Browser::Chrome120)?;

// Add custom headers (these override defaults)
curl.add_header("Accept-Language: en-US,en;q=0.9")?;
curl.add_header("Referer: https://google.com")?;

curl.set_url("https://api.example.com")?;
```

**Note**: Custom headers are added *after* impersonation, so they will override the browser's default headers.

## HTTP/2 Support

Browser impersonation includes HTTP/2 characteristics:

```rust
curl.impersonate(Browser::Chrome120)?;

// HTTP/2 will be used automatically if the server supports it
// You can verify this:
curl.perform(&mut response)?;

let http_version = curl.get_http_version()?;
println!("HTTP Version: {}", http_version);
```

## Real-World Example

Here's a complete example scraping a protected website:

```rust
use hyprcurl::{Curl, CurlError};
use hyprcurl::types::Browser;

fn scrape_protected_site(url: &str) -> Result<String, CurlError> {
    let mut curl = Curl::new()?;

    // Impersonate latest Chrome
    curl.impersonate(Browser::Chrome131)?;

    // Add realistic headers
    curl.add_header("Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")?;
    curl.add_header("Accept-Language: en-US,en;q=0.5")?;
    curl.add_header("Accept-Encoding: gzip, deflate, br")?;
    curl.add_header("DNT: 1")?;
    curl.add_header("Connection: keep-alive")?;
    curl.add_header("Upgrade-Insecure-Requests: 1")?;
    curl.add_header("Sec-Fetch-Dest: document")?;
    curl.add_header("Sec-Fetch-Mode: navigate")?;
    curl.add_header("Sec-Fetch-Site: none")?;

    // Set timeout
    curl.set_timeout(30)?;

    // Make request
    curl.set_url(url)?;
    let mut response = Vec::new();
    curl.perform(&mut response)?;

    // Check status
    let status = curl.get_response_code()?;
    if status != 200 {
        return Err(CurlError::Other(format!("HTTP {}", status)));
    }

    Ok(String::from_utf8_lossy(&response).to_string())
}

fn main() {
    match scrape_protected_site("https://example.com") {
        Ok(html) => println!("Downloaded {} bytes", html.len()),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Best Practices

1. **Choose the right browser** - Use a common, recent version
2. **Match user agent** - The user agent is set automatically, but verify it matches your needs
3. **Add realistic headers** - Real browsers send many headers
4. **Respect robots.txt** - Browser impersonation doesn't justify ignoring site policies
5. **Rate limiting** - Don't hammer servers even if you can bypass detection

## Limitations

While browser impersonation is powerful, it has limitations:

- **JavaScript** - curl-cffi-rs doesn't execute JavaScript
- **Browser APIs** - Features like Canvas fingerprinting aren't replicated
- **Cookies** - You need to manage cookies manually
- **Sessions** - Maintain session state yourself

For full browser automation, consider tools like Puppeteer or Playwright. Use curl-cffi-rs when you need fast, lightweight requests with realistic network characteristics.

## Next Steps

- [SSL/TLS Configuration](./ssl-tls.md) - Advanced TLS settings
- [Headers and Cookies](./headers-cookies.md) - Managing cookies and headers
- [Async Support](../advanced/async.md) - Making concurrent requests
