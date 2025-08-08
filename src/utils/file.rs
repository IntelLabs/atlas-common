//! File system utilities with security features
//! 
//! This module provides safe file operations that protect against common
//! security issues like symlink attacks and hard link manipulation.

use crate::error::{Error, Result};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

/// Validates that a file path is safe to use
/// 
/// This function performs security checks on file paths to prevent:
/// - Unauthorized symlink traversal
/// - Hard link attacks
/// - Access to restricted directories
/// 
/// # Arguments
/// 
/// * `path` - The path to validate
/// * `allow_symlinks` - Whether to allow symlinks (with additional validation)
/// 
/// # Returns
/// 
/// A safe `PathBuf` if the path passes validation, or an error describing the security issue
/// 
/// # Security Checks
/// 
/// 1. **Symlink validation**: If symlinks are not allowed, fails on any symlink.
///    If allowed, validates the target is in a safe location.
/// 2. **Hard link detection**: On Unix systems, detects files with multiple hard links
/// 3. **Target validation**: Ensures symlink targets are in allowed directories
/// 
/// # Examples
/// 
/// ```no_run
/// use atlas_core::utils::safe_file_path;
/// use std::path::Path;
/// 
/// // Check a regular file
/// let path = Path::new("/tmp/data.txt");
/// let safe_path = safe_file_path(&path, false)?;
/// assert_eq!(safe_path, path);
/// 
/// // Allow symlinks with validation
/// let symlink = Path::new("/tmp/link_to_data.txt");
/// let target = safe_file_path(&symlink, true)?;
/// // Returns the actual target path if it's in an allowed location
/// 
/// # Ok::<(), atlas_core::Error>(())
/// ```
/// 
/// # Platform-specific behavior
/// 
/// - **Unix/Linux**: Full symlink and hard link detection
/// - **Windows**: Symlink detection only (hard link detection not implemented)
pub fn safe_file_path(path: &Path, allow_symlinks: bool) -> Result<PathBuf> {
    if path.exists() {
        // Check for symlinks
        if path.is_symlink() {
            if !allow_symlinks {
                return Err(Error::Validation(format!(
                    "Path {} is a symlink, which is not allowed",
                    path.display()
                )));
            }

            // Resolve the symlink target
            let target = std::fs::read_link(path)?;
            
            // Validate the target is in an allowed location
            if !is_safe_symlink_target(&target) {
                return Err(Error::Validation(format!(
                    "Symlink target {} is not in an allowed location",
                    target.display()
                )));
            }

            return Ok(target);
        }

        // Check for hard links on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let metadata = std::fs::metadata(path)?;
            if metadata.nlink() > 1 {
                return Err(Error::Validation(format!(
                    "Path {} has multiple hard links ({}), which may be a security risk",
                    path.display(),
                    metadata.nlink()
                )));
            }
        }
    }

    Ok(path.to_path_buf())
}

/// Validates that a symlink target is in an allowed location
/// 
/// # Arguments
/// 
/// * `target` - The symlink target path to validate
/// 
/// # Returns
/// 
/// `true` if the target is in an allowed location, `false` otherwise
/// 
/// # Allowed Locations
/// 
/// Currently allows:
/// - `/tmp` - Temporary files
/// - `/var/app/data` - Application data directory
/// 
/// You can customize this function to match your security requirements.
fn is_safe_symlink_target(target: &Path) -> bool {
    if let Ok(canonical) = target.canonicalize() {
        canonical.starts_with("/tmp") || canonical.starts_with("/var/app/data")
    } else {
        false
    }
}

