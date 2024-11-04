In Rust, error handling is primarily done through the Result and Option types, 
which provide a way to handle situations where an operation might fail or where a value may be absent.

# Option Type
The `Option` type is used when a value might be `Some` (present) or `None` (absent).
**Example**
Here’s an example of using `Option`:
```
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero!"),
    }
}
```
In this example:

- `divide` returns `Some(value)` if the division is successful, and `None` if the denominator is zero.
- In `main`, we handle both cases with `match`.

**Common** `Option` **Methods**
- `.unwrap()`: Extracts the value inside `Some`, panics if it’s `None`.
- `.expect(msg)`: Like unwrap, but provides a custom error message if it’s `None`.
- `.is_some()` / `.is_none()`: Checks if the value is `Some` or `None`.
- `.map()`: Transforms the `Some` value if it exists.
- `.and_then()`: Chains operations, returning `None` if any of the steps fail.


# Result Type
The `Result` type is used for functions that might succeed or fail, making it suitable for error handling. 
It represents either a success value (`Ok`) or an error value (`Err`).
```
fn division(numerator : f64, denumerator : f64) -> Result<f64, &'static str> {
    if denumerator == 0.0 {
        Err("Cannot divide by zero")
        }
        else {
            Ok(numerator / denumerator)
            }
}

fn main() {
    let result = division(10.0, 2.0);
    match result {
        Ok(value) => println!("Result is {}", value),
        Err(e) => println!("Error: {}",e)
    }
}
```
**Common Result Methods**
- `.unwrap()` / `.expect(msg)`: Extract the value on `Ok`, panic on `Err`.
`.is_ok()` / `.is_err()`: Check if the result is `Ok` or `Err`.
`.map()` / `.map_err()`**: Transform `Ok` or `Err` values.
`.and_then()`: Chain operations, propagating the error if any step fails.
`?` operator: Used for quick error propagation, returning an `Err` immediately if encountered
