// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let msg : String = match level {
        LogLevel::Info    => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error   => error(message),
    };
    return msg;
}
pub fn info(message: &str) -> String {
    let mut msg = "[INFO]: ".to_owned();
    msg.push_str(message);
    return msg
}
pub fn warn(message: &str) -> String {
    let mut msg = "[WARNING]: ".to_owned();
    msg.push_str(message);
    return msg
}
pub fn error(message: &str) -> String {
    let mut msg = "[ERROR]: ".to_owned();
    msg.push_str(message);
    return msg
}
