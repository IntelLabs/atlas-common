//! Secure file operation utilities
//!
//! This module provides safe file operations that protect against common
//! security issues like symlink attacks and hard link manipulation.

mod security;

pub use security::{safe_create_file, safe_file_path, safe_open_file, safe_open_options};
