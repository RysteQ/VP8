use std::env::args;

use emulator::start_emulator;

mod read_file;
mod analyze_code;
mod emulator;
mod verifier;

fn main() {
    let arguments: Vec<String> = vec!["lol".to_string(), "/home/rysteq/Desktop/Programs/Rust/VP8/test.asm".to_string()];//args().collect::<Vec<String>>();

    if arguments.len() != 2 {
        println!("Please input a path to the file");
        std::process::exit(-1);
    }

    let _file_data_lines: Vec<String> = read_file::read_file(arguments[1].to_string());
    let _command_data = analyze_code::get_instructions(_file_data_lines);

    if verifier::verify_data(_command_data) {
        // emulator::start_emulator(_command_data);
    } else {
        panic!("Incorrect data");
    }
}