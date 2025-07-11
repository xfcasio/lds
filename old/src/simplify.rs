use crate::{Term, ArenaTerm};
use bumpalo::Bump;

// consider adding SimplifyState enum with Deref<Target = Term> for cross-recursion communication

impl<'arena> Term<'arena> {
    pub fn simplify_in(self, arena: &'arena Bump) -> ArenaTerm<'arena> {
        use Term::*;

        match self {
            Constant(_) | Var(_) => ArenaTerm::new_in(self, &arena),

            Sum(t1, t2) => match (*t1, *t2) {
                (Constant(c1), Constant(c2)) => ArenaTerm::new_in(Constant(c1 + c2), &arena),

                (t, Constant(0.0)) => ArenaTerm::new_in(t, &arena),
                (Constant(0.0), t) => ArenaTerm::new_in(t, &arena),

                (t1, t2) => {
                    let t1 = t1.simplify_in(&arena);
                    let t2 = t2.simplify_in(&arena);

                    ArenaTerm::new_in(Sum(t1, t2), &arena)
                }
            },

            Scale { coefficient, term } => match *term {
                Constant(c) => ArenaTerm::new_in(Constant(coefficient * c), &arena),

                Scale {
                    coefficient: coefficient2,
                    term: term2,
                } => Scale { coefficient: coefficient * coefficient2, term: term2 }.simplify_in(&arena),

                /* distributivity */
                Sum(t1, t2) => Sum(
                    Scale { coefficient: coefficient, term: t1.simplify_in(&arena) }.simplify_in(&arena),
                    Scale { coefficient: coefficient, term: t2.simplify_in(&arena) }.simplify_in(&arena),
                ).simplify_in(&arena),

                term if matches!(
                    term,
                    Var(_) | Sin(_) | Cos(_) | Power { .. } | Exponential(_, _) | Derivative { .. }
                ) => ArenaTerm::new_in(Scale { coefficient: coefficient, term: ArenaTerm::new_in(term, &arena) }, &arena),

                _ => ArenaTerm::new_in(Scale { coefficient: coefficient, term: term.simplify_in(&arena) }, &arena),
            },

            Product(t1, t2) => match (*t1, *t2) {
                (Constant(c1), Constant(c2)) => ArenaTerm::new_in(Constant(c1 * c2), &arena),

                (_, Constant(0.0)) => ArenaTerm::new_in(Constant(0.0), &arena),
                (Constant(0.0), _) => ArenaTerm::new_in(Constant(0.0), &arena),

                (t, Constant(1.0)) => ArenaTerm::new_in(t, &arena),
                (Constant(1.0), t) => ArenaTerm::new_in(t, &arena),

                (t1, t2) => {
                    let t1 = t1.simplify_in(&arena);
                    let t2 = t2.simplify_in(&arena);
                    ArenaTerm::new_in(Product(t1, t2), &arena)
                }
            },

            Power { base, exponent } => match (*base, *exponent) {
                (Constant(c1), Constant(c2)) => ArenaTerm::new_in(Constant(c1.powf(c2)), &arena),
                (Constant(c), exponent) => ArenaTerm::new_in(Exponential(c, ArenaTerm::new_in(exponent, &arena)), &arena),

                (base, exponent) => {
                    let base = base.simplify_in(&arena);
                    let exponent = exponent.simplify_in(&arena);

                    ArenaTerm::new_in(Power {
                        base: base,
                        exponent: exponent,
                    }, &arena)
                }
            },

            Exponential(base, term) => match *term {
                Constant(c) => ArenaTerm::new_in(Constant(base.powf(c)), &arena),
                _ => {
                    let term = term.simplify_in(&arena);
                    ArenaTerm::new_in(Exponential(base, term), &arena)
                }
            },

            Sin(term) => match *term {
                Constant(c) => ArenaTerm::new_in(Constant(c.sin()), &arena),
                term => ArenaTerm::new_in(Sin(term.simplify_in(&arena)), &arena),
            },

            Cos(term) => match *term {
                Constant(c) => ArenaTerm::new_in(Constant(c.cos()), &arena),
                _ => ArenaTerm::new_in(Cos(term.simplify_in(&arena)), &arena),
            },

            Derivative { order, wrt, term } => {
                match *term.simplify_in(&arena) {
                    Constant(c) => match order {
                        0 => ArenaTerm::new_in(Constant(c), &arena),
                        _ => ArenaTerm::new_in(Constant(0.0), &arena),
                    },

                    Var(_) => match (wrt, order) {
                        (term, 0) => term,
                        (_, 1) => ArenaTerm::new_in(Constant(1.0), &arena),
                        (_, _) => ArenaTerm::new_in(Constant(0.0), &arena),
                    },

                    term => ArenaTerm::new_in(Derivative {
                        order: order,
                        wrt,
                        term: ArenaTerm::new_in(term, &arena),
                    }, &arena),
                }
            }
        }
    }

    pub fn debug_simplify_in(self, arena: &'arena Bump) -> Self {
        fn recursive_simplify<'arena>(__self: Term<'arena>, depth: usize, arena: &'arena Bump) -> Term<'arena> {
            use Term::*;

            println!("{}<simplifying {}> {{", "   ".repeat(depth), __self);

            let simplified = match __self {
                Constant(_) | Var(_) => __self,

                Sum(t1, t2) => match (*t1, *t2) {
                    (Constant(c1), Constant(c2)) => Constant(c1 + c2),

                    (t1, t2) => {
                        let t1 = recursive_simplify(t1, depth + 1, &arena);
                        let t2 = recursive_simplify(t2, depth + 1, &arena);
                        Sum(ArenaTerm::new_in(t1, &arena), ArenaTerm::new_in(t2, &arena))
                    }
                },

                Scale { coefficient, term } => match *term {
                    Constant(c) => Constant(coefficient * c),
                    Scale {
                        coefficient: coefficient2,
                        term: term2,
                    } => recursive_simplify(Scale { coefficient: coefficient * coefficient2, term: term2 }, depth + 1, &arena),

                    /* distributivity */
                    Sum(t1, t2) => {
                        let t1 = recursive_simplify(*t1, depth + 1, &arena);
                        let t2 = recursive_simplify(*t2, depth + 1, &arena);

                        let s1 = ArenaTerm::new_in(recursive_simplify(Scale {
                            coefficient,
                            term: ArenaTerm::new_in(t1, &arena),
                        }, depth + 1, &arena), &arena);

                        let s2 = ArenaTerm::new_in(recursive_simplify(Scale {
                            coefficient,
                            term: ArenaTerm::new_in(t2, &arena),
                        }, depth + 1, &arena), &arena);

                        recursive_simplify(Sum(s1, s2), depth + 1, &arena)
                    }

                    term if matches!(
                        term,
                        Var(_) | Sin(_) | Cos(_) | Power { .. } | Exponential(_, _) | Derivative { .. }
                    ) => Scale { coefficient, term: ArenaTerm::new_in(term, &arena) },

                    _ => Scale {
                        coefficient,
                        term: ArenaTerm::new_in(
                            recursive_simplify(*term, depth + 1, &arena),
                            &arena
                        )
                    },
                },

                Product(t1, t2) => match (*t1, *t2) {
                    (Constant(c1), Constant(c2)) => Constant(c1 * c2),

                    (t1, t2) => {
                        let t1 = recursive_simplify(t1, depth + 1, &arena);
                        let t2 = recursive_simplify(t2, depth + 1, &arena);
                        Product(ArenaTerm::new_in(t1, &arena), ArenaTerm::new_in(t2, &arena))
                    }
                },

                Power { base, exponent } => match (*base, *exponent) {
                    (Constant(c1), Constant(c2)) => Constant(c1.powf(c2)),
                    (base, exponent) => {
                        let base = recursive_simplify(base, depth + 1, &arena);
                        let exponent = recursive_simplify(exponent, depth + 1, &arena);
                        Power {
                            base: ArenaTerm::new_in(base, &arena),
                            exponent: ArenaTerm::new_in(exponent, &arena),
                        }
                    }
                },

                Exponential(base, term) => match *term {
                    Constant(c) => Constant(base.powf(c)),
                    _ => {
                        let term = recursive_simplify(*term, depth + 1, &arena);
                        Exponential(base, ArenaTerm::new_in(term, &arena))
                    }
                },

                Sin(term) => match *term {
                    Constant(c) => Constant(c.sin()),
                    term => Sin(ArenaTerm::new_in(recursive_simplify(term, depth + 1, &arena), &arena)),
                },

                Cos(term) => match *term {
                    Constant(c) => Constant(c.cos()),
                    _ => Cos(ArenaTerm::new_in(recursive_simplify(*term, depth + 1, &arena), &arena)),
                },

                Derivative { order, wrt, term } => {
                    match recursive_simplify(*term, depth + 1, &arena) {
                        Constant(c) => match order {
                            0 => Constant(c),
                            _ => Constant(0.0),
                        },

                        Var(_) => match (wrt, order) {
                            (term, 0) => *term,
                            (_, 1) => Constant(1.0),
                            (_, _) => Constant(0.0),
                        },

                        term => Derivative {
                            order,
                            wrt,
                            term: ArenaTerm::new_in(term, &arena),
                        },
                    }
                }
            };

            println!("{}}}", "   ".repeat(depth));
            simplified
        }

        recursive_simplify(self, 0, arena)
    }
}
