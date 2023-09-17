fn main() {
    // Prints the first 2 decimals after the decimal point
    println!("{:.2}", 1.2345);
    println!("=======================");

    // Print the binary hex and octal format
    println!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);
    println!("=======================");

    // Shifts
    println!("{ten:>ws$}", ten = 10, ws = 5);
    println!("{ten:>0ws$}", ten = 10, ws = 5);
}
