mod arguments_handler;
mod code_reader;

fn main() {
    let code_path = arguments_handler::get_code_path().unwrap();
    let code = code_reader::read_content(code_path).unwrap();

    println!("{}", code);
}
