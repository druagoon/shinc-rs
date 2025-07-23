use std::fs::{self, File};
use std::io::{self, BufRead};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// Reads the lines of a file and returns an iterator over the lines.
///
/// # Arguments
///
/// * `path` - A reference to a type that implements `AsRef<Path>`, representing
///   the path to the file.
///
/// # Returns
///
/// This function returns a `Result` containing an iterator over the lines of
/// the file, wrapped in an `io::Lines<io::BufReader<fs::File>>`.
///
/// # Errors
///
/// This function will return an error if the file cannot be opened.
///
/// # Examples
///
/// ```
/// use crate::utils::fs::read_lines;
///
/// if let Ok(lines) = read_lines("foo.txt") {
///     for line in lines {
///         if let Ok(content) = line {
///             println!("{}", content);
///         }
///     }
/// }
/// ```
#[allow(dead_code)]
pub fn read_lines<P: AsRef<Path>>(path: P) -> anyhow::Result<io::Lines<io::BufReader<fs::File>>> {
    let fp = File::open(path)?;
    Ok(io::BufReader::new(fp).lines())
}

/// Sets the executable bit on a file.
///
/// # Arguments
///
/// * `path` - A reference to a type that implements `AsRef<Path>`, representing
///   the path to the file.
///
/// # Returns
///
/// This function returns `Ok(())` if the operation was successful.
///
/// # Platform-specific
///
/// This function is a no-op on Windows.
///
/// # Examples
///
/// ```
/// use crate::utils::fs::set_executable;
///
/// set_executable("foo")?;
/// ```
#[cfg(unix)]
#[allow(dead_code)]
pub fn set_executable<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;
    Ok(())
}

#[cfg(not(unix))]
pub fn set_executable<P: AsRef<Path>>(_path: P) -> anyhow::Result<()> {
    Ok(())
}

#[allow(dead_code)]
pub fn ensure_dir_all<P: AsRef<Path>>(p: P) -> anyhow::Result<()> {
    let path = p.as_ref();
    if !path.is_dir() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn ensure_file_dir_all<P: AsRef<Path>>(p: P) -> anyhow::Result<()> {
    let path = p.as_ref();
    if let Some(dir) = path.parent() {
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub fn create_file<P: AsRef<Path>>(p: P) -> anyhow::Result<std::fs::File> {
    ensure_file_dir_all(&p)?;
    Ok(fs::File::create(&p)?)
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::path::PathBuf;

    use super::*;

    fn setup_temp_file(content: &str) -> PathBuf {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_file.txt");
        let mut file = File::create(&temp_file).unwrap();
        writeln!(file, "{content}").unwrap();
        temp_file
    }

    #[test]
    fn test_read_lines() {
        let temp_file = setup_temp_file("line1\nline2\nline3");
        let lines = read_lines(&temp_file).unwrap();
        let content: Vec<_> = lines.map(|line| line.unwrap()).collect();
        assert_eq!(content, vec!["line1", "line2", "line3"]);
        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_set_executable() {
        let temp_file = setup_temp_file("");
        set_executable(&temp_file).unwrap();
        #[cfg(unix)]
        {
            let metadata = fs::metadata(&temp_file).unwrap();
            let permissions = metadata.permissions();
            assert_eq!(permissions.mode() & 0o111, 0o111);
        }
        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_ensure_dir_all() {
        let temp_dir = std::env::temp_dir().join("test_dir");
        ensure_dir_all(&temp_dir).unwrap();
        assert!(temp_dir.is_dir());
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_ensure_file_dir_all() {
        let temp_dir = std::env::temp_dir().join("test_dir");
        let temp_file = temp_dir.join("test_file.txt");
        ensure_file_dir_all(&temp_file).unwrap();
        assert!(temp_dir.is_dir());
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_create_file() {
        let temp_dir = std::env::temp_dir().join("test_dir");
        let temp_file = temp_dir.join("test_file.txt");
        let file = create_file(&temp_file).unwrap();
        assert!(temp_file.exists());
        drop(file);
        fs::remove_dir_all(temp_dir).unwrap();
    }
}
