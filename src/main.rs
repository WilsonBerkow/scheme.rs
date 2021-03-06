use std::io;
use std::io::Write;

mod parse;
mod util;
mod eval;

fn main() {
    println!("Welcome to Scheme!");
    let mut scope = eval::arith_table();
    loop {
        print!("> ");
        if let Err(e) = io::stdout().flush() {
            println!("Error flushing stdout! {}", e);
            return;
        }
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
                            match eval::eval(&mut scope, sexp) {
                                Ok(sval) => {
                                    println!("= {:?}", sval);
                                },
                                Err(e) => {
                                    println!("Eval error! {:?}", e);
                                },
                            }
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
