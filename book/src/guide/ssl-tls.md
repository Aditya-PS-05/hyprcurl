# SSL/TLS Configuration

This chapter covers SSL/TLS configuration options in curl-cffi-rs.

## SSL Certificate Verification

By default, curl-cffi-rs verifies SSL certificates to ensure secure connections.

### Default Behavior

```rust
let mut curl = Curl::new()?;
// SSL verification is enabled by default
curl.set_url("https://example.com")?;
```

### Disabling Verification (Not Recommended)

```rust
// Disable SSL verification (use only for testing!)
curl.set_ssl_verify(Some(false))?;
```

**Warning**: Disabling SSL verification makes your connections vulnerable to man-in-the-middle attacks. Only use this for development/testing.

### Re-enabling Verification

```rust
curl.set_ssl_verify(Some(true))?;
```

## Custom CA Certificates

### Using a Custom CA Bundle

```rust
// Use a specific CA certificate file
curl.set_ca_cert("/path/to/cacert.pem")?;
```

### Using System CA Store

```rust
// Use the system's CA certificate store (default on most systems)
curl.set_ca_cert(None)?;
```

## Client Certificates

For mutual TLS (mTLS), you can provide client certificates:

```rust
// Set client certificate
curl.set_ssl_cert("/path/to/client-cert.pem")?;

// Set client private key
curl.set_ssl_key("/path/to/client-key.pem")?;

// Optional: Set key password
curl.set_ssl_key_password("password")?;
```

## TLS Versions

### Specifying TLS Version

```rust
use hyprcurl::types::SslVersion;

// Use TLS 1.2 or higher (recommended)
curl.set_ssl_version(SslVersion::TLSv1_2)?;

// Use TLS 1.3 only
curl.set_ssl_version(SslVersion::TLSv1_3)?;

// Let curl decide (default)
curl.set_ssl_version(SslVersion::Default)?;
```

## Cipher Suites

### Custom Cipher List

```rust
// Specify custom cipher suites (advanced)
curl.set_ssl_cipher_list("ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GCM-SHA384")?;
```

**Note**: When using browser impersonation, cipher suites are set automatically to match the target browser.

## SNI (Server Name Indication)

SNI is enabled by default and uses the hostname from the URL:

```rust
// SNI is automatic based on URL
curl.set_url("https://example.com")?;
// SNI will send "example.com"
```

## Pinning Certificates

For enhanced security, you can pin specific certificates:

```rust
// Pin a specific certificate (PEM format SHA256)
curl.set_pinnedpublickey("sha256//base64-encoded-hash")?;
```

Example:
```rust
curl.set_pinnedpublickey("sha256//YhKJKSzoTt2b5FP18fvpHo7fJYqQCjAa3HWY3tvRMwE=")?;
```

## OCSP Stapling

Enable OCSP stapling for certificate validation:

```rust
curl.set_ssl_verify_status(true)?;
```

## Common SSL/TLS Scenarios

### Corporate Proxy with Custom CA

```rust
let mut curl = Curl::new()?;

// Set custom CA bundle
curl.set_ca_cert("/etc/ssl/certs/corporate-ca.pem")?;

// Set proxy
curl.set_proxy("http://proxy.company.com:8080")?;

curl.set_url("https://api.example.com")?;
```

### Self-Signed Certificates (Development)

```rust
let mut curl = Curl::new()?;

// Option 1: Disable verification (not recommended)
curl.set_ssl_verify(Some(false))?;

// Option 2: Add self-signed cert to CA bundle (better)
curl.set_ca_cert("/path/to/self-signed-ca.pem")?;

curl.set_url("https://localhost:8443")?;
```

### Mutual TLS (mTLS)

```rust
let mut curl = Curl::new()?;

// Server CA certificate
curl.set_ca_cert("/path/to/server-ca.pem")?;

// Client certificate and key
curl.set_ssl_cert("/path/to/client-cert.pem")?;
curl.set_ssl_key("/path/to/client-key.pem")?;

curl.set_url("https://mtls.example.com")?;
```

## Debugging SSL Issues

### Verbose Output

```rust
// Enable verbose output for debugging
curl.set_verbose(true)?;
```

This will print detailed information about the SSL handshake.

### Check Certificate Info

```rust
curl.perform(&mut response)?;

// Get certificate chain info (if available)
let cert_info = curl.get_cert_info()?;
println!("Certificate: {:?}", cert_info);
```

### Common SSL Errors

```rust
match curl.perform(&mut response) {
    Err(CurlError::CurlCode { code: 60, .. }) => {
        eprintln!("SSL certificate problem: verify that the CA cert is OK");
    }
    Err(CurlError::CurlCode { code: 51, .. }) => {
        eprintln!("Server's certificate doesn't match the host name");
    }
    Err(CurlError::CurlCode { code: 35, .. }) => {
        eprintln!("SSL connect error");
    }
    Err(e) => eprintln!("Other error: {}", e),
    Ok(_) => println!("Success!"),
}
```

## Best Practices

1. **Always verify certificates in production** - Only disable for local development
2. **Keep CA bundles updated** - Old bundles may not trust newer certificates
3. **Use TLS 1.2 or higher** - Older versions have known vulnerabilities
4. **Pin certificates for critical services** - Adds extra layer of security
5. **Test certificate expiration** - Monitor and renew certificates before they expire

## Browser Impersonation and SSL

When using browser impersonation, SSL/TLS characteristics are automatically configured:

```rust
use hyprcurl::types::Browser;

let mut curl = Curl::new()?;

// This sets TLS version, cipher suites, extensions, etc. automatically
curl.impersonate(Browser::Chrome120)?;

// You can still override SSL settings if needed
curl.set_ca_cert("/custom/ca.pem")?;
```

## Next Steps

- [Headers and Cookies](./headers-cookies.md) - Managing request headers and cookies
- [Browser Impersonation](./browser-impersonation.md) - Mimic real browsers
- [Error Handling](./error-handling.md) - Handle SSL errors gracefully
