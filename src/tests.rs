#[cfg(test)]
mod tests {
    use crate::Term::*;
    use crate::ArenaTerm;
    use bumpalo::Bump;

    #[test]
    fn it_works() {
        let arena = Bump::new();

        let fterm = Scale {
            coefficient: 5.0,
            term: ArenaTerm::new_in(Sum(
                ArenaTerm::new_in(Constant(5.0), &arena),
                ArenaTerm::new_in(Constant(10.0), &arena)
            ), &arena)
        };

        let dterm = Scale {
            coefficient: 2.0,
            term: ArenaTerm::new_in(Sum(
                ArenaTerm::new_in(Scale {
                    coefficient: 3.0,
                    term: ArenaTerm::new_in(Scale {
                        coefficient: 7.0,
                        term: ArenaTerm::new_in(Sin(ArenaTerm::new_in(Var('x'), &arena)), &arena) 
                    }, &arena)
                }, &arena),

                ArenaTerm::new_in(Scale {
                    coefficient: 3.0,
                    term: ArenaTerm::new_in(Scale {
                        coefficient: 7.0,
                        term: ArenaTerm::new_in(Cos(ArenaTerm::new_in(Var('x'), &arena)), &arena) 
                    }, &arena)
                }, &arena)
            ), &arena)
        };

        let terms = [
            Scale {
                coefficient: 2.0,
                term: ArenaTerm::new_in(Scale {
                    coefficient: 3.0,
                    term: ArenaTerm::new_in(Scale {
                        coefficient: 2.0,
                        term: ArenaTerm::new_in(Var('x'), &arena)
                    }, &arena)
                }, &arena)
            },
            
            Scale {
                coefficient: 2.0,
                term: ArenaTerm::new_in(Scale {
                    coefficient: 3.0,
                    term: ArenaTerm::new_in(Scale {
                        coefficient: 7.0,
                        term: ArenaTerm::new_in(Sin(ArenaTerm::new_in(Var('x'), &arena)), &arena)
                    }, &arena)
                }, &arena)
            },
            
            Derivative { order: 3, wrt: ArenaTerm::new_in(Var('x'), &arena), term: ArenaTerm::new_in(fterm, &arena) },
            Derivative { order: 3, wrt: ArenaTerm::new_in(Var('x'), &arena), term: ArenaTerm::new_in(dterm, &arena) },
        ];

        for t in terms { println!("= {}\n\n", t.debug_simplify_in(&arena) ); }

        let deriv = Derivative { order: 0, wrt: ArenaTerm::new_in(Var('t'), &arena), term: ArenaTerm::new_in(Scale {
            coefficient: 5.0,
            term: ArenaTerm::new_in(Sum(ArenaTerm::new_in(Constant(5.0), &arena), ArenaTerm::new_in(Constant(10.0), &arena)), &arena)
        }, &arena)};

        let deriv_sum = Sum(
            ArenaTerm::new_in(Scale {
                coefficient: 2.0,
                term: ArenaTerm::new_in(Scale {
                    coefficient: 3.0,
                    term: ArenaTerm::new_in(Scale {
                        coefficient: 2.0,
                        term: ArenaTerm::new_in(Var('x'), &arena)
                    }, &arena)
                }, &arena)
            }, &arena),

            ArenaTerm::new_in(Sum(
                ArenaTerm::new_in(Constant(5.0), &arena),
                ArenaTerm::new_in(Constant(10.0), &arena)
            ), &arena)
        );

        let deriv_product = Product(
            ArenaTerm::new_in(Scale { coefficient: 3.0, term: ArenaTerm::new_in(Var('x'), &arena) }, &arena),
            ArenaTerm::new_in(Var('b'), &arena)
        );

        let deriv_sin = Scale {
            coefficient: 12.5,
            term: ArenaTerm::new_in(Sin(ArenaTerm::new_in(Sum(
                ArenaTerm::new_in(Power {
                    base: ArenaTerm::new_in(Var('x'), &arena),
                    exponent: ArenaTerm::new_in(Constant(3.0), &arena)
                }, &arena),
                ArenaTerm::new_in(Constant(-10.0), &arena)
            ), &arena)), &arena)
        };

        println!("{} -> {} =diff= {}",
            deriv,
            deriv.clone().simplify_in(&arena),
            deriv.clone().differentiate_in(Var('x'), &arena)
        );

        println!("{} -> {} ~~> {}",
            deriv_sum,
            deriv_sum.clone().simplify_in(&arena),
            deriv_sum.clone().differentiate_in(Var('x'), &arena)
        );

        println!("{} ~> {}",
            deriv_product.clone(),
            deriv_product.differentiate_in(Var('x'), &arena)
        );

        /*println!("{} ~> {}",
            deriv_sin.clone(),
            deriv_sin
                .differentiate_in(Var('x'), &arena)
                .differentiate_in(Var('x'), &arena)
        );*/
    }
}
