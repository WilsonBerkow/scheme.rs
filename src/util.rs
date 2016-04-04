use std::ops::Index;
use std::ops::Range;
use std::str::FromStr;
use std::clone::Clone;
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Symbol(&'a str),
    Number(f64),
    Bool(bool), // #t and #f
    // Todo: String(String),
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone)]
enum ParsingState<'a> {
    Ready,
    Number(usize),
    NumberAfterE(usize),
    Symbol(usize),
    Hash(usize),
    Error(LexError<'a>),
}

#[derive(Debug, Clone)]
pub enum LexError<'a> {
    IllegalCharacter(char),
    IllegalNumber(&'a str),
    IllegalHash(&'a str),
}

pub fn is_number_char(ch: char) -> bool {
    ch == '-' || ch == '.' || ch.is_digit(10)
}

pub fn is_symbol_char(ch: char) -> bool {
    let others = String::from("+-*/^!@$:");
    let mut pat = String::new();
    pat.push(ch);
    ch.is_alphanumeric() || others.contains(&pat[0..1])
}

// lexer for s-expressions
pub fn tokenize<'a>(src: &'a String) -> Result<Vec<Token<'a>>, LexError<'a>> {
    let mut tokens: Vec<Token> = vec![];
    let mut parsing = ParsingState::Ready;

    for (i, c) in src.char_indices() {
        match parsing {
            ParsingState::Error(_) => break,

            ParsingState::Number(start) => {
                if c == 'e' || c == 'E' {
                    parsing = ParsingState::NumberAfterE(start);
                } else if !is_number_char(c) {
                    let range = Range { start: start, end: i };
                    let slice = src.index(range);
                    let numres = f64::from_str(slice);
                    if let Ok(num) = numres {
                        tokens.push(Token::Number(num));
                        parsing = ParsingState::Ready;
                    } else {
                        parsing = ParsingState::Error(LexError::IllegalNumber(slice));
                    }
                }
            },

            ParsingState::NumberAfterE(start) => {
                if !c.is_digit(10) && c != '-' {
                    let range = Range { start: start, end: i };
                    let slice = src.index(range);
                    let numres = f64::from_str(slice);
                    if let Ok(num) = numres {
                        tokens.push(Token::Number(num));
                        parsing = ParsingState::Ready;
                    } else {
                        parsing = ParsingState::Error(LexError::IllegalNumber(slice));
                    }
                }
            },

            ParsingState::Hash(start) => {
                if i == start + 2 {
                    let slice = &src[start..i];
                    match slice {
                        "#t" => {
                            tokens.push(Token::Bool(true));
                            parsing = ParsingState::Ready;
                        },
                        "#f" => {
                            tokens.push(Token::Bool(true));
                            parsing = ParsingState::Ready;
                        },
                        _ => {
                            parsing = ParsingState::Error(
                                LexError::IllegalHash(&slice));
                        },
                    }
                }
            },

            ParsingState::Symbol(start) => {
                if !is_symbol_char(c) {
                    let range = Range { start: start, end: i };
                    let slice = src.index(range);
                    let token = match slice {
                        "#t" => Token::Bool(true),
                        "#f" => Token::Bool(false),
                        _ => Token::Symbol(slice),
                    };
                    tokens.push(token);
                    parsing = ParsingState::Ready;
                }
            },
            _ => {},
        }

        match parsing {
            ParsingState::Ready => {
                match c {
                    '(' => tokens.push(Token::LeftParen),
                    ')' => tokens.push(Token::RightParen),
                    '#' => {
                        parsing = ParsingState::Hash(i);
                    },
                    _ => {
                        parsing =
                            if is_number_char(c) {
                                ParsingState::Number(i)
                            } else if is_symbol_char(c) {
                                ParsingState::Symbol(i)
                            } else if c.is_whitespace() {
                                ParsingState::Ready
                            } else {
                                ParsingState::Error(LexError::IllegalCharacter(c))
                            };
                    },
                }
            },
            _ => {},
        }
    }
    match parsing {
        ParsingState::Error(e) => Err(e),
        _ => Ok(tokens),
    }
}

#[derive(Clone)]
pub struct ClingyIter<'a, T> where T: 'a, T: Clone {
    iter: Iter<'a, T>, // TODO: Better solution, with a trait
    item: Option<&'a T>,
}

impl<'a, T> ClingyIter<'a, T> where T: 'a, T: Clone {
    pub fn new(iter: Iter<'a, T>) -> ClingyIter<'a, T> {
        let mut citer = ClingyIter { iter: iter, item: None };
        citer.advance();
        citer
    }
    pub fn advance(&mut self) -> &mut ClingyIter<'a, T> {
        self.item = self.iter.next();
        //match self.iter.next() {
        //    Some(thing) => self.item = Some(*thing),
        //    None => self.item = None,
        //}
        self
    }
    pub fn value(&self) -> Option<&T> { self.item.clone() }
}
