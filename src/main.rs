use std::env;
use std::io;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Enumerate through the arguments, skipping the first one (which is the program's input_string)
        for (index, input_string) in args.iter().enumerate().skip(1) {
            println!("Argument {}: Hello, {}!", index, input_string);
        }
    } else {
        // No command-line arguments were provided, ask for user input
        let mut input_string = String::new();
        println!("Please enter your input_string:");
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");
        let input_string = input_string.trim();
        println!("Hello, {}!", input_string);
    }
}

