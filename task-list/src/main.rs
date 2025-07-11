use std;

fn main() {
    const PATH_NAME: &str = "task.txt";
    let mut content = String::from("");

    loop {
        match read_file(PATH_NAME) {
            Ok(task) => {
                println!("Your task: \n {}", task);
                content = task;
            },
            Err(e) => println!("You not have any task. {}", e),
        }

        println!("Enter your new task: ");

        let mut task = String::from("");
        std::io::stdin()
            .read_line(&mut task)
            .expect("The input not valid.");

        content.push_str(&task);

        match write_file(PATH_NAME, &content) {
            Ok(message) => println!("{}", message),
            Err(err) => println!("{}", err),
        }
    }
}

fn write_file(path: &str, contents: &str) -> std::io::Result<String> {
    std::fs::write(path, contents)?;
    let message = String::from("The file edited");
    Ok(message)
}

fn read_file(path: &str) -> std::io::Result<String> {
    let tasks = std::fs::read_to_string(path)?;
    Ok(tasks)
}
