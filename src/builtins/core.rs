extern crate alloc;

use crate::expression::FnBody;
use crate::Environment;
use crate::Error;
use crate::Expression;
use crate::{
    make_builtin__a_and_b, make_builtin__a_b_and_c, make_builtin__args, make_builtin__x_and_ys,
};
use alloc::vec;
use alloc::vec::Vec;

make_builtin__a_and_b!(DEF, |env: &mut Environment,
                             a: Expression,
                             b: Expression| {
    let data = env.eval(b)?;
    if let Expression::Symbol(name) = a {
        env.define_var(name.as_str(), data)?;
        Ok(Expression::Nil)
    } else {
        Err(Error::ExpectedSymbol)
    }
});

pub const LET: (&str, FnBody) = (
    "bindings & exprs",
    FnBody(|env: &mut Environment| {
        if let Ok(bindings) = env.pop_stack_if_named("bindings") {
            let bindings: Vec<_> = bindings.try_into()?;

            // Go ahead and pop expressions to clear the stack, before we begin pushing
            // variables onto the stack. We don't evaluate expressions until after all
            // variables have been pushed to the stack, though.
            let exprs = env.pop_stack_if_named("exprs");

            // Push data into lexical scope for each binding given
            if bindings.len() % 2 != 0 {
                return Err(Error::UnbalancedBindings);
            } else {
                bindings
                    .as_slice()
                    .windows(2)
                    .step_by(2)
                    .map(|s| (s[0].clone(), s[1].clone()))
                    .try_for_each(|(name, expr)| {
                        if let Expression::Symbol(name) = name {
                            let value = env.eval(expr)?;
                            env.push_stack(name.as_str(), value)
                        } else {
                            Err(Error::ExpectedSymbol)
                        }
                    })?;
            }

            // Optionally evaluate the expressions, returning the result of the final
            // evaluation.
            if let Ok(exprs) = exprs {
                let exprs: Vec<Expression> = exprs.try_into()?;
                let mut res = Expression::Nil;
                for expr in exprs.into_iter().rev() {
                    res = env.eval(expr)?;
                }
                Ok(res)
            } else {
                Ok(Expression::Nil)
            }
        } else {
            Err(Error::UnexpectedArgs)
        }
    }),
);

make_builtin__a_b_and_c!(IF, |env: &mut Environment,
                              a: Expression,
                              b: Expression,
                              c: Expression| {
    // Only Nil & `false` are logically false. All other values, including the
    // number `0` are considered true.
    match env.eval(a)? {
        Expression::Nil | Expression::Bool(false) => env.eval(c),
        _ => env.eval(b),
    }
});

make_builtin__args!(
    DO,
    |_env: &mut Environment| { Ok(Expression::Nil) },
    |env: &mut Environment, args: Vec<_>| {
        let mut res = Expression::Nil;
        for expr in args.into_iter().rev() {
            res = env.eval(expr)?;
        }
        Ok(res)
    }
);

make_builtin__x_and_ys!(
    WHILE,
    |_env: &mut Environment, _| { Ok(Expression::Nil) },
    |env: &mut Environment, test_expr: Expression, body_exprs: Vec<Expression>| {
        while env.eval(test_expr.clone())?.try_into()? {
            for expr in body_exprs.iter().rev() {
                env.eval(expr.clone())?;
            }
        }
        Ok(Expression::Nil)
    }
);

