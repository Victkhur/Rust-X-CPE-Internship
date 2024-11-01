# Pattern Matching with match 
It allows you to match a value against a series of patterns and execute code based on the matching pattern.

**Basic Syntax of** `match`
```
match value {
    pattern1 => code_to_run_if_pattern1_matches,
    pattern2 => code_to_run_if_pattern2_matches,
    _ => code_to_run_if_no_other_pattern_matches, // default case
}
```
**Example: Using** `match` **with an Enum**
```
enum Status {
    Active,
    Inactive,
    Pending,
}

fn print_status(status: Status) {
    match status {
        Status::Active => println!("The status is active."),
        Status::Inactive => println!("The status is inactive."),
        Status::Pending => println!("The status is pending."),
    }
}

fn main() {
    let current_status = Status::Active;
    print_status(current_status);
}
```
**In this example:**

The `match` statement checks each variant of Status.
When `status` is `Active`, `Inactive`, or `Pending`, the corresponding code block is executed.

**The** `_` **Pattern (Wildcard)**
The `_` pattern is used as a "catch-all" for cases that don’t match any specific pattern. 
It’s typically used as the last match arm and serves as a default case.
```
fn check_number(x: i32) {
    match x {
        1 => println!("One!"),
        2 => println!("Two!"),
        _ => println!("Some other number."),
    }
}
```
