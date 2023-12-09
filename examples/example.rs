use memoir_logger::logging_utility::{FileLogger, LogLevel}; // Import everything needed from memoir.

fn main() {
    let mut log: FileLogger = FileLogger { // Initialize our FileLogger, and make sure it is mut.
        filepath: "current_log.log".to_string(),
        whitelist: vec![LogLevel::Warning, LogLevel::Info], // Filter what Logs you want to see.
        format: "[%d] %l - %m".to_string(), // Format of the outputted log.
    };
    log.warn("test".to_string()); // Output a warning log onto the filepath, if in whitelist.
    log.set_format("[%l]- %m".to_string()); // Sets a new format.
    log.debug("test".to_string()); // Output an info log in a different format.
}
