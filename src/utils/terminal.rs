use dialoguer::Confirm;

/// Prompts the user with a confirmation message and exits the program if the
/// user declines.
///
/// # Arguments
///
/// * `message` - A string slice that holds the message to display in the
///   confirmation prompt.
///
/// # Behavior
///
/// This function uses the `dialoguer` crate to display a confirmation prompt to
/// the user. If the user confirms (or presses Enter to accept the default
/// `true`), the program continues. If the user declines, the program exits with
/// a status code of `0`.
///
/// # Panics
///
/// This function will panic if there is an error interacting with the terminal
/// (e.g., if the terminal input/output fails).
///
/// # Example
///
/// ```rust
/// use crate::utils::terminal::confirm_or_exit;
///
/// confirm_or_exit("Do you want to proceed?");
/// println!("Continuing execution...");
/// ```
#[allow(dead_code)]
pub fn confirm_or_exit(message: &str) {
    let confirmation = Confirm::new().with_prompt(message).default(true).interact().unwrap();
    if !confirmation {
        std::process::exit(0);
    }
}
