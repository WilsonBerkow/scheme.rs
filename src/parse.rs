use util::Token;
use util::ClingyIter;

#[derive(Debug, PartialEq)]
pub enum Sexp {
    List(Vec<Sexp>), // nil is List(vec![])
    Symbol(String),
    Number(f64),
    Bool(bool),
}

pub fn read_sexp<'a>(mut citer: &mut ClingyIter<Token<'a>>)
                 -> Result<Sexp, &'static str> {
    return if let Some(&token) = citer.value() {
        match token {
            Token::Symbol(sym) => {
                citer.advance();
                Ok(Sexp::Symbol(String::from(sym)))
            },
            Token::Number(num) => {
                citer.advance();
                Ok(Sexp::Number(num))
            },
            Token::Bool(b) => {
                citer.advance();
                Ok(Sexp::Bool(b))
            },
            Token::RightParen => Err("Unexpected ')'"),
            Token::LeftParen => {
                let mut contents = vec![];
                citer.advance();
                while let Some(&token) = citer.value() {
                    if let Token::RightParen = token {
                        citer.advance();
                        break;
                    } else {
                        match read_sexp(&mut citer) {
                            Ok(sexp) => contents.push(sexp),
                            Err(e) => {
                                return Err(e);
                            },
                        }
                    }
                }
                Ok(Sexp::List(contents))
            },
        }
    } else {
        Err("No tokens in input")
    }
}
