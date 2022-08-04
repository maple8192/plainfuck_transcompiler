use std::collections::vec_deque::VecDeque;
use crate::code_generator::command::Command;

pub struct CodeData {
    commands: VecDeque<Command>,
}

impl CodeData {
    pub fn new() -> Self {
        CodeData { commands: VecDeque::new() }
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.push_back(command);
    }

    pub fn consume_command(&mut self) -> Option<Command> {
        let front = self.commands.pop_front();

        if let Some(c) = front {
            Some(c)
        } else {
            None
        }
    }
}
