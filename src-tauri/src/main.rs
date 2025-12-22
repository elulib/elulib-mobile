//! Main entry point for the elulib-mobile application
//!
//! This binary entry point initializes and runs the Tauri mobile application.
//! It uses `run_with_error_handling()` to ensure proper error handling and
//! process termination on errors.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    elulib_mobile::run_with_error_handling()
}
