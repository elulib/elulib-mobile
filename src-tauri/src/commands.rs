/// Tauri commands for keychain operations
///
/// These commands allow the remote PHP frontend to interact with
/// the device keychain/keystore securely.
///
/// Note: The keystore plugin already provides commands, but we wrap them
/// here for easier access from remote frontends and better error handling.

use tauri::AppHandle;
use tauri_plugin_keystore::{KeystoreExt, StoreRequest, RetrieveRequest, RemoveRequest};

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
    
    // For mobile, StoreRequest only needs the value
    // The key will be used as identifier
    let request = StoreRequest {
        value: format!("{}:{}", key, value),
    };
    
    app.keystore().store(request)
        .map_err(|e| {
            log::error!("Failed to store value in keychain: {}", e);
            format!("Keychain store failed: {}", e)
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
    
    let request = RetrieveRequest {
        service: key.clone(),
        user: key,
    };
    
    let response = app.keystore().retrieve(request)
        .map_err(|e| {
            log::error!("Failed to retrieve value from keychain: {}", e);
            format!("Keychain retrieve failed: {}", e)
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
    
    let request = RemoveRequest {
        service: key.clone(),
        user: key,
    };
    
    app.keystore().remove(request)
        .map_err(|e| {
            log::error!("Failed to remove value from keychain: {}", e);
            format!("Keychain remove failed: {}", e)
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
