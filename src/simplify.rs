use crate::Term;

impl Term {
    pub fn simplify(&self) -> Self {
        use Term::*;

        match self {
            Constant(_) | Var(_) => self.clone(),

            Sum(t1, t2) => match (&**t1, &**t2) {
                (Constant(c1), Constant(c2)) => Constant(c1 + c2),
                _ => {
                    let t1 = t1.simplify();
                    let t2 = t2.simplify();
                    Sum(Box::new(t1), Box::new(t2))
                }
            },

            Scale { coefficient, term } => match &**term {
                Constant(c) => Constant(coefficient * c),
                Var(_) | Sin(_) | Cos(_) | Power { .. } | Exponential(_, _) | Derivative { .. } => self.clone(),
                Scale { coefficient: coefficient2, term: term2 } => {
                    Scale { 
                        coefficient: coefficient * coefficient2, 
                        term: term2.clone() 
                    }.simplify()
                },
                _ => {
                    Scale { 
                        coefficient: *coefficient, 
                        term: Box::new(term.simplify()) 
                    }.simplify()
                }
            },

            Product(t1, t2) => match (&**t1, &**t2) {
                (Constant(c1), Constant(c2)) => Constant(c1 * c2),
                _ => {
                    let t1 = t1.simplify();
                    let t2 = t2.simplify();
                    Product(Box::new(t1), Box::new(t2))
                }
            },

            Power { base, exponent } => match (&**base, &**exponent) {
                (Constant(c1), Constant(c2)) => Constant(c1.powf(*c2)),
                _ => {
                    let base = base.simplify();
                    let exponent = exponent.simplify();
                    Power { 
                        base: Box::new(base), 
                        exponent: Box::new(exponent) 
                    }
                }
            },

            Exponential(base, term) => match &**term {
                Constant(c) => Constant(base.powf(*c)),
                _ => {
                    let simplified_term = term.simplify();
                    Exponential(*base, Box::new(simplified_term))
                }
            },

            Sin(term) => match &**term {
                Constant(c) => Constant(c.sin()),
                _ => {
                    let simplified_term = term.simplify();
                    Sin(Box::new(term.simplify()))
                }
            },

            Cos(term) => match &**term {
                Constant(c) => Constant(c.cos()),
                _ => Cos(Box::new(term.simplify()))
            },

            Derivative { order, wrt, term } => match &**term {
                Constant(c) => match order {
                    0 => Constant(*c),
                    _ => Constant(0.0)
                },

                Var(_) => match (wrt, order) {
                    (term, 0) => (**term).clone(),
                    (_, 1) => Constant(1.0),
                    (_, _) => Constant(0.0)
                },

                Sum(s1, s2) => {
                    match (&**s1, &**s2) {
                        (Constant(_), Constant(_)) => Constant(0.0),
                        _ => todo!()
                    }
                },

                _ => todo!()
            },
        }
    }
}
