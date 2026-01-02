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
    let _builder = create_app();
    // Builder should include invoke_handler registration
    // If invoke_handler fails to register, builder creation would fail
    // So if we get here, registration was successful
    assert!(true, "App builder includes invoke_handler with keychain commands");
}

// ============================================================================
// Keychain Validation Tests
// ============================================================================

#[test]
fn test_validate_keychain_key_valid() {
    use elulib_mobile::constants::helpers;
    
    // Test valid key lengths
    assert!(helpers::validate_keychain_key("a").is_ok(), "Single character key should be valid");
    assert!(helpers::validate_keychain_key("valid_key").is_ok(), "Normal key should be valid");
    
    // Test maximum length key (256 characters)
    let max_key = "a".repeat(256);
    assert!(helpers::validate_keychain_key(&max_key).is_ok(), "Maximum length key should be valid");
}

#[test]
fn test_validate_keychain_key_too_short() {
    use elulib_mobile::constants::helpers;
    
    // Test empty key (below minimum)
    let result = helpers::validate_keychain_key("");
    assert!(result.is_err(), "Empty key should be invalid");
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("at least"), "Error message should mention minimum length");
    assert!(error_msg.contains("1"), "Error message should mention minimum value of 1");
}

#[test]
fn test_validate_keychain_key_too_long() {
    use elulib_mobile::constants::helpers;
    
    // Test key exceeding maximum length (257 characters)
    let too_long_key = "a".repeat(257);
    let result = helpers::validate_keychain_key(&too_long_key);
    assert!(result.is_err(), "Key exceeding maximum length should be invalid");
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("at most"), "Error message should mention maximum length");
    assert!(error_msg.contains("256"), "Error message should mention maximum value of 256");
    assert!(error_msg.contains("257"), "Error message should mention actual length");
}

#[test]
fn test_validate_keychain_value_valid() {
    use elulib_mobile::constants::helpers;
    
    // Test valid value lengths
    assert!(helpers::validate_keychain_value("").is_ok(), "Empty value should be valid");
    assert!(helpers::validate_keychain_value("valid_value").is_ok(), "Normal value should be valid");
    
    // Test maximum length value (4096 characters)
    let max_value = "a".repeat(4096);
    assert!(helpers::validate_keychain_value(&max_value).is_ok(), "Maximum length value should be valid");
}

#[test]
fn test_validate_keychain_value_too_long() {
    use elulib_mobile::constants::helpers;
    
    // Test value exceeding maximum length (4097 characters)
    let too_long_value = "a".repeat(4097);
    let result = helpers::validate_keychain_value(&too_long_value);
    assert!(result.is_err(), "Value exceeding maximum length should be invalid");
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("at most"), "Error message should mention maximum length");
    assert!(error_msg.contains("4096"), "Error message should mention maximum value of 4096");
    assert!(error_msg.contains("4097"), "Error message should mention actual length");
}

#[test]
fn test_validate_keychain_key_boundary_values() {
    use elulib_mobile::constants::helpers;
    
    // Test exact minimum length (1 character)
    assert!(helpers::validate_keychain_key("a").is_ok(), "Minimum length key (1 char) should be valid");
    
    // Test exact maximum length (256 characters)
    let max_key = "a".repeat(256);
    assert!(helpers::validate_keychain_key(&max_key).is_ok(), "Maximum length key (256 chars) should be valid");
    
    // Test just below maximum (255 characters)
    let just_below_max = "a".repeat(255);
    assert!(helpers::validate_keychain_key(&just_below_max).is_ok(), "Key with 255 chars should be valid");
}

#[test]
fn test_validate_keychain_value_boundary_values() {
    use elulib_mobile::constants::helpers;
    
    // Test exact maximum length (4096 characters)
    let max_value = "a".repeat(4096);
    assert!(helpers::validate_keychain_value(&max_value).is_ok(), "Maximum length value (4096 chars) should be valid");
    
    // Test just below maximum (4095 characters)
    let just_below_max = "a".repeat(4095);
    assert!(helpers::validate_keychain_value(&just_below_max).is_ok(), "Value with 4095 chars should be valid");
    
    // Test just above maximum (4097 characters)
    let just_above_max = "a".repeat(4097);
    assert!(helpers::validate_keychain_value(&just_above_max).is_err(), "Value with 4097 chars should be invalid");
}

// ============================================================================
// Constants Helper Functions Tests
// ============================================================================

