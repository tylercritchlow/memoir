use log_lib::{FileLogger, LogLevel, Log};

fn main() {
    let F: FileLogger = FileLogger {
        filepath: "test.txt".to_string(),
        whitelist: vec![LogLevel::Debug]
    };
    let LOG_TEST: Log = Log {
        level: LogLevel::Debug,
        message: "test".to_string(),
    };
    F.log(LOG_TEST);
}