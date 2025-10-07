use lalrpop_util::lalrpop_mod;
use num_runtime_fmt::NumFmt;

use crate::{Context, Result, Value, ValueError};

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
    fn evaluate(&self, operand: Value) -> Result {
        match self {
            Self::Negation => Ok(-operand),
            Self::Not => !operand,
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
    fn evaluate(&self, left: Value, right: Value) -> Result {
        match self {
            Self::Add => Ok(left + right),
            Self::Sub => Ok(left - right),
            Self::Mul => Ok(left * right),
            Self::Div => Ok(left / right),
            Self::TruncDiv => Ok(left.trunc_div(right)),
            Self::Rem => Ok(left % right),
            Self::Pow => left.pow(right),
            Self::Lshift => left << right,
            Self::Rshift => left >> right,
            Self::RotateL => left.rotate_left(right),
            Self::RotateR => left.rotate_right(right),
            Self::BitAnd => left & right,
            Self::BitOr => left | right,
            Self::BitXor => left ^ right,
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
    fn evaluate(&self, operand: Value) -> Value {
        match self {
            Self::Abs => operand.abs(),
            Self::Ceil => operand.ceil(),
            Self::Floor => operand.floor(),
            Self::Round => operand.round(),
            Self::Sin => operand.sin(),
            Self::Cos => operand.cos(),
            Self::Tan => operand.tan(),
            Self::Sinh => operand.sinh(),
            Self::Cosh => operand.cosh(),
            Self::Tanh => operand.tanh(),
            Self::Asin => operand.asin(),
            Self::Acos => operand.acos(),
            Self::Atan => operand.atan(),
            Self::Asinh => operand.asinh(),
            Self::Acosh => operand.acosh(),
            Self::Atanh => operand.atanh(),
            Self::Rad => operand.rad(),
            Self::Deg => operand.deg(),
            Self::Sqrt => operand.sqrt(),
            Self::Cbrt => operand.cbrt(),
            Self::Log => operand.log(),
            Self::Lg => operand.lg(),
            Self::Ln => operand.ln(),
            Self::Exp => operand.exp(),
        }
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
    fn evaluate(&self, ctx: &Context) -> Result {
        match self {
            Self::Literal(s) => Value::parse_decimal(s).map_err(Into::into),
            Self::HexLiteral(s) => Value::parse_hex(s).map_err(Into::into),
            Self::OctLiteral(s) => Value::parse_octal(s).map_err(Into::into),
            Self::BinLiteral(s) => Value::parse_binary(s).map_err(Into::into),
            Self::Constant(Constant::E) => Ok(Value::E),
            Self::Constant(Constant::Pi) => Ok(Value::PI),
            Self::History(kind, idx) => {
                let err = || ValueError::HistoryOOB(*kind, *idx, ctx.history.len());
                let real_idx = match kind {
                    HistoryIndexKind::Absolute => *idx,
                    HistoryIndexKind::Relative => {
                        ctx.history.len().checked_sub(*idx).ok_or_else(err)?
                    }
                };
                ctx.history.get(real_idx).cloned().ok_or_else(err)
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
    pub(crate) fn evaluate(&self, ctx: &Context) -> Result {
        match self {
            Self::Term(term) => term.evaluate(ctx),
            Self::Prefix(prefix, expr) => prefix.evaluate(expr.evaluate(ctx)?),
            Self::Infix(left, infix, right) => {
                infix.evaluate(left.evaluate(ctx)?, right.evaluate(ctx)?)
            }
            Self::Func(func, expr) => Ok(func.evaluate(expr.evaluate(ctx)?)),
            Self::Group(expr) => expr.evaluate(ctx),
        }
    }
}

/// Error produced by [`AnnotatedExpr`].
#[derive(Debug, thiserror::Error)]
pub enum AnnotatedError {
    #[error(transparent)]
    Calculation(ValueError),
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
    pub fn evaluate(&self, ctx: &Context) -> Result<(Value, String), AnnotatedError> {
        let value = self
            .expr
            .evaluate(ctx)
            .map_err(AnnotatedError::Calculation)?;
        let formatted = self.format.fmt(value)?;
        Ok((value, formatted))
    }
}
