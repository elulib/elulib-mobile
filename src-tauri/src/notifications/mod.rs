/// Platform-specific notification implementations
///
/// This module provides platform-specific notification functionality
/// for iOS and Android using native APIs.

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "android")]
mod android;

/// Show a native notification on the current platform
///
/// # Arguments
///
/// * `title` - Notification title
/// * `body` - Notification body text
/// * `icon` - Optional icon (path or resource name)
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error message if the operation fails.
pub fn show_notification(title: &str, body: &str, icon: Option<&str>) -> Result<(), String> {
    #[cfg(target_os = "ios")]
    {
        // Generate a unique identifier for the notification
        let identifier = format!("elulib_notification_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        ios::show_notification(title, body, Some(&identifier))
    }
    
    #[cfg(target_os = "android")]
    {
        // Android requires a notification channel
        // Use default channel or create one if needed
        const DEFAULT_CHANNEL_ID: &str = "elulib_default_channel";
        const DEFAULT_CHANNEL_NAME: &str = "élulib Notifications";
        const DEFAULT_CHANNEL_DESCRIPTION: &str = "Notifications from élulib app";
        
        // Ensure channel exists (idempotent operation)
        let _ = android::create_notification_channel(
            DEFAULT_CHANNEL_ID,
            DEFAULT_CHANNEL_NAME,
            DEFAULT_CHANNEL_DESCRIPTION,
        );
        
        android::show_notification(title, body, DEFAULT_CHANNEL_ID, icon)
    }
    
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        // Fallback for other platforms (should not happen in mobile app)
        let _ = (title, body, icon); // Suppress unused variable warnings
        log::warn!("Notifications not implemented for this platform");
        Err("Notifications not supported on this platform".to_string())
    }
}

/// Request notification permissions on the current platform
///
/// # Returns
///
/// Returns `true` if permission is granted, `false` otherwise.
pub fn request_permission() -> Result<bool, String> {
    #[cfg(target_os = "ios")]
    {
        ios::request_permission()
    }
    
    #[cfg(target_os = "android")]
    {
        android::request_permission()
    }
    
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        Ok(false)
    }
}

/// Check notification permission status on the current platform
///
/// # Returns
///
/// Returns `true` if permission is granted, `false` otherwise.
pub fn check_permission() -> Result<bool, String> {
    #[cfg(target_os = "ios")]
    {
        ios::check_permission()
    }
    
    #[cfg(target_os = "android")]
    {
        android::check_permission()
    }
    
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_show_notification_basic() {
        let result = show_notification("Test Title", "Test Body", None);
        // Should succeed on iOS/Android, fail on other platforms
        #[cfg(any(target_os = "ios", target_os = "android"))]
        {
            assert!(result.is_ok(), "show_notification should succeed on mobile platforms");
        }
    }
    
    #[test]
    fn test_request_permission_basic() {
        let result = request_permission();
        assert!(result.is_ok(), "request_permission should return Ok");
    }
    
    #[test]
    fn test_check_permission_basic() {
        let result = check_permission();
        assert!(result.is_ok(), "check_permission should return Ok");
    }
}
