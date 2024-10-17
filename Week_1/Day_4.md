# Day 4 On Rust
I will be learning and documenting about **Function** on Rust
# Function 
Function in Rust is a fundamental building block, allowing you to define reusable blocks of code. 
Rust functions can accept parameters, return values, and have strong type-checking at compile time.

## Basic Function Syntax
Functions in Rust are defined using the `fn` keyword, followed by the function name, parameters, and a return type (if any). 
Here's the basic syntax:
```
fn function_name(parameter1: Type1, parameter2: Type2) -> ReturnType {
    // Function body
}
```
- **Function name:** By convention, function names in Rust are snake_case.
- **Parameters:** Each parameter must have an explicitly defined type.
- **Return type:** Specified after the -> arrow. If the function doesn't return anything, this is omitted.

##  Example of a Function
```
fn main() {
    println!("Hello, world!"); // Built-in function to print to the console
    greet(); // Calling the custom function
}

fn greet() {
    println!("Hello from the greet function!");
}
```
## Example of a Funcion With Parameters
You can pass values into functions through parameters. Each parameter requires a type declaration.
```
fn main() {
    print_sum(5, 3); // Call the function with arguments
}

fn print_sum(x: i32, y: i32) {
    println!("The sum of {} and {} is {}", x, y, x + y);
}
```
