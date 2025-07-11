use std::fmt;
use crate::Term;

impl std::fmt::Display for Term<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Term::*;

        match self {
            Constant(n) => write!(f, "{n}"),
            Var(c) => write!(f, "{c}"),
            Sum(t1, t2) => write!(f, "{t1} + {t2}"),

            Scale { coefficient, term } => match *term.clone() {
                Sum(s1, s2) => write!(f, "{coefficient}({s1} + {s2})"),
                Scale { coefficient: coefficient2, term: term2 } => write!(f, "{coefficient}*{coefficient2}*{}", term2),
                _ => write!(f, "{coefficient}*{term}")
            },

            Product(t1, t2) => match (*t1.clone(), *t2.clone()) {
                (Var(c), Sum(s1, s2)) => write!(f, "{c}({s1} + {s2})"),
                (Sum(s1, s2), Var(c)) => write!(f, "({s1} + {s2}){c}"),
                (t1, Scale { coefficient, term }) => write!(f, "{coefficient}{t1}{term}"),
                (_, _) => write!(f, "{t1}{t2}")
            },

            Power { base, exponent } => match (*base.clone(), *exponent.clone()) {
                (Sum(s1, s2), Sum(s3, s4)) => write!(f, "({s1} + {s2})^({s3} + {s4})"),
                (Sum(s1, s2), exponent) => write!(f, "({s1} + {s2})^{exponent}"),
                (base, Sum(s1, s2)) => write!(f, "{base}^({s1} + {s2})"),
                (Product(s1, s2), exponent) => write!(f, "({s1} * {s2})^{exponent}"),
                (base, Product(s1, s2)) => write!(f, "{base}^({s1} * {s2})"),
                (base, exponent) => write!(f, "{base}^{exponent}")
            },

            Exponential(base, t) => match *t.clone() {
                Sum(s1, s2) => write!(f, "{base}^({s1} + {s2})"),
                Product(s1, s2) => write!(f, "{base}^({s1} * {s2})"),
                _ => write!(f, "{base}^{}", **t)
            },

            Sin(t) => write!(f, "sin({t})"),
            Cos(t) => write!(f, "cos({t})"),

            Derivative { order, wrt, term } => write!(f, "({term}, wrt: {}){}", wrt, "\'".repeat(*order)),
        }
    }
}
