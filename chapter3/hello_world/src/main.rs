// Declare the external crate
extern crate regex;

use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Hello World");
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}
