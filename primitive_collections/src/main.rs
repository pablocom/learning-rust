use std::collections::HashMap;

fn main() {
    vectors();
    strings();
    hash_maps();
}

fn vectors() {
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
}

fn strings() {
    let this_is_a_string_literal = &mut "The value is fixed and stored in the binary file";
    *this_is_a_string_literal = "just testing modifying a string literal reference";

    println!("{}", this_is_a_string_literal);

    let mut dynamic_string = String::from("Some text");
    dynamic_string.replace_range(0..1, "X");
    dynamic_string.push_str("... more stuff");

    let slice = &dynamic_string[0..=3];

    println!("The slice points to: {slice}");
}

fn hash_maps() {
    let mut golden_balls_by_player = HashMap::new();

    golden_balls_by_player.insert(String::from("Messi"), 8u8);
    golden_balls_by_player.insert(String::from("Ronaldo"), 5u8);
    golden_balls_by_player.insert(String::from("Modric"), 1u8);

    golden_balls_by_player.entry(String::from("Modric")).or_insert(2);
    golden_balls_by_player.entry(String::from("Ronaldinho")).or_insert(1);

    let text = "Avanzad, sin temor a la oscuridad.
        Luchad jinetes de Theoden.
        Caerán las lanzas, se quebrarán los escudos. Aún restará la espada.
        Rojo será el día, hasta el nacer del sol.
        Cabalgad, cabalgad, cabalgad hacia la desolación y el fin del mundo.
        Muerte, muerte, muerte";

    let mut words_count = HashMap::new();

    for word in text.split_whitespace() {
        let normalized = word
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();

        let count: &mut i32 = words_count.entry(normalized).or_insert(0);
        *count += 1;
    }

    for (key, value) in &words_count {
        println!("{key}: {value}");
    }
}
