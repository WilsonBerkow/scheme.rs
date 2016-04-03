use std::io;

mod parse;
mod util;

fn main() {
    println!("Welcome to Scheme!");
    println!("Give a string for parsing:");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => println!("Tokens: {:?}", util::tokenize(&input)),
            Err(err) => println!("Error in read_line: {}", err),
        }
    }
}
