fn main() {
    let rand_array = [1, 2, 3];
    println!("random array {:?}", rand_array);

    // indexing start from 0
    println!("random array 1st element {}", rand_array[0]);
    println!("random array length {}", rand_array.len());

    // last two elements
    println!("random array {:?}", &rand_array[1..3]);
}
