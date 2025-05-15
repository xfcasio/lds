use crate::Term;

impl Term {
    pub fn simplify(&self) -> Self {
        use Term::*;

        match self {
            Constant(_) | Var(_) => self.clone(),

            Sum(t1, t2) => match (*t1.clone(), *t2.clone()) {
                (Constant(c1), Constant(c2)) => Constant(c1 + c2),
                (_, _) => Sum(
                    Box::new(t1.simplify()),
                    Box::new(t2.simplify()),
                )
            },

            Scale { coefficient, term } => match **term {
                Constant(c) => Constant(coefficient * c),
                Var(_) => self.clone(),
                _ => Scale { coefficient: *coefficient, term: Box::new(term.clone().simplify()) }
            },

            Product(t1, t2) => match (*t1.clone(), *t2.clone()) {
                (Constant(c1), Constant(c2)) => Constant(c1 * c2),
                (_, _) => Product(
                    Box::new(t1.simplify()),
                    Box::new(t2.simplify())
                )
            },

            Power { base, exponent } => match (*base.clone(), *exponent.clone()) {
                (Constant(c1), Constant(c2)) => Constant(c1.powf(c2)),
                (_, _) => Power { base: Box::new(base.simplify()), exponent: Box::new(exponent.simplify()) }
            },

            Exponential(base, term) => match **term {
                Constant(c) => Constant(base.powf(c)),
                _ => Exponential(*base, Box::new(term.simplify()))
            },

            Sin(term) => match **term {
                Constant(c) => Constant(c.sin()),
                _ => Sin(Box::new(term.simplify()))
            },

            Cos(term) => match **term {
                Constant(c) => Constant(c.cos()),
                _ => Cos(Box::new(term.simplify()))
            },

            Derivative { order, wrt, term } => match *term.clone() {
                Constant(c) => match order {
                    0 => Constant(c),
                    _ => Constant(0.0)
                },

                Var(_) => match (wrt, order) {
                    (term, 0) => *term.clone(),
                    (term, 1) => Constant(1.0),
                    (_, _) => Constant(0.0)
                },
                
                Sum(s1, s2) => match (*s1, *s2) {
                    (Constant(_), Constant(_)) => Constant(0.0),
                    _ => todo!()
                },
                
                _ => todo!()
            }
        }
    }
}
