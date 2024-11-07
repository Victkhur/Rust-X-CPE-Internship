## Iterators
An iterator is an object that can be used to access the elements of a collection, one at a time, 
without needing to know the underlying implementation details of the collection.

Iterators in Rust are implemented using the `Iterator` trait, 
which defines a set of methods that allow you to perform various operations on the elements of a collection, such as filtering, mapping, and reducing.

Here's an example of how you might use an iterator to sum up the numbers in a vector:
```
let numbers = vec![1, 2, 3];
let mut iter = numbers.iter(); // Creates an iterator over `numbers`
```
In this example, we create a `Vec` of numbers, and then use the `iter()` method to create an iterator over the elements of the vector. 
We then call the `sum()` method on the iterator to calculate the sum of all the numbers.

Iterators can also be used to perform more complex operations, such as filtering and mapping:
```
let numbers = vec![1, 2, 3, 4, 5];
let even_numbers: Vec<i32> = numbers.iter().filter(|n| n % 2 == 0).collect();
println!("The even numbers are: {:?}", even_numbers); // Output: The even numbers are: [2, 4]
```
In this example, we use the `filter()` method to create a new iterator that only includes the even numbers from the original vector. 
We then use the `collect()` method to collect the even numbers into a new `Vec`.

## Closures
Closures in Rust are anonymous functions that can capture variables from their surrounding environment. 
This allows you to write concise and expressive code that can be passed as arguments to other functions, such as iterators.

Here's an example of how you might use a closure with an iterator:
```
let numbers = vec![1, 2, 3, 4, 5];
let even_numbers: Vec<i32> = numbers.iter().filter(|n| *n % 2 == 0).collect();
println!("The even numbers are: {:?}", even_numbers); // Output: The even numbers are: [2, 4]
```
In this example, we use a closure as the argument to the `filter()` method of the iterator. 
The closure is `|n| *n % 2 == 0`, which checks whether the current element is even. 
The closure captures the `n` variable from the surrounding environment and dereferences it using the `*` operator to access the value of the element.

Closures in Rust can also capture variables by reference or by value, depending on the needs of the closure. 
They are a powerful tool for writing concise and expressive code, and they work particularly well with iterators to perform complex operations on collections of data.
