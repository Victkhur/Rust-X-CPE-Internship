In Rust, **HashMap** and **String** are two widely used data structures, each serving a specific purpose:

## **HashMap**


A HashMap in Rust is a key-value store, similar to dictionaries in Python or hash tables in other languages.
It is part of Rust’s standard library, specifically the `std::collections` module, and allows you to store and quickly retrieve values based on unique keys.


**Key Features of HashMap**
- Keys and Values: Each key in a `HashMap<K, V>` is associated with a value, where `K` is the type of keys and `V` is the type of values.
- **Creating a HashMap**
```
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Blue", 10);
scores.insert("Red", 20);
```
- **Accessing Elements:**
  - You can access values by passing a reference to the key to the `get` method, which returns an `Option<&V>.`
```
let team_name = "Blue";
match scores.get(team_name) {
    Some(&score) => println!("Score: {}", score),
    None => println!("No score found for team"),
}
```
- **Updating Elements:**
    - If you insert a key that already exists, the value is updated.
```
scores.insert("Blue", 15); // Changes the score for "Blue" to 15
```
- **Iterating:**
  - You can iterate over key-value pairs in a `HashMap`.
```
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```
- **Removing Elements:**
  - Use `remove` to delete an entry by its key.
```
scores.remove("Blue");
```
