//! Secure file operation utilities
//!
//! This module provides safe file operations that protect against common
//! security issues like symlink attacks and hard link manipulation.
//!
//! # Security Features
//!
//! - Symlink attack prevention
//! - Hard link detection
//! - Path validation
//! - Safe file creation and opening
//!
//! # Example
//!
//! ```rust,no_run
//! use atlas_common::file::{safe_create_file, safe_open_file};
//! use std::io::{Read, Write};
//! use std::path::Path;
//!
//! // Safely create a file
//! let mut file = safe_create_file(Path::new("output.txt"), false)?;
//! file.write_all(b"secure data")?;
//!
//! // Safely read a file
//! let mut file = safe_open_file(Path::new("input.txt"), false)?;
//! let mut contents = String::new();
//! file.read_to_string(&mut contents)?;
//! # Ok::<(), atlas_common::Error>(())
//! ```

mod security;

pub use security::{safe_create_file, safe_file_path, safe_open_file, safe_open_options};
