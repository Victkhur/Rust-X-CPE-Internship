# Ownership
Ownership in Rust defines who is responsible for managing the memory of a value (or resource). 
Every value in Rust has a single "owner," and when the owner goes out of scope, the value is automatically dropped (deallocated). 
Rust enforces strict ownership rules to ensure memory safety and avoid data races.

**Ownership Rules:**
- Each value in Rust has a single owner: Only one variable or entity can "own" a value at a time.
- When the owner goes out of scope, the value is dropped: Rust automatically frees the memory when the owner leaves the scope, preventing memory leaks.
- Ownership can be transferred: Ownership can be transferred (moved) to another variable, 
but once ownership is transferred, the original owner can no longer access the value.

**Example of Ownership:**
```
fn main() {
    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1;                    // ownership is moved to s2, s1 is no longer valid

    // println!("{}", s1);  // This would cause a compile-time error because s1 no longer owns the data.
    println!("{}", s2);      // s2 now owns the data
}
```
- In this example, ownership of the `String` is moved from `s1` to `s2`. 
Once `s2` takes ownership, `s1` is no longer valid, and trying to use it would result in a compile-time error.

# Borrowing
Borrowing allows you to access a value without taking ownership of it. When you borrow a value, you do not move the ownership; instead, you create a reference to the original owner’s value. There are two types of borrowing:

1. **Immutable borrowing** (`&T`): You can have many immutable references to a value.
2. **Mutable borrowing** (`&mut T`): Only one mutable reference can exist at a time, and there can't be any immutable references to the same value when a mutable reference exists.

**Example of Borrowing:**
**Immutable Borrowing:**
```
fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);  // borrow s1 (immutable)
    
    println!("The length of '{}' is {}.", s1, len); // s1 is still accessible here
}

fn calculate_length(s: &String) -> usize {
    s.len()  // since s is a reference, it does not own the value
}
```
**Mutable Borrowing:**
```
fn main() {
    let mut s1 = String::from("hello");

    change(&mut s1);  // mutable borrow

    println!("{}", s1);  // s1 can still be used here after the mutable borrow
}

fn change(s: &mut String) {
    s.push_str(", world");  // modifies the original string
}
```

# Lifetime
A lifetime represents the scope during which a reference is valid. Every reference in Rust has a lifetime, which can either be implicit or explicitly specified. The Rust compiler uses lifetimes to keep track of how long references are valid and to ensure that they don’t outlive the data they refer to.

## Lifetime Annotations
When references have lifetimes that depend on one another, Rust may require lifetime annotations. Lifetime annotations are not about changing how long something lives; they are about telling the compiler how the lifetimes of references relate to each other.

**Example Without Lifetime Annotations**

For simple cases, the Rust compiler can infer the lifetimes without requiring explicit annotations. For example:
```
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
Here, `x` and `y` are references to &str, but Rust can infer that both references must be valid for the same lifetime. This is due to Rust's lifetime elision rules, which infer common cases automatically.

**Example Requiring Lifetime Annotations**

Sometimes Rust can't infer lifetimes, especially when dealing with multiple references or returning references. In such cases, we need to use lifetime annotations.
```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
- In this case, the function `longest` accepts two string slices (`x` and `y`), both with the lifetime `'a`.
- The lifetime annotation `'a` tells Rust that the returned reference will be valid as long as both input references are valid.
