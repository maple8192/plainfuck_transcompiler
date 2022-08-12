use std::collections::vec_deque::VecDeque;
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

    pub fn convert(&mut self) -> String {
        let mut code = String::new();

        while let Some(command) = self.queue.consume_command() {
            match command {
                Command::Push(n) => self.push(&mut code, n),
                Command::Add => self.add(&mut code),
                Command::Sub => self.sub(&mut code),
                Command::Mul => self.mul(&mut code),
                Command::Div => self.div(&mut code),
                Command::Not => self.not(&mut code),
                Command::Equal => self.equal(&mut code),
                Command::Less => self.less(&mut code),
                Command::Greater => self.greater(&mut code),
                Command::Print => self.print(&mut code),
            }
        }

        code
    }

    fn push(&mut self, code: &mut String, num: u32) {
        code.push_str(self.format(format!("({})", self.new_address)).as_str());
        for _ in 0..num {
            code.push('+');
        }
        self.stack.push_back(self.new_address);
        self.new_address += 1;
    }

    fn add(&mut self, code: &mut String) {
        let second = self.stack.pop_back().unwrap();
        let first = self.stack.pop_back().unwrap();

        code.push_str(self.format(format!("({1})[({0})+({1})-]", first, second)).as_str());

        self.stack.push_back(first);
    }

    fn sub(&mut self, code: &mut String) {
        let second = self.stack.pop_back().unwrap();
        let first = self.stack.pop_back().unwrap();

        code.push_str(self.format(format!("({})[({})-({})-]", second, first, second)).as_str());

        self.stack.push_back(first);
    }

    fn mul(&mut self, code: &mut String) {
        let second = self.stack.pop_back().unwrap();
        let first = self.stack.pop_back().unwrap();
        let temp0 = self.new_address;
        let temp1 = self.new_address + 1;

        code.push_str(self.format(format!("({0})[({3})+({0})-]({3})[({1})[({0})+({2})+({1})-]({2})[({1})+({2})-]({3})-]", first, second, temp0, temp1)).as_str());

        self.stack.push_back(first);
    }

    fn div(&mut self, code: &mut String) {
        let second = self.stack.pop_back().unwrap();
        let first = self.stack.pop_back().unwrap();
        let temp0 = self.new_address;
        let temp1 = self.new_address + 1;
        let temp2 = self.new_address + 2;
        let temp3 = self.new_address + 3;

        code.push_str(self.format(format!("({0})[({2})+({0})-]({1})[({3})+({1})-]({4})[->-[>+>>]>[[-<+>]+>+>>]<<<<<]>>-", first, second, temp0, temp1, temp2)).as_str());

        self.current_pointer = temp2;
        self.stack.push_back(temp3);
        self.new_address += 4;
    }

    fn not(&mut self, code: &mut String) {
        let target = self.stack.pop_back().unwrap();
        let temp = self.new_address;

        code.push_str(self.format(format!("({1})+({0})[[-]({1})-({0})]({1})[-({0})+({1})]", target, temp)).as_str());

        self.stack.push_back(target);
    }

    fn equal(&mut self, code: &mut String) {
        let second = self.stack.pop_back().unwrap();
        let first = self.stack.pop_back().unwrap();

        code.push_str(self.format(format!("({0})[-({1})-({0})]+({1})[({0})-({1})[-]]", first, second)).as_str());

        self.stack.push_back(first);
    }

    fn less(&mut self, code: &mut String) {
        let second = self.stack.pop_back().unwrap();
        let first = self.stack.pop_back().unwrap();
        let result = self.new_address;
        let temp0 = self.new_address + 1;
        let temp1 = self.new_address + 2;

        code.push_str(self.format(format!("({1})[({3})+({0})[-({3})[-]({4})+({0})]({3})[-({2})+({3})]({4})[-({0})+({4})]({0})-({1})-]", first, second, result, temp0, temp1)).as_str());

        self.stack.push_back(result);
        self.new_address += 1;
    }

    fn greater(&mut self, code: &mut String) {
        let second = self.stack.pop_back().unwrap();
        let first = self.stack.pop_back().unwrap();
        let result = self.new_address;
        let temp0 = self.new_address + 1;
        let temp1 = self.new_address + 2;

        code.push_str(self.format(format!("({0})[({3})+({1})[-({3})[-]({4})+({1})]({3})[-({2})+({3})]({4})[-({1})+({4})]({1})-({0})-]", first, second, result, temp0, temp1)).as_str());

        self.stack.push_back(result);
        self.new_address += 1;
    }

    fn print(&mut self, code: &mut String) {
        let target = self.stack.pop_back().unwrap();
        let temp = self.new_address;

        code.push_str(self.format(format!("({0})[({1})+({0})-]({1})>>++++++++++<<[->+>-[>+>>]>[+[-<+>]>+>>]<<<<<<]>>[-]>>>++++++++++<[->-[>+>>]>[+[-<+>]>+>>]<<<<<]>[-]>>[>++++++[-<++++++++>]<.<<+>+>[-]]<[<[->-<]++++++[->++++++++<]>.[-]]<<++++++[-<++++++++>]<.[-]<<[-<+>]<", target, temp)).as_str());

        self.current_pointer = temp;
        self.new_address += 1;
    }

    fn format(&mut self, code: String) -> String {
        let mut result = String::new();

        let mut index = 0;
        let mut num_buffer;
        while index < code.len() {
            if code.chars().nth(index).unwrap() == '(' {
                num_buffer = String::new();

                for i in (index + 1)..code.len() {
                    let c = code.chars().nth(i).unwrap();
                    if let '0'..='9' = c {
                        num_buffer.push(c);
                    } else if c == ')' {
                        if i == index + 1 {
                            panic!("");
                        }
                        index = i;
                        break;
                    } else {
                        panic!("");
                    }

                    if i == code.len() - 1 {
                        panic!("");
                    }
                }

                let address = num_buffer.parse::<u32>().unwrap();
                if self.current_pointer > address {
                    for _ in 0..(self.current_pointer - address) {
                        result.push('<');
                    }
                } else if self.current_pointer < address {
                    for _ in 0..(address - self.current_pointer) {
                        result.push('>');
                    }
                }
                self.current_pointer = address;
            } else {
                result.push(code.chars().nth(index).unwrap());
            }

            index += 1;
        }

        result
    }
}
