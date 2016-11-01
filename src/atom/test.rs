#[cfg(test)]
mod tests {
    mod test_intrinsics {
        use atom::atom::SimplexAtom;
        use expression::traits::BaseExpression;

        extern crate decimal;
        use decimal::d128;

        extern crate num;
        use std::str::FromStr;
    }

    mod test_basic_numbers {
        mod test_real_numbers {
            use atom::atom::SimplexAtom;
            use expression::traits::BaseExpression;

            extern crate decimal;
            use decimal::d128;

            extern crate num;
            use std::str::FromStr;

            #[test]
            fn it_instantiates() {
                let s_atom = SimplexAtom::from(1.21);
            }
        }

        mod test_int_numbers {
            use atom::atom::SimplexAtom;
            use expression::traits::BaseExpression;

            #[test]
            fn it_instantiates() {
                let s_atom = SimplexAtom::from(1.00);
            }
        }

        mod test_type_deduction {
            use atom::atom::SimplexAtom;
            use expression::traits::BaseExpression;

            #[test]
            fn it_instantiates() {
                let s_atom: SimplexAtom = SimplexAtom::from(32);
            }
        }
    }
}
