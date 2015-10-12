use ast::Term;
use ast::Term::*;
use self::Error::*;

#[derive(Debug)]
pub enum Error {
    IsZeroOfBool,
    SuccOrPredOfNonNat,
    IfNonBool,
}

fn isnumeric(v: &Term) -> bool {
    match *v {
        Pred(ref f) | Succ(ref f) => isnumeric(f),
        Zero => true,
        _ => false
    }
}

// a small-step evaluator
pub fn small_eval(v: &Term) -> Term {
    match *v {
        If(ref con, ref the, ref els) => match **con {
            True => *the.clone(),
            False => *els.clone(),
            _ => If(Box::new(small_eval(con)), the.clone(), els.clone()),
        },
        Succ(ref s) => Succ(Box::new(small_eval(s))),
        Pred(ref s) => match **s {
            Zero => Zero,
            Succ(ref v) if isnumeric(v) => {
                *v.clone()
            },
            _ => Pred(Box::new(small_eval(s))),
        },
        IsZero(ref s) => match **s {
            Zero => True,
            Succ(ref s) if isnumeric(s) => False,
            _ => IsZero(Box::new(small_eval(s))),
        },
        _ => panic!("No rule applies")
    }
}

// a big-step evaluator
pub fn big_eval(v: &Term) -> Result<Term, Error> {
    match *v {
        True => Ok(True),
        False => Ok(False),
        Zero => Ok(Zero),
        If(ref con, ref the, ref els) => match try!(big_eval(con)) {
            True => big_eval(the),
            False => big_eval(els),
            _ => Err(IfNonBool)
        },
        Succ(ref v) => match try!(big_eval(v)) {
            Succ(s) => Ok(Succ(s)),
            Pred(s) => Ok(*s),
            Zero => Ok(Succ(Box::new(Zero))),
            _ => Err(SuccOrPredOfNonNat),
        },
        Pred(ref v) => match try!(big_eval(v)) {
            Succ(s) => Ok(*s),
            Pred(s) => Ok(Pred(s)),
            Zero => Ok(Zero),
            _ => Err(SuccOrPredOfNonNat),
        },
        IsZero(ref v) => match try!(big_eval(v)) {
            Pred(_) | Succ(_) => Ok(False),
            Zero => Ok(True),
            _ => Err(IsZeroOfBool)
        }
    }
}
