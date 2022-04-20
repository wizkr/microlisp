extern crate alloc;

use crate::builtins::{core, operators};
use crate::expression::FnBody;
use crate::Error;
use crate::Expression;
use alloc::collections::LinkedList;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// The environment stores builtin functions and runtime data, in order to
// evaluate microlisp scripts.
#[derive(Clone)]
pub struct Environment {
    builtins: Vec<(String, Expression)>,
    stack: LinkedList<Vec<(String, Expression)>>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            builtins: Vec::new(),
            stack: LinkedList::new(),
        };
        env.stack.push_front(Vec::new());
        env
    }

    pub fn load_builtin(
        &mut self,
        name: &str,
        (params, body): (&str, FnBody),
    ) -> Result<(), Error> {
        // Must not duplicate symbol
        if self.builtins.iter().map(|(k, _)| k).any(|k| k == &name) {
            Err(Error::DuplicateSymbol)
        } else {
            self.builtins.push((
                name.to_string(),
                Expression::Function(params.to_string(), body),
            ));
            Ok(())
        }
    }

    pub fn load_default_builtins(&mut self) -> Result<(), Error> {
        self.load_builtin("def", core::DEF)?;
        self.load_builtin("let", core::LET)?;
        self.load_builtin("if", core::IF)?;
        self.load_builtin("do", core::DO)?;
        self.load_builtin("while", core::WHILE)?;
        self.load_builtin("doseq", core::DOSEQ)?;
        self.load_builtin("dotimes", core::DOTIMES)?;
        self.load_builtin("vector", core::VECTOR)?;
        self.load_builtin("nth", core::NTH)?;
        self.load_builtin("peek", core::PEEK)?;
        self.load_builtin("pop", core::POP)?;
        self.load_builtin("+", operators::ADD)?;
        self.load_builtin("-", operators::SUB)?;
        self.load_builtin("*", operators::MUL)?;
        self.load_builtin("/", operators::DIV)?;
        self.load_builtin("rem", operators::REM)?;
        self.load_builtin("inc", operators::INC)?;
        self.load_builtin("dec", operators::DEC)?;
        self.load_builtin("max", operators::MAX)?;
        self.load_builtin("min", operators::MIN)?;
        self.load_builtin("==", operators::EQ)?;
        self.load_builtin(">", operators::GT)?;
        self.load_builtin(">=", operators::GTE)?;
        self.load_builtin("<", operators::LT)?;
        self.load_builtin("<=", operators::LTE)?;
        self.load_builtin("and", operators::AND)?;
        self.load_builtin("or", operators::OR)?;
        self.load_builtin("not", operators::NOT)?;
        Ok(())
    }

    pub(crate) fn find_builtin(&self, name: &str) -> Option<&Expression> {
        if let Some((_, v)) = self.builtins.iter().find(|(k, _)| k == name) {
            Some(v)
        } else {
            None
        }
    }

    pub fn define_var(&mut self, name: &str, var: Expression) -> Result<(), Error> {
        // Variable definitions go in the oldest (backmost) stack frame
        let frame = self.stack.back_mut().ok_or(Error::StackError)?;
        if let Some((idx, _)) = frame.iter().enumerate().find(|(_, (k, _))| k == name) {
            frame[idx].1 = var;
        } else {
            frame.push((name.to_string(), var));
        }
        Ok(())
    }

    pub(crate) fn push_stack(&mut self, name: &str, var: Expression) -> Result<(), Error> {
        // Overwrite variable if it already exists in the top stack frame
        if let Some(idx) = self
            .stack
            .front()
            .ok_or(Error::StackError)?
            .iter()
            .enumerate()
            .find_map(|(idx, (k, _))| if k == name { Some(idx) } else { None })
        {
            self.stack.front_mut().ok_or(Error::StackError)?[idx].1 = var;
            Ok(())
        } else {
            self.stack
                .front_mut()
                .ok_or(Error::StackError)?
                .push((name.to_string(), var));
            Ok(())
        }
    }

    // If the top item in the stack matches the given name, pop that item from the
    // stack Only searches top stack frame. Does not recurse into lower stack
    // frames.
    pub(crate) fn pop_stack_if_named(&mut self, name: &str) -> Result<Expression, Error> {
        if self
            .stack
            .front()
            .ok_or(Error::StackError)?
            .last()
            .ok_or(Error::DataNotFound)?
            .0
            == name
        {
            Ok(self.stack.front_mut().unwrap().pop().unwrap().1)
        } else {
            Err(Error::DataNotFound)
        }
    }

    // Search the stack for a symbol. If the symbol exists in multiple stack frames,
    // this method will return the instance from the newest (frontmost) frame.
    pub(crate) fn find_data(&self, name: &str) -> Option<&Expression> {
        self.stack
            .iter()
            .map(|v| v.iter())
            .flatten()
            .find_map(|(k, v)| if k == name { Some(v) } else { None })
    }

    pub fn stack_height(&self) -> usize {
        self.stack.len()
    }

    pub fn stack_frame_len(&self) -> usize {
        self.stack.front().unwrap().len()
    }

    pub fn eval(&mut self, expr: Expression) -> Result<Expression, Error> {
        match expr {
            // Evaluating a list is the most complicated, because a list must be evaluated as a
            // function form. That is: the first item must be a symbol referring to a function, and
            // the following items must be suppliable as function args.
            Expression::List(mut args) => {
                if args.is_empty() {
                    Ok(Expression::Nil)
                } else {
                    // First expression must be a symbol referencing the name of a function
                    let name: String = args[0].clone().try_into()?;

                    // Create new stack frame for evaluation of this function
                    self.stack.push_front(Vec::new());

                    // Load the function params/body from the builtins or from data
                    let (params, FnBody(body)) = if let Some(Expression::Function(params, body)) =
                        self.find_builtin(&name)
                    {
                        Ok((params, body))
                    } else if let Some(Expression::Function(params, body)) = self.find_data(&name) {
                        Ok((params, body))
                    } else {
                        Err(Error::ExpectedFunction)
                    }?;
                    // TODO: Consider using a Box or Arc to prevent cloning these
                    let params = params.clone();
                    let body = body.clone();

                    // Split the parameters into a list of named parameters and positional
                    // parameters.
                    let (named, positional) = match params.split_once('&') {
                        Some((a, b)) => (a, Some(b)),
                        _ => (params.as_str(), None),
                    };
                    let named_count = named.split_whitespace().count();
                    let offset_func = 0;
                    let offset_named = offset_func + 1;
                    let offset_positional = offset_named + named_count;

                    // Args are pushed onto the stack in reverse order. First, push the positional
                    // args (if any) in reverse order. Then push the named args in reverse order.

                    if let Some(pos_str) = positional {
                        // Must be exactly 0 or 1 positional names supplied. If 0 names are
                        // supplied, collect all positional args under the generic name `&`.
                        // Else, use the name given.
                        let pos_name_count = pos_str.split_whitespace().count();
                        if pos_name_count > 1 {
                            return Err(Error::UnexpectedSymbol);
                        }

                        // Optionally push positional args. Collect the args into a vector, and push
                        // that vector under the given name, if a name was given.
                        let mut pos_args = Vec::new();
                        while args.len() > offset_positional {
                            pos_args.push(args.pop().unwrap());
                        }
                        if pos_args.len() > 0 {
                            let name = pos_str.split_whitespace().next().unwrap_or("&");
                            self.push_stack(name, Expression::Vector(pos_args))?;
                        }
                    } else if args.len() > offset_positional {
                        return Err(Error::TooManyArgs);
                    }

                    // Push all named args onto the stack
                    named.split_whitespace().rev().try_for_each(|param| {
                        self.push_stack(param, args.pop().ok_or(Error::TooFewArgs)?)
                    })?;

                    // After pushing all args to the stack, there should be one arg left: the
                    // function name.
                    if args.len() != 1 {
                        return Err(Error::TooFewArgs);
                    }

                    // Evaluate the function body
                    let res = body(self);

                    // Pop the new stack frame and return the result
                    self.stack.pop_front();
                    res
                }
            }

            // Look up referenced data, if symbol
            Expression::Symbol(s) => {
                if let Some(data) = self.find_data(&s) {
                    Ok(data.clone())
                } else {
                    Err(Error::DataNotFound)
                }
            }

            // All other expressions just evaluate to themselves.
            _ => Ok(expr),
        }
    }

    pub fn parse_eval(&mut self, s: &str) -> Result<Expression, Error> {
        self.eval(s.parse()?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
