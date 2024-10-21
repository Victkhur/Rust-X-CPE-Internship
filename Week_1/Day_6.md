# Day-6 On Rust
Here’s a simple calculator in Rust that performs addition, subtraction, multiplication, and division. 
I'll break down the code and document the steps to explain how the calculator works.

## Step-by-Step Guide:
1.  **Import the Necessary Library**
    - We'll need the `std::io` library to handle user input. Rust provides this library by default, so there’s no need to install anything additional.
```
use std::io;
```
2. **Create Variables for Input and Operation**
    - Three mutable variables (input_1, input_2, and operation) are created to store the user input as string. These variables are strings (String::new() initializes them as empty strings).
```
let mut input_1 = String::new();
let mut input_2 = String::new();
let mut operation = String::new();
```
3. **Read User Input for the First Number, Second Number and Operation**
     - This block prompts the user to input something and reads the input from the terminal using `io::stdin().read_line(&mut input)`.
     - If there is an issue reading the input, the program will panic with the error message `"failed to read line"`.
   ```
    println!("Enter the first number: ");
    io::stdin().read_line(&mut input_1).expect("failed to read line");

    println!("Enter the second number: ");
    io::stdin().read_line(&mut input_2).expect("failed to read line");

    println!("Enter the operation you want to do (+,-,*,/): ");
    io::stdin().read_line(&mut operation).expect("failed to read line");
   ```
4. **Convert Input Strings to Numbers and Trim Whitespace**   
      - The first two lines convert the input strings (`input_1` and `input_2`) to floating-point numbers of type f32 using the `trim()` and `parse()` functions.
      - `.trim()` removes any surrounding whitespace or newlines from the input. .`parse()` attempts to convert the trimmed string into the specified type (`f32` in this case).
```
let num_1: f32 = input_1.trim().parse().expect("Input a number");
let num_2: f32 = input_2.trim().parse().expect("Input a number");
let operations = operation.trim();
```
5. **Perform the Chosen Operation**
    - The program checks the value of operations and performs the appropriate arithmetic operation using conditional if, else if, and else statements.
```
if operations == "+" {
    let result = num_1 + num_2;
    println!("The sum is {}", result);
} else if operations == "-" {
    let result = num_1 - num_2;
    println!("The difference is {}", result);
} else if operations == "*" {
    let result = num_1 * num_2;
    println!("The product is {}", result);
} else if operations == "/" {
    if num_2 == 0.0 {
        println!("Error: Division by zero is not allowed");
    }
    let result = num_1 / num_2;
    println!("The division is {}", result);
} else {
    println!("Error: Invalid operation, enter the correct operation +,-,*,/");
}
```
6. **Division by Zero Handling**
    - This block ensures that division by zero is properly handled by printing an error message if the second number (`num_2`) is zero. This prevents a runtime error from occurring when attempting to divide by zero.
```
if num_2 == 0.0 {
    println!("Error: Division by zero is not allowed");
}
```
## Example Run of the Program
Here’s what happens when the program runs:
```
Enter the first number: 
5
Enter the second number: 
2
Enter the operation you want to do (+,-,*,/): 
/
The division is 2.5
```
