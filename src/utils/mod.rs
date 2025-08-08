//! Utility functions

mod file;
mod validation;

pub use file::{safe_file_path, safe_open_file, safe_create_file};
pub use validation::{
    validate_manifest_id, 
    validate_manifest_hash,
    ensure_c2pa_urn,
    extract_uuid_from_urn,
};