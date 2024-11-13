use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

// Trait for contact information
trait ContactInfo: Clone + Display {
    fn get_id(&self) -> &str;
    fn matches_search(&self, query: &str) -> bool;
    fn compare_by_field(&self, other: &Self, field: &str) -> std::cmp::Ordering;
}

// Generic storage trait
trait Storage<T: ContactInfo> {
    fn add(&mut self, item: T);
    fn remove(&mut self, id: &str) -> Option<T>;
    fn get(&self, id: &str) -> Option<&T>;
    fn get_all(&self) -> Vec<&T>;
}

// Implementation of ContactInfo for a basic contact
#[derive(Clone)]
struct BasicContact {
    name: String,
    phone: String,
    email: String,
}

impl ContactInfo for BasicContact {
    fn get_id(&self) -> &str {
        &self.name
    }

    fn matches_search(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.name.to_lowercase().contains(&query)
            || self.phone.contains(&query)
            || self.email.to_lowercase().contains(&query)
    }

    fn compare_by_field(&self, other: &Self, field: &str) -> std::cmp::Ordering {
        match field {
            "name" => self.name.cmp(&other.name),
            "phone" => self.phone.cmp(&other.phone),
            "email" => self.email.cmp(&other.email),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl Display for BasicContact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} ({})",
            self.name, self.phone, self.email
        )
    }
}

// HashMap-based storage implementation
struct HashMapStorage<T: ContactInfo> {
    items: HashMap<String, T>,
}

impl<T: ContactInfo> Storage<T> for HashMapStorage<T> {
    fn add(&mut self, item: T) {
        self.items.insert(item.get_id().to_string(), item);
    }

    fn remove(&mut self, id: &str) -> Option<T> {
        self.items.remove(id)
    }

    fn get(&self, id: &str) -> Option<&T> {
        self.items.get(id)
    }

    fn get_all(&self) -> Vec<&T> {
        self.items.values().collect()
    }
}

impl<T: ContactInfo> HashMapStorage<T> {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

// Generic contact book
struct ContactBook<T: ContactInfo, S: Storage<T>> {
    storage: S,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: ContactInfo, S: Storage<T>> ContactBook<T, S> {
    fn new(storage: S) -> Self {
        Self {
            storage,
            _phantom: std::marker::PhantomData,
        }
    }

    fn add_contact(&mut self, contact: T) {
        self.storage.add(contact);
    }

    fn remove_contact(&mut self, id: &str) -> Option<T> {
        self.storage.remove(id)
    }

    fn get_contact(&self, id: &str) -> Option<&T> {
        self.storage.get(id)
    }

    fn list_contacts(&self, sort_by: Option<&str>, filter_by: Option<&str>) -> Vec<&T> {
        let mut contacts = self.storage.get_all();

        // Apply sorting if specified
        if let Some(field) = sort_by {
            contacts.sort_by(|a, b| a.compare_by_field(b, field));
        }

        // Apply filtering if specified
        if let Some(query) = filter_by {
            contacts = contacts
                .into_iter()
                .filter(|contact| contact.matches_search(query))
                .collect();
        }

        contacts
    }
}

// Example of a different type of contact
#[derive(Clone)]
struct BusinessContact {
    company_name: String,
    contact_person: String,
    phone: String,
    email: String,
    address: String,
}

impl ContactInfo for BusinessContact {
    fn get_id(&self) -> &str {
        &self.company_name
    }

    fn matches_search(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.company_name.to_lowercase().contains(&query)
            || self.contact_person.to_lowercase().contains(&query)
            || self.phone.contains(&query)
            || self.email.to_lowercase().contains(&query)
            || self.address.to_lowercase().contains(&query)
    }

    fn compare_by_field(&self, other: &Self, field: &str) -> std::cmp::Ordering {
        match field {
            "company" => self.company_name.cmp(&other.company_name),
            "contact" => self.contact_person.cmp(&other.contact_person),
            "phone" => self.phone.cmp(&other.phone),
            "email" => self.email.cmp(&other.email),
            "address" => self.address.cmp(&other.address),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl Display for BusinessContact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) - {} - {}",
            self.company_name, self.contact_person, self.phone, self.email
        )
    }
}

fn main() {
    // Create a personal contact book
    let mut personal_book: ContactBook<BasicContact, HashMapStorage<BasicContact>> = 
        ContactBook::new(HashMapStorage::new());

    // Add personal contacts
    personal_book.add_contact(BasicContact {
        name: "John Doe".to_string(),
        phone: "123-456-7890".to_string(),
        email: "john.doe@example.com".to_string(),
    });

    personal_book.add_contact(BasicContact {
        name: "Jane Smith".to_string(),
        phone: "987-654-3210".to_string(),
        email: "jane.smith@example.com".to_string(),
    });

    // Create a business contact book
    let mut business_book: ContactBook<BusinessContact, HashMapStorage<BusinessContact>> = 
        ContactBook::new(HashMapStorage::new());

    // Add business contacts
    business_book.add_contact(BusinessContact {
        company_name: "Acme Corp".to_string(),
        contact_person: "Bob Wilson".to_string(),
        phone: "555-123-4567".to_string(),
        email: "bob@acme.com".to_string(),
        address: "123 Business St".to_string(),
    });

    // Example usage
    println!("Personal Contacts:");
    for contact in personal_book.list_contacts(Some("name"), None) {
        println!("{}", contact);
    }

    println!("\nBusiness Contacts:");
    for contact in business_book.list_contacts(Some("company"), None) {
        println!("{}", contact);
    }
}
