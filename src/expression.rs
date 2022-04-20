extern crate alloc;

use crate::Environment;
use crate::Error;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use core::fmt;
use core::ops::Neg;
use core::str::FromStr;

#[derive(Clone)]
pub struct FnBody(pub fn(&mut Environment) -> Result<Expression, Error>);

#[derive(Clone)]
pub enum Expression {
    Bool(bool),
    Function(String, FnBody),
    List(Vec<Expression>),
    Nil,
    Number(i64),
    Symbol(String),
    Vector(Vec<Expression>),
}

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let res = match self {
            Expression::Number(x) => Expression::Number(-x),
            _ => panic!(),
        };
        res
    }
}

impl Eq for Expression {}

impl Ord for Expression {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Expression::Bool(l), Expression::Bool(r)) => l.cmp(r),
            (Expression::List(l), Expression::List(r)) => {
                match l
                    .iter()
                    .zip(r.iter())
                    .map(|(l, r)| l.cmp(r))
                    .find(|&c| c != Ordering::Equal)
                {
                    Some(order) => order,
                    None => Ordering::Equal,
                }
            }
            (Expression::Nil, Expression::Nil) => Ordering::Equal,
            (Expression::Number(l), Expression::Number(r)) => l.cmp(r),
            (Expression::Symbol(l), Expression::Symbol(r)) => l.cmp(r),
            (Expression::Vector(l), Expression::Vector(r)) => {
                match l
                    .iter()
                    .zip(r.iter())
                    .map(|(l, r)| l.cmp(r))
                    .find(|&c| c != Ordering::Equal)
                {
                    Some(order) => order,
                    None => Ordering::Equal,
                }
            }
            _ => panic!(),
        }
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expression::Bool(l), Expression::Bool(r)) => l == r,
            (Expression::List(l), Expression::List(r)) => {
                (l.len() == r.len())
                    && (l
                        .iter()
                        .zip(r.iter())
                        .map(|(l, r)| l == r)
                        .reduce(|acc, x| acc && x)
                        .unwrap_or(false))
            }
            (Expression::Nil, Expression::Nil) => true,
            (Expression::Number(l), Expression::Number(r)) => l == r,
            (Expression::Symbol(l), Expression::Symbol(r)) => l == r,
            (Expression::Vector(l), Expression::Vector(r)) => {
                (l.len() == r.len())
                    && (l
                        .iter()
                        .zip(r.iter())
                        .map(|(l, r)| l == r)
                        .reduce(|acc, x| acc && x)
                        .unwrap_or(false))
            }
            _ => false,
        }
    }
}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Expression::Bool(_), Expression::Bool(_))
            | (Expression::Nil, Expression::Nil)
            | (Expression::Number(_), Expression::Number(_))
            | (Expression::Symbol(_), Expression::Symbol(_)) => Some(self.cmp(other)),
            _ => None,
        }
    }
}

impl TryInto<bool> for Expression {
    type Error = Error;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Expression::Bool(x) => Ok(x),
            _ => Err(Error::ImpossibleConversion),
        }
    }
}

impl TryInto<(String, FnBody)> for Expression {
    type Error = Error;

    fn try_into(self) -> Result<(String, FnBody), Self::Error> {
        match self {
            Expression::Function(a, b) => Ok((a, b)),
            _ => Err(Error::ImpossibleConversion),
        }
    }
}

impl TryInto<String> for Expression {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Expression::Symbol(x) => Ok(x),
            _ => Err(Error::ImpossibleConversion),
        }
    }
}

impl TryInto<Vec<Expression>> for Expression {
    type Error = Error;

    fn try_into(self) -> Result<Vec<Expression>, Self::Error> {
        match self {
            Expression::List(x) => Ok(x),
            Expression::Vector(x) => Ok(x),
            _ => Err(Error::ImpossibleConversion),
        }
    }
}

impl TryInto<i64> for Expression {
    type Error = Error;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Expression::Number(x) => Ok(x),
            _ => Err(Error::ImpossibleConversion),
        }
    }
}

