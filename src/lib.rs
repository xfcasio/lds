#![allow(unused)]

mod display;
mod simplify;
mod tests;

use std::fmt;
use std::ops::Deref;

/*struct Equation<'a> {
    pub lhs: Vec<Term<'a>>,
    pub rhs: Vec<Term<'a>>
}*/

#[derive(Clone, PartialEq)]
enum Term {
    Constant(f64), // for now, arbitrary symbolic constants later.
    Var(char),
    Sum(Box<Term>, Box<Term>),
    
    Scale { coefficient: f64, term: Box<Term>},
    Product(Box<Term>, Box<Term>),
    
    Power { base: Box<Term>, exponent: Box<Term>},
    Exponential(f64, Box<Term>),
    
    Sin(Box<Term>),
    Cos(Box<Term>),

    Derivative { order: usize, wrt: Box<Term>, term: Box<Term>},
}
