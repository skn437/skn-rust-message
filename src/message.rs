use console::style;

pub fn error(message: &str) -> String {
  format!("{} ❌ \n", style(message).red())
}

pub fn success(message: &str) -> String {
  format!("{} ✅ \n", style(message).green())
}

pub fn info(message: &str) -> String {
  format!("{} 📚 \n", style(message).blue())
}

pub fn action_failure(name: &str) {
  print!("'{}' {} \n", name, error("Action Failed To Complete!"));
}

pub fn action_complete(name: &str) {
  print!(
    "'{}' {} \n",
    name,
    success("Action Completed Successfully!")
  );
}

pub fn action_notify(name: &str, notification: &str) {
  print!("'{}': {} \n", name, info(notification));
}
