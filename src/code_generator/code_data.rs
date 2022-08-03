use std::collections::vec_deque::VecDeque;
use crate::code_generator::command::Command;

struct CodeData {
    commands: VecDeque<Command>,
}
