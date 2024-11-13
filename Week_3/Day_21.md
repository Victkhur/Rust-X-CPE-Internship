I refactor the contact book to be more generic and reusable using traits and generics. 
This will make it more flexible and allow it to work with different types of contacts and storage methods.

**Key improvements in this refactored version:**

1. **Generic Traits:**
    - `ContactInfo`: A trait that defines the required behavior for any type of contact.
    - `Storage`: A trait that defines the storage interface, allowing different storage implementations.

2. **Flexible Contact Types:**
    - `BasicContact`: For personal contacts
    - `BusinessContact`: For business contacts
    - Easy to add new contact types by implementing the `ContactInfo` trait

3. **Modular Storage:**
     - `HashMapStorage`: A basic HashMap-based storage implementation
     - The system is designed to easily add new storage implementations (e.g., database, file system)

4. **Type Safety:**
     - The contact book is generic over both the contact type and storage type
     - Compile-time guarantees that only valid combinations will work

5. **Extensibility:**
     - New contact types can be added without modifying existing code.
     - New storage implementations can be added without changing the contact book logic
     - Additional functionality can be added through trait extensions
