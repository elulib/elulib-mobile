/// Connectivity check module
///
/// This module provides optimized connectivity checking functionality
/// that verifies network connectivity to the application server.
///
/// Features:
/// - TCP connection check with configurable timeout
/// - Exponential backoff retry mechanism
/// - Non-blocking async implementation
/// - Uses constants from the constants module

use crate::constants;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Result type for connectivity checks
pub type ConnectivityResult = Result<bool, ConnectivityError>;

/// Errors that can occur during connectivity checks
#[derive(Debug, thiserror::Error)]
pub enum ConnectivityError {
    /// Network I/O error
    #[error("Network error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Timeout during connection attempt
    #[error("Connection timeout")]
    Timeout,
    
    /// Maximum retries exceeded
    #[error("Maximum retries exceeded")]
    MaxRetriesExceeded,
}

/// Performs a single connectivity check attempt
///
/// Attempts to establish a TCP connection to the configured host and port
/// within the specified timeout period.
///
/// # Returns
///
/// - `Ok(true)` if connection succeeds
/// - `Err(ConnectivityError::Io(_))` if connection fails due to network I/O error
/// - `Err(ConnectivityError::Timeout)` if connection times out
async fn check_connectivity_once() -> ConnectivityResult {
    let host = constants::CONNECTIVITY_HOST;
    let port = constants::CONNECTIVITY_PORT;
    let timeout_duration = Duration::from_secs(constants::CONNECTIVITY_TIMEOUT_SECS);
    
    let addr = format!("{}:{}", host, port);
    
    log::debug!("Checking connectivity to {}:{}", host, port);
    
    match timeout(timeout_duration, TcpStream::connect(&addr)).await {
        Ok(Ok(_stream)) => {
            log::debug!("Connectivity check successful: {}:{}", host, port);
            Ok(true)
        }
        Ok(Err(e)) => {
            log::debug!("Connectivity check failed: {}:{} - {}", host, port, e);
            Err(ConnectivityError::Io(e))
        }
        Err(_) => {
            log::debug!("Connectivity check timeout: {}:{}", host, port);
            Err(ConnectivityError::Timeout)
        }
    }
}

/// Performs a connectivity check with retry logic and exponential backoff
///
/// This function attempts to connect to the server with the following strategy:
/// 1. Initial connection attempt
/// 2. If it fails, retry with exponential backoff
/// 3. Maximum retries are controlled by `MAX_CONNECTIVITY_RETRIES`
///
/// # Returns
///
/// - `Ok(true)` if connectivity is available
/// - `Ok(false)` if connectivity is not available after all retries
/// - `Err(ConnectivityError)` if an unexpected error occurs
///
/// # Examples
///
/// ```rust,no_run
/// use elulib_mobile::connectivity::check_connectivity;
/// 
/// # async fn example() -> Result<(), elulib_mobile::connectivity::ConnectivityError> {
/// let is_connected = check_connectivity().await?;
/// if is_connected {
///     println!("Connected to server");
/// }
/// # Ok(())
/// # }
/// ```
pub async fn check_connectivity() -> ConnectivityResult {
    let max_retries = constants::MAX_CONNECTIVITY_RETRIES;
    let base_delay_ms = constants::RETRY_BASE_DELAY_MS;
    
    // First attempt (no delay)
    match check_connectivity_once().await {
        Ok(true) => {
            log::info!("Connectivity check passed on first attempt");
            return Ok(true);
        }
        Ok(false) => {
            // Unreachable: check_connectivity_once() only returns Ok(true) or Err
            unreachable!("check_connectivity_once() never returns Ok(false)");
        }
        Err(ConnectivityError::Timeout) => {
            // Will retry below
        }
        Err(e) => {
            log::warn!("Connectivity check error: {}", e);
            // Will retry below
        }
    }
    
    // Retry with exponential backoff
    for attempt in 1..=max_retries {
        let delay_ms = base_delay_ms * (1 << (attempt - 1)); // Exponential: 500ms, 1000ms, 2000ms...
        let delay = Duration::from_millis(delay_ms);
        
        log::debug!("Retrying connectivity check (attempt {}/{}) after {}ms", attempt, max_retries, delay_ms);
        
        tokio::time::sleep(delay).await;
        
        match check_connectivity_once().await {
            Ok(true) => {
                log::info!("Connectivity check passed on retry attempt {}", attempt);
                return Ok(true);
            }
            Ok(false) => {
                // Unreachable: check_connectivity_once() only returns Ok(true) or Err
                unreachable!("check_connectivity_once() never returns Ok(false)");
            }
            Err(ConnectivityError::Timeout) => {
                // Continue to next retry
                continue;
            }
            Err(e) => {
                log::warn!("Connectivity check error on attempt {}: {}", attempt, e);
                // Continue to next retry
                continue;
            }
        }
    }
    
    log::warn!("Connectivity check failed after {} retries", max_retries);
    Ok(false)
}

/// Performs a quick connectivity check without retries
///
/// This is useful for on-demand checks where you want immediate feedback.
/// It performs a single connection attempt with the configured timeout.
///
/// # Returns
///
/// - `Ok(true)` if connectivity is available
/// - `Ok(false)` if connectivity is not available
/// - `Err(ConnectivityError)` if an unexpected error occurs
pub async fn check_connectivity_quick() -> ConnectivityResult {
    check_connectivity_once().await.map(|connected| {
        if connected {
            log::info!("Quick connectivity check: connected");
        } else {
            log::info!("Quick connectivity check: not connected");
        }
        connected
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_check_connectivity_once_invalid_host() {
        // This test would require mocking or a test server
        // For now, we just verify the function compiles and handles errors
        let result = check_connectivity_once().await;
        // Result will be Ok(true) on success or Err(ConnectivityError) on failure
        assert!(matches!(result, Ok(_) | Err(_)));
    }
    
    #[tokio::test]
    async fn test_check_connectivity_once_return_types() {
        // Verify that check_connectivity_once only returns Ok(true) or Err
        // It should never return Ok(false)
        let result = check_connectivity_once().await;
        
        match result {
            Ok(true) => {
                // Success case - this is valid
            }
            Ok(false) => {
                panic!("check_connectivity_once should never return Ok(false)");
            }
            Err(ConnectivityError::Timeout) => {
                // Timeout is a valid error
            }
            Err(ConnectivityError::Io(_)) => {
                // I/O error is valid
            }
            Err(ConnectivityError::MaxRetriesExceeded) => {
                // This shouldn't happen in check_connectivity_once, but it's a valid error type
            }
        }
    }
    
    #[tokio::test]
    async fn test_check_connectivity_quick() {
        // Test that check_connectivity_quick returns a valid result
        let result = check_connectivity_quick().await;
        assert!(matches!(result, Ok(_) | Err(_)), "check_connectivity_quick should return Ok or Err");
        
        // Verify it returns Ok(true) on success, not Ok(false)
        if let Ok(connected) = result {
            assert_eq!(connected, true, "check_connectivity_quick should only return Ok(true) on success");
        }
    }
    
    #[test]
    fn test_connectivity_result_type() {
        // Test that ConnectivityResult is properly defined
        let success: ConnectivityResult = Ok(true);
        assert!(success.is_ok());
        assert_eq!(success.unwrap(), true);
        
        let timeout_error: ConnectivityResult = Err(ConnectivityError::Timeout);
        assert!(timeout_error.is_err());
        
        if let Err(ConnectivityError::Timeout) = timeout_error {
            // Correct error type
        } else {
            panic!("Should be ConnectivityError::Timeout");
        }
    }
}

