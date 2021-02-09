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
    Sub,
    Mul,
    Div,
    TruncDiv,
    Exp,
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
    HexLiteral(&'input str),
    OctLiteral(&'input str),
    BinLiteral(&'input str),
    Constant(Constant),
    History(HistoryIndexKind, usize),
}

/// An expression or subexpression
pub enum Expr<'input> {
    Term(Term<'input>),
    Prefix(PrefixOperator, Box<Expr<'input>>),
    Infix(Box<Expr<'input>>, InfixOperator, Box<Expr<'input>>),
    Func(Function, Box<Expr<'input>>),
    Group(Box<Expr<'input>>),
}
