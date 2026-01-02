//! Application constants and configuration values
//!
//! This module contains all configuration constants used throughout the application.
//! Centralizing constants makes it easier to maintain and modify application behavior.

// ============================================================================
// Application Configuration
// ============================================================================

/// Main web application URL
pub const APP_URL: &str = "https://app.elulib.com";

/// Host for connectivity verification
pub const CONNECTIVITY_HOST: &str = "app.elulib.com";

/// Port for connectivity verification (HTTPS)
pub const CONNECTIVITY_PORT: u16 = 443;

/// Application title
pub const APP_TITLE: &str = "élulib";

/// Application bundle identifier
pub const APP_IDENTIFIER: &str = "com.elulib.mobile";

/// Authorized identifier for keychain/keystore storage
pub const KEYCHAIN_SERVICE_ID: &str = "com.elulib.mobile";

// ============================================================================
// Platform Requirements
// ============================================================================

/// Minimum iOS system version required
pub const IOS_MIN_SYSTEM_VERSION: &str = "14.0";

/// Minimum Android SDK version required
pub const ANDROID_MIN_SDK_VERSION: u32 = 24;

// ============================================================================
// Keychain/Keystore Limits
// ============================================================================

/// Maximum allowed size for keychain username/key identifier (bytes/characters)
///
/// This limit ensures compatibility with platform-specific keychain implementations
/// (iOS Keychain Services and Android Keystore) which may have length restrictions.
/// Keys exceeding this length will be rejected with a validation error.
pub const MAX_KEYCHAIN_KEY_LENGTH: usize = 256;

/// Maximum allowed size for a stored value in keychain (bytes/characters)
///
/// This limit ensures compatibility with platform-specific keychain implementations
/// and prevents excessive memory usage. Values exceeding this length will be rejected
/// with a validation error before attempting to store in the keychain.
pub const MAX_KEYCHAIN_VALUE_LENGTH: usize = 4096;

/// Minimum length for keychain key identifier (additional validation)
///
/// Keys must be non-empty to be valid. This constant enforces that requirement
/// and provides clear error messages when validation fails.
pub const MIN_KEYCHAIN_KEY_LENGTH: usize = 1;

// ============================================================================
// Connectivity & Timeouts
// ============================================================================

/// Timeout for connectivity verification (seconds)
///
/// This is the initial timeout for each connection attempt. The timeout is applied
/// to each individual TCP connection attempt. If a connection attempt times out or
/// fails, the system will retry with exponential backoff (see `RETRY_BASE_DELAY_MS`
/// and `MAX_CONNECTIVITY_RETRIES`).
///
/// Value of 2 seconds provides a balance between:
/// - Fast failure detection on network issues
/// - Allowing time for slower network connections
/// - Avoiding excessive wait times during connectivity checks
pub const CONNECTIVITY_TIMEOUT_SECS: u64 = 2;

/// Maximum number of retry attempts for connectivity check
///
/// After the initial connection attempt, this specifies how many additional retry
/// attempts will be made before giving up. Combined with `RETRY_BASE_DELAY_MS`,
/// this determines the total time spent attempting to connect.
///
/// With `MAX_CONNECTIVITY_RETRIES = 2` and `RETRY_BASE_DELAY_MS = 500`:
/// - Initial attempt: immediate
/// - Retry 1: after 500ms
/// - Retry 2: after 1000ms (exponential backoff: 500ms * 2^1)
/// - Total maximum time: ~3.5 seconds (2s timeout + delays + connection attempts)
pub const MAX_CONNECTIVITY_RETRIES: u32 = 2;

/// Base delay for exponential backoff (milliseconds)
///
/// This is the base delay used in the exponential backoff algorithm for retry
/// attempts. Each retry waits progressively longer:
/// - Retry 1: `RETRY_BASE_DELAY_MS * 2^0` = 500ms
/// - Retry 2: `RETRY_BASE_DELAY_MS * 2^1` = 1000ms
/// - Retry N: `RETRY_BASE_DELAY_MS * 2^(N-1)`
///
/// The value of 500ms provides quick retries for transient network issues while
/// avoiding excessive load on the network stack.
pub const RETRY_BASE_DELAY_MS: u64 = 500;

// ============================================================================
// Rate Limiting
// ============================================================================

// TODO: Implement rate limiting for keychain operations to prevent abuse
// Rate limiting constants are defined below but not yet used in the codebase

