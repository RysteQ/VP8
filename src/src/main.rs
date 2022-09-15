use std::env::args;

mod read_file;

fn main() {
    let arguments: Vec<String> = args().collect::<Vec<String>>();

    if arguments.len() != 1 {
        println!("Please input a path to the file");
        std::process::exit(-1);
    }

    let file_data_lines: Vec<String> = read_file::ReadFile(arguments[1].to_string());
}