/// Safely opens a file for reading with security checks
/// 
/// This function validates the file path before opening to prevent
/// security issues like symlink attacks.
/// 
/// # Arguments
/// 
/// * `path` - Path to the file to open
/// * `allow_symlinks` - Whether to allow opening symlinked files
/// 
/// # Returns
/// 
/// An open `File` handle for reading, or an error if validation fails
/// 
/// # Examples
/// 
/// ```no_run
/// use atlas_core::utils::safe_open_file;
/// use std::path::Path;
/// use std::io::Read;
/// 
/// let path = Path::new("/tmp/data.txt");
/// let mut file = safe_open_file(&path, false)?;
/// 
/// let mut contents = String::new();
/// file.read_to_string(&mut contents)?;
/// println!("File contents: {}", contents);
/// 
/// # Ok::<(), atlas_core::Error>(())
/// ```
/// 
/// # Security
/// 
/// This function will fail if:
/// - The path is a symlink and `allow_symlinks` is false
/// - The path is a symlink to a restricted location
/// - The file has multiple hard links (Unix only)
/// - The file doesn't exist or can't be opened
pub fn safe_open_file(path: &Path, allow_symlinks: bool) -> Result<File> {
    let safe_path = safe_file_path(path, allow_symlinks)?;
    File::open(&safe_path).map_err(Error::from)
}

/// Safely creates a file for writing with security checks
/// 
/// This function validates the file path before creating/truncating
/// to prevent security issues. If the file exists, it will be truncated.
/// 
/// # Arguments
/// 
/// * `path` - Path where the file should be created
/// * `allow_symlinks` - Whether to allow creating files through symlinks
/// 
/// # Returns
/// 
/// An open `File` handle for writing, or an error if validation fails
/// 
/// # Examples
/// 
/// ```no_run
/// use atlas_core::utils::safe_create_file;
/// use std::path::Path;
/// use std::io::Write;
/// 
/// let path = Path::new("/tmp/output.txt");
/// let mut file = safe_create_file(&path, false)?;
/// 
/// file.write_all(b"Hello, World!")?;
/// println!("File created successfully");
/// 
/// # Ok::<(), atlas_core::Error>(())
/// ```
/// 
/// # Security
/// 
/// This function will fail if:
/// - The path is a symlink and `allow_symlinks` is false
/// - The path is a symlink to a restricted location
/// - An existing file has multiple hard links (Unix only)
/// - The file can't be created due to permissions or disk issues
/// 
/// # Warning
/// 
/// This function will **truncate** existing files. Use `safe_open_options`
/// for more control over file creation behavior.
pub fn safe_create_file(path: &Path, allow_symlinks: bool) -> Result<File> {
    let safe_path = safe_file_path(path, allow_symlinks)?;
    File::create(&safe_path).map_err(Error::from)
}

/// Creates `OpenOptions` for a file with security validation
/// 
/// This function validates the file path and returns `OpenOptions` that
/// can be configured before opening the file. This provides more control
/// than `safe_open_file` or `safe_create_file`.
/// 
/// # Arguments
/// 
/// * `path` - Path to the file
/// * `allow_symlinks` - Whether to allow operations on symlinked files
/// 
/// # Returns
/// 
/// An `OpenOptions` instance that can be further configured
/// 
/// # Examples
/// 
/// ```no_run
/// use atlas_core::utils::safe_open_options;
/// use std::path::Path;
/// use std::io::Write;
/// 
/// let path = Path::new("/tmp/append.txt");
/// 
/// // Create options and configure for appending
/// let mut options = safe_open_options(&path, false)?;
/// let mut file = options
///     .create(true)
///     .append(true)
///     .open(&path)?;
/// 
/// file.write_all(b"Appended text\n")?;
/// 
/// # Ok::<(), atlas_core::Error>(())
/// ```
/// 
/// # Advanced Usage
/// 
/// ```no_run
/// use atlas_core::utils::safe_open_options;
/// use std::path::Path;
/// 
/// let path = Path::new("/tmp/custom.txt");
/// 
/// // Configure for read-write without truncation
/// let mut options = safe_open_options(&path, false)?;
/// let file = options
///     .read(true)
///     .write(true)
///     .create(true)
///     .truncate(false)
///     .open(&path)?;
/// 
/// # Ok::<(), atlas_core::Error>(())
/// ```
pub fn safe_open_options(path: &Path, allow_symlinks: bool) -> Result<OpenOptions> {
    let _safe_path = safe_file_path(path, allow_symlinks)?;
    Ok(OpenOptions::new())
}