use log_lib::{FileLogger, LogParser, LogLevel};

fn main() {
    let mut f: FileLogger = FileLogger {
        filepath: "test.txt".to_string(),
        whitelist: vec![LogLevel::Warning],
        format: "[%d] %l - %m".to_string(),
    };
    f.warn("test".to_string());

    let x = LogParser {
        filepath: "test.txt".to_string();
    };
    x.parse_logs();
}