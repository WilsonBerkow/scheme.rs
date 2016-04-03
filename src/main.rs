use std::io;
use std::io::Write;

mod parse;
mod util;

fn main() {
    println!("Welcome to Scheme!");
    loop {
        print!("> ");
        io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let "\\exit" = input.trim() {
                    return;
                }
                let rtokens = util::tokenize(&input);
                match rtokens {
                    Ok(toks) => {
                        println!("\n        Tokens: {:?}", toks);
                        let mut citer = util::ClingyIter::new(toks.iter());
                        while let Ok(sexp) = parse::read_sexp(&mut citer) {
                            println!(": {:?}", sexp);
                        }
                    },
                    Err(e) => {
                        println!("Error while lexing: {:?}", e);
                    },
                }
            },
            Err(err) => println!("Error in read_line: {}", err),
        }
    }
}
