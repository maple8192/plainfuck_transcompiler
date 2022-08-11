use crate::code_generator::bf_token_queue::BFTokenQueue;
use crate::code_generator::command::Command;
use crate::code_generator::command_converter::CommandConverter;
use crate::code_generator::command_queue::CommandQueue;
use crate::code_generator::expression_parser::expression::Expression;
use crate::code_generator::expression_parser::expression_parser::ExpressionParser;
use crate::tokenizer::token_queue::TokenQueue;

pub fn generate_code(queue: TokenQueue) -> BFTokenQueue {
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

    if let Expression::Add(a, b) |
           Expression::Sub(a, b) |
           Expression::Mul(a, b) |
           Expression::Div(a, b) |
           Expression::Equal(a, b) |
           Expression::NotEqual(a, b) |
           Expression::Less(a, b) |
           Expression::LessOrEqual(a, b) |
           Expression::Greater(a, b) |
           Expression::GreaterOrEqual(a, b) = expr {
        command_queue = expr_to_command(command_queue, a.as_ref());
        command_queue = expr_to_command(command_queue, b.as_ref());

        match expr {
            &Expression::Add(_, _) => command_queue.add_command(Command::Add),
            &Expression::Sub(_, _) => command_queue.add_command(Command::Sub),
            &Expression::Mul(_, _) => command_queue.add_command(Command::Mul),
            &Expression::Div(_, _) => command_queue.add_command(Command::Div),
            &Expression::Equal(_, _) => command_queue.add_command(Command::Equal),
            &Expression::NotEqual(_, _) => { command_queue.add_command(Command::Equal); command_queue.add_command(Command::Not); }
            &Expression::Less(_, _) => command_queue.add_command(Command::Less),
            &Expression::LessOrEqual(_, _) => { command_queue.add_command(Command::Greater); command_queue.add_command(Command::Not); }
            &Expression::Greater(_, _) => command_queue.add_command(Command::Greater),
            &Expression::GreaterOrEqual(_, _) => { command_queue.add_command(Command::Less); command_queue.add_command(Command::Not); }
            _ => (),
        }
    }

    command_queue
}
