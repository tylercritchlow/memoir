use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono;

#[derive(Debug, PartialEq)]
pub enum LogLevel {
    Info,
    Debug,
    Warning,
    Error,
}

#[derive(Debug)]
pub struct Log {
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug)]
pub struct FileLogger {
    pub filepath: String, 
    pub whitelist: Vec<LogLevel>,
    pub format: String,
}

impl FileLogger {
    fn log(&mut self, log: Log) {
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
                    '%' => { is_format_char = true; }
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
                            conformed_message.push_str(&*log.message);
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

    pub fn warn(&mut self, message: String) {
        self.log(Log {level: LogLevel::Warning, message});
    }

    pub fn error(&mut self, message: String) {
        self.log(Log {level: LogLevel::Error, message});
    }

    pub fn info(&mut self, message: String) {
        self.log(Log {level: LogLevel::Info, message});
    }

    pub fn debug(&mut self, message: String) {
        self.log(Log {level: LogLevel::Debug, message});
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_file_creation() {
        let mut logger = FileLogger {
            filepath: String::from("test_file_creation.log"),
            whitelist: vec![LogLevel::Info, LogLevel::Debug],
            format: String::from("%d [%l] %m"),
        };

        logger.set_format("%d [%l] %m".to_string());
        logger.info("Test log".to_string());

        assert!(fs::metadata("test_file_creation.log").is_ok());
        fs::remove_file("test_file_creation.log").expect("Unable to remove test file");
    }

    #[test]
    fn test_file_logging() {
        let mut logger = FileLogger {
            filepath: String::from("test_file_logging.log"),
            whitelist: vec![LogLevel::Info, LogLevel::Debug],
            format: String::from("%d [%l] %m"),
        };

        logger.info("Test log 1".to_string());
        logger.debug("Test log 2".to_string());

        let content = fs::read_to_string("test_file_logging.log").expect("Unable to read test file");
        assert!(content.contains("Test log 1"));
        assert!(content.contains("Test log 2"));

        fs::remove_file("test_file_logging.log").expect("Unable to remove test file");
    }

    #[test]
    fn test_file_format() {
        let mut logger = FileLogger {
            filepath: String::from("test_file_format.log"),
            whitelist: vec![LogLevel::Info, LogLevel::Debug],
            format: String::from("%d [%l] %m"),
        };

        logger.info("Test log".to_string());

        let content = fs::read_to_string("test_file_format.log").expect("Unable to read test file");
        assert!(content.contains("[Info] Test log"));

        fs::remove_file("test_file_format.log").expect("Unable to remove test file");
    }
}
