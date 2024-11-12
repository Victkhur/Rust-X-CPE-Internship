# Traits
Traits in Rust are similar to interfaces in other languages. 
They define a set of methods that a type must implement. 
Traits allow you to define shared behavior across different types.
**Example**
```
trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("Woof!")
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        String::from("Meow!")
    }
}

fn main() {
    let dog = Dog { name: String::from("Buddy") };
    let cat = Cat { name: String::from("Whiskers") };

    println!("Dog says: {}", dog.make_sound());
    println!("Cat says: {}", cat.make_sound());
}
```
In this example:
- The `Animal` trait defines a method `make_sound`.
- The `Dog` and `Cat` structs implement the Animal trait, providing their own versions of `make_sound`.

