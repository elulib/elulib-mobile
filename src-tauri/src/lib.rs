/// Application error type
///
/// This enum represents all possible errors that can occur in the application.
/// Errors are automatically converted from Tauri errors using the `From` trait.
///
/// # Future Improvements
///
/// Consider adding error variants for:
/// - Connectivity errors (`ConnectivityError`)
/// - Keychain validation errors
/// - Structured keychain operation errors
///
/// Note: Tauri commands currently return `Result<T, String>`, so errors are
/// converted to strings for serialization. To use structured errors in commands,
/// they would need to implement serialization (e.g., via `serde::Serialize`).
///
/// # Examples
///
/// ```rust,no_run
/// use elulib_mobile::AppError;
/// let error = AppError::Tauri(tauri::Error::FailedToReceiveMessage);
/// println!("Error: {}", error);
/// ```
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// Error originating from the Tauri framework
    #[error("Tauri runtime error: {0}")]
    Tauri(#[from] tauri::Error),
}

/// Result type for application operations
///
/// This is a convenience type alias for `Result<T, AppError>`, making error
/// handling more ergonomic throughout the application.
///
/// # Examples
///
/// ```rust,no_run
/// use elulib_mobile::AppResult;
/// fn my_function() -> AppResult<()> {
///     Ok(())
/// }
/// ```
pub type AppResult<T> = Result<T, AppError>;

/// Application commands module
pub mod commands;

/// Application constants module
pub mod constants;

/// Connectivity check module
pub mod connectivity;

/// Notification bridge module
pub mod notification_bridge;

/// Platform-specific notifications module
pub mod notifications;

/// Builds and returns a configured Tauri application builder
///
/// This function creates a Tauri application builder that can be
/// configured with plugins, commands, and setup logic before running.
///
/// The builder allows you to:
/// - Register Tauri commands
/// - Configure plugins
/// - Set up application state
/// - Configure window properties
///
/// Logging is automatically configured via `tauri-plugin-log` with:
/// - Standard output (stdout) for console logging
/// - Log directory for persistent file logging
/// - Webview console for in-app logging
///
/// Secure storage (keychain) is configured via `tauri-plugin-keystore`:
/// - iOS: Uses Keychain Services for secure data storage
/// - Android: Uses Android Keystore for secure data storage
///
/// # Returns
///
/// A `tauri::Builder` instance ready for configuration
///
/// # Examples
///
/// ```rust,no_run
/// use elulib_mobile::create_app;
/// let builder = create_app();
/// // Configure the builder as needed
/// ```
pub fn create_app() -> tauri::Builder<tauri::Wry> {
    use tauri_plugin_log::{Target, TargetKind};
    
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_keystore::init())
}

/// Runs the Tauri application
///
/// This is the main entry point for the Tauri application. It initializes
/// the app builder, sets up any required configuration, and starts the
/// Tauri runtime.
///
/// # Returns
///
/// - `Ok(())` if the application runs successfully
/// - `Err(AppError)` if an error occurs during initialization or runtime
///
/// # Errors
///
/// This function will return an error if:
/// - The Tauri runtime fails to initialize
/// - Application setup fails
/// - The runtime encounters an unrecoverable error
///
/// # Examples
///
/// ```rust,no_run
/// use elulib_mobile::run;
/// match run() {
///     Ok(()) => println!("Application exited successfully"),
///     Err(e) => eprintln!("Application error: {}", e),
/// }
/// ```
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> AppResult<()> {
    log::info!("Initializing Tauri application");
    
    create_app()
        .invoke_handler(tauri::generate_handler![
            commands::keychain_store,
            commands::keychain_retrieve,
            commands::keychain_remove,
            commands::keychain_exists,
            commands::check_connectivity,
            commands::check_connectivity_quick,
            notification_bridge::show_notification,
            notification_bridge::request_notification_permission,
            notification_bridge::check_notification_permission,
            notification_bridge::is_notification_supported,
        ])
        .setup(|_app| {
            log::debug!("Setting up application");
            
            // Application setup logic can go here
            // For example: initialize plugins, setup state, etc.
            #[cfg(debug_assertions)]
            {
                log::debug!("Debug mode enabled");
                // Enable devtools in debug mode if needed
                // app.handle().plugin(tauri_plugin_devtools::init())?;
            }
            
            // Note: For remote frontends, the notification bridge script should be
            // injected by the frontend itself or via a content script.
            // The JavaScript bridge file is available at src-tauri/notification-bridge.js
            // and should be loaded by the remote frontend or injected via Tauri's
            // content script mechanism if available.
            log::info!("Notification bridge module loaded - frontend should inject bridge script");
            
            // Perform connectivity check at startup (non-blocking)
            tauri::async_runtime::spawn(async move {
                log::info!("Starting background connectivity check...");
                match connectivity::check_connectivity().await {
                    Ok(true) => {
                        log::info!("Startup connectivity check: connected");
                    }
                    Ok(false) => {
                        log::warn!("Startup connectivity check: not connected");
                    }
                    Err(e) => {
                        log::error!("Startup connectivity check error: {}", e);
                    }
                }
            });
            
            log::info!("Application setup completed successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .map_err(|e| {
            log::error!("Tauri runtime error: {}", e);
            AppError::Tauri(e)
        })?;
    
    log::info!("Tauri application started successfully");
    Ok(())
}

/// Runs the application with error handling and logging
///
/// This is a convenience wrapper around `run()` that handles errors by:
/// - Logging error messages using the `log` crate
/// - Printing error messages to stderr (for environments without logger)
/// - Exiting the process with code 1 on failure
///
/// This function is intended to be called from `main()` as it will terminate
/// the process if an error occurs.
///
/// # Examples
///
/// ```rust,no_run
/// fn main() {
///     elulib_mobile::run_with_error_handling();
/// }
/// ```
pub fn run_with_error_handling() {
    use constants::exit;
    
    if let Err(e) = run() {
        // Log error using log crate (will work if logger is initialized)
        log::error!("Failed to run application: {}", e);
        // Also print to stderr for environments without logger
        eprintln!("Failed to run application: {}", e);
        std::process::exit(exit::FAILURE);
    }
}
