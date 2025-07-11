use crate::{Term, ArenaTerm};
use bumpalo::Bump;

impl<'arena> Term<'arena> {
    pub fn differentiate_in(self, wrt: Term, arena: &'arena Bump) -> ArenaTerm<'arena> {
        use Term::*;

        if let Var(wrt) = wrt {
            match *self.simplify_in(&arena) { /* try with debug_simplify() */
                Constant(_) => ArenaTerm::new_in(Constant(0.0), &arena),
                
                Var(x) => match x {
                    _ if x == wrt => ArenaTerm::new_in(Constant(1.0), &arena),
                    _ => ArenaTerm::new_in(Constant(0.0), &arena)
                },

                Sum(t1, t2) => Sum(
                    ArenaTerm::new_in(*t1.simplify_in(&arena).differentiate_in(Var(wrt), &arena), &arena),
                    ArenaTerm::new_in(*t2.simplify_in(&arena).differentiate_in(Var(wrt), &arena), &arena)
                ).simplify_in(&arena),

                Scale { coefficient, term } => Scale {
                    coefficient: coefficient,
                    term: ArenaTerm::new_in(*term.differentiate_in(Var(wrt), &arena), &arena)
                }.simplify_in(&arena),

                Product(t1, t2) => Sum(
                    Product(
                        t1.clone().differentiate_in(Var(wrt), &arena),
                        t2.clone()
                    ).simplify_in(&arena),

                    Product(
                        t1,
                        t2.differentiate_in(Var(wrt), &arena)
                    ).simplify_in(&arena),
                ).simplify_in(&arena),

                Power { base, exponent } => match *exponent {
                    Constant(c) => ArenaTerm::new_in(
                        Product(
                            ArenaTerm::new_in(Scale {
                                coefficient: c,
                                term: ArenaTerm::new_in(Power {
                                    base: base.clone(),
                                    exponent: ArenaTerm::new_in(Constant(c - 1.0), &arena)
                                }, &arena)
                            }, &arena),

                            base.differentiate_in(Var(wrt), &arena)
                        ),
                    &arena),

                    _ => todo!("requires natural logarithmic terms"),
                },

                Sin(term) => ArenaTerm::new_in(Product(
                    ArenaTerm::new_in(Cos(term.clone()), &arena),
                    term.differentiate_in(Var(wrt), &arena)
                ), &arena),

                _ => todo!("didn't implement differentiation fully")
            }
        } else {
            todo!("can't differentiate with respect to a non-variable term as of now.")
        }
    }
}
