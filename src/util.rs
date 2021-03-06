use std::ops::Index;
use std::ops::Range;
use std::str::FromStr;
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Symbol(&'a str),
    Number(f64),
    Bool(bool), // #t and #f
    String(&'a str),
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone)]
enum ParsingState<'a> {
    Ready,
    Symbol(usize),
    Hash(usize),
    String(usize, Option<usize>),
    Error(LexError<'a>),
}

#[derive(Debug, Clone)]
pub enum LexError<'a> {
    IllegalCharacter(char),
    IllegalNumber(&'a str),
    IllegalHash(&'a str),
}

pub fn is_symbol_char(ch: char) -> bool {
    let others = String::from("!$%&*+-./:<=>?@^_~");
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
                    match slice {
                        "#t" => {
                            tokens.push(Token::Bool(true));
                            parsing = ParsingState::Ready;
                        },
                        "#f" => {
                            tokens.push(Token::Bool(false));
                            parsing = ParsingState::Ready;
                        },
                        _ => {
                            if let Ok(num) = f64::from_str(slice) {
                                tokens.push(Token::Number(num));
                                parsing = ParsingState::Ready;
                            } else {
                                if i64::from_str(&slice[..1]).is_ok() {
                                    parsing = ParsingState::Error(LexError::IllegalNumber(slice));
                                } else {
                                    tokens.push(Token::Symbol(slice));
                                    parsing = ParsingState::Ready;
                                }
                            }
                        }
                    }
                }
            },

            ParsingState::String(start, o_esc) => {
                match o_esc {
                    None => { // Not in an escape sequence
                        if c == '\\' {
                            parsing = ParsingState::String(start, Some(i))
                        } else if c == '"' {
                            tokens.push(Token::String(&src[start..i]));
                            parsing = ParsingState::Ready;
                        }
                    },
                    Some(_backslash_pos) => {
                        // ignore one character
                        parsing = ParsingState::String(start, None);
                    }
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
                    '"' => {
                        parsing = ParsingState::String(i + 1, None);
                    }
                    _ => {
                        parsing =
                            if is_symbol_char(c) {
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

pub struct ClingyIter<'a, T> where T: 'a {
    iter: Iter<'a, T>, // TODO: Better solution, with a trait
    item: Option<&'a T>,
}

impl<'a, T> ClingyIter<'a, T> {
    pub fn new(iter: Iter<'a, T>) -> ClingyIter<'a, T> {
        let mut citer = ClingyIter { iter: iter, item: None };
        citer.advance();
        citer
    }
    pub fn advance(&mut self) {
        self.item = self.iter.next();
        //match self.iter.next() {
        //    Some(thing) => self.item = Some(*thing),
        //    None => self.item = None,
        //}
    }
    pub fn value(&'a self) -> Option<&'a T> {
        match self.item {
            Some(ref x) => Some(x),
            None => None,
        }
    }
}
