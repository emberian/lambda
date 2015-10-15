use std::fmt;
#[derive(Debug, Clone)]
pub enum Term {
    True,
    False,
    Zero,
    If(Box<Term>, Box<Term>, Box<Term>),
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
}

use self::Term::*;

impl Term {
    fn to_integer(&self) -> Option<u64> {
        match *self {
            Pred(ref a) => a.to_integer().map(|a| a.saturating_sub(1)),
            Succ(ref a) => a.to_integer().map(|a| a.saturating_add(1)),
            _ => None
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            True => f.write_str("true"),
            False => f.write_str("false"),
            Zero => f.write_str("0"),
            If(ref a, ref b, ref c) => write!(f, "if {} then {} else {}", a, b, c),
            Pred(_) | Succ(_) => write!(f, "{:?}", self.to_integer()),
            IsZero(ref a) => write!(f, "iszero {}", a)
        }
    }
}
