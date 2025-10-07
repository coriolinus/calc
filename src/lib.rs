//! Calculator library for evaluating freehand mathematical expressions.
//!
//! The general workflow goes like this:
//!
//! - Create a [`Context`]: a reusable type which contains expression history.
//!
//! - Parse an [`ast::Expr`] with [`ast::parser::ExprParser`].
//! - Evaluate that expression with [`Context::evaluate`].
//!
//! You can freely modify the parsed expression; the types in [`ast`] are all public.
//!
//! To enable calculation based on your custom numeric type, just impl [`types::Calcable`] for your type.

pub mod ast;
mod value;

use ast::{
    parser::{AnnotatedExprParser, ExprParser},
    AnnotatedError, ParseError as UserParseError,
};
use lalrpop_util::ParseError;
pub(crate) use value::Result;
pub use value::{ArithmeticError, Error as ValueError, ParseValueError, Value};

/// Calculation context.
///
/// Stores a history of calculated values, so that the history lookups (`@`) work properly.
/// Also reifies the numeric type backing the calculations.
#[derive(Default)]
pub struct Context {
    pub history: Vec<Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parsing")]
    Parse(#[from] ParseError<usize, &'static str, UserParseError>),
    #[error("Evaluating")]
    Eval(#[from] ValueError),
    #[error("Formatting")]
    Format(#[source] num_runtime_fmt::Error),
}

impl From<AnnotatedError> for Error {
    fn from(err: AnnotatedError) -> Self {
        match err {
            AnnotatedError::Calculation(err) => Self::Eval(err),
            AnnotatedError::Format(err) => Self::Format(err),
        }
    }
}

impl Context {
    /// Evaluate an expression in this context.
    ///
    /// This both returns the calculated value and stores a copy in the context's history.
    pub fn evaluate(&mut self, expr: &str) -> Result<Value, Error> {
        let parser = ExprParser::new();
        let expr = parser.parse(expr).map_err(|err| err.map_token(|_| ""))?;
        let result = expr.evaluate(self).map_err(Error::Eval)?;
        self.history.push(result);
        Ok(result)
    }
}

impl Context {
    /// Evaluate an annotated expression in this context.
    ///
    /// Annotations can include output formatting directives. Therefore, the return value
    /// is a formatted `String`.
    ///
    /// This also stores a copy in the context's history.
    pub fn evaluate_annotated(&mut self, expr: &str) -> Result<String, Error> {
        let parser = AnnotatedExprParser::new();
        let expr = parser.parse(expr).map_err(|err| err.map_token(|_| ""))?;
        let (result, formatted) = expr.evaluate(self)?;
        self.history.push(result);
        Ok(formatted)
    }
}
