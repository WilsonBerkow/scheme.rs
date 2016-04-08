use std::collections::linked_list::LinkedList;
use util::Token;
use util::ClingyIter;

#[derive(Debug, PartialEq, Clone)]
pub enum Sexp {
    List(LinkedList<Sexp>), // nil is List(vec![])
    Symbol(String),
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    // todo: Char(char), String(String)
}

fn parse_str_contents(s: &str) -> Result<String, String> {
    // s excludes the enclosing " around the source string, but
    // leaves all escapement sequences as in the source.
    let mut out = String::new();
    let mut chars = s.chars();
    loop {
        match chars.next() {
            Some('\\') => {
                match chars.next() {
                    Some('\\') => out.push('\\'),
                    Some('n') => out.push('\n'),
                    Some('t') => out.push('\t'),
                    Some('"') => out.push('"'),
                    Some('\r') => out.push('\r'),
                    // todo: \b, \f, \u####, and others?
                    Some(c) => {
                        return Err(format!("Invalid escape sequence: \\{}", c));
                    },
                    None => {
                        // This should not happen unless there was an error
                        // in the lexer
                        return Err(String::from(
                            "Unexpected end of input in string escapement"));
                    },
                }
            },
            Some(c) => {
                out.push(c);
            },
            None => {
                break;
            },
        }
    }
    Ok(out)
}

pub fn read_sexp<'a>(mut citer: &mut ClingyIter<Token<'a>>)
                 -> Result<Sexp, String> {
    return if let Some(&token) = citer.value() {
        match token {
            Token::Symbol(sym) => {
                citer.advance();
                Ok(Sexp::Symbol(String::from(sym)))
            },
            Token::String(s) => {
                citer.advance();
                match parse_str_contents(s) {
                    Ok(string) => Ok(Sexp::String(string)),
                    Err(e) => Err(e),
                }
            },
            Token::Integer(num) => {
                citer.advance();
                Ok(Sexp::Integer(num))
            },
            Token::Float(num) => {
                citer.advance();
                Ok(Sexp::Float(num))
            },
            Token::Bool(b) => {
                citer.advance();
                Ok(Sexp::Bool(b))
            },
            Token::RightParen => Err(String::from("Unexpected ')'")),
            Token::LeftParen => {
                let mut contents = LinkedList::new();//vec![];
                citer.advance();
                while let Some(&token) = citer.value() {
                    if let Token::RightParen = token {
                        citer.advance();
                        break;
                    } else {
                        match read_sexp(&mut citer) {
                            Ok(sexp) => contents.push_back(sexp),
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
        Err(String::from("No tokens in input"))
    }
}
