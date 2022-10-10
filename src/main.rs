use std::env::args;

mod read_file;
mod analyze_code;
mod emulator;
mod verifier;
mod instruction_functions;
mod system;
mod window;

fn main() {
    // let arguments: Vec<String> = vec!["ignore_this".to_string(), "/home/rysteq/Desktop/Programs/Rust/VP8/test.asm".to_string()];
    let arguments: Vec<String> = args().collect::<Vec<String>>();

    if arguments.len() != 2 {
        println!("Please input a path to the file");
        std::process::exit(-1);
    }
    
    let _file_data_lines: Vec<String> = read_file::read_file(arguments[1].to_string());
    let _command_data: Vec<analyze_code::Instruction> = analyze_code::get_instructions(_file_data_lines.clone());
    
    if verifier::verify_data(_command_data.clone()) {
        emulator::start_emulator(_command_data);
    }
}