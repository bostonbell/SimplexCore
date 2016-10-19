#[cfg(test)]
mod tests {
    mod test_basic_numbers {
        mod test_real_numbers  {
            use atom::atom::SimplexAtom;
            use expression::structures::attributes::BaseExpression;
            use expression::structures::attributes::PrimitiveConverter;

            extern crate decimal;
            use decimal::d128;

            extern crate num;
            use num::{ToPrimitive, FromPrimitive};
            use std::str::FromStr;

            #[test]
            fn it_instantiates() {
                let s_atom = SimplexAtom::from_str("1.21");
                assert_eq!(s_atom.get_expression_name(), "Simplex`Atom");
                assert_eq!(s_atom.get_head_name(), "Numeric");
            }

            #[test]
            fn it_gets_float_value() {
                let s_atom = SimplexAtom::from_str("100.201");
                assert_eq!(s_atom.get_float_value(), Some(d128::from_str("100.201").unwrap()));
            }

            #[test]
            fn it_doesnt_get_int_value() {
                let s_atom = SimplexAtom::from_str("100.201");
                assert_eq!(s_atom.get_int_value(), None);
            }

            #[test]
            fn it_doesnt_get_string_value() {
                let s_atom = SimplexAtom::from_str("100.201");
                assert_eq!(s_atom.get_string_value(), None);
            }
        }

        mod test_int_numbers  {
            use atom::atom::SimplexAtom;
            use expression::structures::attributes::BaseExpression;
            use expression::structures::attributes::PrimitiveConverter;

            extern crate decimal;
            use decimal::d128;

            extern crate num;
            use num::{ToPrimitive, FromPrimitive};
            use std::str::FromStr;

            #[test]
            fn it_instantiates() {
                let s_atom = SimplexAtom::from_str("1.00");
                assert_eq!(s_atom.get_expression_name(), "Simplex`Atom");
                assert_eq!(s_atom.get_head_name(), "Numeric");
            }

            #[test]
            fn it_gets_int_value() {
                let s_atom = SimplexAtom::from_str("100.000");
                assert_eq!(s_atom.get_int_value(), Some(100));
            }

            #[test]
            fn it_doesnt_get_float_value() {
                let s_atom = SimplexAtom::from_str("100.000");
                assert_eq!(s_atom.get_float_value(), None);
            }

            #[test]
            fn it_doesnt_get_string_value() {
                let s_atom = SimplexAtom::from_str("100.00");
                assert_eq!(s_atom.get_string_value(), None);
            }
        }
    }
}