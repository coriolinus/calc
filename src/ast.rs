use lalrpop_util::lalrpop_mod;
use num_runtime_fmt::NumFmt;

use crate::{
    types::{Calcable, CalcableError},
    Context,
};

// no point getting style warnings for generated code
lalrpop_mod!(#[allow(clippy::all)] pub parser);

/// Error encountered while parsing an expression
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("index must fit into usize")]
    Index(#[source] std::num::ParseIntError),
    #[error("failed to parse format string")]
    Format(#[from] num_runtime_fmt::parse::Error),
}

/// A prefix operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixOperator {
    Negation,
    Not,
}

impl PrefixOperator {
    fn evaluate<N: Calcable>(&self, operand: N) -> Result<N, <N as Calcable>::Err> {
        match self {
            Self::Negation => operand.neg().ok_or_else(|| N::Err::unimplemented("-")),
            Self::Not => operand.not().ok_or_else(|| N::Err::unimplemented("!")),
        }
    }
}

/// An infix operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfixOperator {
    Add,
    Sub,
    Mul,
    Div,
    TruncDiv,
    Pow,
    Rem,
    Lshift,
    Rshift,
    RotateL,
    RotateR,
    BitAnd,
    BitOr,
    BitXor,
}

impl InfixOperator {
    fn evaluate<N: Calcable>(&self, left: N, right: N) -> Result<N, <N as Calcable>::Err> {
        match self {
            Self::Add => <N as Calcable>::add(left, right),
            Self::Sub => <N as Calcable>::sub(left, right),
            Self::Mul => <N as Calcable>::mul(left, right),
            Self::Div => <N as Calcable>::div(left, right),
            Self::TruncDiv => left.trunc_div(right),
            Self::Pow => left.pow(right),
            Self::Rem => left.rem(right),
            Self::Lshift => left.shl(right),
            Self::Rshift => left.shr(right),
            Self::RotateL => left.rotate_left(right),
            Self::RotateR => left.rotate_right(right),
            Self::BitAnd => left
                .bit_and(right)
                .ok_or_else(|| N::Err::unimplemented("&")),
            Self::BitOr => left.bit_or(right).ok_or_else(|| N::Err::unimplemented("|")),
            Self::BitXor => left
                .bit_xor(right)
                .ok_or_else(|| N::Err::unimplemented("^")),
        }
    }
}

/// A function name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Function {
    Abs,
    Ceil,
    Floor,
    Round,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Asin,
    Acos,
    Atan,
    Asinh,
    Acosh,
    Atanh,
    Rad,
    Deg,
    Sqrt,
    Cbrt,
    Log,
    Lg,
    Ln,
    Exp,
}

impl Function {
    fn evaluate<N: Calcable>(&self, operand: N) -> Result<N, <N as Calcable>::Err> {
        let (result, symbol) = match self {
            Self::Abs => (operand.abs(), "abs"),
            Self::Ceil => (operand.ceil(), "ceil"),
            Self::Floor => (operand.floor(), "floor"),
            Self::Round => (operand.round(), "round"),
            Self::Sin => (operand.sin(), "sin"),
            Self::Cos => (operand.cos(), "cos"),
            Self::Tan => (operand.tan(), "tan"),
            Self::Sinh => (operand.sinh(), "sinh"),
            Self::Cosh => (operand.cosh(), "cosh"),
            Self::Tanh => (operand.tanh(), "tanh"),
            Self::Asin => (operand.asin(), "asin"),
            Self::Acos => (operand.acos(), "acos"),
            Self::Atan => (operand.atan(), "atan"),
            Self::Asinh => (operand.asinh(), "asinh"),
            Self::Acosh => (operand.acosh(), "acosh"),
            Self::Atanh => (operand.atanh(), "atanh"),
            Self::Rad => (operand.rad(), "rad"),
            Self::Deg => (operand.deg(), "deg"),
            Self::Sqrt => (operand.sqrt(), "sqrt"),
            Self::Cbrt => (operand.cbrt(), "cbrt"),
            Self::Log => (operand.log(), "log"),
            Self::Lg => (operand.lg(), "lg"),
            Self::Ln => (operand.ln(), "ln"),
            Self::Exp => (operand.exp(), "exp"),
        };
        result.ok_or_else(|| N::Err::unimplemented(symbol))
    }
}

