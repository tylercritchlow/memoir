use std::fs;

mod startlog {
    use std::fs::File;

    pub fn startlog(filename: &str) {
        // Validate the filename
        if !filename.is_empty() && filename.chars().all(|c| c.is_ascii()) {
            // Create the file
            match File::create(filename) {
                Ok(_) => println!("Log file '{}' created successfully.", filename),
                Err(e) => println!("Error creating log file: {}", e),
            }
        } else {
            println!("Invalid filename provided. Please use a valid filename.");
        }
    }
}

    #[cfg(test)]
    mod tests {
        use std::fs;
        use crate::startlog::startlog;

        fn test_startlog_valid_filename() {
            let filename = "my_log.txt";
            startlog(filename);

            assert!(fs::metadata(filename).is_ok());
        }

        fn test_startlog_empty_filename() {
            let filename = "";
            startlog(filename);

            assert!(!fs::metadata(filename).is_ok());
        }

        fn test_startlog_invalid_filename() {
            let filename = "!@#$%^&*";
            startlog(filename);

            assert!(!fs::metadata(filename).is_ok());
        }

        fn main() {
            test_startlog_valid_filename();
            test_startlog_empty_filename();
            test_startlog_invalid_filename();
        }

    }
