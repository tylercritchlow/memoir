use chrono;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
pub struct Log<T> {
    pub level: LogLevel,
    pub message: T,
}

#[derive(Debug)]
pub struct FileLogger {
    pub filepath: String,
    pub whitelist: Vec<LogLevel>,
    pub format: String,
}

#[derive(Debug, PartialEq)]
pub enum LogLevel {
    Info,
    Debug,
    Warning,
    Error,
}

impl<T: std::fmt::Display> FileLogger {
    fn log(&mut self, log: Log<T>) {
        if self.whitelist.contains(&log.level) {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(self.filepath.clone());

            let mut conformed_message = "".to_string();
            let mut is_format_char = false;

            for char in self.format.chars() {
                match char {
                    '%' => {
                        is_format_char = true;
                    }
                    'd' => {
                        if is_format_char {
                            conformed_message.push_str(&*format!("{}", chrono::offset::Utc::now()));
                        } else {
                            conformed_message.push_str("d");
                            is_format_char = false;
                        }
                    }
                    'l' => {
                        if is_format_char {
                            conformed_message.push_str(&*format!("{:?}", log.level));
                        } else {
                            conformed_message.push_str("l");
                            is_format_char = false;
                        }
                    }
                    'm' => {
                        if is_format_char {
                            conformed_message.push_str(&*format!("{}", log.message));
                        } else {
                            conformed_message.push_str("m");
                            is_format_char = false;
                        }
                    }
                    _ => {
                        conformed_message.push_str(&*char.to_string());
                        is_format_char = false;
                    }
                }
            }

            if let Err(e) = writeln!(file.expect(""), "{}", conformed_message) {
                eprintln!("Couldn't write to file {}", e)
            }
        }
    }

    pub fn set_format(&mut self, format: String) {
        self.format = format
    }

    pub fn warn<T: Into<String>>(&mut self, message: T) {
        self.log(Log {
            level: LogLevel::Warning,
            message: message.into(),
        });
    }

    pub fn error<T: Into<String>>(&mut self, message: T) {
        self.log(Log {
            level: LogLevel::Error,
            message: message.into(),
        });
    }

    pub fn info<T: Into<String>>(&mut self, message: T) {
        self.log(Log {
            level: LogLevel::Info,
            message: message.into(),
        });
    }

    pub fn debug<T: Into<String>>(&mut self, message: T) {
        self.log(Log {
            level: LogLevel::Debug,
            message: message.into(),
        });
    }
}
