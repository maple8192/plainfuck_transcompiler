use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn export_code(name: String) {
    let file_path_str = format!(".\\{}.bf", name);
    let file_path = Path::new(file_path_str.as_str());

    let file = File::create(file_path).unwrap();

    let mut writer = BufWriter::new(file);
    writer.write(b"Hello World").unwrap();
}
