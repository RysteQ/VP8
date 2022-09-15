use std::fs;

pub fn read_file(path: String) -> Vec<String> {
    let file_data_raw: String = fs::read_to_string(path).expect("Error reading file");
    let file_data_lines= file_data_raw.split('\n');

    file_data_lines.map(|p| p.to_string()).collect()
}