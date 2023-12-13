use std::fs;

pub fn parse_file(file_to_path: &str) -> String {
    let content = fs::read_to_string(file_to_path).unwrap();

    content
}