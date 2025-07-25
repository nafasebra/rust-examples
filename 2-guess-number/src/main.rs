use std::io;
use rand::{self, Rng};

fn main() {
    let mut count = 5;
    let mut rng = rand::rng();
    let random_number = rng.random_range(1..=100);
    println!("**Guess number game** \n you have 5 chances to guess");

    while count > 0 {
        println!("Enter number in range 1 to 100 (guess {}): ", count);
    
        let mut entered_number: String = String::from("");
        io::stdin().read_line(&mut entered_number).expect("The entered number not validate!");
    
        let number = entered_number.trim().parse::<i32>().unwrap();
    
        if number > random_number {
            println!("The guess grader than the random number");
        } else if number < random_number {
            println!("The guess less than the random number");
        } else {
            println!("You win!");
            break;
        }

        count = count - 1;
    }
    
    println!("The random number is: {}", random_number);
}
