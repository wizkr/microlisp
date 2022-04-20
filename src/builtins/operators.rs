extern crate alloc;

use crate::expression::FnBody;
use crate::Environment;
use crate::Error;
use crate::Expression;
use crate::{make_builtin__a_and_b, make_builtin__args, make_builtin__x, make_builtin__x_and_ys};
use alloc::vec::Vec;

make_builtin__args!(
    ADD,
    |_env: &mut Environment| { Ok(Expression::Number(0)) },
    |env: &mut Environment, args: Vec<_>| {
        Ok(Expression::Number(args.into_iter().try_fold(
            0,
            |acc: i64, data| {
                acc.checked_add(env.eval(data)?.try_into()?)
                    .ok_or(Error::MathError)
            },
        )?))
    }
);

make_builtin__x_and_ys!(
    SUB,
    |env: &mut Environment, x: Expression| Ok(-env.eval(x)?),
    |env: &mut Environment, x: Expression, ys: Vec<_>| {
        Ok(Expression::Number(ys.into_iter().try_fold(
            env.eval(x)?.try_into()?,
            |acc: i64, y| {
                acc.checked_sub(env.eval(y)?.try_into()?)
                    .ok_or(Error::MathError)
            },
        )?))
    }
);

make_builtin__args!(
    MUL,
    |_env: &mut Environment| { Ok(Expression::Number(1)) },
    |env: &mut Environment, args: Vec<_>| {
        Ok(Expression::Number(args.into_iter().try_fold(
            1,
            |acc: i64, data| {
                acc.checked_mul(env.eval(data)?.try_into()?)
                    .ok_or(Error::MathError)
            },
        )?))
    }
);

make_builtin__x_and_ys!(
    DIV,
    |env: &mut Environment, x: Expression| {
        Ok(Expression::Number(
            1_i64
                .checked_div(env.eval(x)?.try_into()?)
                .ok_or(Error::MathError)?,
        ))
    },
    |env: &mut Environment, x: Expression, ys: Vec<_>| {
        Ok(Expression::Number(ys.into_iter().try_fold(
            env.eval(x)?.try_into()?,
            |acc: i64, y| {
                acc.checked_div(env.eval(y)?.try_into()?)
                    .ok_or(Error::MathError)
            },
        )?))
    }
);

make_builtin__a_and_b!(REM, |env: &mut Environment,
                             a: Expression,
                             b: Expression| {
    let num: i64 = env.eval(a)?.try_into()?;
    let div: i64 = env.eval(b)?.try_into()?;
    Ok(Expression::Number(
        num.checked_rem(div).ok_or(Error::MathError)?,
    ))
});

make_builtin__x!(INC, |env: &mut Environment, x: Expression| {
    let x: i64 = env.eval(x)?.try_into()?;
    Ok(Expression::Number(x + 1))
});

make_builtin__x!(DEC, |env: &mut Environment, x: Expression| {
    let x: i64 = env.eval(x)?.try_into()?;
    Ok(Expression::Number(x - 1))
});

make_builtin__x_and_ys!(
    MAX,
    |env: &mut Environment, x: Expression| { Ok(env.eval(x)?) },
    |env: &mut Environment, x: Expression, mut ys: Vec<_>| {
        ys.push(env.eval(x)?);
        ys.into_iter()
            .map(|expr| env.eval(expr).unwrap())
            .max()
            .ok_or(Error::TypeMismatch)
    }
);

make_builtin__x_and_ys!(
    MIN,
    |env: &mut Environment, x: Expression| { Ok(env.eval(x)?) },
    |env: &mut Environment, x: Expression, mut ys: Vec<_>| {
        ys.push(env.eval(x)?);
        ys.into_iter()
            .map(|expr| env.eval(expr).unwrap())
            .min()
            .ok_or(Error::TypeMismatch)
    }
);

make_builtin__x_and_ys!(
    EQ,
    |_env: &mut Environment, _x: Expression| { Ok(Expression::Bool(true)) },
    |env: &mut Environment, x: Expression, ys: Vec<_>| {
        Ok(Expression::Bool(
            ys.into_iter()
                .try_fold((true, env.eval(x)?), |acc, y| {
                    let y = env.eval(y)?;
                    Ok((acc.0 && (acc.1 == y), y))
                })?
                .0,
        ))
    }
);

