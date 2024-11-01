# Inventory System in Rust
Here's a step-by-step documentation for setting up and using the Inventory system in Rust
## Step 1: Define the Category Enum
The `Category` enum defines the possible types of items you can store in the inventory. Each variant represents a different category of items.
```
#[derive(Debug)]
enum Category {
    Electronics,
    Groceries,
    Clothing,
}
```
## Step 2: Define the Item Struct
The `Item` struct represents an individual item in the inventory. Each item has a unique ID, name, category, quantity, and price.
```
struct Item {
    id: i32,
    name: String,
    category: Category,
    quantity: i32,
    price: i32,
}
```
### Step 3: Define the Inventory Struct
The `Inventory` struct represents a collection of items. 
It stores all items in a `Vec<Item>` (a vector of `Item` structs), which allows dynamic resizing as items are added or removed.
```
struct Inventory {
    items: Vec<Item>,
}
```
## Step 4: Implement Methods for Inventory
Implement methods for creating a new inventory and adding items to it.
```
impl Inventory {
    fn new() -> Inventory {
        Inventory { items: Vec::new() }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}
```
## Step 5: Use the Inventory in main
In the `main` function, create an inventory, add items to it, and access an item to print its details.
```
fn main() {
    let mut inv = Inventory::new();

    // Create an item
    let item1 = Item {
        id: 1,
        name: "Phone".to_string(),
        category: Category::Electronics,
        quantity: 5,
        price: 1000,
    };

   // Adding the items to the inventory
    inv.add_item(item1);
    inv.add_item(item2);
    inv.add_item(item3);

    // Access and print details of the first item in the inventory
    let item = &inv.items[0];
    println!(
        "The id {}, name {}, category {:?}, quantity {}, and price {}.",
        item.id, item.name, item.category, item.quantity, item.price
    );
}
```
This completes the setup of a basic inventory system in Rust. 
