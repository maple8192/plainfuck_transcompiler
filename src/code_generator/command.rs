pub enum Command {
    Push(u32),
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
}
