# Crate
In Rust, a crate is a package of Rust code that can include modules, libraries, or executables. Crates are the building blocks of a Rust project, and you can use them to share functionality across projects or include external libraries. Here's how to use a crate effectively:

## 1. Types of Crates
- **Binary Crates:** Build to an executable, usually containing a `main` function.
- **Library Crates:** Provide reusable code but don’t directly create an executable.

## 2. Using an External Crate
### Step 1: Add the Crate to Your Project
- Open the `Cargo.toml` file.
- Add the crate under `[dependencies]`. For example, to use the `rand` crate, add the following line:
```
[dependencies]
rand = "0.8"
```
this will download and compile the `rand` crate and make it available to your project.

### Step 2: Use the Crate in Your Code
- In your Rust code, you can now use the `rand` crate by importing it:
```
use rand::Rng;
```
- You can now use any of the functions and types provided by the `rand` crate. For example, to generate a random number between 0 and 1:
```
fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen();
    println!("Random number: {}", random_number);
}
```

