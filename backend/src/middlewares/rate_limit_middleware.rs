use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use dashmap::DashMap;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

/// Rate limiter using DashMap for thread-safe in-memory storage
#[derive(Clone)]
pub struct RateLimiter {
    /// Map of IP address to list of request timestamps
    requests: Arc<DashMap<String, Vec<Instant>>>,
    /// Maximum number of requests allowed in the time window
    max_requests: usize,
    /// Time window for rate limiting
    window: Duration,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: Arc::new(DashMap::new()),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    /// Check if request should be allowed
    fn check_rate_limit(&self, key: &str) -> bool {
        let now = Instant::now();

        // Get or create entry for this key
        let mut entry = self.requests.entry(key.to_string()).or_insert(Vec::new());

        // Remove timestamps outside the current window
        entry.retain(|&timestamp| now.duration_since(timestamp) < self.window);

        // Check if limit exceeded
        if entry.len() >= self.max_requests {
            return false; // Rate limit exceeded
        }

        // Add current timestamp
        entry.push(now);
        true
    }

    /// Middleware function for rate limiting
    pub async fn middleware(
        &self,
        req: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        // Extract IP address from request
        let ip = extract_ip_from_request(&req).unwrap_or_else(|| "unknown".to_string());

        // Check rate limit
        if !self.check_rate_limit(&ip) {
            tracing::warn!(ip = %ip, "Rate limit exceeded");
            return Ok((
                StatusCode::TOO_MANY_REQUESTS,
                "Too many requests. Please try again later.",
            )
                .into_response());
        }

        // Allow request
        Ok(next.run(req).await)
    }

    /// Cleanup old entries (call periodically to prevent memory growth)
    pub fn cleanup(&self) {
        let now = Instant::now();
        self.requests.retain(|_, timestamps| {
            timestamps.retain(|&ts| now.duration_since(ts) < self.window);
            !timestamps.is_empty()
        });
    }
}

/// Extract IP address from request
fn extract_ip_from_request(req: &Request) -> Option<String> {
    // Try X-Forwarded-For header (behind proxy/load balancer)
    if let Some(forwarded) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(ip_str) = forwarded_str.split(',').next() {
                return Some(ip_str.trim().to_string());
            }
        }
    }

    // Try X-Real-IP header
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            return Some(ip_str.to_string());
        }
    }

    // Try to get from connection info
    if let Some(connect_info) = req.extensions().get::<std::net::SocketAddr>() {
        return Some(connect_info.ip().to_string());
    }

    None
}

/// Create rate limiter for authentication endpoints
/// Limits: 5 requests per second per IP
pub fn auth_rate_limiter() -> RateLimiter {
    RateLimiter::new(5, 1) // 5 requests per 1 second
}

/// Create rate limiter for general API endpoints
/// Limits: 30 requests per second per IP
pub fn api_rate_limiter() -> RateLimiter {
    RateLimiter::new(30, 1) // 30 requests per 1 second
}

/// Create rate limiter for strict endpoints (e.g., password reset)
/// Limits: 3 requests per minute per IP
pub fn strict_rate_limiter() -> RateLimiter {
    RateLimiter::new(3, 60) // 3 requests per 60 seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_allows_requests_within_limit() {
        let limiter = RateLimiter::new(5, 1);

        for _ in 0..5 {
            assert!(limiter.check_rate_limit("test-ip"));
        }
    }

    #[test]
    fn test_rate_limiter_blocks_excess_requests() {
        let limiter = RateLimiter::new(3, 1);

        // First 3 should pass
        for _ in 0..3 {
            assert!(limiter.check_rate_limit("test-ip"));
        }

        // 4th should be blocked
        assert!(!limiter.check_rate_limit("test-ip"));
    }

    #[test]
    fn test_rate_limiter_different_ips() {
        let limiter = RateLimiter::new(2, 1);

        assert!(limiter.check_rate_limit("ip1"));
        assert!(limiter.check_rate_limit("ip2"));
        assert!(limiter.check_rate_limit("ip1"));
        assert!(limiter.check_rate_limit("ip2"));

        // Both IPs should be blocked now
        assert!(!limiter.check_rate_limit("ip1"));
        assert!(!limiter.check_rate_limit("ip2"));
    }

    #[tokio::test]
    async fn test_rate_limiter_window_reset() {
        let limiter = RateLimiter::new(2, 1);

        // Use up the limit
        assert!(limiter.check_rate_limit("test-ip"));
        assert!(limiter.check_rate_limit("test-ip"));
        assert!(!limiter.check_rate_limit("test-ip"));

        // Wait for window to reset
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Should be allowed again
        assert!(limiter.check_rate_limit("test-ip"));
    }
}
