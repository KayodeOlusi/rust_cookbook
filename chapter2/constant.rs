const UPPER_LIMIT: i32 = 12;

fn is_big(n: i32) -> bool {
    n > UPPER_LIMIT
}

fn main() {
    let random_number = 15;

    println!("The threshold is {}", UPPER_LIMIT);
    println!(
        "{} is {}",
        random_number,
        if is_big(random_number) {
            "big"
        } else {
            "small"
        }
    );
}
