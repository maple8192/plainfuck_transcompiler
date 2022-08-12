use crate::code_generator::command::Command;
use crate::code_generator::command_converter::CommandConverter;
use crate::code_generator::command_queue::CommandQueue;
use crate::code_generator::parser::Parser;
use crate::code_generator::parser::node::Node;
use crate::code_generator::parser::node::binary_operator_type::BinaryOperatorType;
use crate::tokenizer::token_queue::TokenQueue;

pub fn generate_code(queue: TokenQueue) -> String {
    let mut expr_queue = Parser::new(queue).parse();

    let mut command_queue = CommandQueue::new();

    while let Some(expr) = expr_queue.consume() {
        expr_to_command(&mut command_queue, &expr);
        command_queue.add_command(Command::Print);
    }

    CommandConverter::new(command_queue).convert()
}

fn expr_to_command(command_queue: &mut CommandQueue, expr: &Node) {
    if let &Node::Number(n) = expr {
        command_queue.add_command(Command::Push(n));
        return;
    }

    if let Node::BinaryOperator(t, a, b) = expr {
        expr_to_command(command_queue, a.as_ref());
        expr_to_command(command_queue, b.as_ref());

        match *t {
            BinaryOperatorType::Add => command_queue.add_command(Command::Add),
            BinaryOperatorType::Sub => command_queue.add_command(Command::Sub),
            BinaryOperatorType::Mul => command_queue.add_command(Command::Mul),
            BinaryOperatorType::Div => command_queue.add_command(Command::Div),
            BinaryOperatorType::Equal => command_queue.add_command(Command::Equal),
            BinaryOperatorType::NotEqual => { command_queue.add_command(Command::Equal); command_queue.add_command(Command::Not); }
            BinaryOperatorType::Less => command_queue.add_command(Command::Less),
            BinaryOperatorType::LessOrEqual => { command_queue.add_command(Command::Greater); command_queue.add_command(Command::Not); }
            BinaryOperatorType::Greater => command_queue.add_command(Command::Greater),
            BinaryOperatorType::GreaterOrEqual => { command_queue.add_command(Command::Less); command_queue.add_command(Command::Not); }
        }
    }
}
