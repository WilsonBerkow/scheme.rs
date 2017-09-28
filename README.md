# scheme.rs

An interpreter for a subset of Scheme, written in Rust.

(I developed this while learning Rust, and figured this would be
a nice second-nontrivial-program to write.)

Example:

```
  $ cargo run
   Compiling scheme v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s) in ...
     Running `target/debug/scheme`
Welcome to Scheme!
> (define id (lambda (x) x))

        Tokens: [LeftParen, Symbol("define"), Symbol("id"), LeftParen, Symbol("lambda"), LeftParen, Symbol("x"), RightParen, Symbol("x"), RightParen, RightParen]
: List([Symbol("define"), Symbol("id"), List([Symbol("lambda"), List([Symbol("x")]), Symbol("x")])])
= )
> (define const (lambda (x) (lambda (y) x)))

        Tokens: [LeftParen, Symbol("define"), Symbol("const"), LeftParen, Symbol("lambda"), LeftParen, Symbol("x"), RightParen, LeftParen, Symbol("lambda"), LeftParen, Symbol("y"), RightParen, Symbol("x"), RightParen, RightParen, RightParen]
: List([Symbol("define"), Symbol("const"), List([Symbol("lambda"), List([Symbol("x")]), List([Symbol("lambda"), List([Symbol("y")]), Symbol("x")])])])
= )
> ((const 4) 10)

        Tokens: [LeftParen, LeftParen, Symbol("const"), Number(4), RightParen, Number(10), RightParen]
: List([List([Symbol("const"), Number(4)]), Number(10)])
= 4
> (define S (lambda (x) (lambda (y) (lambda (z) ((x z) (y z))))))

        Tokens: [LeftParen, Symbol("define"), Symbol("S"), LeftParen, Symbol("lambda"), LeftParen, Symbol("x"), RightParen, LeftParen, Symbol("lambda"), LeftParen, Symbol("y"), RightParen, LeftParen, Symbol("lambda"), LeftParen, Symbol("z"), RightParen, LeftParen, LeftParen, Symbol("x"), Symbol("z"), RightParen, LeftParen, Symbol("y"), Symbol("z"), RightParen, RightParen, RightParen, RightParen, RightParen, RightParen]
: List([Symbol("define"), Symbol("S"), List([Symbol("lambda"), List([Symbol("x")]), List([Symbol("lambda"), List([Symbol("y")]), List([Symbol("lambda"), List([Symbol("z")]), List([List([Symbol("x"), Symbol("z")]), List([Symbol("y"), Symbol("z")])])])])])])
= )
> (+ (* 16 3) -6)

        Tokens: [LeftParen, Symbol("+"), LeftParen, Symbol("*"), Number(16), Number(3), RightParen, Number(-6), RightParen]
: List([Symbol("+"), List([Symbol("*"), Number(16), Number(3)]), Number(-6)])
= 42
>
```
