use num_derive::{FromPrimitive, ToPrimitive};
use parse_display::{Display, FromStr};
use std::{u16, ptr::null};


#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, Display, FromStr, PartialEq)]
enum Opcode {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI,
    BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI,
    CLV, CMP, CPX, CPY, DEC, DEX, DEY, EOR,
    INC, INX, INY, JMP, JSR, LDA, LDX, LDY,
    LSR, NOP, ORA, PHA, PHP, PLA, LPL, ROL,
    ROR, RTI, RTS, SBC, SEC, SED, SEI, STA,
    STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,

    LABEL,
}

#[derive(Debug, PartialEq, Clone)]
enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Relative
}

pub struct Instruction {
    opcode: Opcode,
    addressing_mode: AddressingMode,
    value: u16,
    size: u8,
    label_name: String
}

pub fn get_instructions(instructions: Vec<String>) -> Vec<Instruction> {
    let mut _to_return: Vec<Instruction> = Vec::new();

    for _i in 0..instructions.len() {
        let opcode_str: &str = &instructions[_i][0..3].trim();
        let operand: &str = &instructions[_i][3..].trim();
        let opcode: Opcode = get_opcode(opcode_str);
        
        if opcode != Opcode::LABEL {
            let addressing_mode: AddressingMode = get_addressing_mode(operand);
            let value: u16 = get_operand_value(operand, addressing_mode.clone());
            let mut label_name: String = String::new();

            if addressing_mode.clone() == AddressingMode::Relative {
                label_name = operand.to_string();
            }

            _to_return.push(Instruction {
                opcode: opcode,
                addressing_mode: get_addressing_mode(operand),
                value: value,
                size: 1, // TODO: find a way to get the size of a given instruction
                label_name: label_name
            });
        } else {
            _to_return.push(Instruction { 
                opcode: opcode, 
                addressing_mode: AddressingMode::Absolute, 
                value: 0, 
                size: 1, 
                label_name: instructions.clone()[_i][0..&instructions.len() - 1].to_string()
            });
        }
    }

    _to_return
}

fn get_opcode(opcode_to_analyze: &str) -> Opcode {
    if opcode_to_analyze.ends_with(':') == false {
        opcode_to_analyze.parse().unwrap()
    } else {
        Opcode::LABEL
    }
}

fn get_addressing_mode(parameters_to_analyze: &str) -> AddressingMode {
    match parameters_to_analyze.chars().next().unwrap() {
        '#' => {
            if parameters_to_analyze.len() == 4 {
                AddressingMode::Immediate
            } else {
                inform_error_and_exit("Error analyzing parameters", -1)
            }
        },
        
        '$' => {
            match parameters_to_analyze.len() {
                3 => {
                    AddressingMode::ZeroPage
                },

                5 => {
                    if parameters_to_analyze.chars().nth(4).unwrap() == 'X' {
                        AddressingMode::ZeroPageX
                    } else {
                        AddressingMode::Absolute
                    }
                },

                7 => {
                    if parameters_to_analyze.chars().nth(6).unwrap() == 'X' {
                        AddressingMode::AbsoluteX
                    } else if parameters_to_analyze.chars().nth(6).unwrap() == 'Y' {
                        AddressingMode::AbsoluteY
                    } else {
                        inform_error_and_exit("Error analyzing parameters", -1)
                    }
                }

                _ => inform_error_and_exit("Error analyzing parameters", -1)
            }
        },

        '(' => {
            if parameters_to_analyze.chars().nth(6).unwrap() == ')' {
                AddressingMode::IndirectX
            } else if parameters_to_analyze.chars().nth(6).unwrap() == 'Y' {
                AddressingMode::IndirectY
            } else {
                inform_error_and_exit("Error analyzing parameters", -1)
            }
        },

        _ => inform_error_and_exit("Error analyzing parameters", -1)
    }
}

fn get_operand_value(parameters: &str, addressing_mode: AddressingMode) -> u16 {
    let mut hex_raw: String = String::new();

    match addressing_mode {
        AddressingMode::ZeroPage | AddressingMode::ZeroPageX | AddressingMode::IndirectX | AddressingMode::IndirectY => hex_raw = parameters[1..3].to_string(),
        AddressingMode::Immediate => hex_raw = parameters[2..4].to_string(),
        AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => hex_raw = parameters[1..5].to_string(),
        AddressingMode::Relative => hex_raw = "ffff".to_string()
    }
    
    u16::from_str_radix(hex_raw.as_str(), 16).unwrap()
}

fn inform_error_and_exit(msg: &str, exit_code: i32) -> ! {
    println!("{msg}");
    std::process::exit(exit_code);
}