pub enum Expression {
    BinaryOperator(BinaryOperatorType, Box<Expression>, Box<Expression>),
    Number(u32),
}

pub enum BinaryOperatorType {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}