/// Rate limiting: Maximum number of keychain operations per time window
///
/// This constant defines the maximum number of keychain operations (store, retrieve,
/// remove, exists) that can be performed within the time window defined by
/// `RATE_LIMIT_WINDOW_SECS`.
///
/// **Note**: Rate limiting is not yet implemented. These constants are reserved
/// for future implementation to prevent abuse and excessive keychain access.
///
/// Example: With `RATE_LIMIT_MAX_REQUESTS = 10` and `RATE_LIMIT_WINDOW_SECS = 60`,
/// a maximum of 10 keychain operations would be allowed per 60-second window.
pub const RATE_LIMIT_MAX_REQUESTS: u32 = 10;

/// Rate limiting: Time window in seconds for keychain operations
///
/// This constant defines the time window (in seconds) used for rate limiting
/// keychain operations. Combined with `RATE_LIMIT_MAX_REQUESTS`, it determines
/// how many operations are allowed per time period.
///
/// **Note**: Rate limiting is not yet implemented. These constants are reserved
/// for future implementation.
///
/// Example: With `RATE_LIMIT_WINDOW_SECS = 60` and `RATE_LIMIT_MAX_REQUESTS = 10`,
/// a maximum of 10 keychain operations would be allowed per 60-second window.
pub const RATE_LIMIT_WINDOW_SECS: u64 = 60;

// ============================================================================
// Error Messages
// ============================================================================

/// Error messages returned to the frontend
pub mod error {
    /// Error message prefix for keychain store operations
    pub const KEYCHAIN_STORE_FAILED: &str = "Keychain store failed: {}";
    
    /// Error message prefix for keychain retrieve operations
    pub const KEYCHAIN_RETRIEVE_FAILED: &str = "Keychain retrieve failed: {}";
    
    /// Error message prefix for keychain remove operations
    pub const KEYCHAIN_REMOVE_FAILED: &str = "Keychain remove failed: {}";
}

// ============================================================================
// Format Strings
// ============================================================================

/// Format strings for string formatting operations
pub mod format {
    /// Format string for key-value pairs (key:value)
    pub const KEY_VALUE_SEPARATOR: &str = "{}:{}";
}

// ============================================================================
// Process Exit Codes
// ============================================================================

/// Process exit codes
pub mod exit {
    /// Exit code for application failure
    pub const FAILURE: i32 = 1;
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper functions for formatting messages with constants
///
/// These functions allow us to use constants with format macros
/// while working within Rust's requirement for string literals.
pub mod helpers {

    /// Format a keychain store error message
    pub fn keychain_store_error(e: &dyn std::fmt::Display) -> String {
        format!("Keychain store failed: {}", e)
    }

    /// Format a keychain retrieve error message
    pub fn keychain_retrieve_error(e: &dyn std::fmt::Display) -> String {
        format!("Keychain retrieve failed: {}", e)
    }

    /// Format a keychain remove error message
    pub fn keychain_remove_error(e: &dyn std::fmt::Display) -> String {
        format!("Keychain remove failed: {}", e)
    }

    /// Format a key-value pair string
    pub fn key_value_pair(key: &str, value: &str) -> String {
        format!("{}:{}", key, value)
    }

    use super::{MIN_KEYCHAIN_KEY_LENGTH, MAX_KEYCHAIN_KEY_LENGTH, MAX_KEYCHAIN_VALUE_LENGTH};

    /// Validate keychain key length
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the key length is valid, or an error message if invalid.
    pub fn validate_keychain_key(key: &str) -> Result<(), String> {
        let len = key.len();
        if len < MIN_KEYCHAIN_KEY_LENGTH {
            return Err(format!(
                "Key length must be at least {} characters, got {}",
                MIN_KEYCHAIN_KEY_LENGTH, len
            ));
        }
        if len > MAX_KEYCHAIN_KEY_LENGTH {
            return Err(format!(
                "Key length must be at most {} characters, got {}",
                MAX_KEYCHAIN_KEY_LENGTH, len
            ));
        }
        Ok(())
    }

    /// Validate keychain value length
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the value length is valid, or an error message if invalid.
    pub fn validate_keychain_value(value: &str) -> Result<(), String> {
        let len = value.len();
        if len > MAX_KEYCHAIN_VALUE_LENGTH {
            return Err(format!(
                "Value length must be at most {} characters, got {}",
                MAX_KEYCHAIN_VALUE_LENGTH, len
            ));
        }
        Ok(())
    }
}