make_builtin__x_and_ys!(
    GT,
    |_env: &mut Environment, _x: Expression| { Ok(Expression::Bool(true)) },
    |env: &mut Environment, x: Expression, ys: Vec<_>| {
        Ok(Expression::Bool(
            ys.into_iter()
                .rev()
                .try_fold((true, env.eval(x)?), |acc, y| {
                    let y = env.eval(y)?;
                    Ok((acc.0 && (acc.1 > y), y))
                })?
                .0,
        ))
    }
);

make_builtin__x_and_ys!(
    GTE,
    |_env: &mut Environment, _x: Expression| { Ok(Expression::Bool(true)) },
    |env: &mut Environment, x: Expression, ys: Vec<_>| {
        Ok(Expression::Bool(
            ys.into_iter()
                .rev()
                .try_fold((true, env.eval(x)?), |acc, y| {
                    let y = env.eval(y)?;
                    Ok((acc.0 && (acc.1 >= y), y))
                })?
                .0,
        ))
    }
);

make_builtin__x_and_ys!(
    LT,
    |_env: &mut Environment, _x: Expression| { Ok(Expression::Bool(true)) },
    |env: &mut Environment, x: Expression, ys: Vec<_>| {
        Ok(Expression::Bool(
            ys.into_iter()
                .rev()
                .try_fold((true, env.eval(x)?), |acc, y| {
                    let y = env.eval(y)?;
                    Ok((acc.0 && (acc.1 < y), y))
                })?
                .0,
        ))
    }
);

make_builtin__x_and_ys!(
    LTE,
    |_env: &mut Environment, _x: Expression| { Ok(Expression::Bool(true)) },
    |env: &mut Environment, x: Expression, ys: Vec<_>| {
        Ok(Expression::Bool(
            ys.into_iter()
                .rev()
                .try_fold((true, env.eval(x)?), |acc, y| {
                    let y = env.eval(y)?;
                    Ok((acc.0 && (acc.1 <= y), y))
                })?
                .0,
        ))
    }
);

make_builtin__args!(
    AND,
    |_env: &mut Environment| { Ok(Expression::Bool(true)) },
    |env: &mut Environment, args: Vec<_>| {
        Ok(Expression::Bool(
            args.into_iter().fold(Ok(true), |acc, y| {
                Ok(acc.unwrap() && env.eval(y)?.try_into()?)
            })?,
        ))
    }
);

make_builtin__args!(
    OR,
    |_env: &mut Environment| { Ok(Expression::Bool(false)) },
    |env: &mut Environment, args: Vec<_>| {
        Ok(Expression::Bool(
            args.into_iter().fold(Ok(false), |acc, y| {
                Ok(acc.unwrap() || env.eval(y)?.try_into()?)
            })?,
        ))
    }
);

make_builtin__x!(NOT, |env: &mut Environment, x: Expression| {
    let x: bool = env.eval(x)?.try_into()?;
    Ok(Expression::Bool(!x))
});

#[cfg(test)]
mod tests {
    use crate::Environment;
    use crate::Error;
    use crate::Expression;

