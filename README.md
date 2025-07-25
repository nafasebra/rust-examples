# Rust Examples

A collection of beginner-friendly Rust programming examples designed to help you learn the fundamentals of Rust programming language.

## Overview

This repository contains various Rust projects that demonstrate different programming concepts, from basic string manipulation to file handling and user input processing. Each project is self-contained and can be run independently.

## Project Structure

### 1. Duplicate String (`1-duplicate-string/`)
A simple program that demonstrates string manipulation in Rust. This example shows how to work with strings and perform basic operations.

### 2. Guess Number (`2-guess-number/`)
An interactive number guessing game that teaches user input handling, random number generation, and control flow in Rust.

### 3. Birthday Calculator (`3-birthday-calculator/`)
A program that calculates age or time-related information based on birthday input. Demonstrates date/time handling and calculations.

### 4. Simple Calculator (`4-simple-calculator/`)
A basic calculator implementation that performs arithmetic operations. Shows how to handle mathematical operations and user input validation.

### 5. Task List (`5-task-list/`)
A task management application that allows adding, viewing, and managing tasks. Demonstrates data structures and basic CRUD operations.

### 6. File Line Counter (`6-file-line-counter/`)
A utility that counts lines in files. Shows file I/O operations and error handling in Rust.

### 7. String Reverser (`7-string-reverser/`)
A program that reverses strings. Demonstrates string manipulation and algorithm implementation.

## Getting Started

### Prerequisites
- Rust installed on your system (install from [rustup.rs](https://rustup.rs/))

### Running the Examples

Each project can be run independently. Navigate to any project directory and run:

```bash
# Navigate to a specific example
cd 1-duplicate-string

# Build and run the project
cargo run
```

### Building All Projects

To build all projects at once, you can run:

```bash
# From the root directory
for dir in */; do
    if [ -f "$dir/Cargo.toml" ]; then
        echo "Building $dir"
        cd "$dir"
        cargo build
        cd ..
    fi
done
```

## Features

- **Self-contained**: Each project has its own `Cargo.toml` and dependencies
- **Beginner-friendly**: Clear, well-commented code
- **Progressive difficulty**: Projects increase in complexity
- **Practical examples**: Real-world scenarios and use cases

## Contributing

Feel free to contribute by:
- Adding new examples
- Improving existing code
- Adding better documentation
- Fixing bugs or issues

## License

This project is open source and available under the MIT License.

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Playground](https://play.rust-lang.org/)

---

Happy coding with Rust! 🦀 by nafasebra
