use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use chrono;


#[derive(Debug, PartialEq, Clone)]
pub enum LogLevel {
    Info,
    Debug,
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Warning => write!(f, "Warning"),
            LogLevel::Error => write!(f, "Error")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Log {
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct Group {
    // pub cur_group: Option<Box<Group>>,
    pub logs: Vec<Log>,
}

impl Group {
    pub fn new() -> Self {
        Group {
            logs: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct FileLogger {
    pub filepath: String,
    pub whitelist: Vec<LogLevel>,
    pub format: String,
    pub group: Vec<Option<Group>>,
    pub(crate) indent: usize,
}

impl FileLogger {
    pub fn new(filepath: String, whitelist: Vec<LogLevel>, format: String) -> Self {
        Self { 
            filepath, 
            whitelist, 
            format,
            indent: 0,
            group: vec![None], 
        }
    }
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
            
            let mut group = &mut self.group;
            let final_idx = group.clone().len() - 1;

            if group[final_idx].is_some() {
                let l = log.clone();
                let filling = if log.message.contains("|--") { "" } else { "|--" };
                let enum_value_length = log.level.to_string().len();
                let spaces = if enum_value_length > 4 { enum_value_length - 4 } else { 4 };
                
                group[final_idx].as_mut().unwrap().logs.push(Log {
                    level: log.level,
                    message: format!("{}{} {}", " ".repeat(self.indent * spaces), filling, log.message),
                });
                self.indent = 1;
            } else {
                if let Err(e) = writeln!(file.expect(""), "{}", conformed_message) {
                    eprintln!("Couldn't write to file {}", e);
                }
            }
        }
    }

    pub fn start_group(&mut self, log: Log) {
        // Start a new group with the given log
        self.group.append(&mut vec![Some(Group::new())]);
        println!("{:?}", self.group);
        self.log(log);
    }

    pub fn end_group(&mut self) {
        // If there is a current group, print its logs and reset the group
        let group = self.group.pop().unwrap();
        if group.is_some() {
            for log in group.clone().unwrap().logs {
                self.log(log.clone());
                println!("{:?} [{:?}]", log, group)
            }
        }
        self.indent = 0;
    }

    pub fn set_format(&mut self, format: String) {
        self.format = format
    }

    pub fn warn(&mut self, message: String) {
        self.log(Log {
            level: LogLevel::Warning,
            message,
        });
    }

    pub fn error(&mut self, message: String) {
        self.log(Log {
            level: LogLevel::Error,
            message,
        });
    }

    pub fn info(&mut self, message: String) {
        self.log(Log {
            level: LogLevel::Info,
            message,
        });
    }

    pub fn debug(&mut self, message: String) {
        self.log(Log {
            level: LogLevel::Debug,
            message,
        });
    }
}