impl Expression {
    pub fn is_bool(&self) -> bool {
        matches!(self, Expression::Bool(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Expression::Function(_, _))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Expression::List(_))
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Expression::Nil)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Expression::Number(_))
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Expression::Symbol(_))
    }

    pub fn is_vector(&self) -> bool {
        matches!(self, Expression::Vector(_))
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Expression::Bool(ref b) => fmt::Display::fmt(b, f),
            Expression::Function(ref params, ref _closure) => {
                f.write_str("(fn [")?;
                for e in params.split_whitespace() {
                    fmt::Display::fmt(e, f)?;
                }
                f.write_str("] (...))")
            }
            Expression::List(ref l) => {
                f.write_str("(")?;
                l.iter().try_for_each(|e| {
                    f.write_str(" ")?;
                    fmt::Display::fmt(e, f)
                })?;
                f.write_str(" )")
            }
            Expression::Nil => f.write_str("nil"),
            Expression::Number(ref n) => fmt::Display::fmt(n, f),
            Expression::Vector(ref v) => {
                f.write_str("[")?;
                v.iter().try_for_each(|e| {
                    f.write_str(" ")?;
                    fmt::Display::fmt(e, f)
                })?;
                f.write_str(" ]")
            }
            Expression::Symbol(ref s) => fmt::Display::fmt(s, f),
        }
    }
}

impl FromStr for Expression {
    type Err = Error;

    fn from_str(s: &str) -> Result<Expression, Self::Err> {
        // To parse a string, it must be a list of expressions (i.e. starts with '(')
        // We strip the list opener, so that tokenization will complete when it hits the
        // final closing delimiter: ')'
        if let Some(rem) = s.strip_prefix('(') {
            let (rem, expr) = tokenize(rem)?;
            if rem.is_empty() {
                Ok(expr)
            } else {
                // We require the entire string form a list. If there is some remaining data
                // after tokenization, then that data represents expressions outside of the
                // top-level list, which we do not allow.
                Err(Error::IncompleteTokenization)
            }
        } else {
            Err(Error::ExpectedList)
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Bool(ref b) => fmt::Display::fmt(b, f),
            Expression::Function(ref params, ref _closure) => {
                f.write_str("(fn [")?;
                for e in params.split_whitespace() {
                    fmt::Display::fmt(e, f)?;
                }
                f.write_str("] (...))")
            }
            Expression::List(ref l) => {
                f.write_str("(")?;
                l.iter().try_for_each(|e| {
                    f.write_str(" ")?;
                    fmt::Display::fmt(e, f)
                })?;
                f.write_str(" )")
            }
            Expression::Nil => f.write_str("nil"),
            Expression::Number(ref n) => fmt::Display::fmt(n, f),
            Expression::Vector(ref v) => {
                f.write_str("[")?;
                v.iter().try_for_each(|e| {
                    f.write_str(" ")?;
                    fmt::Display::fmt(e, f)
                })?;
                f.write_str(" ]")
            }
            Expression::Symbol(ref s) => fmt::Display::fmt(s, f),
        }
    }
}

fn tokenize(s: &str) -> Result<(&str, Expression), Error> {
    let mut exprs = Vec::<Expression>::new();
    let at_delims = |c| {
        c == ' '
            || c == '\t'
            || c == '\n'
            || c == '\r'
            || c == '('
            || c == ')'
            || c == '['
            || c == ']'
    };
    let mut rem = s;
    while !rem.is_empty() {
        let (before, after) = match rem.split_once(at_delims) {
            Some((b, a)) => (b, a),
            None => return Err(Error::UnterminatedList),
        };
        let tok = before.strip_suffix(at_delims).unwrap_or(before).trim();
        let delim = rem
            .strip_prefix(before)
            .unwrap_or(" ")
            .chars()
            .next()
            .unwrap();

        // Parse the atom
        if let Ok(x) = tok.parse::<i64>() {
            exprs.push(Expression::Number(x));
        } else if let Ok(x) = tok.parse::<bool>() {
            exprs.push(Expression::Bool(x));
        } else if !tok.is_empty() {
            // If the token is not empty, but we were unable to parse it into any other
            // atomic types, it must be either the `nil` keyword or a symbol.
            match tok {
                "nil" => exprs.push(Expression::Nil),
                _ => exprs.push(Expression::Symbol(tok.to_string())),
            };
        }

        // Handle lists and vectors
        match delim {
            '(' => {
                let (after, expr) = tokenize(after)?;
                if expr.is_list() {
                    exprs.push(expr);
                    rem = after;
                } else {
                    return Err(Error::MismatchedDelimiter);
                }
            }
            '[' => {
                let (after, expr) = tokenize(after)?;
                if expr.is_vector() {
                    exprs.push(expr);
                    rem = after;
                } else {
                    return Err(Error::MismatchedDelimiter);
                }
            }
            ')' => {
                return Ok((after, Expression::List(exprs)));
            }
            ']' => {
                return Ok((after, Expression::Vector(exprs)));
            }
            _ => {
                rem = after;
            }
        }
    }
    Ok((rem, Expression::List(exprs)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
