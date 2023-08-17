//! Functions used to load and write JSON to a file.

use std::{fs, io, path::Path};

/// Loads JSON from a file to a string.
///
/// # Arguments
///
/// * `path` - The file path.
///
/// # Examples
///
/// ```rust,ignore
/// use std::path::Path;
/// use json_keyquotes_convert::{load_write_utils};
///
/// let path = Path::new("./test_resources/Test_with_keyquotes.json");
/// let json: String = load_write_utils::load_json(&path).expect("Couldn't load from file!");
/// ```
pub fn load_json(path: &Path) -> Result<String, io::Error> {
    match fs::read_to_string(path) {
        Ok(val) => return Ok(val),
        Err(err) => return Err(err),
    };
}

/// Writes JSON from a string to a file.
///
/// # Arguments
///
/// * `path` - The file path.
/// * `json` - The JSON string to write.
///
/// # Examples
///
/// ```rust,ignore
/// use std::path::Path;
/// use json_keyquotes_convert::{load_write_utils};
///
/// let path = Path::new("./test_resources/Test_with_keyquotes.json");
/// load_write_utils::write_json(&path, &json).expect("Couldn't write to file!");
/// ```
pub fn write_json(path: &Path, json: &str) -> Result<(), io::Error> {
    match fs::write(path, json) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    }
}
