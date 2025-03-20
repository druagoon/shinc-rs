/// Exits the process with a status code of 0 if the given condition is true.
///
/// # Arguments
///
/// * `r` - A boolean value. If `true`, the process will terminate with a
///   successful exit code (0).
///
/// # Example
///
/// ```rust
/// use crate::utils::process::quit_if;
///
/// let condition = true;
/// quit_if(condition); // This will terminate the program if `condition` is true.
/// ```
#[allow(dead_code)]
pub fn quit_if(r: bool) {
    if r {
        std::process::exit(0);
    }
}

#[allow(dead_code)]
pub fn quit() {
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "process didn't exit")]
    fn test_quit_if_does_not_exit() {
        quit_if(false);
        panic!("process didn't exit");
    }

    #[test]
    #[should_panic] // This test will fail because the process exits
    fn test_quit_if_exits() {
        quit_if(true);
    }
}
