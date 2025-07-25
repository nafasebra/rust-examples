fn main() {
    duplicate("Hello", 10);
}

fn duplicate(strvar: &str, num: i8) {
    let mut new_string: String = String::new();
    for _i in 1..num {
        new_string += strvar
    }
    println!("{}", new_string);
}
