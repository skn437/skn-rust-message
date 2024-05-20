use console::style;

/// Turns a plain String Slice into a red colored String
///
/// ## Params:
///
/// - **_message_** The message as String Slice (&str)
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
/// - Don't use `println!` as the return String has already a new line
/// - To print out, use `print!`, `format!`, `panic!` macros
///
/// ## Example:
///
/// ```rust
/// use best_skn_message::message;
///
/// let error: String = message::error("Error occured!");
/// print!("{}", error);
/// ```
pub fn error(message: &str) -> String {
  format!("{} ❌ \n", style(message).red())
}

/// Turns a plain String Slice into a green colored String
///
/// ## Params:
///
/// - **_message_** The message as String Slice (&str)
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
/// - Don't use `println!` as the return String has already a new line
/// - To print out, use `print!`, `format!`, `panic!` macros
///
/// ## Example:
///
/// ```rust
/// use best_skn_message::message;
///
/// let success: String = message::success("Process completed!");
/// print!("{}", success);
/// ```
pub fn success(message: &str) -> String {
  format!("{} ✅ \n", style(message).green())
}

/// Turns a plain String Slice into a blue colored String
///
/// ## Params:
///
/// - **_message_** The message as String Slice (&str)
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
/// - Don't use `println!` as the return String has already a new line
/// - To print out, use `print!`, `format!`, `panic!` macros
///
/// ## Example:
///
/// ```rust
/// use best_skn_message::message;
///
/// let info: String = message::info("Process is safe to pause!");
/// print!("{}", info);
/// ```
pub fn info(message: &str) -> String {
  format!("{} 📚 \n", style(message).blue())
}

/// Gives a formatted static red colored action failure notification
///
/// ## Params:
///
/// - **_name_** Action name as String Slice (&str)
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
/// - Don't use `println!` as the return String has already a new line
/// - To print out, use `print!`, `format!`, `panic!` macros
///
/// ## Example:
///
/// ```rust
/// use best_skn_message::message;
///
/// let failure: String = message::action_failure("Copy Files");
/// print!("{}", failure);
/// ```
pub fn action_failure(name: &str) -> String {
  format!("'{}' {}", name, error("Action Failed To Complete!"))
}

/// Gives a formatted static green colored action complete notification
///
/// ## Params:
///
/// - **_name_** Action name as String Slice (&str)
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
/// - Don't use `println!` as the return String has already a new line
/// - To print out, use `print!`, `format!`, `panic!` macros
///
/// ## Example:
///
/// ```rust
/// use best_skn_message::message;
///
/// let complete: String = message::action_complete("Read Config File");
/// print!("{}", complete);
/// ```
pub fn action_complete(name: &str) -> String {
  format!("'{}' {}", name, success("Action Completed Successfully!"))
}

/// Gives a formatted static blue colored action info notification
///
/// ## Params:
///
/// - **_name_** Action name as String Slice (&str)
/// - **_notification_** The info notification message about the action as String Slice (&str)
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
/// - Don't use `println!` as the return String has already a new line
/// - To print out, use `print!`, `format!`, `panic!` macros
///
/// ## Example:
///
/// ```rust
/// use best_skn_message::message;
///
/// let notify: String = message::action_notify("Run Shell Scripts", "Safe to use without error!");
/// print!("{}", notify);
/// ```
pub fn action_notify(name: &str, notification: &str) -> String {
  format!("'{}': {}", name, info(notification))
}
