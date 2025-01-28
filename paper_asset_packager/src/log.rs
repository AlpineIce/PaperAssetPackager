use std::fmt::Display;

pub enum LogType {
    INFO,
    WARNING,
    ERROR
}

pub fn log<T: Display>(log_type: LogType, text: &T) {
    match log_type {
        LogType::INFO => println!("INFO: {}", text),
        LogType::WARNING => println!("WARNING: {}", text),
        LogType::ERROR => println!("ERROR: {}", text)
    };
}