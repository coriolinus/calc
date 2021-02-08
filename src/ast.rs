#[derive(Debug)]
pub enum PrefixOperator {
    Negation,
    Not,
}

#[derive(Debug)]
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

pub enum Constant {
    E,
    Pi,
}
