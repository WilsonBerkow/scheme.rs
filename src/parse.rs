#[derive(Debug, PartialEq)]
enum SExp {
    List(Vec<SExp>), // nil is List(vec![])
    Symbol(String),
    Number(f64),
    Bool(bool),
}
