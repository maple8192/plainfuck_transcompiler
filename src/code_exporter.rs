use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn export_code(name: String, code: String) {
    let file_path_str = format!(".\\{}.bf", name);
    let file_path = Path::new(file_path_str.as_str());

    let file = File::create(file_path).unwrap();

    let mut writer = BufWriter::new(file);

    for c in code.chars() {
        let mut program = String::new();
        for _ in 0..c as u8 {
            program.push('+');
        }
        program.push_str(".>");

        writer.write(program.as_bytes()).unwrap();
    }
}
