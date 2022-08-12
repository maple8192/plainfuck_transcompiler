pub enum Node {
    BinaryOperator(BinaryOperatorType, Box<Node>, Box<Node>),
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
