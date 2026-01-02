/// Tauri commands for keychain operations
///
/// These commands allow the remote PHP frontend to interact with
/// the device keychain/keystore securely.
///
/// Note: The keystore plugin already provides commands, but we wrap them
/// here for easier access from remote frontends and better error handling.

use tauri::AppHandle;
use tauri_plugin_keystore::{KeystoreExt, StoreRequest, RetrieveRequest, RemoveRequest};

use crate::constants::helpers;
use crate::connectivity;

/// Store a value in the keychain
///
/// # Arguments
///
/// * `app` - The Tauri app handle
/// * `key` - The key to store the value under (used as both service and username)
/// * `value` - The value to store securely
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the operation fails.
#[tauri::command]
pub async fn keychain_store(app: AppHandle, key: String, value: String) -> Result<(), String> {
    log::info!("Storing value in keychain for key: {}", key);
    
    // Validate input lengths
    helpers::validate_keychain_key(&key)
        .map_err(|e| {
            log::warn!("Keychain store validation failed for key: {}", e);
            e
        })?;
    helpers::validate_keychain_value(&value)
        .map_err(|e| {
            log::warn!("Keychain store validation failed for value: {}", e);
            e
        })?;
    
    // For mobile, StoreRequest only needs the value
    // The key will be used as identifier
    let request = StoreRequest {
        value: helpers::key_value_pair(&key, &value),
    };
    
    app.keystore().store(request)
        .map_err(|e| {
            log::error!("Failed to store value in keychain: {}", e);
            helpers::keychain_store_error(&e)
        })?;
    log::info!("Successfully stored value for key: {}", key);
    Ok(())
}

/// Retrieve a value from the keychain
///
/// # Arguments
///
/// * `app` - The Tauri app handle
/// * `key` - The key to retrieve the value for (used as both service and username)
///
/// # Returns
///
/// Returns the stored value as a String, or an error if the key doesn't exist
/// or the operation fails.
#[tauri::command]
pub async fn keychain_retrieve(app: AppHandle, key: String) -> Result<String, String> {
    log::info!("Retrieving value from keychain for key: {}", key);
    
    // Validate input length
    helpers::validate_keychain_key(&key)
        .map_err(|e| {
            log::warn!("Keychain retrieve validation failed for key: {}", e);
            e
        })?;
    
    // Clone is necessary: RetrieveRequest requires owned Strings for both service and user fields
    // We use the same key for both fields, so we clone for service and move key into user
    let request = RetrieveRequest {
        service: key.clone(),
        user: key,
    };
    
    let response = app.keystore().retrieve(request)
        .map_err(|e| {
            log::error!("Failed to retrieve value from keychain: {}", e);
            helpers::keychain_retrieve_error(&e)
        })?;
    
    log::info!("Successfully retrieved value for key");
    Ok(response.value.unwrap_or_default())
}

/// Remove a value from the keychain
///
/// # Arguments
///
/// * `app` - The Tauri app handle
/// * `key` - The key to remove from the keychain (used as both service and username)
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the operation fails.
#[tauri::command]
pub async fn keychain_remove(app: AppHandle, key: String) -> Result<(), String> {
    log::info!("Removing value from keychain for key: {}", key);
    
    // Validate input length
    helpers::validate_keychain_key(&key)
        .map_err(|e| {
            log::warn!("Keychain remove validation failed for key: {}", e);
            e
        })?;
    
    // Clone is necessary: RemoveRequest requires owned Strings for both service and user fields
    // We use the same key for both fields, so we clone for service and move key into user
    let request = RemoveRequest {
        service: key.clone(),
        user: key,
    };
    
    app.keystore().remove(request)
        .map_err(|e| {
            log::error!("Failed to remove value from keychain: {}", e);
            helpers::keychain_remove_error(&e)
        })?;
    log::info!("Successfully removed value for key");
    Ok(())
}

/// Check if a key exists in the keychain
///
/// # Arguments
///
/// * `app` - The Tauri app handle
/// * `key` - The key to check (used as both service and username)
///
/// # Returns
///
/// Returns `true` if the key exists, `false` otherwise.
#[tauri::command]
pub async fn keychain_exists(app: AppHandle, key: String) -> Result<bool, String> {
    log::debug!("Checking if key exists in keychain: {}", key);
    
    // Validate input length
    helpers::validate_keychain_key(&key)
        .map_err(|e| {
            log::warn!("Keychain exists validation failed for key: {}", e);
            e
        })?;
    
    // Clone is necessary: RetrieveRequest requires owned Strings for both service and user fields
    // We use the same key for both fields, so we clone for service and move key into user
    let request = RetrieveRequest {
        service: key.clone(),
        user: key,
    };
    
    match app.keystore().retrieve(request) {
        Ok(_) => {
            log::debug!("Key exists in keychain");
            Ok(true)
        }
        Err(_) => {
            log::debug!("Key does not exist in keychain");
            Ok(false)
        }
    }
}

/// Check connectivity to the application server
///
/// This command performs a connectivity check with retry logic and exponential backoff.
/// It attempts to establish a TCP connection to the configured server.
///
/// # Returns
///
/// Returns `true` if connectivity is available, `false` otherwise.
/// Returns an error string if an unexpected error occurs.
///
/// # Examples
///
/// ```javascript
/// const isConnected = await invoke('check_connectivity');
/// if (isConnected) {
///   console.log('Connected to server');
/// }
/// ```
#[tauri::command]
pub async fn check_connectivity() -> Result<bool, String> {
    log::info!("Connectivity check requested via command");
    
    connectivity::check_connectivity()
        .await
        .map_err(|e| {
            let error_msg = format!("Connectivity check failed: {}", e);
            log::error!("{}", error_msg);
            error_msg
        })
}

/// Perform a quick connectivity check without retries
///
/// This command performs a single connectivity check attempt without retry logic.
/// It's faster than `check_connectivity` but less reliable.
///
/// # Returns
///
/// Returns `true` if connectivity is available, `false` otherwise.
/// Returns an error string if an unexpected error occurs.
///
/// # Examples
///
/// ```javascript
/// const isConnected = await invoke('check_connectivity_quick');
/// ```
#[tauri::command]
pub async fn check_connectivity_quick() -> Result<bool, String> {
    log::info!("Quick connectivity check requested via command");
    
    connectivity::check_connectivity_quick()
        .await
        .map_err(|e| {
            let error_msg = format!("Quick connectivity check failed: {}", e);
            log::error!("{}", error_msg);
            error_msg
        })
}
