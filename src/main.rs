use log_lib::*;

fn main() {
    let mut logger = FileLogger::new(
        "example_log.txt".to_string(),
        vec![LogLevel::Info, LogLevel::Error, LogLevel::Debug, LogLevel::Warning],
        String::from("[%d] [%l] %m"),
    );

    // Log messages outside of groups
    logger.info(String::from("Application started"));
    logger.error(String::from("Critical error"));

    // Start a new group for startup logs
    logger.start_group(Log {
        level: LogLevel::Info,
        message: String::from("Startup logs"),
    });

    // Log messages within the startup group
    logger.info(String::from("Initializing components"));
    logger.debug(String::from("Debugging startup process"));
    logger.error(String::from("Error during startup"));


    // End the startup group
    logger.end_group();

    // Start a new group for general use
    logger.start_group(Log {
        level: LogLevel::Info,
        message: String::from("General use logs"),
    });

    // Log messages within the general use group
    logger.info(String::from("Sub app initialized"));

    // Start a subgroup within the general use group for PDF viewer
    logger.start_group(Log {
        level: LogLevel::Info,
        message: String::from("PDF Viewer logs"),
    });

    // Log messages within the PDF viewer subgroup
    logger.info(String::from("{extension} initialized and is in effect"));

    // End the PDF viewer subgroup
    logger.end_group();

    // End the general use group
    logger.end_group();

    // Log messages outside of groups again
    logger.warn(String::from("A warning outside of groups"));
    logger.info(String::from("Application exiting"));

    // End any remaining groups
    logger.end_group();
}
