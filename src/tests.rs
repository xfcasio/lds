#[cfg(test)]
mod tests {
    use crate::Term::*;

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
            Derivative { order: 3, wrt: Box::new(Var('x')), term: Box::new(fterm) },
            Derivative { order: 3, wrt: Box::new(Var('x')), term: Box::new(dterm) },
        ];

        for t in terms { println!("= {}\n\n", t.debug_simplify()); }
    }
}
