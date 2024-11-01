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

struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory { items: Vec::new() }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    // Method to categorize items
    fn categorize_items(&self) -> (Vec<&Item>, Vec<&Item>, Vec<&Item>) {
        let mut electronics = Vec::new();
        let mut groceries = Vec::new();
        let mut clothing = Vec::new();

        for item in &self.items {
            match item.category {
                Category::Electronics => electronics.push(item),
                Category::Groceries => groceries.push(item),
                Category::Clothing => clothing.push(item),
            }
        }

        (electronics, groceries, clothing)
    }
}
fn main() {
    let mut inv = Inventory::new();

    // Add items to the inventory
    inv.add_item(Item {
        id: 1,
        name: "Phone".to_string(),
        category: Category::Electronics,
        quantity: 5,
        price: 1000,
    });
    inv.add_item(Item {
        id: 2,
        name: "Apple".to_string(),
        category: Category::Groceries,
        quantity: 10,
        price: 2,
    });
    inv.add_item(Item {
        id: 3,
        name: "T-Shirt".to_string(),
        category: Category::Clothing,
        quantity: 20,
        price: 15,
    });

    // Categorize items
    let (electronics, groceries, clothing) = inv.categorize_items();

    // Print categorized items
    println!("Electronics:");
    for item in electronics {
        println!(
            "  ID: {}, Name: {}, Quantity: {}, Price: {}",
            item.id, item.name, item.quantity, item.price
        );
    }

    println!("Groceries:");
    for item in groceries {
        println!(
            "  ID: {}, Name: {}, Quantity: {}, Price: {}",
            item.id, item.name, item.quantity, item.price
        );
    }

    println!("Clothing:");
    for item in clothing {
        println!(
            "  ID: {}, Name: {}, Quantity: {}, Price: {}",
            item.id, item.name, item.quantity, item.price
        );
    }
}
