use std::io;
use chrono::{self, Datelike};

fn main() {
    let current_year: i32 = chrono::Utc::now().year();
    let mut year: String = String::from(current_year.to_string());

    println!("Enter your birth year: ");
    io::stdin().read_line(&mut year).expect("the entered value is incurret");
    let age: i32 = 2025 - year.trim().parse::<i32>().unwrap();

    println!("You are {} years old.", age);
}
