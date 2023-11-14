
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
    pub whitelist: Vec<LogLevel>
}

impl FileLogger {
    fn log(&self, log: Log) {
        if self.whitelist.contains(&log.level) {
            println!("continue")
        }
    }

    pub fn warn(&self, message: String) {
        self.log(Log {level: LogLevel::Warning, message});
    }

    pub fn error(&self, message: String) {
        self.log(Log {level: LogLevel::Error, message});
    }

    pub fn info(&self, message: String) {
        self.log(Log {level: LogLevel::Info, message});
    }

    pub fn debug(&self, message: String) {
        self.log(Log {level: LogLevel::Debug, message});
    }
}
`11`
