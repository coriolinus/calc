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
#[derive(Debug, PartialEq)]
pub enum Term<N> {
    Literal(T),
    Constant(Constant),
    History(HistoryIndexKind, usize),
}

impl<T: Clone> Clone for Term<N> {
    fn clone(&self) -> Self {
        match self {
            Self::Literal(t) => Self::Literal(t.clone()),
            Self::Constant(c) => Self::Constant(*c),
            Self::History(ik, n) => Self::History(*ik, *n),
        }
    }
}

impl<T: Copy> Copy for Term<N> {}

impl<T: Eq> Eq for Term<N> {}

/// An expression or subexpression
pub enum Expr<N> {
    Term(Term<N>),
    Prefix(PrefixOperator, Box<Expr<N>>),
    Infix(Box<Expr<N>>, InfixOperator, Term<N>),
    Parenthesized(Box<Expr<N>>),
}
