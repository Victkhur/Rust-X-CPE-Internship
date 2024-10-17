# Day 2 Rust
Today I will be creating my first code, compiling and running the code. I also learn variables in rust 
# Creating a New Project Using Cargo 
1. Open your terminal.
2. Navigate to the directory where you want to create the new project.
3. Use the following command to create a new project:
```
cargo new project
```

## Navigate to the project folder
Move into the newly created project directory:
```
cd project
```
This opens the new project in visual studio code
```
>code .
```
Cargo already generates a "Hello, world!" program for you in `src/main.rs`, but you can open the file.
The default content will look like this:

```
fn main() {
    println!("Hello, world!");
}
```
## Running the Rust Program
To compile and run the project, use the following command:
```
cargo run
```
You should see the following output in your terminal:
```
   Compiling hello_world v0.1.0 (path-to-project/project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/hello_world`
Hello, world!
```
# Variable in Rust
In Rust, variables are declared using the `let` keyword, and by default, they are immutable. If you want a variable to be mutable (i.e., you want to change its value later), you must explicitly use the `mut` keyword. Here's a breakdown of how variables work in Rust:

### 1. Immutable Variable (Default)

By default, variables in Rust are immutable, meaning their value cannot be changed after they are assigned.
```
fn main() {
    let age = 25;
    println!("Your age is {}", age);
    
    // This will cause an error because age is immutable
    // age = 26;
}
```
### 2. Mutable Variable
To allow a variable to be changed (mutable), use the `mut` keyword:
```
fn main() {
    let mut age = 25;
    println!("Your age is: {}", age);
    
    age = 26; // Now you can change the value
    println!("Your next age will be {}", age);
}
```
Here, `age` is mutable, so you can change its value after it has been assigned.
