use std::collections::HashMap;

struct Contact {
    name: String,
    phone: String,
    email: String,
}

struct ContactBook {
    contacts: HashMap<String, Contact>,
}

impl ContactBook {
    fn new() -> Self {
        ContactBook {
            contacts: HashMap::new(),
        }
    }

    fn add_contact(&mut self, contact: Contact) {
        self.contacts.insert(contact.name.clone(), contact);
    }

    fn remove_contact(&mut self, name: &str) {
        self.contacts.remove(name);
    }

    fn search_contact(&self, name: &str) -> Option<&Contact> {
        self.contacts.get(name)
    }
}

fn main() {
    let mut contact_book = ContactBook::new();

    // Add some contacts
    contact_book.add_contact(Contact {
        name: "John Doe".to_string(),
        phone: "123-456-7890".to_string(),
        email: "john.doe@example.com".to_string(),
    });
    contact_book.add_contact(Contact {
        name: "Jane Smith".to_string(),
        phone: "987-654-3210".to_string(),
        email: "jane.smith@example.com".to_string(),
    });

    // Search for a contact
    if let Some(contact) = contact_book.search_contact("John Doe") {
        println!("Found contact: {}\nPhone: {}\nEmail: {}", contact.name, contact.phone, contact.email);
    } else {
        println!("Contact not found");
    }

    // Remove a contact
    contact_book.remove_contact("Jane Smith");
}
