pub enum BFToken {
    Add(u32),
    Sub(u32),
    IncPtr(u32),
    DecPtr(u32),
    LoopIn,
    LoopOut,
    Print,
    Read,
}
