use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

/// A prefix operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixOperator {
    Negation,
    Not,
}

/// An infix operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfixOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    TruncateDivide,
    Exponentiate,
    Rem,
    Lshift,
    Rshift,
    BitAnd,
    BitOr,
    BitXor,
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
    Dec,
    Sqrt,
    Cbrt,
    Log,
    Lg,
    Ln,
    Exp,
}

/// A constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constant {
    E,
    Pi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryIndexKind {
    Relative,
    Absolute,
}

/// A term in the expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Term<'input> {
    Literal(&'input str),
    Constant(Constant),
    History(HistoryIndexKind, usize),
}

/// An expression or subexpression
pub enum Expr<'input> {
    Term(Term<'input>),
    Prefix(PrefixOperator, Box<Expr<'input>>),
    Infix(Box<Expr<'input>>, InfixOperator, Term<'input>),
    Parenthesized(Box<Expr<'input>>),
}
