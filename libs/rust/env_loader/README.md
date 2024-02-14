# Env Loader

The `Env Loader` is a Rust library designed to load environment variables from a `.env` file into your application's environment. It provides a simple, yet effective way of managing application configurations outside your codebase, making it easier to adjust settings without the need to recompile.

## Features

- Load environment variables from a specified `.env` file.
- Custom error handling to manage file not found scenarios gracefully.

## Requirements

This library requires the [`dotenv`](https://crates.io/crates/dotenv) crate to parse `.env` files. Ensure you have it in your `Cargo.toml`:

```toml
[dependencies]
dotenv = "0.15.0"
```
`Note: Check for the latest version of dotenv to keep your application up-to-date.`

## Usage
To utilize the Env Loader, incorporate it into your project and invoke the load function with the path to your .env file.

## Basic Example
```rust
// Copy code
use env_loader::env_loader::load;

fn main() {
    match load("./.env") {
        Ok(_) => println!("Environment variables loaded successfully."),
        Err(e) => println!("Error loading .env file: {:?}", e),
    }
}
```
## Error Handling
The Env Loader defines a custom error type, EnvLoaderError, to distinguish different errors that may occur during the loading process. Currently, it includes the following variant:

`FileNotFound:` Indicates the .env file was not found at the specified path.
Testing
This library includes tests to verify its functionality. Run these tests using Cargo to ensure everything works as expected:

```shell
# Copy code
cargo test
```
## Contributing
Contributions to the Env Loader are warmly welcomed. Feel free to submit pull requests or open issues to suggest improvements or discuss potential features.

