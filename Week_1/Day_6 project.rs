use std::io;

fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut operation = String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut input_1).expect("failed to read line");

    println!("Enter the second number: ");
    io::stdin().read_line(&mut input_2).expect("failed to read line");

    println!("Enter the operation you want to do (+,-,*,/): ");
    io::stdin().read_line(&mut operation).expect("failed to read line");

    //To trim spaces and convert it to intergers
    let num_1: f32 = input_1.trim().parse().expect("Input a number");
    let num_2: f32 = input_2.trim().parse().expect("Input a number");
    let operations = operation.trim();

    if operations == "+" {
        let result = num_1 + num_2;
        println!("The sum is {}", result)
    } else if operations == "-" {
        let result = num_1 - num_2;
        println!("The difference is {}", result)
    } else if operations == "*" {
        let result = num_1 * num_2;
        println!("The product is {}", result)
    } else if operations == "/" {
        if num_2 == 0.0 {
            println!("Error: Division by zero is not allowed");
        }
        let result = num_1 / num_2;
        println!("The division is {}", result);
    } else {
        println!("Error: Invalid operation, enter the correct operation +,-,*,/");
    }


}
