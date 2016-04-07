use std::fmt;
use std::collections::linked_list::LinkedList;
use parse::Sexp;
use parse;
use util;

#[derive(PartialEq, Clone)]
pub enum SValue {
    List(LinkedList<SValue>),
    Symbol(String),
    String(String),
    Number(f64),
    Bool(bool),
    Lambda(SymTable, LinkedList<String>, Sexp),
    //Closure(&'a mut SymTable<'a>, Vec<String>, Sexp), // an environment a list of params and a return expression
}

impl SValue {
    pub fn nil() -> SValue { SValue::List(LinkedList::new()) }
    fn r5rs_write(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SValue::String(ref s) => write!(f, "{:?}", s),
            x => x.r5rs_display(f)
        }
    }
    fn r5rs_display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SValue::List(ref ll) => {
                let mut s = String::new();
                s.push('(');
                for x in ll {
                    s.push_str(format!("{} ", x).as_ref());
                }
                s.pop();
                s.push(')');
                write!(f, "{}", s)
            },
            &SValue::Symbol(ref s) => write!(f, "{}", s),
            &SValue::String(ref s) => write!(f, "{}", s),
            &SValue::Number(x) => write!(f, "{}", x),
            &SValue::Bool(b) => write!(f, "{}", b),
            &SValue::Lambda(_, _, _) => {
                write!(f, "#<procedure>")
            }
        }
    }
}

impl fmt::Display for SValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.r5rs_display(f) }
}

impl fmt::Debug for SValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.r5rs_write(f) }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SymTable {
    items: Vec<(String, SValue)>,
}

impl SymTable {
    pub fn empty() -> SymTable {
        SymTable { items: vec![] }
    }

    pub fn from(items: Vec<(String, SValue)>) -> SymTable {
        SymTable { items: items }
    }

    pub fn lookup<'a, 'b>(&'a self, sym: &'b String) -> Option<SValue> {
        for &(ref s, ref v) in self.items.iter() {
            if s == sym {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn assign<'a, 'b>(&'a mut self, sym: &'b String, val: SValue) {
        let mut loc = 0;
        for &(ref s, _) in self.items.iter() {
            if s == sym {
                break;
            }
            loc += 1;
        }
        self.items.push((sym.clone(), val));
        if loc < self.items.len() - 1 {
            self.items.swap_remove(loc);
        }
    }
}

fn check_sym<'a>(sexp: &'a Sexp, s: &'static str) -> bool {
    sexp == &Sexp::Symbol(String::from(s))
}

pub fn quote(sexp: &Sexp) -> SValue {
    match sexp {
        &Sexp::List(ref ll) =>
            SValue::List(
                ll.iter().map(|x| quote(&x.clone())).collect()),
        &Sexp::Symbol(ref s) => SValue::Symbol(s.clone()),
        &Sexp::String(ref s) => SValue::String(s.clone()),
        &Sexp::Number(ref f) => SValue::Number(f.clone()),
        &Sexp::Bool(ref b) => SValue::Bool(b.clone()),
    }
}

type BinOp = Fn(SValue, SValue) -> Result<SValue, String>;
fn binop(op: Box<BinOp>, ident: SValue, ll: LinkedList<SValue>)
             -> Result<SValue, String> {
    let mut total = ident;
    for x in ll {
        match op(total, x) {
            Ok(v) => total = v,
            Err(e) => return Err(e),
        }
    }
    Ok(total)
}

fn get_binop(s: String) -> Option<(Box<BinOp>, SValue)> {
    match &*s {
        "+" => {
            let f = Box::new(|x, y| match (x, y) {
                (SValue::Number(fx), SValue::Number(fy)) =>
                    Ok(SValue::Number(fx + fy)),
                _ => Err(String::from("Expected numbers for +")),
            });
            Some((f, SValue::Number(0f64)))
        },

        "*" => Some((Box::new(|x: SValue, y: SValue| match (x, y) {
            (SValue::Number(fx), SValue::Number(fy)) => Ok(SValue::Number(fx * fy)),
            _ => Err(String::from("Expected numbers for *")),
        }), SValue::Number(1f64))),

        "reciprocal" => Some((Box::new(|x: SValue, y: SValue| match (x, y) {
            (SValue::Number(fx), SValue::Number(fy)) => Ok(SValue::Number(fx / fy)),
            _ => Err(String::from("Expected numbers for /")),
        }), SValue::Number(1f64))),

        // Todo: do as macro?
        // "and" => {
        //     let f = Box::new(|x, y| match (x, y) {
        //         (SValue::Bool(p), SValue::Bool(q)) =>
        //             Ok(SValue::Bool(p && q)),
        //         _ => Err("Expected bool in `and` call"),
        //     });
        //     Some((f, SValue::Bool(true)))
        // },

        _ => None,
    }
}

fn check_binop<'a>(sexp: &'a Sexp) -> Option<(Box<BinOp>, SValue)> {
    if let &Sexp::Symbol(ref s) = sexp {
        let x: Option<(Box<BinOp>, SValue)> = get_binop(s.clone());
        return x;
    } else {
        return None;
    }
}

fn get_param_list(ss: LinkedList<Sexp>) -> Result<LinkedList<String>, String> {
    let mut vals = LinkedList::new();
    for x in ss {
        if let Sexp::Symbol(s) = x {
            vals.push_back(s);
        } else {
            return Err(String::from("Expected symbol in argument list"));
        }
    }
    return Ok(vals);
}

pub fn eval_from_src(src: String) -> Result<SValue, String> {
    let tokens = util::tokenize(&src);
    if let Ok(toks) = tokens {
        if let Ok(ast) = parse::read_sexp(&mut util::ClingyIter::new(toks.iter())) {
            eval(&mut SymTable::empty(), ast)
        } else {
            Err(String::from("sdfjklsdfjkl"))
        }
    } else {
        Err(String::from("sdfjklsdfjkl"))
    }
}

pub fn arith_table() -> SymTable {
    let x = eval_from_src(String::from("(lambda (x) (+ x x))"));
    if let Ok(f) = x {
        SymTable::from(vec![(String::from("double"), f)])
    } else {
        SymTable::empty()
    }
}

// todo: make lambdas capture environ; put values in Boxes; add  mutability
// tables like prototypes, having links to superscopes

fn mk_sub_scope(table: &SymTable) -> SymTable {
    table.clone()
}

fn invoc_sub_scope<'a>(table: &'a SymTable, params: LinkedList<String>, args: LinkedList<SValue>) -> SymTable {
    let mut new_scope = table.clone();
    for (name, val) in params.iter().zip(args.iter()) {
        new_scope.assign(&name, val.clone());
    }
    new_scope
}

