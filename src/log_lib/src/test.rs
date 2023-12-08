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

