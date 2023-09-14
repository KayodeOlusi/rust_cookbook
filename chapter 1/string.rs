fn main() {
    let rand_string = "I love Rust cookbook <3";

    // printing the length of the string
    println!("length of the string is {}", rand_string.len());

    // Splits the string
    let (first, second) = rand_string.split_at(7);
    println!("First part : {0}  Second part : {1}", first, second);

    // Count using iterator count
    let count = rand_string.chars().count();
    println!("count {}", count);

    println!("________________________");
    // printing all characters

    let mut chars = rand_string.chars();
    let mut indiv_chars = chars.next();

    loop {
        match indiv_chars {
            Some(x) => println!("{}", x),
            None => break,
        }

        indiv_chars = chars.next();
    }

    println!("________________________");
    let mut iter = rand_string.split_whitespace();
    let mut indiv_word = iter.next();

    loop {
        match indiv_word {
            Some(x) => println!("{}", x),
            None => break,
        }

        indiv_word = iter.next();
    }

    println!("________________________");
    let rand_string2 = "I love \n everything about \n Rust <3";
    let mut inter_line = rand_string2.lines();
    let mut indiv_sent = inter_line.next();

    loop {
        match indiv_sent {
            Some(x) => println!("{}", x),
            None => break,
        }

        indiv_sent = inter_line.next();
    }
}