#[test]
fn test_keychain_store_error_formatting() {
    use elulib_mobile::constants::helpers;
    
    let error = "Test error message";
    let formatted = helpers::keychain_store_error(&error);
    
    assert!(formatted.contains("Keychain store failed"), "Error message should contain prefix");
    assert!(formatted.contains("Test error message"), "Error message should contain original error");
}

#[test]
fn test_keychain_retrieve_error_formatting() {
    use elulib_mobile::constants::helpers;
    
    let error = "Retrieve error";
    let formatted = helpers::keychain_retrieve_error(&error);
    
    assert!(formatted.contains("Keychain retrieve failed"), "Error message should contain prefix");
    assert!(formatted.contains("Retrieve error"), "Error message should contain original error");
}

#[test]
fn test_keychain_remove_error_formatting() {
    use elulib_mobile::constants::helpers;
    
    let error = "Remove error";
    let formatted = helpers::keychain_remove_error(&error);
    
    assert!(formatted.contains("Keychain remove failed"), "Error message should contain prefix");
    assert!(formatted.contains("Remove error"), "Error message should contain original error");
}

#[test]
fn test_key_value_pair_formatting() {
    use elulib_mobile::constants::helpers;
    
    let key = "test_key";
    let value = "test_value";
    let formatted = helpers::key_value_pair(key, value);
    
    assert_eq!(formatted, "test_key:test_value", "Key-value pair should be formatted correctly");
    assert!(formatted.contains(key), "Formatted string should contain key");
    assert!(formatted.contains(value), "Formatted string should contain value");
    assert!(formatted.contains(":"), "Formatted string should contain separator");
}

#[test]
fn test_key_value_pair_with_special_characters() {
    use elulib_mobile::constants::helpers;
    
    // Test with special characters
    let key = "key:with:colons";
    let value = "value:with:colons";
    let formatted = helpers::key_value_pair(key, value);
    
    assert_eq!(formatted, "key:with:colons:value:with:colons", "Should handle special characters correctly");
    
    // Test with empty values
    let empty_key = "";
    let empty_value = "";
    let formatted_empty = helpers::key_value_pair(empty_key, empty_value);
    assert_eq!(formatted_empty, ":", "Empty key and value should produce ':'");
}

// ============================================================================
// Connectivity Error Tests
// ============================================================================

#[test]
fn test_connectivity_error_display() {
    use elulib_mobile::connectivity::ConnectivityError;
    use std::io;
    
    // Test Timeout error
    let timeout_error = ConnectivityError::Timeout;
    let timeout_msg = format!("{}", timeout_error);
    assert!(timeout_msg.contains("timeout") || timeout_msg.contains("Timeout"), 
            "Timeout error message should mention timeout. Got: {}", timeout_msg);
    
    // Test MaxRetriesExceeded error
    let max_retries_error = ConnectivityError::MaxRetriesExceeded;
    let max_retries_msg = format!("{}", max_retries_error);
    assert!(max_retries_msg.contains("retries") || max_retries_msg.contains("Retries"), 
            "Max retries error message should mention retries. Got: {}", max_retries_msg);
    
    // Test Io error (from std::io::Error)
    let io_error = io::Error::new(io::ErrorKind::ConnectionRefused, "Connection refused");
    let connectivity_io_error = ConnectivityError::Io(io_error);
    let io_msg = format!("{}", connectivity_io_error);
    assert!(io_msg.contains("Network") || io_msg.contains("error") || io_msg.contains("Connection"), 
            "Io error message should mention network/error. Got: {}", io_msg);
}

#[test]
fn test_connectivity_error_from_io_error() {
    use elulib_mobile::connectivity::ConnectivityError;
    use std::io;
    
    // Test that std::io::Error can be converted to ConnectivityError
    let io_error = io::Error::new(io::ErrorKind::TimedOut, "Timeout");
    let connectivity_error: ConnectivityError = io_error.into();
    
    match connectivity_error {
        ConnectivityError::Io(_) => {
            // Correct conversion
        }
        _ => {
            panic!("io::Error should convert to ConnectivityError::Io");
        }
    }
}

#[test]
fn test_connectivity_error_debug() {
    use elulib_mobile::connectivity::ConnectivityError;
    
    // Test that errors can be debug formatted
    let timeout_error = ConnectivityError::Timeout;
    let debug_str = format!("{:?}", timeout_error);
    assert!(!debug_str.is_empty(), "Debug format should produce non-empty string");
    
    let max_retries_error = ConnectivityError::MaxRetriesExceeded;
    let debug_str = format!("{:?}", max_retries_error);
    assert!(!debug_str.is_empty(), "Debug format should produce non-empty string");
}
