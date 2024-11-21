Async programming in Rust, especially using the Tokio library, is a powerful way to handle concurrent tasks efficiently.
# What is Async Programming?
Asynchronous programming allows you to write non-blocking, concurrent code that can handle multiple tasks simultaneously. 
Instead of waiting for one task to complete, the program can switch to another task, maximizing CPU and I/O efficiency. 
In Rust, async is particularly useful for I/O-bound operations, like networking or file handling.

# What is Tokio?
**Tokio** is a popular async runtime for Rust. It provides the infrastructure to run asynchronous code, including:

1. An **event loop** to manage tasks.
2. **Async I/O utilities** for networking, timers, and more.
3. **Task spawning** to run tasks concurrently.

# Key Concepts in Tokio
1 Async Functions (`async fn`)
    - Functions defined with `async fn` return a **future**, a value representing a computation that hasn’t completed yet.
    - Example
```
async fn example() {
    println!("Hello, async!");
}
```
2. Futures
   - A **future** is a type that implements the `Future` trait and represents a value that will become available later.
   - Futures don’t execute until **polled** by an async runtime like Tokio.

3. The Runtime
   - Tokio’s runtime executes your futures and handles task scheduling.
   - You need to create a runtime to run async code
```
#[tokio::main]
async fn main() {
    println!("Running with Tokio!");
}
```
4. Task Spawning
   - Use `tokio::spawn` to run multiple async tasks concurrently.
   - Each spawned task runs independently and doesn’t block others.
```
#[tokio::main]
async fn main() {
    tokio::spawn(async {
        println!("Task 1");
    });
    tokio::spawn(async {
        println!("Task 2");
    });
    println!("Main function");
}
```
5. Async I/O
    - Tokio provides async versions of I/O operations, like reading from or writing to a network socket.
    - Example with `tokio::net::TcpStream`
```
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("Connected to the server!");
}
```
