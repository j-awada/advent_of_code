use std::fs;

pub fn read_file_to_lines(file_path: String) -> Vec<String> {
    let content = fs::read_to_string(file_path).unwrap();
    let lines: Vec<String> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    lines
}