/// Application error type
///
/// This enum represents all possible errors that can occur in the application.
/// Errors are automatically converted from Tauri errors using the `From` trait.
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
    tauri::Builder::default()
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
    create_app()
        .setup(|_app| {
            // Application setup logic can go here
            // For example: initialize plugins, setup state, etc.
            #[cfg(debug_assertions)]
            {
                // Enable devtools in debug mode if needed
                // app.handle().plugin(tauri_plugin_devtools::init())?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .map_err(AppError::Tauri)?;
    
    Ok(())
}

/// Runs the application with error handling and logging
///
/// This is a convenience wrapper around `run()` that handles errors by:
/// - Printing error messages to stderr
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
    if let Err(e) = run() {
        eprintln!("Failed to run application: {}", e);
        std::process::exit(1);
    }
}