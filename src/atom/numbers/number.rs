use std::io;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::borrow::Cow;
use parsing::utilities::numerics::get_representable_integer;

extern crate decimal;
use decimal::d128;

extern crate num;
use num::{ToPrimitive, FromPrimitive};
use std::str::FromStr;

// TODO: Make emulated integer using d128.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Numeric {
    LittleInteger(i64),
    LittleReal(d128),
    NaN
}

impl Numeric {
    pub fn from_str(s: &str) -> Numeric {
        match s.parse::<i64>() {
            Ok(num) => {
                Numeric::LittleInteger(num)
            }, Err(_) => {
                match d128::from_str(s) {
                    Ok(num) => {
                        if num.to_string() != "NaN" {
                          Numeric::LittleReal(num)
                        } else {
                          Numeric::NaN
                        }
                    }, Err(_) => {
                        // In reality, this should be offloaded to
                        // arbitrary precision structure.
                        Numeric::NaN
                    }
                }
            }
        }
    }

    pub fn as_str<'a>(&'a self) -> Cow<'a, str> {
        match self {
            &Numeric::LittleInteger(i) => {
                Cow::Owned(i.to_string())
            }, &Numeric::LittleReal(ref r) => {
                Cow::Owned(r.to_string())
            }, &Numeric::NaN => {
                Cow::Borrowed("NaN")
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            &Numeric::LittleInteger(i) => {
                format!("{}", i)
            }, &Numeric::LittleReal(ref r) => {
                format!("{}", r)
            }, &Numeric::NaN => {
                format!("NaN")
            }
        }
    }

    pub fn simplify(self) -> Numeric {
        match self {
            Numeric::LittleReal(r) => {
                let real_number = r.to_string();

                match get_representable_integer(real_number.as_str()) {
                    Some(num) => {
                        Numeric::LittleInteger(num)
                    },

                    None => {
                        if real_number.contains("NaN") {
                            Numeric::NaN
                        } else {
                            self
                        }
                    }
                }
            }, _ => {
               self
            }
        }
    }

    pub fn capacity(&self) -> usize {
        match self {
            &Numeric::LittleInteger(_) => {
                64
            },
            &Numeric::LittleReal(_) => {
                128
            },
            &Numeric::NaN => {
                8
            }
        }
    }

    pub fn head(&self) -> &str {
        match self {
            &Numeric::LittleInteger(_) => {
                "Simplex`Integer"
            }, &Numeric::LittleReal(_) => {
                "Simplex`Real"
            }, &Numeric::NaN => {
                "Simplex`NaN"
            }
        }
    }
}

impl Add for Numeric {
    type Output = Numeric;

    fn add(self, other: Numeric) -> Numeric {
        //TODO: Use d128 constructor more intellegently: This is extremely slow.
        match (self, other) {
            (Numeric::NaN, _) => {
                Numeric::NaN
            }, (_, Numeric::NaN) => {
                Numeric::NaN
            }, (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleInteger(lhs + rhs)
            }, (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() + rhs).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs + d128::from_str(rhs.to_string().as_str()).unwrap()).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs + rhs).simplify()
            }
        }
    }
}

impl Sub for Numeric {
    type Output = Numeric;

    fn sub(self, other: Numeric) -> Numeric {
        match (self, other) {
            (Numeric::NaN, _) => {
                Numeric::NaN
            }, (_, Numeric::NaN) => {
                Numeric::NaN
            }, (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleInteger(lhs - rhs)
            }, (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() - rhs).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs - d128::from_str(rhs.to_string().as_str()).unwrap()).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs - rhs).simplify()
            }
        }
    }
}

impl Mul for Numeric {
    type Output = Numeric;

    fn mul(self, other: Numeric) -> Numeric {
        //TODO: Use d128 constructor more intellegently: This is extremely slow.
        match (self, other) {
            (Numeric::NaN, _) => {
                Numeric::NaN
            }, (_, Numeric::NaN) => {
                Numeric::NaN
            }, (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleInteger(lhs * rhs)
            }, (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() * rhs).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs * d128::from_str(rhs.to_string().as_str()).unwrap()).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs * rhs).simplify()
            }
        }
    }
}

impl Div for Numeric {
    type Output = Numeric;

    fn div(self, other: Numeric) -> Numeric {
        //TODO: Use d128 constructor more intellegently: This is extremely slow.
        match (self, other) {
            (Numeric::NaN, _) => {
                Numeric::NaN
            }, (_, Numeric::NaN) => {
                Numeric::NaN
            }, (Numeric::LittleInteger(lhs), Numeric::LittleInteger(rhs)) => {
                //Unsafe, very very unsafe.
                // This should use two d128s wrapped from lhs and rhs as division.
                Numeric::LittleReal(d128::from_str((lhs as f64 / rhs as f64).to_string().as_str()).unwrap()).simplify()
            }, (Numeric::LittleInteger(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(d128::from_str(lhs.to_string().as_str()).unwrap() / rhs).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleInteger(rhs)) => {
                Numeric::LittleReal(lhs / d128::from_str(rhs.to_string().as_str()).unwrap()).simplify()
            }, (Numeric::LittleReal(lhs), Numeric::LittleReal(rhs)) => {
                Numeric::LittleReal(lhs / rhs).simplify()
            }
        }
    }
}
