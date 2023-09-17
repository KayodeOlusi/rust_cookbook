use std::i32;
use std::io;

fn main() {
    // Request for entering number 1
    println!("Enter First number ?");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    // Request for entering number 2
    println!("Enter Second number ?");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    // Converting string to integer
    let aint: i32 = input1
        .trim()
        .parse()
        .ok()
        .expect("Program only processes numbers, Enter a number");
    let bint: i32 = input2
        .trim()
        .parse()
        .ok()
        .expect("Program only processes numbers, Enter a number");

    // Output of basic operations
    println!("sum is {}", aint + bint);
    println!("difference is {}", aint - bint);
    println!("multiply is {}", aint * bint);
    println!("divide is {}", aint / bint);
}
