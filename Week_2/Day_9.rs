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

    let item2: Item = Item {
        id: 2,
        name: "Shirt".to_string(),
        category: Category::Clothing,
        quantity: 3,
        price: 20,
    };

    let item3 = Item {
        id: 3,
        name: "Bread".to_string(),
        category: Category::Groceries,
        quantity: 2,
        price: 10,
    };
    
    // Adding the items to the inventory
    inv.add_item(item1);
    inv.add_item(item2);
    inv.add_item(item3);

    // Accessing the item from the inventory to print
    let item = &inv.items[0];

    println!(
        "The id {}, name {}, category {:?}, quantity {}, and price {}.",
        item.id, item.name, item.category, item.quantity, item.price
    );
}
