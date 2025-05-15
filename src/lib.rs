#![allow(unused)]

mod display;
mod simplify;

use std::fmt;
use std::ops::Deref;

/*struct Equation<'a> {
    pub lhs: Vec<Term<'a>>,
    pub rhs: Vec<Term<'a>>
}*/

#[derive(Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use Term::*;


    #[test]
    fn it_works() {
        let terms = [
            Scale { coefficient: 2.0, term: Box::new(Scale { coefficient: 3.0, term: Box::new(Scale { coefficient: 2.0, term: Box::new(Var('x')) })}) },
            Scale { coefficient: 2.0, term: Box::new(Scale { coefficient: 3.0, term: Box::new(Scale { coefficient: 2.0, term: Box::new(Sin(Box::new(Var('x')))) })}) },
            //Derivative { order: 3, wrt: Box::new(Var('x')), term: Box::new(Sin(Box::new(Product(Box::new(Constant(2.0)), Box::new(Var('x')))))) },
            /*Sum(&Var('x'), &Constant(5.0)),
            Product(&Var('x'), &Scale { coefficient: 3.0, term: &Var('y') }),
            Scale { coefficient: 2.0, term: &Sin(&Var('x')) },
            Power { base: &Var('x'), exponent: &Constant(3.0) },
            Derivative { order: 1, wrt: &Var('y'), term: &Sin(&Product(&Constant(2.0), &Var('x'))) },
            Sum(
            &Power { base: &Var('x'), exponent: &Constant(2.0) },
            &Product(&Constant(3.0), &Var('x')),
            ),
            Exponential(2.0, &Sum(&Var('x'), &Constant(1.0))),
            Exponential(2.0, &Product(&Var('z'), &Constant(1.0))),
            Cos(&Scale { coefficient: 5.0, term: &Var('t') }),
            Derivative { order: 2, wrt: &Var('z'), term: &Product(&Var('x'), &Cos(&Var('x'))) },
            Product(
            &Exponential(3.0, &Var('x')),
            &Sin(&Power { base: &Var('x'), exponent: &Constant(2.0) }),
            ),*/
        ];

        println!("{}\n", "=".repeat(40));

        println!("{}", terms[0].simplify());

        println!("\n{}", "=".repeat(40));
    }
}
