use crate::Term;

impl Term {
    pub fn simplify(self) -> Self {
        use Term::*;

        match self {
            Constant(_) | Var(_) => self,

            Sum(t1, t2) => match (*t1, *t2) {
                (Constant(c1), Constant(c2)) => Constant(c1 + c2),

                (t1, t2) => {
                    let t1 = t1.simplify();
                    let t2 = t2.simplify();
                    Sum(Box::new(t1), Box::new(t2))
                }
            },

            Scale { coefficient, term } => match *term {
                Constant(c) => Constant(coefficient * c),

                Scale {
                    coefficient: coefficient2,
                    term: term2,
                } => Scale { coefficient: coefficient * coefficient2, term: term2 }.simplify(),

                /* distributivity */
                Sum(t1, t2) => Sum(
                    Box::new(
                        Scale { coefficient, term: Box::new(t1.simplify()) }.simplify(),
                    ),
                    Box::new(
                        Scale { coefficient, term: Box::new(t2.simplify()) }.simplify(),
                    ),
                ).simplify(),

                term if matches!(
                    term,
                    Var(_) | Sin(_) | Cos(_) | Power { .. } | Exponential(_, _) | Derivative { .. }
                ) => Scale { coefficient, term: Box::new(term) },

                _ => Scale { coefficient, term: Box::new(term.simplify()) },
            },

            Product(t1, t2) => match (*t1, *t2) {
                (Constant(c1), Constant(c2)) => Constant(c1 * c2),

                (t1, t2) => {
                    let t1 = t1.simplify();
                    let t2 = t2.simplify();
                    Product(Box::new(t1), Box::new(t2))
                }
            },

            Power { base, exponent } => match (*base, *exponent) {
                (Constant(c1), Constant(c2)) => Constant(c1.powf(c2)),

                (base, exponent) => {
                    let base = base.simplify();
                    let exponent = exponent.simplify();
                    Power {
                        base: Box::new(base),
                        exponent: Box::new(exponent),
                    }
                }
            },

            Exponential(base, term) => match *term {
                Constant(c) => Constant(base.powf(c)),
                _ => {
                    let simplified_term = term.simplify();
                    Exponential(base, Box::new(simplified_term))
                }
            },

            Sin(term) => match *term {
                Constant(c) => Constant(c.sin()),
                term => Sin(Box::new(term.simplify())),
            },

            Cos(term) => match *term {
                Constant(c) => Constant(c.cos()),
                _ => Cos(Box::new(term.simplify())),
            },

            Derivative { order, wrt, term } => {
                match term.simplify() {
                    Constant(c) => match order {
                        0 => Constant(c),
                        _ => Constant(0.0),
                    },

                    Var(_) => match (wrt, order) {
                        (term, 0) => (*term),
                        (_, 1) => Constant(1.0),
                        (_, _) => Constant(0.0),
                    },

                    term => Derivative {
                        order,
                        wrt,
                        term: Box::new(term),
                    },
                }
            }
        }
    }

    pub fn debug_simplify(self) -> Self {
        fn recursive_simplify(__self: Term, depth: usize) -> Term {
            use Term::*;

            println!("{}<simplifying {}> {{", "   ".repeat(depth), __self);

            let simplified = match __self {
                Constant(_) | Var(_) => __self,

                Sum(t1, t2) => match (*t1, *t2) {
                    (Constant(c1), Constant(c2)) => Constant(c1 + c2),

                    (t1, t2) => {
                        let t1 = recursive_simplify(t1, depth + 1);
                        let t2 = recursive_simplify(t2, depth + 1);
                        Sum(Box::new(t1), Box::new(t2))
                    }
                },

                Scale { coefficient, term } => match *term {
                    Constant(c) => Constant(coefficient * c),
                    Scale {
                        coefficient: coefficient2,
                        term: term2,
                    } => recursive_simplify(Scale { coefficient: coefficient * coefficient2, term: term2 }, depth + 1),

                    /* distributivity */
                    Sum(t1, t2) => {
                        let t1_simplified = recursive_simplify(*t1, depth + 1);
                        let t2_simplified = recursive_simplify(*t2, depth + 1);

                        let s1_simplified = Box::new(recursive_simplify(Scale {
                            coefficient,
                            term: Box::new(t1_simplified),
                        }, depth + 1));

                        let s2_simplified = Box::new(recursive_simplify(Scale {
                            coefficient,
                            term: Box::new(t2_simplified),
                        }, depth + 1));

                        recursive_simplify(Sum(s1_simplified, s2_simplified), depth + 1)
                    }

                    term if matches!(
                        term,
                        Var(_) | Sin(_) | Cos(_) | Power { .. } | Exponential(_, _) | Derivative { .. }
                    ) => Scale { coefficient, term: Box::new(term) },

                    _ => Scale { coefficient, term: Box::new(recursive_simplify(*term, depth + 1)) },
                },

                Product(t1, t2) => match (*t1, *t2) {
                    (Constant(c1), Constant(c2)) => Constant(c1 * c2),

                    (t1, t2) => {
                        let t1 = recursive_simplify(t1, depth + 1);
                        let t2 = recursive_simplify(t2, depth + 1);
                        Product(Box::new(t1), Box::new(t2))
                    }
                },

                Power { base, exponent } => match (*base, *exponent) {
                    (Constant(c1), Constant(c2)) => Constant(c1.powf(c2)),
                    (base, exponent) => {
                        let base = recursive_simplify(base, depth + 1);
                        let exponent = recursive_simplify(exponent, depth + 1);
                        Power {
                            base: Box::new(base),
                            exponent: Box::new(exponent),
                        }
                    }
                },

                Exponential(base, term) => match *term {
                    Constant(c) => Constant(base.powf(c)),
                    _ => {
                        let simplified_term = recursive_simplify(*term, depth + 1);
                        Exponential(base, Box::new(simplified_term))
                    }
                },

                Sin(term) => match *term {
                    Constant(c) => Constant(c.sin()),
                    term => Sin(Box::new(recursive_simplify(term, depth + 1))),
                },

                Cos(term) => match *term {
                    Constant(c) => Constant(c.cos()),
                    _ => Cos(Box::new(recursive_simplify(*term, depth + 1))),
                },

                Derivative { order, wrt, term } => {
                    match recursive_simplify(*term, depth + 1) {
                        Constant(c) => match order {
                            0 => Constant(c),
                            _ => Constant(0.0),
                        },

                        Var(_) => match (wrt, order) {
                            (term, 0) => (*term),
                            (_, 1) => Constant(1.0),
                            (_, _) => Constant(0.0),
                        },

                        term => Derivative {
                            order,
                            wrt,
                            term: Box::new(term),
                        },
                    }
                }
            };

            println!("{}}}", "   ".repeat(depth));
            simplified
        }

        recursive_simplify(self, 0)
    }
}
