use std::fs;
use std::path::{Path, PathBuf};

/// Expands a tilde (`~`) in the given string to the user's home directory.
///
/// # Arguments
///
/// * `s` - A string slice that may contain a tilde (`~`) representing the
///   user's home directory.
///
/// # Returns
///
/// A `PathBuf` with the tilde expanded to the full path of the user's home
/// directory.
///
/// # Examples
///
/// ```
/// let path = expand_tilde("~/.config");
/// assert!(path.starts_with(std::env::home_dir().unwrap()));
/// ```
#[allow(unused)]
pub fn expand_tilde(s: &str) -> PathBuf {
    let p = shellexpand::tilde(s);
    PathBuf::from(p.as_ref())
}

/// Ensures that the specified directory exists, creating it if necessary.
///
/// # Arguments
///
/// * `p` - A path reference to the directory that should be ensured to exist.
///
/// # Returns
///
/// This function returns an `anyhow::Result<()>`. If the directory already
/// exists or is successfully created, it returns `Ok(())`. If an error occurs
/// while attempting to create the directory, it returns an error.
///
/// # Errors
///
/// This function will return an error if the directory cannot be created due to
/// insufficient permissions, invalid path, or other I/O-related issues.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use shinc_rs::utils::path::ensure_dir;
///
/// let dir_path = Path::new("/tmp/example_dir");
/// ensure_dir(dir_path).expect("Failed to ensure directory exists");
/// ```
#[allow(dead_code)]
pub fn ensure_dir<P: AsRef<Path>>(p: P) -> anyhow::Result<()> {
    let path = p.as_ref();
    if !path.is_dir() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn add_extension<P: AsRef<Path>>(p: P, extension: &str) -> anyhow::Result<PathBuf> {
    let path = p.as_ref();
    let filename = path
        .file_name()
        .ok_or(anyhow::format_err!("Failed to get file name from path: {}", path.display()))?;
    let new_filename = format!("{}.{}", filename.to_string_lossy(), extension);
    let mut new_path = path.to_path_buf();
    new_path.set_file_name(new_filename);
    Ok(new_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let home_dir = dirs::home_dir().expect("Home directory not found");
        let input = "~/.config";
        let expanded = expand_tilde(input);
        assert!(expanded.starts_with(home_dir));
        assert!(expanded.ends_with(".config"));
    }

    #[test]
    fn test_expand_tilde_no_tilde() {
        let input = "/usr/local/bin";
        let expanded = expand_tilde(input);
        assert_eq!(expanded, PathBuf::from(input));
    }

    #[test]
    fn test_ensure_dir_creates_directory() {
        let temp_dir = std::env::temp_dir().join("test_ensure_dir_creates_directory");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        ensure_dir(&temp_dir).expect("Failed to create directory");
        assert!(temp_dir.is_dir());
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_ensure_dir_already_exists() {
        let temp_dir = std::env::temp_dir().join("test_ensure_dir_already_exists");
        fs::create_dir_all(&temp_dir).unwrap();
        ensure_dir(&temp_dir).expect("Failed to ensure directory exists");
        assert!(temp_dir.is_dir());
        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
