use std::fs;

pub fn read_file(path: String) -> Vec<String> {
    let file_data_raw: String = fs::read_to_string(path).expect("Error reading file");
    let file_data_lines_raw= file_data_raw.split('\n');
    let file_data_lines: Vec<String> = file_data_lines_raw.map(|p| p.to_string()).collect();
    let mut to_return: Vec<String> = vec![];

    for i in 0..file_data_lines.len() {
        if file_data_lines[i].trim().len() == 0 || file_data_lines[i].trim().chars().nth(0).unwrap() == ';' {
            continue;
        }

        if file_data_lines[i].contains(";") {
            let to_push: &str = file_data_lines[i].split(&";".to_string()).collect::<Vec<&str>>()[0];
            to_return.push(to_push.trim().to_string());

            continue;
        }

        to_return.push(file_data_lines[i].clone().trim().to_string());
    }

    to_return
}