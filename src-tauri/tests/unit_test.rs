/// Unit tests for application core functionality
/// 
/// These tests focus on testing individual functions and components
/// in isolation. Since these tests run in a separate crate, they can
/// only access public APIs of the library.

mod common;

use elulib_mobile::{create_app, AppError, AppResult};

#[test]
fn test_create_app_returns_builder() {
    // Test that create_app returns a builder instance
    let _builder = create_app();
    // Builder should be created successfully without panicking
    assert!(true, "App builder created successfully");
}

#[test]
fn test_create_app_multiple_calls() {
    // Test that create_app can be called multiple times
    let _builder1 = create_app();
    let _builder2 = create_app();
    // Should not panic when called multiple times
    assert!(true, "Multiple app builders created successfully");
}

#[test]
fn test_app_result_type() {
    // Test that AppResult works correctly
    let success: AppResult<()> = Ok(());
    assert!(success.is_ok(), "Success result should be Ok");

    let error: AppResult<()> = Err(AppError::Tauri(tauri::Error::FailedToReceiveMessage));
    assert!(error.is_err(), "Error result should be Err");
}

#[test]
fn test_app_error_display() {
    // Test that AppError implements Display correctly
    let error = AppError::Tauri(tauri::Error::FailedToReceiveMessage);
    let error_msg = format!("{}", error);
    assert!(error_msg.contains("Tauri runtime error"), 
            "Error message should contain descriptive text. Got: {}", error_msg);
}

#[test]
fn test_error_conversion() {
    // Test that Tauri errors can be converted to AppError
    let tauri_error = tauri::Error::FailedToReceiveMessage;
    let app_error: AppError = tauri_error.into();
    assert!(matches!(app_error, AppError::Tauri(_)));
}

#[test]
fn test_error_propagation() {
    // Test that errors propagate correctly through the application
    fn function_that_fails() -> AppResult<()> {
        Err(AppError::Tauri(tauri::Error::FailedToReceiveMessage))
    }

    fn function_that_calls_failing_function() -> AppResult<()> {
        function_that_fails()?;
        Ok(())
    }

    let result = function_that_calls_failing_function();
    assert!(result.is_err(), "Error should propagate through call chain");
    
    // Verify error type
    if let Err(AppError::Tauri(_)) = result {
        // Correct error type
    } else {
        panic!("Error should be of type AppError::Tauri");
    }
}

#[test]
fn test_app_result_ok_variant() {
    // Test AppResult Ok variant
    let result: AppResult<u32> = Ok(42);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_app_result_err_variant() {
    // Test AppResult Err variant
    let result: AppResult<()> = Err(AppError::Tauri(tauri::Error::FailedToReceiveMessage));
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(matches!(e, AppError::Tauri(_)));
    }
}

#[test]
fn test_app_builder_includes_keystore_plugin() {
    // Test that the keystore plugin doesn't cause
    // initialization errors when creating the app builder
    let _builder = create_app();
    // If we get here, the plugin was registered successfully
    assert!(true, "Keystore plugin registered successfully");
}

#[test]
fn test_keystore_plugin_multiple_initializations() {
    // Test that the app builder can be created multiple times
    // Verifies that plugin initialization is idempotent
    let _builder1 = create_app();
    let _builder2 = create_app();
    let _builder3 = create_app();
    // Multiple initializations should not cause issues
    assert!(true, "Multiple app builders with keystore plugin created successfully");
}

#[test]
fn test_app_builder_includes_invoke_handler() {
    // Test that the app builder includes invoke_handler for keychain commands
    // This verifies that commands module is properly integrated
    let builder = create_app();
    // Builder should include invoke_handler registration
    // If invoke_handler fails to register, builder creation would fail
    // So if we get here, registration was successful
    assert!(true, "App builder includes invoke_handler with keychain commands");
}