pub const DOSEQ: (&str, FnBody) = (
    "bindings & exprs",
    FnBody(|env: &mut Environment| {
        if let Ok(bindings) = env.pop_stack_if_named("bindings") {
            let bindings: Vec<_> = bindings.try_into()?;

            // Go ahead and pop expressions to clear the stack, before we begin pushing
            // variables onto the stack. We don't evaluate expressions until after all
            // variables have been pushed to the stack, though.
            let exprs = env.pop_stack_if_named("exprs");

            // Push data into lexical scope for each binding given
            if bindings.len() % 2 != 0 {
                return Err(Error::UnbalancedBindings);
            }

            // Collect bindings into an iterator of tuples, e.g:
            // [(x, [...]), (y, [...]), ...]
            let binds = bindings
                .as_slice()
                .windows(2)
                .step_by(2)
                .map(|s| match (&s[0], &s[1]) {
                    (Expression::Symbol(var), Expression::Vector(vals)) => Ok((var, vals)),
                    _ => Err(Error::TypeMismatch),
                });

            // Determine the number of loop iterations, & check that each var has the same
            // number of vals to iterate
            let loop_count = binds
                .clone()
                .map(|bind| match bind {
                    Ok((_, val)) => Ok(val.len()),
                    Err(e) => Err(e),
                })
                .try_fold(None, |acc, vlen| {
                    let len = vlen?;
                    match (acc, len) {
                        (None, _) => Ok(Some(len)),
                        (Some(acc), len) => match acc == len {
                            true => Ok(Some(acc)),
                            false => Err(Error::UnbalancedBindings),
                        },
                    }
                })?
                .unwrap_or(0);

            // Iterate over all bindings
            for i in 0..loop_count {
                // Update each binding for this loop iteration
                binds.clone().try_for_each(|bind| {
                    let (var, vals) = bind?;
                    let val = env.eval(vals[i].clone())?;
                    env.push_stack(var, val)
                })?;

                // Optionally evaluate the expressions
                if let Ok(exprs) = exprs.clone() {
                    let exprs: Vec<Expression> = exprs.try_into()?;
                    for expr in exprs.into_iter().rev() {
                        env.eval(expr)?;
                    }
                }
            }
            Ok(Expression::Nil)
        } else {
            Err(Error::UnexpectedArgs)
        }
    }),
);

make_builtin__a_and_b!(DOTIMES, |env: &mut Environment,
                                 binds: Expression,
                                 body: Expression| {
    let binds: Vec<_> = env.eval(binds)?.try_into()?;
    if 2 != binds.len() {
        Err(Error::UnbalancedBindings)
    } else if let Expression::Symbol(var) = &binds[0] {
        let range = env.eval(binds[1].clone())?.try_into()?;
        for i in 0..range {
            env.push_stack(var, Expression::Number(i))?;
            env.eval(body.clone())?;
        }
        Ok(Expression::Nil)
    } else {
        Err(Error::ExpectedSymbol)
    }
});

make_builtin__args!(
    VECTOR,
    |_env: &mut Environment| { Ok(Expression::Vector(vec![])) },
    |env: &mut Environment, args: Vec<_>| {
        Ok(Expression::Vector(
            args.into_iter().rev().map(|e| env.eval(e)).try_collect()?,
        ))
    }
);

pub const NTH: (&str, FnBody) = (
    "vec & args",
    FnBody(|env: &mut Environment| {
        let vec: Vec<_> = env
            .pop_stack_if_named("vec")?
            .try_into()
            .or(Err(Error::ExpectedVector))?;
        let args: Vec<_> = env
            .pop_stack_if_named("args")
            .or(Err(Error::TooFewArgs))?
            .try_into()?;
        let (idx, dnf) = match args.len() {
            0 => Err(Error::TooFewArgs),
            1 => Ok((env.eval(args[0].clone())?, Expression::Nil)),
            2 => Ok((env.eval(args[1].clone())?, args[0].clone())),
            _ => Err(Error::TooManyArgs),
        }?;
        match idx {
            Expression::Number(idx) => {
                if idx >= 0 && idx < (vec.len() as i64) {
                    Ok(vec[idx as usize].clone())
                } else {
                    env.eval(dnf)
                }
            }
            _ => Err(Error::TypeMismatch),
        }
    }),
);

pub const PEEK: (&str, FnBody) = (
    "vec",
    FnBody(|env: &mut Environment| {
        let vec: Vec<_> = env
            .pop_stack_if_named("vec")?
            .try_into()
            .or(Err(Error::ExpectedVector))?;
        match vec.last() {
            Some(expr) => Ok(expr.clone()),
            None => Err(Error::Empty),
        }
    }),
);

pub const POP: (&str, FnBody) = (
    "vec",
    FnBody(|env: &mut Environment| {
        let mut vec: Vec<_> = env
            .pop_stack_if_named("vec")?
            .try_into()
            .or(Err(Error::ExpectedVector))?;
        match vec.pop() {
            Some(_) => Ok(Expression::Vector(vec)),
            None => Err(Error::Empty),
        }
    }),
);

