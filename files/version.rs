//! Static build info for Adobe Firefly.

pub const BUILD_VERSION: &str  = "v3.1.18";
pub const BUILD_CHANNEL: &str  = "lts";
pub const BUILD_COMMIT: &str   = "426fc6ef4a72";
pub const BUILD_TIMESTAMP: u64 = 1799362594;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Adobe Firefly", BUILD_VERSION, BUILD_CHANNEL)
}
