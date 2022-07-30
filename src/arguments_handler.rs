use std::error::Error;
use clap::{App, Arg};

pub fn get_code_path() -> Result<String, Box<dyn Error>> {
    let app = App::new("plainfuck_transcompiler")
        .arg(Arg::new("code_path")
            .required(true));

    let args = app.try_get_matches()?;
    Ok(match args.value_of("code_path") { Some(path) => path.to_string(), None => "".to_string() })
}
