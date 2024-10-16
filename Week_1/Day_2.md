# Creating a New Project Using Cargo 
1. Open your terminal.
2. Navigate to the directory where you want to create the new project.
3. Use the following command to create a new project:
```
cargo new project
```
This creates a new folder called `project` with the following structure:
```
project/
├── Cargo.toml
└── src/
    └── main.rs
```
- `Cargo.toml:` This is the configuration file for the project, containing information about dependencies, project metadata, etc.

- `src/main.rs:` This is the main Rust source file where your code will be written.


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
