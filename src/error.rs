use core::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    DataNotFound,
    DuplicateSymbol,
    Empty,
    ExpectedFunction,
    ExpectedList,
    ExpectedSymbol,
    ExpectedVector,
    ImpossibleConversion,
    IncompleteTokenization,
    MathError,
    MismatchedDelimiter,
    StackError,
    TooFewArgs,
    TooManyArgs,
    TypeMismatch,
    UnbalancedBindings,
    UnexpectedArgs,
    UnexpectedSymbol,
    Unimplemented,
    Uninitialized,
    UnterminatedList,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DataNotFound => f.write_str("Entry was not found."),
            Error::DuplicateSymbol => f.write_str("Duplicate symbol."),
            Error::Empty => f.write_str("Vector/list is empty."),
            Error::ExpectedFunction => f.write_str("Expected a function."),
            Error::ExpectedList => f.write_str("Expected a list."),
            Error::ExpectedSymbol => f.write_str("Expected a symbol."),
            Error::ExpectedVector => f.write_str("Expected a vector."),
            Error::ImpossibleConversion => f.write_str("Conversion is not possible."),
            Error::IncompleteTokenization => f.write_str("Incomplete tokenization."),
            Error::MathError => f.write_str("Underflow, overflow, or divide by zero error."),
            Error::MismatchedDelimiter => f.write_str("Mismatched delimiter."),
            Error::StackError => f.write_str("Error accessing stack data."),
            Error::TooFewArgs => f.write_str("Not enough args were supplied."),
            Error::TooManyArgs => f.write_str("Too many args were supplied."),
            Error::TypeMismatch => f.write_str("Mismatched types."),
            Error::UnbalancedBindings => f.write_str("Some bindings do not have a value to bind."),
            Error::UnexpectedArgs => f.write_str("The expected arguments were not found."),
            Error::UnexpectedSymbol => f.write_str("Unexpected symbol found."),
            Error::Unimplemented => f.write_str("Logic has not been implemented."),
            Error::Uninitialized => f.write_str("Item has not been initialized."),
            Error::UnterminatedList => f.write_str("Unterminated list."),
        }
    }
}
