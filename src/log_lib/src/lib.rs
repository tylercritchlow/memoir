
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
    pub fn log(&self, log: Log) {
        if self.whitelist.contains(&log.level) {
            println!("continue")
        }
        fn NewFunction {

        }
    }
}

