#![feature(allocator_api)]

mod display;
mod simplify;
mod differentiate;
mod tests;

use bumpalo::Bump;

/*struct Equation<'a> {
    pub lhs: Vec<Term<'a>>,
    pub rhs: Vec<Term<'a>>
}*/

type ArenaTerm<'arena> = Box<Term<'arena>, &'arena Bump>;

#[derive(Clone, PartialEq)]
enum Term<'arena> {
    Constant(f64), // for now, arbitrary symbolic constants later.
    Var(char),
    Sum(ArenaTerm<'arena>, ArenaTerm<'arena>),
    
    Scale { coefficient: f64, term: ArenaTerm<'arena> },
    Product(ArenaTerm<'arena>, ArenaTerm<'arena>),
    
    Power { base: ArenaTerm<'arena>, exponent: ArenaTerm<'arena> },
    Exponential(f64, ArenaTerm<'arena>),
    
    Sin(ArenaTerm<'arena>),
    Cos(ArenaTerm<'arena>),

    Derivative { order: usize, wrt: ArenaTerm<'arena>, term: ArenaTerm<'arena> },
}
