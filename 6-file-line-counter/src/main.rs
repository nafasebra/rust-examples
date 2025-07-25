use std::fs::File;
use std::io::{BufRead, BufReader, Result};

// A program that takes a file path as input and prints the total number of lines in that file.

fn main() -> Result<()> {
    let file = File::open("task.txt")?;
    let reader = BufReader::new(file);
    let count = reader.lines().filter_map(|line| line.ok()).count();

    println!("Count: {}", count);
    Ok(())
}
