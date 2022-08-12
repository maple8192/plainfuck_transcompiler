use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn export_code<T: AsRef<Path>>(code: String, path: T) {
    let file = File::create(path).unwrap();

    let mut writer = BufWriter::new(file);

    writer.write(code.as_bytes()).unwrap();
}
