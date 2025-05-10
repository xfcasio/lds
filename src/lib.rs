#![allow(unused)]

use std::fmt;

struct Equation<'a> {
    pub lhs: Vec<Term<'a>>,
    pub rhs: Vec<Term<'a>>
}

enum Term<'a> {
    Constant(f64), // for now, arbitrary variables later.
    Var(char),
    Sum(&'a Term<'a>, &'a Term<'a>),
    
    Scale { coefficient: f64, term: &'a Term <'a>},
    Product(&'a Term<'a>, &'a Term<'a>),
    
    Power { base: &'a Term<'a>, exponent: &'a Term <'a>},
    Exponential(f64, &'a Term<'a>),
    
    Sin(&'a Term<'a>),
    Cos(&'a Term<'a>),

    Derivative { order: usize, term: &'a Term <'a>},
}

impl std::fmt::Display for Term<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Term::*;

        match self {
            Constant(n) => write!(f, "{n}"),
            Var(c) => write!(f, "{c}"),
            Sum(t1, t2) => write!(f, "{t1} + {t2}"),

            Scale { coefficient, term } => match term {
                Sum(s1, s2) => write!(f, "{coefficient}({s1} + {s2})"),
                _ => write!(f, "{coefficient}{term}")
            },

            Product(t1, t2) => match (t1, t2) {
                (Var(c), Sum(s1, s2)) => write!(f, "{c}({s1} + {s2})"),
                (Sum(s1, s2), Var(c)) => write!(f, "({s1} + {s2}){c}"),
                (t1, Scale { coefficient, term }) => write!(f, "{coefficient}{t1}{term}"),
                (_, _) => write!(f, "{t1}{t2}")
            },

            Power { base, exponent } => match (base, exponent) {
                (Sum(s1, s2), Sum(s3, s4)) => write!(f, "({s1} + {s2})^({s3} + {s4})"),
                (Sum(s1, s2), exponent) => write!(f, "({s1} + {s2})^{exponent}"),
                (base, Sum(s1, s2)) => write!(f, "{base}^({s1} + {s2})"),
                (Product(s1, s2), exponent) => write!(f, "({s1} * {s2})^{exponent}"),
                (base, Product(s1, s2)) => write!(f, "{base}^({s1} * {s2})"),
                (base, exponent) => write!(f, "{base}^{exponent}")
            },

            Exponential(base, t) => match t {
                Sum(s1, s2) => write!(f, "{base}^({s1} + {s2})"),
                Product(s1, s2) => write!(f, "{base}^({s1} * {s2})"),
                t => write!(f, "{base}^{t}")
            },

            Sin(t) => write!(f, "sin({t})"),
            Cos(t) => write!(f, "cos({t})"),

            Derivative { order, term } => write!(f, "({term}){}", "\'".repeat(*order)),

            _ => { Ok(()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use Term::*;

        let terms = vec![
            Derivative { order: 0, term: &Sin(&Product(&Constant(2.0), &Var('x'))) },
            Derivative { order: 1, term: &Sin(&Product(&Constant(2.0), &Var('x'))) },
            Derivative { order: 2, term: &Sin(&Product(&Constant(2.0), &Var('x'))) },
            Derivative { order: 3, term: &Sin(&Product(&Constant(2.0), &Var('x'))) },
            Sum(&Var('x'), &Constant(5.0)),
            Product(&Var('x'), &Scale { coefficient: 3.0, term: &Var('y') }),
            Scale { coefficient: 2.0, term: &Sin(&Var('x')) },
            Power { base: &Var('x'), exponent: &Constant(3.0) },
            Derivative { order: 1, term: &Sin(&Product(&Constant(2.0), &Var('x'))) },
            Sum(
                &Power { base: &Var('x'), exponent: &Constant(2.0) },
                &Product(&Constant(3.0), &Var('x')),
            ),
            Exponential(2.0, &Sum(&Var('x'), &Constant(1.0))),
            Exponential(2.0, &Product(&Var('z'), &Constant(1.0))),
            Cos(&Scale { coefficient: 5.0, term: &Var('t') }),
            Derivative { order: 2, term: &Product(&Var('x'), &Cos(&Var('x'))) },
            Product(
                &Exponential(3.0, &Var('x')),
                &Sin(&Power { base: &Var('x'), exponent: &Constant(2.0) }),
            ),
        ];

        println!("{}\n", "=".repeat(40));

        terms.iter().for_each(|term| println!("{term}"));

        println!("\n{}", "=".repeat(40));
    }
}
