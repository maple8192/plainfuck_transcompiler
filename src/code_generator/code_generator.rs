use crate::code_generator::command::Command;
use crate::code_generator::command_converter::CommandConverter;
use crate::code_generator::command_queue::CommandQueue;
use crate::code_generator::expression_parser::expression::{BinaryOperatorType, Expression};
use crate::code_generator::expression_parser::expression_parser::ExpressionParser;
use crate::tokenizer::token_queue::TokenQueue;

pub fn generate_code(queue: TokenQueue) -> String {
    let expr = ExpressionParser::new(queue).parse();

    let mut command_queue = CommandQueue::new();

    command_queue = expr_to_command(command_queue, &expr);
    command_queue.add_command(Command::Print);

    CommandConverter::new(command_queue).convert()
}

fn expr_to_command(mut command_queue: CommandQueue, expr: &Expression) -> CommandQueue {
    if let &Expression::Number(n) = expr {
        command_queue.add_command(Command::Push(n));
        return command_queue;
    }

    if let Expression::BinaryOperator(t, a, b) = expr {
        command_queue = expr_to_command(command_queue, a.as_ref());
        command_queue = expr_to_command(command_queue, b.as_ref());

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

    command_queue
}
