
**File I/O (Input/Output)** in Rust is performed using the `std::fs` module, 
which provides functions and types for interacting with the file system. Here's a breakdown of the key concepts:

## 1. Opening a File
Rust provides the File type from `std::fs` to open and manipulate files.
- Read a file: Use `File::open` to open a file in read-only mode.
```
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("example.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("File content: {}", content);
    Ok(())
}
```
