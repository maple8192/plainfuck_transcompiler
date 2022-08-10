use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use crate::code_generator::bf_token::BFToken;
use crate::code_generator::bf_token_queue::BFTokenQueue;

pub fn export_code<T: AsRef<Path>>(token_queue: BFTokenQueue, path: T) {
    let code_str = stringify_code(token_queue);

    export_code_file(path, code_str);
}

fn stringify_code(mut queue: BFTokenQueue) -> String {
    let mut code = String::new();

    while let Some(token) = queue.consume_token() {
        match token {
            BFToken::Add(n) => for _ in 0..n { code.push('+'); }
            BFToken::Sub(n) => for _ in 0..n { code.push('-'); }
            BFToken::IncPtr(n) => for _ in 0..n { code.push('>'); }
            BFToken::DecPtr(n) => for _ in 0..n { code.push('<'); }
            BFToken::LoopIn => code.push('['),
            BFToken::LoopOut => code.push(']'),
            BFToken::Print => code.push('.'),
            BFToken::Read => code.push(','),
        }
    }

    code
}

fn export_code_file<T: AsRef<Path>>(path: T, content: String) {
    let file = File::create(path).unwrap();

    let mut writer = BufWriter::new(file);

    writer.write(content.as_bytes()).unwrap();
}
