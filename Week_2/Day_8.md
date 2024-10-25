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

# Enums
In Rust, an enum (short for enumeration) is a custom data type that allows you to define a type by enumerating its possible values. Enums are useful when you need to represent a value that can take on multiple "variants," where each variant can hold different data types or even no data at all.

**Example of a Simple Enum**
Here’s an example where we define an enum to represent the four cardinal directions:
```
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let direction = Direction::North;

    match direction {
        Direction::North => println!("Going North!"),
        Direction::South => println!("Going South!"),
        Direction::East => println!("Going East!"),
        Direction::West => println!("Going West!"),
    }
}
```
In this example:

- The Direction enum has four variants: `North`, `South`, `East`, and `West`.
- The `match` expression is used to handle each possible variant of `Direction`.