#[cfg(test)]
mod tests {
    use crate::Environment;
    use crate::Error;
    use crate::Expression;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn def() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(def)"), Err(Error::TooFewArgs));
        assert_eq!(env.parse_eval("(def myVar 2)"), Ok(Expression::Nil));
        assert_eq!(env.parse_eval("(+ myVar 9)"), Ok(Expression::Number(11)));
        assert_eq!(env.parse_eval("(def myVar 8)"), Ok(Expression::Nil));
        assert_eq!(env.parse_eval("(+ myVar 0)"), Ok(Expression::Number(8)));
        assert_eq!(
            env.parse_eval("(def myVar (dec myVar))"),
            Ok(Expression::Nil)
        );
        assert_eq!(env.parse_eval("(+ myVar 0)"), Ok(Expression::Number(7)));
    }

    #[test]
    fn op_let() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(let)"), Err(Error::TooFewArgs));
        assert_eq!(env.parse_eval("(let [a])"), Err(Error::UnbalancedBindings));
        assert_eq!(env.parse_eval("(let [myVar 2])"), Ok(Expression::Nil));
        assert_eq!(
            env.parse_eval("(let [myVar 2] (+ myVar 1))"),
            Ok(Expression::Number(3))
        );
        assert_eq!(
            env.parse_eval("(let [myVar (inc 2)] (+ myVar 1))"),
            Ok(Expression::Number(4))
        );
        assert_eq!(
            env.parse_eval("(let [a 1 b 2] (+ a b))"),
            Ok(Expression::Number(3))
        );
        assert_eq!(
            env.parse_eval("(let [a 1 b 2] (+ a b) (inc a))"),
            Ok(Expression::Number(2))
        );
    }

    #[test]
    fn op_if() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(if)"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(if (< 10 11 12) true false)"),
            Ok(Expression::Bool(true))
        );
        assert_eq!(
            env.parse_eval("(if (< 10 11 10) true false)"),
            Ok(Expression::Bool(false))
        );
        assert_eq!(
            env.parse_eval("(if (inc 10) true false)"),
            Ok(Expression::Bool(true))
        );
        assert_eq!(
            env.parse_eval("(if true true false)"),
            Ok(Expression::Bool(true))
        );
        assert_eq!(
            env.parse_eval("(if false true false)"),
            Ok(Expression::Bool(false))
        );
        assert_eq!(
            env.parse_eval("(if nil true false)"),
            Ok(Expression::Bool(false))
        );
        assert_eq!(
            env.parse_eval("(if (dec 1) true false)"),
            Ok(Expression::Bool(true))
        );
    }

    #[test]
    fn op_do() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(do)"), Ok(Expression::Nil));
        assert_eq!(
            env.parse_eval("(do true true false)"),
            Ok(Expression::Bool(false))
        );
        assert_eq!(
            env.parse_eval("(do (+ 1 3) (- 9 (+ 8 1)))"),
            Ok(Expression::Number(0))
        );
    }

    #[test]
    fn op_while() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(while)"), Err(Error::TooFewArgs));
        assert_eq!(env.parse_eval("(while false)"), Ok(Expression::Nil));
        assert_eq!(env.parse_eval("(def myVar 8)"), Ok(Expression::Nil));
        assert_eq!(
            env.parse_eval("(do (while (> myVar 3) (def myVar (dec myVar))) myVar)"),
            Ok(Expression::Number(3))
        );
    }

    #[test]
    fn doseq() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(doseq)"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(doseq [a])"),
            Err(Error::UnbalancedBindings)
        );
        assert_eq!(
            env.parse_eval("(doseq [a [1 2] b [9]])"),
            Err(Error::UnbalancedBindings)
        );
        assert_eq!(
            env.parse_eval("(doseq [a [1] b [9 8]])"),
            Err(Error::UnbalancedBindings)
        );
        assert_eq!(env.parse_eval("(doseq [a 1])"), Err(Error::TypeMismatch));
        assert_eq!(
            env.parse_eval("(doseq [a [1 2] b [9 8]])"),
            Ok(Expression::Nil)
        );
        // Define some vars
        env.parse_eval("(do (def sum 0) (def prod 1))").unwrap();
        assert_eq!(
            env.parse_eval(
                "(doseq [a [1 2] b [9 8]] (def sum (+ sum a b)) (def prod (* prod a b)))"
            ),
            Ok(Expression::Nil)
        );
        assert_eq!(env.parse_eval("(+ sum 0)"), Ok(Expression::Number(20)));
        assert_eq!(env.parse_eval("(+ prod 0)"), Ok(Expression::Number(144)));
    }

    #[test]
    fn dotimes() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(dotimes)"), Err(Error::TooFewArgs));
        assert_eq!(env.parse_eval("(dotimes [n 5])"), Err(Error::TooFewArgs));
        assert_eq!(
            env.parse_eval("(dotimes true (+ n 10))"),
            Err(Error::ImpossibleConversion)
        );
        // Define some vars
        env.parse_eval("(do (def sum 0))").unwrap();
        assert_eq!(
            env.parse_eval("(dotimes [4 5] (+ sum n))"),
            Err(Error::ExpectedSymbol)
        );
        assert_eq!(
            env.parse_eval("(dotimes [n 5] (def sum (+ sum n)))"),
            Ok(Expression::Nil)
        );
        assert_eq!(env.parse_eval("(+ sum 0)"), Ok(Expression::Number(10)));
    }

    #[test]
    fn vector() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        let res: Vec<_> = env
            .parse_eval("(vector)")
            .expect("Failed to evaluate expression.")
            .try_into()
            .expect("Result is not a vector.");
        assert!(res.into_iter().eq(vec![].into_iter()));
        let res: Vec<_> = env
            .parse_eval("(vector 1)")
            .expect("Failed to evaluate expression.")
            .try_into()
            .expect("Result is not a vector.");
        assert!(res.into_iter().eq(vec![Expression::Number(1)].into_iter()));
        let res: Vec<_> = env
            .parse_eval("(vector 1 2 3)")
            .expect("Failed to evaluate expression.")
            .try_into()
            .expect("Result is not a vector.");
        assert!(res.into_iter().eq(vec![
            Expression::Number(1),
            Expression::Number(2),
            Expression::Number(3)
        ]
        .into_iter()));
    }

    #[test]
    fn nth() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(nth)"), Err(Error::TooFewArgs));
        assert_eq!(env.parse_eval("(nth [1 2 3])"), Err(Error::TooFewArgs));
        assert_eq!(env.parse_eval("(nth 3)"), Err(Error::ExpectedVector));
        assert_eq!(env.parse_eval("(nth [1 2 3] 0)"), Ok(Expression::Number(1)));
        assert_eq!(env.parse_eval("(nth [1 2 3] 1)"), Ok(Expression::Number(2)));
        assert_eq!(env.parse_eval("(nth [1 2 3] 2)"), Ok(Expression::Number(3)));
        assert_eq!(env.parse_eval("(nth [1 2 3] 4)"), Ok(Expression::Nil));
        assert_eq!(
            env.parse_eval("(nth [1 2 3] 4 9)"),
            Ok(Expression::Number(9))
        );
        assert_eq!(
            env.parse_eval("(nth [1 2 3] 4 9 8)"),
            Err(Error::TooManyArgs)
        );
    }

    #[test]
    fn peek() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(peek)"), Err(Error::TooFewArgs));
        assert_eq!(env.parse_eval("(peek [1 2 3] 3)"), Err(Error::TooManyArgs));
        assert_eq!(env.parse_eval("(peek 3)"), Err(Error::ExpectedVector));
        assert_eq!(env.parse_eval("(peek [1 2 3])"), Ok(Expression::Number(3)));
    }

    #[test]
    fn pop() {
        let mut env = Environment::new();
        env.load_default_builtins().unwrap();
        assert_eq!(env.parse_eval("(pop)"), Err(Error::TooFewArgs));
        assert_eq!(env.parse_eval("(pop [1 2 3] 3)"), Err(Error::TooManyArgs));
        assert_eq!(env.parse_eval("(pop 3)"), Err(Error::ExpectedVector));
        assert_eq!(
            env.parse_eval("(pop [1 2 3])"),
            env.parse_eval("(vector 1 2)")
        );
    }
}
