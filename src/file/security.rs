//! Security-focused file operations

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
/// * `path` - Path to validate
/// * `allow_symlinks` - Whether to allow symlinks
///
/// # Errors
///
/// Returns an error if the path is unsafe.
///
/// # Example
///
/// ```rust,no_run
/// use atlas_common::file::safe_file_path;
/// use std::path::Path;
///
/// let safe_path = safe_file_path(Path::new("data.txt"), false)?;
/// # Ok::<(), atlas_common::Error>(())
/// ```
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
/// Currently allows /tmp and /var/app/data directories.
fn is_safe_symlink_target(target: &Path) -> bool {
    if let Ok(canonical) = target.canonicalize() {
        canonical.starts_with("/tmp") || canonical.starts_with("/var/app/data")
    } else {
        false
    }
}

/// Safely opens a file for reading with security checks
///
/// # Arguments
///
/// * `path` - Path to the file
/// * `allow_symlinks` - Whether to allow symlinks
///
/// # Errors
///
/// Returns an error if the file cannot be opened safely.
///
/// # Example
///
/// ```rust,no_run
/// use atlas_common::file::safe_open_file;
/// use std::path::Path;
///
/// let file = safe_open_file(Path::new("data.txt"), false)?;
/// # Ok::<(), atlas_common::Error>(())
/// ```
pub fn safe_open_file(path: &Path, allow_symlinks: bool) -> Result<File> {
    let safe_path = safe_file_path(path, allow_symlinks)?;
    File::open(&safe_path).map_err(Error::from)
}

/// Safely creates a file for writing with security checks
///
/// # Arguments
///
/// * `path` - Path to the file
/// * `allow_symlinks` - Whether to allow symlinks
///
/// # Errors
///
/// Returns an error if the file cannot be created safely.
///
/// # Example
///
/// ```rust,no_run
/// use atlas_common::file::safe_create_file;
/// use std::path::Path;
///
/// let file = safe_create_file(Path::new("output.txt"), false)?;
/// # Ok::<(), atlas_common::Error>(())
/// ```
pub fn safe_create_file(path: &Path, allow_symlinks: bool) -> Result<File> {
    let safe_path = safe_file_path(path, allow_symlinks)?;
    File::create(&safe_path).map_err(Error::from)
}

/// Creates `OpenOptions` for a file with security validation
///
/// Returns configured `OpenOptions` that can be further customized
/// before opening the file.
///
/// # Arguments
///
/// * `path` - Path to the file
/// * `allow_symlinks` - Whether to allow symlinks
///
/// # Errors
///
/// Returns an error if the path is unsafe.
pub fn safe_open_options(path: &Path, allow_symlinks: bool) -> Result<OpenOptions> {
    let _safe_path = safe_file_path(path, allow_symlinks)?;
    Ok(OpenOptions::new())
}
