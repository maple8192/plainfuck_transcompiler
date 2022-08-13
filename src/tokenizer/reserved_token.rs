#[derive(PartialEq, Eq)]
pub enum ReservedToken {
    Add,
    Sub,
    Mul,
    Div,
    OpenBracket,
    CloseBracket,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Assign,
    If,
    EndStatement,
}
