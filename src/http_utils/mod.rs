//! Shared HTTP utilities for proxy functionality.

use http::header::HeaderName;

/// Hop-by-hop headers that should not be forwarded to the backend.
/// These are connection-specific headers as defined in RFC 2616.
pub const HOP_BY_HOP_HEADERS: &[&str] = &[
    "connection",
    "keep-alive",
    "proxy-authenticate",
    "proxy-authorization",
    "te",
    "trailers",
    "transfer-encoding",
    "upgrade",
    "host",
];

/// Check if a header is a hop-by-hop header that should not be forwarded.
pub fn is_hop_by_hop_header(name: &HeaderName) -> bool {
    HOP_BY_HOP_HEADERS
        .iter()
        .any(|h| name.as_str().eq_ignore_ascii_case(h))
}

/// Filter hop-by-hop headers from a header map, returning only end-to-end headers.
pub fn filter_hop_by_hop_headers(
    headers: &http::HeaderMap,
) -> impl Iterator<Item = (&HeaderName, &http::HeaderValue)> {
    headers
        .iter()
        .filter(|(name, _)| !is_hop_by_hop_header(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hop_by_hop_headers() {
        assert!(is_hop_by_hop_header(&HeaderName::from_static("connection")));
        assert!(is_hop_by_hop_header(&HeaderName::from_static("keep-alive")));
        assert!(is_hop_by_hop_header(&HeaderName::from_static(
            "transfer-encoding"
        )));
        assert!(is_hop_by_hop_header(&HeaderName::from_static("host")));
        assert!(is_hop_by_hop_header(&HeaderName::from_static("upgrade")));
    }

    #[test]
    fn test_non_hop_by_hop_headers() {
        assert!(!is_hop_by_hop_header(&HeaderName::from_static(
            "content-type"
        )));
        assert!(!is_hop_by_hop_header(&HeaderName::from_static(
            "authorization"
        )));
        assert!(!is_hop_by_hop_header(&HeaderName::from_static(
            "x-custom-header"
        )));
        assert!(!is_hop_by_hop_header(&HeaderName::from_static("accept")));
    }
}
