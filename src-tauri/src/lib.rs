
/// Application error type
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Tauri runtime error: {0}")]
    Tauri(#[from] tauri::Error),
}

/// Result type for application operations
pub type AppResult<T> = Result<T, AppError>;

/// Builds and returns a configured Tauri application builder
/// 
/// This function creates a Tauri application builder that can be
/// configured with plugins, commands, and setup logic before running.
pub fn create_app() -> tauri::Builder<tauri::Wry> {
    tauri::Builder::default()
}

/// Runs the Tauri application
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
pub fn run_with_error_handling() {
    if let Err(e) = run() {
        eprintln!("Failed to run application: {}", e);
        std::process::exit(1);
    }
}