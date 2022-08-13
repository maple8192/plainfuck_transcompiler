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
}
