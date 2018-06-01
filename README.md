# Etc - Efficient Transfer Coding for Rust

[Etc in Go](https://github.com/superp00t/etc)

[Etc in JavaScript](https://github.com/superp00t/etc-js)

# Usage
```rust
extern crate etc_rs;

use etc_rs::buffer::Buffer;

fn main() {
  // Create buffer in memory
  let mut buf = Buffer::new();

  // Create buffer as an alias to a file
  let mut buf = Buffer::from_file("/tmp/testingData");

  // Write some data
  buf.write(&[1, 2, 3, 4, 5]);

  // Write signed 64-bit integer using LEB128 compression
  buf.write_i(12345678);
}
```