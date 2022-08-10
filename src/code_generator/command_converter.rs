use std::collections::vec_deque::VecDeque;
use crate::code_generator::bf_token::BFToken;
use crate::code_generator::bf_token_queue::BFTokenQueue;
use crate::code_generator::command::Command;
use crate::code_generator::command_queue::CommandQueue;

pub struct CommandConverter {
    queue: CommandQueue,
    stack: VecDeque<u32>,
    current_pointer: u32,
    new_address: u32,
}

impl CommandConverter {
    pub fn new(command_queue: CommandQueue) -> Self {
        CommandConverter { queue: command_queue, stack: VecDeque::new(), current_pointer: 0, new_address: 0 }
    }

    pub fn convert(&mut self) -> BFTokenQueue {
        let mut tokens = BFTokenQueue::new();

        while let Some(command) = self.queue.consume_command() {
            match command {
                Command::Push(n) => self.push(&mut tokens, n),
                Command::Add => self.add(&mut tokens),
                Command::Sub => self.sub(&mut tokens),
                Command::Mul => self.mul(&mut tokens),
                Command::Div => self.div(&mut tokens),
                Command::Print => self.print(&mut tokens),
            }
        }

        tokens
    }

    fn push(&mut self, tokens: &mut BFTokenQueue, num: u32) {
        self.move_pointer(tokens, self.new_address);
        tokens.add_token(BFToken::Add(num));
        self.stack.push_back(self.new_address);
        self.new_address += 1;
    }

    fn add(&mut self, tokens: &mut BFTokenQueue) {
        let second = self.stack.pop_back().unwrap();
        let first = self.stack.pop_back().unwrap();

        self.move_pointer(tokens, second);
        tokens.add_token(BFToken::LoopIn);
        self.move_pointer(tokens, first);
        tokens.add_token(BFToken::Add(1));
        self.move_pointer(tokens, second);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        self.move_pointer(tokens, first);

        self.stack.push_back(first);
    }

    fn sub(&mut self, tokens: &mut BFTokenQueue) {
        let second = self.stack.pop_back().unwrap();
        let first = self.stack.pop_back().unwrap();

        self.move_pointer(tokens, second);
        tokens.add_token(BFToken::LoopIn);
        self.move_pointer(tokens, first);
        tokens.add_token(BFToken::Sub(1));
        self.move_pointer(tokens, second);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        self.move_pointer(tokens, first);

        self.stack.push_back(first);
    }

    fn mul(&mut self, tokens: &mut BFTokenQueue) {
        let second = self.stack.pop_back().unwrap();
        let first = self.stack.pop_back().unwrap();
        let temp0 = self.new_address;
        let temp1 = self.new_address + 1;

        self.move_pointer(tokens, first);
        tokens.add_token(BFToken::LoopIn);
        self.move_pointer(tokens, temp1);
        tokens.add_token(BFToken::Add(1));
        self.move_pointer(tokens, first);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        self.move_pointer(tokens, temp1);
        tokens.add_token(BFToken::LoopIn);
        self.move_pointer(tokens, second);
        tokens.add_token(BFToken::LoopIn);
        self.move_pointer(tokens, first);
        tokens.add_token(BFToken::Add(1));
        self.move_pointer(tokens, temp0);
        tokens.add_token(BFToken::Add(1));
        self.move_pointer(tokens, second);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        self.move_pointer(tokens, temp0);
        tokens.add_token(BFToken::LoopIn);
        self.move_pointer(tokens, second);
        tokens.add_token(BFToken::Add(1));
        self.move_pointer(tokens, temp0);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        self.move_pointer(tokens, temp1);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);

        self.stack.push_back(first);
    }

    fn div(&mut self, tokens: &mut BFTokenQueue) {
        let second = self.stack.pop_back().unwrap();
        let first = self.stack.pop_back().unwrap();
        let temp0 = self.new_address;
        let temp1 = self.new_address + 1;
        let temp2 = self.new_address + 2;
        let temp3 = self.new_address + 3;

        self.move_pointer(tokens, first);
        tokens.add_token(BFToken::LoopIn);
        self.move_pointer(tokens, temp0);
        tokens.add_token(BFToken::Add(1));
        self.move_pointer(tokens, first);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        self.move_pointer(tokens, second);
        tokens.add_token(BFToken::LoopIn);
        self.move_pointer(tokens, temp1);
        tokens.add_token(BFToken::Add(1));
        self.move_pointer(tokens, second);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        self.move_pointer(tokens, temp2);
        tokens.add_token(BFToken::Add(1));
        self.move_pointer(tokens, temp0);

        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(2));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(2));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::DecPtr(5));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::IncPtr(2));
        tokens.add_token(BFToken::Sub(1));
        self.current_pointer = temp2;

        self.stack.push_back(temp3);
        self.new_address += 4;
    }

    fn print(&mut self, tokens: &mut BFTokenQueue) {
        let target = self.stack.pop_back().unwrap();
        let temp = self.new_address;

        self.move_pointer(tokens, target);
        tokens.add_token(BFToken::LoopIn);
        self.move_pointer(tokens, temp);
        tokens.add_token(BFToken::Add(1));
        self.move_pointer(tokens, target);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        self.move_pointer(tokens, temp);

        tokens.add_token(BFToken::IncPtr(2));
        tokens.add_token(BFToken::Add(10));
        tokens.add_token(BFToken::DecPtr(2));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(2));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(2));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::DecPtr(6));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::IncPtr(2));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::IncPtr(3));
        tokens.add_token(BFToken::Add(10));
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(2));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(2));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::DecPtr(5));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::IncPtr(2));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Add(6));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::Add(8));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::Print);
        tokens.add_token(BFToken::DecPtr(2));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::Add(6));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Add(8));
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::Print);
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::DecPtr(2));
        tokens.add_token(BFToken::Add(6));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::Add(8));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::Print);
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::DecPtr(2));
        tokens.add_token(BFToken::LoopIn);
        tokens.add_token(BFToken::Sub(1));
        tokens.add_token(BFToken::DecPtr(1));
        tokens.add_token(BFToken::Add(1));
        tokens.add_token(BFToken::IncPtr(1));
        tokens.add_token(BFToken::LoopOut);
        tokens.add_token(BFToken::DecPtr(1));
        self.current_pointer = temp;

        self.new_address += 1;
    }

    fn move_pointer(&mut self, tokens: &mut BFTokenQueue, address: u32) {
        if self.current_pointer > address {
            tokens.add_token(BFToken::DecPtr(self.current_pointer - address));
        } else if self.current_pointer < address {
            tokens.add_token(BFToken::IncPtr(address - self.current_pointer));
        }
        self.current_pointer = address;
    }
}
