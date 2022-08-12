pub mod binary_operator_type;

pub enum Node {
    BinaryOperator(BinaryOperatorType, Box<Node>, Box<Node>),
    Number(u32),
}
