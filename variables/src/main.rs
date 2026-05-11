mod farenheit_celsius_converter;
mod fibonacci_number;

use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("three hours are {THREE_HOURS_IN_SECONDS} seconds");

    let mut x = 5;
    println!("x just initialized is -> {x}");

    x = x + 1;
    println!("x reassigned to -> {x}");

    let x = x + 27;
    println!("x shadowed with value -> {x}");

    {
        let x = x * 69;
        println!("x shadowed within a scope -> {x}");
    }

    println!("x value at scope exit -> {x}");

    let tup = (500, 6.4, 1u8);

    let (x, y, z) = tup;
    println!("Point coordinates (x, y, z) -> ({x}, {y}, {z})");

    let numbers = [1, 2, 3, 4, 5];
    let last_element = numbers[4];
    println!("Last elemenent of an array -> {last_element}");

    testing_arrays();
}

fn testing_arrays() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
