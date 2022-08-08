use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use crate::code_generator::command_queue::CommandQueue;
use crate::code_generator::command::Command;

pub fn export_code<T: AsRef<Path>>(code_data: CommandQueue, path: T) {
    let code_str = stringify_code(code_data);

    export_code_file(path, code_str);
}

fn stringify_code(mut code_data: CommandQueue) -> String {
    let mut code = String::new();

    loop {
        let command = code_data.consume_command();

        if let Some(command) = command {
            match command {
                Command::Add(n) => for _ in 0..n { code.push('+'); }
                Command::Sub(n) => for _ in 0..n { code.push('-'); }
            }
        } else {
            break;
        }
    }

    code
}

fn export_code_file<T: AsRef<Path>>(path: T, content: String) {
    let file = File::create(path).unwrap();

    let mut writer = BufWriter::new(file);

    writer.write(content.as_bytes()).unwrap();
}
