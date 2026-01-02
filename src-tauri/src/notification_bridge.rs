/// Notification bridge module
///
/// This module provides functionality to convert web notifications
/// from the remote frontend into native push notifications.

use tauri::AppHandle;
use crate::notifications;

/// Show a native notification
///
/// This command receives notification data from the frontend and displays
/// it as a native notification on iOS/Android using platform-specific APIs.
///
/// # Arguments
///
/// * `app` - The Tauri app handle
/// * `title` - Notification title
/// * `body` - Notification body text
/// * `icon` - Optional icon URL or path (used on Android)
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the operation fails.
#[tauri::command]
pub async fn show_notification(
    _app: AppHandle,
    title: String,
    body: String,
    icon: Option<String>,
) -> Result<(), String> {
    log::info!("Showing native notification: {} - {}", title, body);
    
    // Use platform-specific notification implementation
    notifications::show_notification(
        &title,
        &body,
        icon.as_deref(),
    )
}

/// Request notification permissions
///
/// On mobile platforms, notification permissions are requested from the system.
/// This command requests permission and returns the result.
///
/// # Returns
///
/// Returns `true` if permission is granted, `false` otherwise.
#[tauri::command]
pub async fn request_notification_permission(_app: AppHandle) -> Result<bool, String> {
    log::info!("Requesting notification permission");
    
    // Use platform-specific permission request
    notifications::request_permission()
}

/// Check if notifications are supported
///
/// # Returns
///
/// Returns `true` if notifications are supported on this platform.
#[tauri::command]
pub async fn is_notification_supported() -> Result<bool, String> {
    // Notifications are supported on both iOS and Android
    #[cfg(any(target_os = "ios", target_os = "android"))]
    {
        Ok(true)
    }
    
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        Ok(false)
    }
}

/// Check notification permission status
///
/// # Returns
///
/// Returns `true` if permission is granted, `false` otherwise.
#[tauri::command]
pub async fn check_notification_permission(_app: AppHandle) -> Result<bool, String> {
    log::info!("Checking notification permission status");
    
    // Use platform-specific permission check
    notifications::check_permission()
}

