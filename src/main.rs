mod arguments_handler;

fn main() {
    let code_path = arguments_handler::get_code_path();

    if let Ok(path) = code_path {
        println!("{}", path);
    }
}
