use std::io;

fn main() {

    /* Task 1

    Print out an items in reverse from 50 to 1 (hint: range operator)
    Task 2
    Create a program that takes in an input on the terminal(hint: input should be an integer, print out from 0 to the input(see a loop, for in and while loop))
    Task 3
    Create a program that takes in an input from the terminal and subtract, addition, multiply and division (any number)

     */

    // Task 1
    for i in (1..=50).rev() {
        println!("{}", i);
    }


    // task 2
    println!("Enter a number:");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Failed to read  line");
    let number: u32 = number.trim().parse().expect("Please enter a number!");

    println!("The number is {}", number);

    let mut count = 0;

    //  using while loop
    while count <= number {
        println!("count {}", count);
        count+=1;
    }

    //using for in loop
    for num in 0..=number{
        println!("The numbers are {}", num);
    }


    //Task 3

    println!("Enter a number:");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Failed to read  line");
    let number: u32 = number.trim().parse().expect("Please enter a number!");

    println!("The number is {}", number);

    //Addition

    let add_one = number + 1;
    println!("Addition Result: {}", add_one);

    // Subtraction
    let subtract_one = number - 1;
    println!("Subtraction Result: {}",subtract_one);


    // Multiply
    let multiply_one = number * 1;
    println!("Multiply Result: {}", multiply_one);


    // Division
    let division_one = number / 1;

    println!("Division Result: {}", division_one);


}