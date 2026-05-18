fn main() {
    let mut dynamic_size_vector = Vec::new();

    for _ in 0..=rand::random_range(20..100) {
        dynamic_size_vector.push(rand::random_range(0..=1));
    }

    println!(
        "Items in collection: {}",
        dynamic_size_vector.len()
    );

    let mut preinitialized_vector = vec![1, 2, 27];

    println!(
        "1 - Third item of 'preinitialized_vector': {}",
        preinitialized_vector[2]
    );

    let third_number = &mut preinitialized_vector[2];
    *third_number = 32;

    println!(
        "2 - Third item of 'preinitialized_vector': {}",
        preinitialized_vector[2]
    );

    let sum_of_random_bits: i32 = dynamic_size_vector.iter().sum();
    println!("Sum of random bits {sum_of_random_bits}");

    understanding_strings();
}

fn understanding_strings() {
    let this_is_a_string_literal = &mut "The value is fixed and stored in the binary file";
    *this_is_a_string_literal = "just testing modifying a string literal reference";

    println!("{}", this_is_a_string_literal);

    let mut dynamic_string = String::from("Some text");
    dynamic_string.replace_range(0..1, "X");
    dynamic_string.push_str("... more stuff");

    let slice = &dynamic_string[0..=3];

    println!("The slice points to: {slice}");
}