fn eval_all(table: &mut SymTable, ll: LinkedList<Sexp>) -> Result<LinkedList<SValue>, String> {
    let mut vals = LinkedList::new();
    for x in ll {
        match eval(table, x) {
            Ok(v) => vals.push_back(v),
            Err(e) => return Err(e)
        }
    }
    Ok(vals)
}

// TODO: parameterize by stx_forms to allow macro extensibility
pub fn eval<'a>(table: &'a mut SymTable, sexp: Sexp) -> Result<SValue, String> {
    match sexp {
        Sexp::Number(f) => Ok(SValue::Number(f)),

        Sexp::Bool(b) => Ok(SValue::Bool(b)),

        Sexp::Symbol(s) => {
            if let Some(sval) = table.lookup(&s) {
                Ok(sval)
            } else {
                Err(format!("Symbol is not bound: {}.\nScope: {:?}", &s, table))
            }
        },

        Sexp::String(s) => Ok(SValue::String(s)),

        Sexp::List(items) => {
            let mut item_ll = items.clone();

            if let Some(cmd) = item_ll.pop_front() {
                if check_sym(&cmd, "quote") {
                    if let Some(sexp) = item_ll.pop_front() {
                        Ok(quote(&sexp))
                    } else {
                        Err(String::from("`quote` expected 1 arg; was given 0"))
                    }

                } else if check_sym(&cmd, "define") {
                    if let Some(Sexp::Symbol(s)) = item_ll.pop_front() {
                        if let Some(sexp) = item_ll.pop_front() {
                            match eval(table, sexp) {
                                Ok(v) => {
                                    table.assign(&s, v);
                                    Ok(SValue::nil())
                                },
                                Err(e) => Err(e)
                            }
                        } else {
                            Err(String::from("Expected value in define statement"))
                        }
                    } else {
                        Err(String::from("Expected symbol after `define`"))
                    }

                } else if check_sym(&cmd, "lambda") {
                    if let Some(Sexp::List(arg_sexps)) = item_ll.pop_front() {
                        match get_param_list(arg_sexps) {
                            Ok(params) => {
                                if let Some(body) = item_ll.pop_front() {
                                    Ok(SValue::Lambda(mk_sub_scope(table), params, body))
                                } else {
                                    Err(String::from("Expected body after argument list in lambda"))
                                }
                            },
                            Err(e) => Err(e),
                        }
                    } else {
                        Err(String::from("Expected argument list after `lambda`"))
                    }

                } else if check_sym(&cmd, "display") {
                    if let Some(sexp) = item_ll.pop_front() {
                        match eval(table, sexp) {
                            Ok(v) => {
                                println!("{}", v);
                                Ok(SValue::nil())
                            },
                            Err(e) => Err(e),
                        }
                    } else {
                        Err(String::from("`display` expected 1 arg; was given 0"))
                    }

                } else if check_sym(&cmd, "write") {
                    if let Some(sexp) = item_ll.pop_front() {
                        match eval(table, sexp) {
                            Ok(v) => {
                                println!("{:?}", v);
                                Ok(SValue::nil())
                            },
                            Err(e) => Err(e),
                        }
                    } else {
                        Err(String::from("`write` expected 1 arg; was given 0"))
                    }

                } else if let Some((op, ident)) = check_binop(&cmd) {
                    let mut vals = LinkedList::new();
                    for e in item_ll {
                        let rval = eval(table, e);
                        if let Ok(v) = rval {
                            vals.push_back(v);
                        } else {
                            return rval;
                        }
                    }
                    binop(op, ident, vals)

                } else {
                    match eval(table, cmd) {
                        Ok(SValue::Lambda(mut sub_table, params, body)) => {
                            // item_ll is the list of args
                            match eval_all(table, item_ll) {
                                Ok(args) => {
                                    let mut new_table = invoc_sub_scope(&mut sub_table, params, args);
                                    eval(&mut new_table, body)
                                },
                                Err(e) => Err(e),
                            }
                        },
                        Ok(_) => Err(String::from("Expected callable value")),
                        Err(e) => Err(e),
                    }
                }
            } else {
                Err(String::from("Unexpected ()"))
            }
        },
    }
}
