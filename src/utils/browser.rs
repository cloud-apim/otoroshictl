use std::fmt;
use url::Url;

/// Errors that can occur when opening a URL in the browser.
#[derive(Debug, PartialEq)]
pub enum BrowserError {
    /// The URL string is empty
    EmptyUrl,
    /// The URL could not be parsed
    InvalidUrl(String),
    /// The URL scheme is not http or https
    InvalidScheme(String),
    /// Failed to open the browser
    OpenFailed(String),
}

impl fmt::Display for BrowserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrowserError::EmptyUrl => write!(f, "URL cannot be empty"),
            BrowserError::InvalidUrl(e) => write!(f, "Invalid URL: {}", e),
            BrowserError::InvalidScheme(scheme) => {
                write!(
                    f,
                    "Invalid URL scheme '{}'. Only http and https are allowed",
                    scheme
                )
            }
            BrowserError::OpenFailed(e) => write!(f, "Failed to open browser: {}", e),
        }
    }
}

impl std::error::Error for BrowserError {}

/// Validates that a URL is well-formed and uses http or https scheme.
///
/// # Arguments
/// * `url` - The URL string to validate
///
/// # Returns
/// * `Ok(Url)` if the URL is valid
/// * `Err(BrowserError)` with a descriptive error otherwise
pub fn validate_url(url: &str) -> Result<Url, BrowserError> {
    if url.is_empty() {
        return Err(BrowserError::EmptyUrl);
    }

    let parsed = Url::parse(url).map_err(|e| BrowserError::InvalidUrl(e.to_string()))?;

    match parsed.scheme() {
        "http" | "https" => Ok(parsed),
        other => Err(BrowserError::InvalidScheme(other.to_string())),
    }
}

/// Opens a URL in the system's default browser using a custom opener function.
/// This allows for dependency injection in tests.
///
/// # Arguments
/// * `url` - The URL to open (must be http:// or https://)
/// * `opener` - A function that takes a URL string and attempts to open it
///
/// # Returns
/// * `Ok(())` if the browser was opened successfully
/// * `Err(BrowserError)` with a descriptive error otherwise
pub fn open_url_with<F>(url: &str, opener: F) -> Result<(), BrowserError>
where
    F: FnOnce(&str) -> Result<(), String>,
{
    let validated = validate_url(url)?;
    opener(validated.as_str()).map_err(BrowserError::OpenFailed)
}

/// Opens a URL in the system's default browser.
///
/// # Arguments
/// * `url` - The URL to open (must be http:// or https://)
///
/// # Returns
/// * `Ok(())` if the browser was opened successfully
/// * `Err(String)` with a descriptive error message otherwise
pub fn open_url(url: &str) -> Result<(), String> {
    open_url_with(url, |u| webbrowser::open(u).map_err(|e| e.to_string()))
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Tests for validate_url - rejection cases
    // -------------------------------------------------------------------------

    #[test]
    fn test_validate_url_rejects_empty() {
        let result = validate_url("");
        assert_eq!(result.unwrap_err(), BrowserError::EmptyUrl);
    }

    #[test]
    fn test_validate_url_rejects_invalid_url() {
        let result = validate_url("not a url at all");
        assert!(matches!(result.unwrap_err(), BrowserError::InvalidUrl(_)));
    }

    #[test]
    fn test_validate_url_rejects_ftp_scheme() {
        let result = validate_url("ftp://example.com");
        assert_eq!(
            result.unwrap_err(),
            BrowserError::InvalidScheme("ftp".to_string())
        );
    }

    #[test]
    fn test_validate_url_rejects_file_scheme() {
        let result = validate_url("file:///etc/passwd");
        assert_eq!(
            result.unwrap_err(),
            BrowserError::InvalidScheme("file".to_string())
        );
    }

    #[test]
    fn test_validate_url_rejects_javascript_scheme() {
        let result = validate_url("javascript:alert(1)");
        assert_eq!(
            result.unwrap_err(),
            BrowserError::InvalidScheme("javascript".to_string())
        );
    }

    #[test]
    fn test_validate_url_rejects_data_scheme() {
        let result = validate_url("data:text/html,<script>alert(1)</script>");
        assert_eq!(
            result.unwrap_err(),
            BrowserError::InvalidScheme("data".to_string())
        );
    }

    #[test]
    fn test_validate_url_rejects_no_scheme() {
        let result = validate_url("example.com");
        assert!(matches!(result.unwrap_err(), BrowserError::InvalidUrl(_)));
    }

    // -------------------------------------------------------------------------
    // Tests for validate_url - acceptance cases
    // -------------------------------------------------------------------------

    #[test]
    fn test_validate_url_accepts_https() {
        let result = validate_url("https://example.com");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().scheme(), "https");
    }

    #[test]
    fn test_validate_url_accepts_http() {
        let result = validate_url("http://example.com");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().scheme(), "http");
    }

    #[test]
    fn test_validate_url_accepts_https_with_path() {
        let result = validate_url("https://example.com/path/to/resource");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_url_accepts_https_with_port() {
        let result = validate_url("https://example.com:8443/admin");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_url_accepts_localhost() {
        let result = validate_url("http://localhost:8080");
        assert!(result.is_ok());
    }

    // -------------------------------------------------------------------------
    // Tests for open_url_with - dependency injection
    // -------------------------------------------------------------------------

    #[test]
    fn test_open_url_with_calls_opener_on_valid_url() {
        let mut called_with: Option<String> = None;
        let result = open_url_with("https://example.com", |url| {
            called_with = Some(url.to_string());
            Ok(())
        });
        assert!(result.is_ok());
        assert_eq!(called_with, Some("https://example.com/".to_string()));
    }

    #[test]
    fn test_open_url_with_does_not_call_opener_on_invalid_url() {
        let mut called = false;
        let result = open_url_with("ftp://example.com", |_| {
            called = true;
            Ok(())
        });
        assert!(result.is_err());
        assert!(!called);
    }

    #[test]
    fn test_open_url_with_propagates_opener_error() {
        let result = open_url_with("https://example.com", |_| {
            Err("Browser not found".to_string())
        });
        assert_eq!(
            result.unwrap_err(),
            BrowserError::OpenFailed("Browser not found".to_string())
        );
    }

    // -------------------------------------------------------------------------
    // Tests for BrowserError display
    // -------------------------------------------------------------------------

    #[test]
    fn test_browser_error_display_empty_url() {
        let err = BrowserError::EmptyUrl;
        assert_eq!(err.to_string(), "URL cannot be empty");
    }

    #[test]
    fn test_browser_error_display_invalid_scheme() {
        let err = BrowserError::InvalidScheme("ftp".to_string());
        assert_eq!(
            err.to_string(),
            "Invalid URL scheme 'ftp'. Only http and https are allowed"
        );
    }
}
