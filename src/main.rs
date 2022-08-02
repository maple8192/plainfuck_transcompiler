mod arguments_handler;
mod code_reader;
mod code_exporter;
mod tokenizer;

fn main() {
    let code_path = arguments_handler::get_code_path().unwrap();
    let code = code_reader::read_content(code_path).unwrap();

    code_exporter::export_code("result".to_string(), code);
}
