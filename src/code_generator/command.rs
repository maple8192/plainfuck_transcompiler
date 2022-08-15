use crate::code_generator::command_queue::CommandQueue;

pub enum Command {
    Push(u32),
    Copy(String),
    Add,
    Sub,
    Mul,
    Div,
    Not,
    Equal,
    Less,
    Greater,
    Print,
    Assign(String),
    If(CommandQueue, CommandQueue),
}
