# Struct
In Rust, a struct (short for structure) is a custom data type that allows you to group related data together into meaningful collections. 
You can think of a struct as a way to define your own data types by combining multiple fields of different types into a single type.

**Example:**
```
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    // Creating an instance of the Person struct
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };

    // Accessing fields of the struct
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
    println!("Email: {}", person1.email);
}
```
- Here, the Person struct has three fields: `name`, `age`, and `email`. Each field has a type, and instances of the struct can store values for those fields.
- You can access individual fields using dot notation (e.g., `person1.name`).