    #[test]
    fn add() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(+ 10 2)"), Ok(Expression::Number(12)));
        assert_eq!(env.parse_eval("(+ -9 2)"), Ok(Expression::Number(-7)));
        assert_eq!(
            env.parse_eval("(+ -2 -1 0 1 2 3)"),
            Ok(Expression::Number(3))
        );
        assert_eq!(env.parse_eval("(+)"), Ok(Expression::Number(0)));
        assert_eq!(
            env.parse_eval("(let [a 9] (+ a 1))"),
            Ok(Expression::Number(10))
        );
    }

    #[test]
    fn sub() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(- 10 2)"), Ok(Expression::Number(8)));
        assert_eq!(env.parse_eval("(- -1 200)"), Ok(Expression::Number(-201)));
        assert_eq!(env.parse_eval("(- 1 200)"), Ok(Expression::Number(-199)));
        assert_eq!(env.parse_eval("(- 1 1 1)"), Ok(Expression::Number(-1)));
        assert_eq!(env.parse_eval("(- 10)"), Ok(Expression::Number(-10)));
        assert_eq!(env.parse_eval("(- )"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(let [a 9] (- a 1))"),
            Ok(Expression::Number(8))
        );
    }

    #[test]
    fn mul() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(* 10 2)"), Ok(Expression::Number(20)));
        assert_eq!(env.parse_eval("(* -10 2)"), Ok(Expression::Number(-20)));
        assert_eq!(env.parse_eval("(* -99)"), Ok(Expression::Number(-99)));
        assert_eq!(env.parse_eval("(*)"), Ok(Expression::Number(1)));
        assert_eq!(
            env.parse_eval("(let [a 9] (* a 9))"),
            Ok(Expression::Number(81))
        );
    }

    #[test]
    fn div() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(/ 10 2)"), Ok(Expression::Number(5)));
        assert_eq!(env.parse_eval("(/ -10 2)"), Ok(Expression::Number(-5)));
        assert_eq!(env.parse_eval("(/ -10 7)"), Ok(Expression::Number(-1)));
        assert_eq!(env.parse_eval("(/ 7)"), Ok(Expression::Number(1 / 7)));
        assert_eq!(env.parse_eval("(/)"), Err(Error::TooFewArgs));
        assert_eq!(env.parse_eval("(/ 1 0)"), Err(Error::MathError));
        assert_eq!(
            env.parse_eval("(let [a 9] (/ a 2))"),
            Ok(Expression::Number(4))
        );
    }

    #[test]
    fn rem() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(rem 10 2)"), Ok(Expression::Number(0)));
        assert_eq!(env.parse_eval("(rem -10 2)"), Ok(Expression::Number(0)));
        assert_eq!(env.parse_eval("(rem -10 7)"), Ok(Expression::Number(-3)));
        assert_eq!(env.parse_eval("(rem 1 2 3)"), Err(Error::TooManyArgs));
        assert_eq!(env.parse_eval("(rem 1)"), Err(Error::TooFewArgs));
        assert_eq!(env.parse_eval("(rem)"), Err(Error::TooFewArgs));
        assert_eq!(env.parse_eval("(rem 1 0)"), Err(Error::MathError));
        assert_eq!(
            env.parse_eval("(let [a 9] (rem a 2))"),
            Ok(Expression::Number(1))
        );
    }

    #[test]
    fn inc() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(inc 10)"), Ok(Expression::Number(11)));
        assert_eq!(env.parse_eval("(inc -10)"), Ok(Expression::Number(-9)));
        assert_eq!(env.parse_eval("(inc 1 2)"), Err(Error::TooManyArgs));
        assert_eq!(env.parse_eval("(inc )"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(let [a 9] (inc a))"),
            Ok(Expression::Number(10))
        );
    }

    #[test]
    fn dec() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(dec 10)"), Ok(Expression::Number(9)));
        assert_eq!(env.parse_eval("(dec -10)"), Ok(Expression::Number(-11)));
        assert_eq!(env.parse_eval("(dec 1 2)"), Err(Error::TooManyArgs));
        assert_eq!(env.parse_eval("(dec )"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(let [a 9] (dec a))"),
            Ok(Expression::Number(8))
        );
    }

    #[test]
    fn max() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(max 8 6 -1 9)"), Ok(Expression::Number(9)));
        assert_eq!(env.parse_eval("(max 88)"), Ok(Expression::Number(88)));
        assert_eq!(env.parse_eval("(max)"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(let [a 9] (max 8 a))"),
            Ok(Expression::Number(9))
        );
    }

    #[test]
    fn min() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(min 8 6 -1 9)"), Ok(Expression::Number(-1)));
        assert_eq!(env.parse_eval("(min 88)"), Ok(Expression::Number(88)));
        assert_eq!(env.parse_eval("(min)"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(let [a 9] (min a 10))"),
            Ok(Expression::Number(9))
        );
    }

    #[test]
    fn eq() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(== 8 8 8)"), Ok(Expression::Bool(true)),);
        assert_eq!(env.parse_eval("(== 8 6 8)"), Ok(Expression::Bool(false)),);
        assert_eq!(env.parse_eval("(== 8)"), Ok(Expression::Bool(true)));
        assert_eq!(env.parse_eval("(==)"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(let [a 9] (== a 9))"),
            Ok(Expression::Bool(true))
        );
    }

    #[test]
    fn gt() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(> 3 2 1)"), Ok(Expression::Bool(true),));
        assert_eq!(env.parse_eval("(> 3 2 1 2)"), Ok(Expression::Bool(false)),);
        assert_eq!(env.parse_eval("(> 2)"), Ok(Expression::Bool(true),));
        assert_eq!(env.parse_eval("(>)"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(let [a 9] (> a 8))"),
            Ok(Expression::Bool(true))
        );
    }

    #[test]
    fn gte() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(>= 3 2 2 1)"), Ok(Expression::Bool(true)),);
        assert_eq!(env.parse_eval("(>= 3 2 1 2)"), Ok(Expression::Bool(false)),);
        assert_eq!(env.parse_eval("(>= 2)"), Ok(Expression::Bool(true)));
        assert_eq!(env.parse_eval("(>=)"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(let [a 9] (>= a 9))"),
            Ok(Expression::Bool(true))
        );
    }

    #[test]
    fn lt() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(< 1 2 3)"), Ok(Expression::Bool(true),));
        assert_eq!(env.parse_eval("(< 1 2 1 3)"), Ok(Expression::Bool(false)),);
        assert_eq!(env.parse_eval("(< 2)"), Ok(Expression::Bool(true)));
        assert_eq!(env.parse_eval("(<)"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(let [a 9] (< a 10))"),
            Ok(Expression::Bool(true))
        );
    }

    #[test]
    fn lte() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(<= 1 2 2 3)"), Ok(Expression::Bool(true)),);
        assert_eq!(
            env.parse_eval("(<= 1 2 2 1 3)"),
            Ok(Expression::Bool(false)),
        );
        assert_eq!(env.parse_eval("(<= 2)"), Ok(Expression::Bool(true)));
        assert_eq!(env.parse_eval("(<=)"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(let [a 9] (<= a 9))"),
            Ok(Expression::Bool(true))
        );
    }

    #[test]
    fn and() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(
            env.parse_eval("(and true true true)"),
            Ok(Expression::Bool(true)),
        );
        assert_eq!(
            env.parse_eval("(and true true false)"),
            Ok(Expression::Bool(false)),
        );
        assert_eq!(env.parse_eval("(and false)"), Ok(Expression::Bool(false)));
        assert_eq!(env.parse_eval("(and true)"), Ok(Expression::Bool(true)));
        assert_eq!(env.parse_eval("(and)"), Ok(Expression::Bool(true)));
        assert_eq!(
            env.parse_eval("(let [a true] (and a true))"),
            Ok(Expression::Bool(true))
        );
    }

    #[test]
    fn or() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(
            env.parse_eval("(or false false false)"),
            Ok(Expression::Bool(false)),
        );
        assert_eq!(
            env.parse_eval("(or true true false)"),
            Ok(Expression::Bool(true)),
        );
        assert_eq!(env.parse_eval("(or false)"), Ok(Expression::Bool(false)));
        assert_eq!(env.parse_eval("(or true)"), Ok(Expression::Bool(true)));
        assert_eq!(env.parse_eval("(or)"), Ok(Expression::Bool(false)));
        assert_eq!(
            env.parse_eval("(let [a true] (or a false))"),
            Ok(Expression::Bool(true))
        );
    }

    #[test]
    fn not() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(not true)"), Ok(Expression::Bool(false)),);
        assert_eq!(env.parse_eval("(not false)"), Ok(Expression::Bool(true)),);
        assert_eq!(env.parse_eval("(not true true)"), Err(Error::TooManyArgs));
        assert_eq!(env.parse_eval("(not)"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(let [a true] (not a))"),
            Ok(Expression::Bool(false))
        );
    }
}
