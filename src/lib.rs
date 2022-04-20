#![cfg_attr(not(feature = "std"), no_std)]
#![feature(iterator_try_collect)]
#![feature(iterator_try_reduce)]

extern crate alloc;

pub mod builtins;
pub mod environment;
pub mod error;
pub mod expression;

pub use environment::Environment;
pub use error::Error;
pub use expression::Expression;
