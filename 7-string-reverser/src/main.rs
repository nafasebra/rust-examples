use std::io;

/**
 * A program that takes a string from the user and prints it in reverse.
 */

fn main() {
    println!("Enter your string: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    let reversed = reverse_string(input);
    println!("Reversed string: {}", reversed);
}

fn reverse_string(input: &str) -> String {
    let mut new_string = String::new();

    for ch in input.chars().rev() {
        new_string.push(ch);
    }

    new_string
}