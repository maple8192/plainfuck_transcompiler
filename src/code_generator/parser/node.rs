pub mod binary_operator_type;

use crate::code_generator::parser::node::binary_operator_type::BinaryOperatorType;

pub enum Node {
    BinaryOperator(BinaryOperatorType, Box<Node>, Box<Node>),
    Variable(String),
    If(Box<Node>, Box<Node>, Option<Box<Node>>),
    Number(u32),
}
