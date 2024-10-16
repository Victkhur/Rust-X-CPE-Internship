# Data Type in Rust
Every value in Rust has a specific data type known at compile time. 
This helps prevent many common programming errors and makes Rust both fast and reliable.

Some Basic Data Types in Rust:
### 1. Integer Types
Integer types represent whole numbers without fractional components.
Rust supports both signed and unsigned integers of varying sizes.
- Signed integers (`i8`, `i16`, `i32`, etc.) can hold both positive and negative values.
- Unsigned integers (`u8`, `u16`, `u32`, etc.) can only hold positive values.
```
fn main() {
    let a: i32 = -10; // 32-bit signed integer
    let b: u32 = 42;  // 32-bit unsigned integer
}
```
### 2. Floating-Point Types
Floating-point types represent numbers with fractional components. Rust supports two floating-point types:

- `f32`: 32-bit floating-point.
- `f64`: 64-bit floating-point (default).