/// A constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constant {
    E,
    Pi,
}

/// What kind of history lookup is desired.
///
/// Absolute history lookups begin at 0 and increment.
/// Relative history lookups count backwards from the current expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryIndexKind {
    Relative,
    Absolute,
}

/// A term in the expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Term<'input> {
    Literal(&'input str),
    HexLiteral(&'input str),
    OctLiteral(&'input str),
    BinLiteral(&'input str),
    Constant(Constant),
    History(HistoryIndexKind, usize),
}

impl<'input> Term<'input> {
    fn evaluate<N: Calcable>(&self, ctx: &Context<N>) -> Result<N, <N as Calcable>::Err> {
        match self {
            Self::Literal(s) => N::parse_decimal(s),
            Self::HexLiteral(s) => N::parse_hex(s),
            Self::OctLiteral(s) => N::parse_octal(s),
            Self::BinLiteral(s) => N::parse_binary(s),
            Self::Constant(Constant::E) => N::E.ok_or_else(|| N::Err::unimplemented("e")),
            Self::Constant(Constant::Pi) => N::PI.ok_or_else(|| N::Err::unimplemented("pi")),
            Self::History(kind, idx) => {
                let real_idx = match kind {
                    HistoryIndexKind::Absolute => *idx,
                    HistoryIndexKind::Relative => {
                        ctx.history.len().checked_sub(*idx).ok_or_else(|| {
                            N::Err::history_out_of_bounds(*kind, *idx, ctx.history.len())
                        })?
                    }
                };
                match ctx.history.get(real_idx) {
                    Some(n) => Ok(n.clone()),
                    None => Err(N::Err::history_out_of_bounds(
                        *kind,
                        *idx,
                        ctx.history.len(),
                    )),
                }
            }
        }
    }
}

/// An expression or subexpression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'input> {
    Term(Term<'input>),
    Prefix(PrefixOperator, Box<Expr<'input>>),
    Infix(Box<Expr<'input>>, InfixOperator, Box<Expr<'input>>),
    Func(Function, Box<Expr<'input>>),
    Group(Box<Expr<'input>>),
}

impl<'input> Expr<'input> {
    /// Evaluate this expression into its mathematical result.
    pub(crate) fn evaluate<N: Calcable>(
        &self,
        ctx: &Context<N>,
    ) -> Result<N, <N as Calcable>::Err> {
        match self {
            Self::Term(term) => term.evaluate(ctx),
            Self::Prefix(prefix, expr) => prefix.evaluate(expr.evaluate(ctx)?),
            Self::Infix(left, infix, right) => {
                infix.evaluate(left.evaluate(ctx)?, right.evaluate(ctx)?)
            }
            Self::Func(func, expr) => func.evaluate(expr.evaluate(ctx)?),
            Self::Group(expr) => expr.evaluate(ctx),
        }
    }
}

/// Error produced by [`AnnotatedExpr`].
#[derive(Debug, thiserror::Error)]
pub enum AnnotatedError<N>
where
    N: std::fmt::Debug + Calcable,
    <N as Calcable>::Err: 'static,
{
    #[error(transparent)]
    Calculation(<N as Calcable>::Err),
    #[error("failed to render calculation result in desired format")]
    Format(#[from] num_runtime_fmt::Error),
}

/// An expression annotated with some metadata.
pub struct AnnotatedExpr<'input> {
    pub expr: Expr<'input>,
    pub format: NumFmt,
}

impl<'input> AnnotatedExpr<'input> {
    /// Evaluate this expression into its mathematical result.
    ///
    /// Return the result as a bare type and also formatted according to the
    /// requested format string.
    pub fn evaluate<N>(&self, ctx: &Context<N>) -> Result<(N, String), AnnotatedError<N>>
    where
        N: std::fmt::Debug + Calcable + num_runtime_fmt::Numeric,
        <N as Calcable>::Err: 'static,
    {
        let value = self
            .expr
            .evaluate(ctx)
            .map_err(AnnotatedError::Calculation)?;
        let formatted = self.format.fmt(value.clone())?;
        Ok((value, formatted))
    }
}
