//! # Vars
//!
//! Wrapper for std::env lib

use std::env;

/// Get the cmd ( cargo run [cmd] [args...])
///
/// # Examples
///
/// ```
/// use grey::vars;
///
/// let cmd: String = vars::get_cmd();
/// ```
pub fn get_cmd() -> String {
    match env::args().nth(1) {
        Some(arg) => arg,
        None => String::new(),
    }
}
/// Get the args ( cargo run [cmd] [args...])
///
/// # Examples
///
/// ```
/// use grey::vars;
///
/// let args: Vec<String> = vars::get_args();
/// ```
pub fn get_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        args.remove(0); // remove the default
        return args;
    }
    args.remove(0); // remove the default
    args.remove(0); // remove the cmd
    args
}
