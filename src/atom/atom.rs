use atom::numbers::number::Numeric;
use atom::symbols::symbol::Symbol;
use atom::strings::string::SString;

use expression::structures::attributes::BaseExpression;
use expression::structures::attributes::PrimitiveConverter;

use parsing::utilities::numerics::representable_numeric;
use parsing::utilities::string::representable_string;
use parsing::utilities::symbols::representable_symbol;

extern crate decimal;
use decimal::d128;

extern crate num;
use num::{ToPrimitive, FromPrimitive};
use std::str::FromStr;

pub enum SimplexAtom {
    SimplexNumeric(Numeric),
    SimplexString(SString),
    SimplexSymbol(Symbol)
}

impl SimplexAtom {
    pub fn from_str(s: &str) -> Option<SimplexAtom> {
        if representable_numeric(s) {
            Some(SimplexAtom::SimplexNumeric(Numeric::from_str(s).unwrap()))
        } else if representable_string(s) {
            Some(SimplexAtom::SimplexString(SString::from_str(s).unwrap()))
        } else if representable_symbol(s) {
            Some(SimplexAtom::SimplexSymbol(Symbol::from_str(s).unwrap()))
        } else {
            None
        }
    }
}

impl BaseExpression for SimplexAtom {
    fn get_expression_name(&self) -> &str {
        "Simplex`Atom"
    }

    fn get_head_name(&self) -> &str {
        match self {
            &SimplexAtom::SimplexNumeric(num) => {
                match num.simplify() {
                    Numeric::LittleInteger(_) => "Integer",
                    Numeric::LittleReal(_) => "Real",
                    Numeric::NaN => "Symbol"

                }
            },
            &SimplexAtom::SimplexString(_) => "String",
            &SimplexAtom::SimplexSymbol(_) => "Symbol"
        }
    }
}

impl PrimitiveConverter for SimplexAtom {
    fn get_int_value(&self) -> Option<i64>{
        match self {
            &SimplexAtom::SimplexNumeric(numeric) => {
                match numeric.simplify() {
                    Numeric::LittleInteger(i) => Some(i),
                    _ => None
                }
            },
            &SimplexAtom::SimplexString(_) => None,
            &SimplexAtom::SimplexSymbol(_) => None 
        }
    }

    fn get_float_value(&self) -> Option<d128>{
        match self {
            &SimplexAtom::SimplexNumeric(numeric) => {
                match numeric.simplify() {
                    Numeric::LittleReal(i) => Some(i),
                    _ => None
                }
            },

            &SimplexAtom::SimplexString(_) => None,
            &SimplexAtom::SimplexSymbol(_) => None 
        }
    }

    fn get_string_value(&self) -> Option<&String>{
        match self {
            &SimplexAtom::SimplexNumeric(_) => None,
            &SimplexAtom::SimplexString(ref s) => Some(&s.contents),
            &SimplexAtom::SimplexSymbol(_) => None 
        }
    }
}
