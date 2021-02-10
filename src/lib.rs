pub mod ast;
pub mod types;

use ast::parser::ExprParser;
use lalrpop_util::ParseError;
use types::Calcable;

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
    Parse(#[from] ParseError<usize, &'static str, &'static str>),
    #[error("Evaluating")]
    Eval(#[source] <N as Calcable>::Err),
}

impl<N> Context<N>
where
    N: std::fmt::Debug + Calcable,
    <N as Calcable>::Err: 'static,
{
    /// Evaluate an expression in this context.
    pub fn evaluate(&mut self, expr: &str) -> Result<N, Error<N>> {
        let parser = ExprParser::new();
        let expr = parser.parse(expr).map_err(|err| err.map_token(|_| ""))?;
        let result = expr.evaluate(&self).map_err(Error::Eval)?;
        self.history.push(result.clone());
        Ok(result)
    }
}
