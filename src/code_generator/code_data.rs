use std::collections::vec_deque::VecDeque;
use crate::code_generator::command::Command;

pub struct CodeData {
    pub commands: VecDeque<Command>,
}
