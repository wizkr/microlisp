pub mod core;
pub mod operators;

/// Macro to make a builtin function with one mandatory argument 'x'.
///
/// This macro expects three arguments:
///  1) pub const name/identifier
///  2) closure of form |env, x| to be executed
/// 'env' is the local environment
#[macro_export]
macro_rules! make_builtin__x {
    ($name:ident, $x_only:expr) => {
        pub const $name: (&str, FnBody) = (
            "x",
            FnBody(|env: &mut Environment| {
                let x = env.pop_stack_if_named("x")?;
                $x_only(env, x)
            }),
        );
    };
}

/// Macro to make a builtin function with two mandatory argument 'a' & 'b'.
///
/// This macro expects two arguments:
///  1) pub const name/identifier
///  2) closure of form |env, a, b| to be executed
/// 'env' is the local environment
#[macro_export]
macro_rules! make_builtin__a_and_b {
    ($name:ident, $a_and_b:expr) => {
        pub const $name: (&str, FnBody) = (
            "a b",
            FnBody(|env: &mut Environment| {
                let a = env.pop_stack_if_named("a")?;
                let b = env.pop_stack_if_named("b")?;
                $a_and_b(env, a, b)
            }),
        );
    };
}

/// Macro to make a builtin function with two mandatory argument 'a', 'b', &
/// 'c'.
///
/// This macro expects two arguments:
///  1) pub const name/identifier
///  2) closure of form |env, a, b, c| to be executed
/// 'env' is the local environment
#[macro_export]
macro_rules! make_builtin__a_b_and_c {
    ($name:ident, $a_b_and_c:expr) => {
        pub const $name: (&str, FnBody) = (
            "a b c",
            FnBody(|env: &mut Environment| {
                let a = env.pop_stack_if_named("a")?;
                let b = env.pop_stack_if_named("b")?;
                let c = env.pop_stack_if_named("c")?;
                $a_b_and_c(env, a, b, c)
            }),
        );
    };
}

/// Macro to make a builtin function with 0 or more arguments, collected into a
/// vector named 'args'
///
/// This macro expects three arguments:
///  1) pub const name/identifier
///  2) closure of form |env| to be executed when no args are present
///  2) closure of form |env, args| to be executed with 1 or more args
/// 'env' is the local environment
#[macro_export]
macro_rules! make_builtin__args {
    ($name:ident, $without_args:expr, $with_args:expr) => {
        pub const $name: (&str, FnBody) = (
            "&",
            FnBody(|env: &mut Environment| {
                if env.stack_frame_len() > 0 {
                    let args: Vec<_> = env.pop_stack_if_named("&")?.try_into()?;
                    $with_args(env, args)
                } else {
                    $without_args(env)
                }
            }),
        );
    };
}

/// Macro to make a builtin function with one mandatory argument 'x' and 0 or
/// more optional arguments, collected into a vector named `ys`.
///
/// This macro expects three arguments:
///  1) name/identifier of this builtin
///  2) closure of form |env, x| to be executed when no 'ys' are received
///  3) closure of form |env, x, ys| to be executed `x` & `ys` are present
/// 'env' is the local environment
#[macro_export]
macro_rules! make_builtin__x_and_ys {
    ($name:ident, $x_only:expr, $x_and_ys:expr) => {
        pub const $name: (&str, FnBody) = (
            "x & ys",
            FnBody(|env: &mut Environment| {
                if let Ok(x) = env.pop_stack_if_named("x") {
                    if let Ok(ys) = env.pop_stack_if_named("ys") {
                        let ys: Vec<_> = ys.try_into()?;
                        $x_and_ys(env, x, ys)
                    } else {
                        $x_only(env, x)
                    }
                } else {
                    Err(Error::UnexpectedArgs)
                }
            }),
        );
    };
}
