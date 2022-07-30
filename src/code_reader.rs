use std::{fs, io};
use std::path::Path;

pub fn read_content<T: AsRef<Path>>(path: T) -> io::Result<String> {
    Ok(fs::read_to_string(path)?)
}
