#![allow(unused)]

mod display;
mod simplify;

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

#[cfg(test)]
mod tests {
    use super::*;
    use Term::*;


    #[test]
    fn it_works() {
        let fterm = Scale { coefficient: 5.0, term: Box::new(Sum(Box::new(Constant(5.0)), Box::new(Constant(10.0)))) };

        let dterm = Scale { coefficient: 2.0, term: Box::new(Sum(
                Box::new(Scale {
                    coefficient: 3.0,
                    term: Box::new(Scale {
                        coefficient: 7.0,
                        term: Box::new(Sin(Box::new(Var('x')))) 
                    })
                }),

                Box::new(Scale {
                    coefficient: 3.0,
                    term: Box::new(Scale {
                        coefficient: 7.0,
                        term: Box::new(Cos(Box::new(Var('x')))) 
                    })
                })
            ))
        };

        let terms = [
            Scale { coefficient: 2.0, term: Box::new(Scale { coefficient: 3.0, term: Box::new(Scale { coefficient: 2.0, term: Box::new(Var('x')) })}) },
            Scale { coefficient: 2.0, term: Box::new(Scale { coefficient: 3.0, term: Box::new(Scale { coefficient: 7.0, term: Box::new(Sin(Box::new(Var('x')))) })}) },
            Derivative { order: 3, wrt: Box::new(Var('x')), term: Box::new(fterm) }
        ];

        for t in terms { println!("= {}\n\n", t.debug_simplify()); }
    }
}
