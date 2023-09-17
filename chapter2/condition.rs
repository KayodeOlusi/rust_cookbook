use std::i32;

fn main() {
    let age: i32 = 10;

    if age <= 18 {
        println!("Go to school");
    } else if (age > 18) && (age <= 28) {
        println!("Go to college");
    } else {
        println!("Do something with your life");
    }

    // if else in one line
    let can_vote = if age >= 18 { true } else { false };
    println!("Can vote: {}", can_vote);
}
