# Day Five on Rust
I will be learning and documenting Conditional and Loops on Rust.
# Conditional
In Rust, conditional statements allow you to control the flow of your program based on certain conditions. 
The main conditional structure in Rust is the `if` statement, which works similarly to conditionals in other programming languages. 
Rust also supports `else if` and `else` clauses to handle multiple conditions.
```
fn main() {
    let age = 33;

    if age >40 {
        print!("Age is greater than 40");
    } else if age == 40 {
        print!("Age is equal to 40");
    } else {
        print!("Age is less than 40");
    }

}
```
# Loops
In Rust, there are three main types of loops that allow you to repeat code: `loop`, `while`, and `for`.
1. **Loop**: Infinite Loop
The `loop` keyword creates an infinite loop.
It keeps running until you explicitly stop it, usually by using the `break` keyword to exit the loop.
```
fn main() {
    let mut count =0;
    loop {
        count += 1;
        print!("Count: {}", count);
    
    if count == 5 {
    break;
    }
}
}
```
- `break` is used to exit the loop.
- If you do not use `break`, the loop will continue indefinitely.

2. **while Loop**
The `while` loop runs as long as a specified condition is true.
```
fn main() {
    let mut count = 1;

    while count <= 10 {
        print!("Count: {}", count);
        count += 1;
    }
}
```
The condition is checked before each iteration, so if the condition is false at the beginning, the loop won't run at all.

3. **for Loop**
The `for` loop is used to iterate over a range or a collection, and it is the most commonly used loop in Rust.


You can use the `for` loop to iterate over a range of numbers, which is defined using `start..end` syntax. 
Note that the range is inclusive of the start and exclusive of the end.
```
fn main() {
    for count in 1..5{
        print!("count is: {}", count);
    }
}
```
- If you want the range to be inclusive of the end value, you can use `..=`, like this:

```
fn main() {
    for count in 1..=5{
        print!("count is: {}", count);
    }
}
```
## Summary:
- `loop`: An infinite loop that continues until explicitly broken with `break`.
- `while`: Loops as long as the condition is true.
- `for`: Loops over a range or collection, the most common loop in Rust.
