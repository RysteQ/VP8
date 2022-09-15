use std::env::args;

mod read_file;
mod analyze_code;

fn main() {
    let arguments: Vec<String> = args().collect::<Vec<String>>();

    if arguments.len() != 1 {
        println!("Please input a path to the file");
        std::process::exit(-1);
    }

    let _file_data_lines: Vec<String> = read_file::ReadFile(arguments[1].to_string());
}
