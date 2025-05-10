#![allow(unused)]

struct Equation {
    pub lhs: Vec<Term>,
    pub rhs: Vec<Term>
}

enum Term {
    Var(char),
    Sum(Box<Term>, Box<Term>),
    Scale { coefficient: usize, term: Box<Term> },
    Product(Box<Term>, Box<Term>),
    Power { base: Box<Term>, exponent: Box<Term> },
    Exponential(usize, Box<Term>),
    Sin(Box<Term>),
    Cos(Box<Term>),
}

//enum DiffEq {
//    Homogeneous(Equation)
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use Term::*;

        let _ = Scale { coefficient: 3, term: Box::new(Var('x')) };
    }
}
