//! Calculator library for evaluating freehand mathematical expressions.
//!
//! The general workflow goes like this:
//!
//! - Create a [`Context`]: a reusable type which contains expression history.
//!
//!   This type is parametrized by the numeric type which all calculations will use.
//!
//! - Parse an [`ast::Expr`] with [`ast::parser::ExprParser`].
//! - Evaluate that expression with [`Context::evaluate`].
//!
//! You can freely modify the parsed expression; the types in [`ast`] are all public.
//!
//! To enable calculation based on your custom numeric type, just impl [`types::Calcable`] for your type.

pub mod ast;
pub mod types;

use ast::{
    parser::{AnnotatedExprParser, ExprParser},
    AnnotatedError, ParseError as UserParseError,
};
use lalrpop_util::ParseError;
use num_runtime_fmt::Numeric;
use types::Calcable;

/// Calculation context.
///
/// Stores a history of calculated values, so that the history lookups (`@`) work properly.
/// Also reifies the numeric type backing the calculations.
#[derive(Default)]
pub struct Context<N> {
    pub history: Vec<N>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error<N>
where
    N: std::fmt::Debug + Calcable,
    <N as Calcable>::Err: 'static,
{
    #[error("Parsing")]
    Parse(#[from] ParseError<usize, &'static str, UserParseError>),
    #[error("Evaluating")]
    Eval(#[source] <N as Calcable>::Err),
    #[error("Formatting")]
    Format(#[source] num_runtime_fmt::Error),
}

impl<N> From<AnnotatedError<N>> for Error<N>
where
    N: std::fmt::Debug + Calcable,
    <N as Calcable>::Err: 'static,
{
    fn from(err: AnnotatedError<N>) -> Self {
        match err {
            AnnotatedError::Calculation(err) => Self::Eval(err),
            AnnotatedError::Format(err) => Self::Format(err),
        }
    }
}

impl<N> Context<N>
where
    N: std::fmt::Debug + Calcable,
    <N as Calcable>::Err: 'static,
{
    /// Evaluate an expression in this context.
    ///
    /// This both returns the calculated value and stores a copy in the context's history.
    pub fn evaluate(&mut self, expr: &str) -> Result<N, Error<N>> {
        let parser = ExprParser::new();
        let expr = parser.parse(expr).map_err(|err| err.map_token(|_| ""))?;
        let result = expr.evaluate(self).map_err(Error::Eval)?;
        self.history.push(result.clone());
        Ok(result)
    }
}

impl<N> Context<N>
where
    N: std::fmt::Debug + Calcable + Numeric,
    <N as Calcable>::Err: 'static,
{
    /// Evaluate an annotated expression in this context.
    ///
    /// Annotations can include output formatting directives. Therefore, the return value
    /// is a formatted `String`.
    ///
    /// This also stores a copy in the context's history.
    pub fn evaluate_annotated(&mut self, expr: &str) -> Result<String, Error<N>> {
        let parser = AnnotatedExprParser::new();
        let expr = parser.parse(expr).map_err(|err| err.map_token(|_| ""))?;
        let (result, formatted) = expr.evaluate(self)?;
        self.history.push(result);
        Ok(formatted)
    }
}
