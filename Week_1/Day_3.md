# Day 3 on Rust
I will be learning and documenting the data types in rust
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
```
fn main() {
    let x: f64 = 2.5;  // 64-bit floating-point
    let y: f32 = 3.14; // 32-bit floating-point
}
```
### 3. Boolean Type
The `bool` type represents a value that can be either `true` or `false`.
```
fn main() {
    let t: bool = true;
    let f: bool = false;
}
```
### 4. Character Type
The `char` type represents a single Unicode scalar value, which can be a letter, number, symbol, or even emoji. Characters are specified with single quotes (`'`).
```
fn main() {
    let letter: char = 'A';
    let emoji: char = '😊';
}
```
### 5. Tuples
`Tuples` can group multiple values of different types into a single compound type. Tuples have a fixed size once defined.
```
fn main() {
    let tuple: (i32, f64, char) = (42, 6.4, 'A');
    let (x, y, z) = tuple; // Destructuring
    println!("x: {}, y: {}, z: {}", x, y, z);
    
    // Accessing tuple elements by index
    println!("First value: {}", tuple.0);
}
```

### 6. Arrays
`Arrays` are collections of multiple values of the same type. `Arrays` in Rust have a fixed size.
```
fn main() {
    let array: [i32; 3] = [1, 2, 3]; // Array of 3 elements, all i32
    println!("First element: {}", array[0]);
    
    // Arrays can also be initialized with the same value
    let repeated_array = [3; 5]; // Same as [3, 3, 3, 3, 3]
}
```
