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
