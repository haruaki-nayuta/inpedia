use colored::Colorize;
use serde::Serialize;
use serde_json::json;

/// Print a success value. In JSON mode, serialize as `{"ok": <value>}`.
pub fn print_ok<T: Serialize + std::fmt::Display>(value: &T, json: bool) {
    if json {
        println!("{}", json!({"ok": value}));
    } else {
        println!("{} {}", "✓".green(), value);
    }
}

/// Print a structured JSON result (always pretty in text mode, compact in json mode).
pub fn print_data<T: Serialize>(label: &str, value: &T, json: bool) {
    if json {
        println!("{}", serde_json::to_string(value).unwrap_or_default());
    } else {
        println!("{}", label.cyan());
        println!("{}", serde_json::to_string_pretty(value).unwrap_or_default().dimmed());
    }
}

/// Print an error. In JSON mode, serialize as `{"error": "..."}` to stdout.
/// In text mode, print to stderr in red.
pub fn print_error(msg: &str, json: bool) {
    if json {
        println!("{}", json!({"error": msg}));
    } else {
        eprintln!("{} {}", "✗".red(), msg.red());
    }
}

/// Print a simple message (text mode only; JSON mode suppresses it).
pub fn print_info(msg: &str, json: bool) {
    if !json {
        eprintln!("{}", msg.dimmed());
    }
}
