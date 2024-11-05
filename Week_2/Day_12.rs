#[derive(Debug)]
enum Category {
    Electronics,
    Groceries,
    Clothing,
}

struct Item {
    id: i32,
    name: String,
    category: Category,
    quantity: i32,
    price: i32,
}

#[derive(Debug)]
enum InventoryError {
    DuplicateItemId,
    ItemNotFound,
}

use std::fmt;

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InventoryError::DuplicateItemId => write!(f, "Item with the same ID already exists."),
            InventoryError::ItemNotFound => write!(f, "Item not found in the inventory."),
        }
    }
}

struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory { items: Vec::new() }
    }

    fn add_item(&mut self, item: Item) -> Result<(), InventoryError> {
        if self.items.iter().any(|existing_item| existing_item.id == item.id) {
            return Err(InventoryError::DuplicateItemId);
        }
        self.items.push(item);
        Ok(())
    }

    fn get_item_by_id(&self, id: i32) -> Result<&Item, InventoryError> {
        self.items.iter().find(|&item| item.id == id).ok_or(InventoryError::ItemNotFound)
    }
}

fn main() {
    let mut inv = Inventory::new();

    let item1 = Item {
        id: 1,
        name: "Phone".to_string(),
        category: Category::Electronics,
        quantity: 5,
        price: 1000,
    };

    match inv.add_item(item1) {
        Ok(()) => println!("Item added successfully."),
        Err(e) => println!("Error adding item: {}", e),
    }

    let duplicate_item = Item {
        id: 1,
        name: "Laptop".to_string(),
        category: Category::Electronics,
        quantity: 2,
        price: 1500,
    };

    match inv.add_item(duplicate_item) {
        Ok(()) => println!("Duplicate item added successfully."),
        Err(e) => println!("Error adding duplicate item: {}", e),
    }

    match inv.get_item_by_id(1) {
        Ok(item) => println!(
            "Retrieved item - ID: {}, Name: {}, Quantity: {}, Price: {}",
            item.id, item.name, item.quantity, item.price
        ),
        Err(e) => println!("Error retrieving item: {}", e),
    }

    match inv.get_item_by_id(99) {
        Ok(item) => println!(
            "Retrieved item - ID: {}, Name: {}, Quantity: {}, Price: {}",
            item.id, item.name, item.quantity, item.price
        ),
        Err(e) => println!("Error retrieving item: {}", e),
    }
}