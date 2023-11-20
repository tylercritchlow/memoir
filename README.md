# memoir 📝

Memoir is a powerful and flexible logging library for Rust that makes it easy to capture, record, and analyze events in your code. With its intuitive interface and rich features, Memoir is the ideal tool for developers who want to gain deeper insights into their applications.


## Installation

You can use the package manager [`cargo`](https://github.com/rust-lang/cargo) to install memoir.

```bash
cargo install memoir
```

or add it to your current project by also using [`cargo`](https://github.com/rust-lang/cargo) to install memoir.
```bash
cargo add memoir
```

## Usage
More documentation available at [MemoirDocs](https://brodycritchlow.github.io/memoir-docs/log_lib/)
```rust
use memoir::*; // Import everything needed from memoir.

fn main() {
    let mut f: FileLogger = FileLogger { // Initialize our FileLogger, and make sure it is mut.
        filepath: "current_log.log".to_string(),
        whitelist: vec![LogLevel::Warning], // Filter what Logs you want to see.
        format: "[%d] %l - %m".to_string(), // Format of the outputted log.
    };
    f.warn("test".to_string()); // Output a warning log onto the filepath, if in whitelist.
    f.set_format("%d::%l - %m") // Sets a new format.
}
```

## Contributing

Pull requests are **welcome**. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
