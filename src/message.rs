use console::style;
use std::fmt::Display;

/// Turns a plain String Slice into a red colored String
///
/// ## Params:
///
/// - **_message_** The message as Generic Type of `Display` trait
///
/// ## Returns:
///
/// a red colored String
///
/// ## Since:
///
/// v1.0.0
///
/// ## Usage:
///
/// - The return String doesn't contain a new line character
/// - To print out, use `print!`, `println!`, `format!`, `panic!` macros
///
/// ## Example:
///
/// ```rust
/// use best_skn_message::message;
///
/// let error: String = message::error("Error occurred!");
/// println!("{}", error);
/// ```
pub fn error<T>(message: T) -> String
where
  T: Display,
{
  format!("{} ❌", style(message).red())
}

/// Turns a plain String Slice into a green colored String
///
/// ## Params:
///
/// - **_message_** The message as Generic Type of `Display` trait
///
/// ## Returns:
///
/// a green colored String
///
/// ## Since:
///
/// v1.0.0
///
/// ## Usage:
///
/// - The return String doesn't contain a new line character
/// - To print out, use `print!`, `println!`, `format!`, `panic!` macros
///
/// ## Example:
///
/// ```rust
/// use best_skn_message::message;
///
/// let success: String = message::success("Process completed!");
/// println!("{}", success);
/// ```
pub fn success<T>(message: T) -> String
where
  T: Display,
{
  format!("{} ✅", style(message).green())
}

/// Turns a plain String Slice into a blue colored String
///
/// ## Params:
///
/// - **_message_** The message as Generic Type of `Display` trait
///
/// ## Returns:
///
/// a blue colored String
///
/// ## Since:
///
/// v1.0.0
///
/// ## Usage:
///
/// - The return String doesn't contain a new line character
/// - To print out, use `print!`, `println!`, `format!`, `panic!` macros
///
/// ## Example:
///
/// ```rust
/// use best_skn_message::message;
///
/// let info: String = message::info("Process is safe to pause!");
/// println!("{}", info);
/// ```
pub fn info<T>(message: T) -> String
where
  T: Display,
{
  format!("{} 📚", style(message).blue())
}

/// Gives a formatted static red colored action failure notification
///
/// ## Params:
///
/// - **_name_** Action name as Generic Type of `Display` trait
///
/// ## Returns:
///
/// a formatted static red colored String message
///
/// ## Since:
///
/// v1.0.0
///
/// ## Usage:
///
/// - The return String doesn't contain a new line character
/// - To print out, use `print!`, `println!`, `format!`, `panic!` macros
///
/// ## Example:
///
/// ```rust
/// use best_skn_message::message;
///
/// let failure: String = message::action_failure("Copy Files");
/// println!("{}", failure);
/// ```
pub fn action_failure<T>(name: T) -> String
where
  T: Display,
{
  format!("'{}' {}", name, error("Action Failed To Complete!"))
}

/// Gives a formatted static green colored action complete notification
///
/// ## Params:
///
/// - **_name_** Action name as Generic Type of `Display` trait
///
/// ## Returns:
///
/// a formatted static green colored String message
///
/// ## Since:
///
/// v1.0.0
///
/// ## Usage:
///
/// - The return String doesn't contain a new line character
/// - To print out, use `print!`, `println!`, `format!`, `panic!` macros
///
/// ## Example:
///
/// ```rust
/// use best_skn_message::message;
///
/// let complete: String = message::action_complete("Read Config File");
/// println!("{}", complete);
/// ```
pub fn action_complete<T>(name: T) -> String
where
  T: Display,
{
  format!("'{}' {}", name, success("Action Completed Successfully!"))
}

/// Gives a formatted static blue colored action info notification
///
/// ## Params:
///
/// - **_name_** Action name as Generic Type of `Display` trait
/// - **_notification_** The info notification message about the action as Generic Type of `Display` trait
///
/// ## Returns:
///
/// a formatted static blue colored String message
///
/// ## Since:
///
/// v1.0.0
///
/// ## Usage:
///
/// - The return String doesn't contain a new line character
/// - To print out, use `print!`, `println!`, `format!`, `panic!` macros
///
/// ## Example:
///
/// ```rust
/// use best_skn_message::message;
///
/// let notify: String = message::action_notify("Run Shell Scripts", "Safe to use without error!");
/// println!("{}", notify);
/// ```
pub fn action_notify<T, U>(name: T, notification: U) -> String
where
  T: Display,
  U: Display,
{
  format!("'{}': {}", name, info(notification))
}
