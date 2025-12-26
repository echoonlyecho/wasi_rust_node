// Import rust's io and filesystem module
use std::io::prelude::*;
use std::fs;

// Entry point to our WASI applications
fn main() {

  // Print out hello world!
  // This will handle writing to stdout for us using the WASI APIs (e.g fd_write)
  println!("Hello world!！！！");

  // Create a file
  // We are creating a `helloworld.txt` file in the `helloworld` directory
  // This is a local path for running with `cargo run`.
  // Ensure the `helloworld/` directory exists next to the Cargo.toml.
  fs::create_dir_all("helloworld").unwrap();
  let mut file = fs::File::create("helloworld/helloworld.txt").unwrap();

  // Write the text to the file we created
  write!(file, "Hello world\n").unwrap();
}
