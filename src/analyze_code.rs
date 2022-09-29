use parse_display::{Display, FromStr};
use std::u16;

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq, Eq)]
pub enum Opcode {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI,
    BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI,
    CLV, CMP, CPX, CPY, DEC, DEX, DEY, EOR,
    INC, INX, INY, JMP, JSR, LDA, LDX, LDY,
    LSR, NOP, ORA, PHA, PLA, ROL, ROR, RTS, 
    SBC, SEC, SED, SEI, STA, STX, STY, TAX, 
    TAY, TSX, TXA, TXS, TYA,

    LABEL,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Relative,
    Implied,
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
    pub value: u16,
    pub label_name: String
}

pub fn get_instructions(instructions: Vec<String>) -> Vec<Instruction> {
    let mut to_return: Vec<Instruction> = Vec::new();

    for i in 0..instructions.len() {
        let opcode_str: &str = instructions[i][0..3].trim();
        let operand: &str = instructions[i][3..].trim();
        let opcode: Opcode = get_opcode(opcode_str);
        
        if opcode != Opcode::LABEL {
            let addressing_mode: AddressingMode = get_addressing_mode(operand);
            let value: u16 = get_operand_value(operand, addressing_mode);
            let mut label_name: String = String::new();

            if addressing_mode == AddressingMode::Relative {
                label_name = operand.to_string();
            }

            to_return.push(Instruction {
                opcode,
                addressing_mode,
                value,
                label_name
            });

            continue;
        }

        to_return.push(Instruction { 
            opcode, 
            addressing_mode: AddressingMode::Absolute, 
            value: 0,
            label_name: instructions.clone()[i][0..&instructions.len() - 1].to_string()
        });
    }
    
    to_return
}

fn get_opcode(opcode_to_analyze: &str) -> Opcode {
    if !opcode_to_analyze.ends_with(':') {
        opcode_to_analyze.parse().unwrap()
    } else {
        Opcode::LABEL
    }
}

fn get_addressing_mode(parameters_to_analyze: &str) -> AddressingMode {
    let to_analyze = remove_whitespaces(parameters_to_analyze);

    if to_analyze.is_empty() {
        return AddressingMode::Implied;
    }

    match to_analyze.chars().next().unwrap() {
        '#' => {
            if to_analyze.len() == 4 {
                return AddressingMode::Immediate;
            }

            panic!("Error in analyze_code.rs at get_addressing_mode");
        },
        
        '$' => {
            match to_analyze.len() {
                3 => {
                    AddressingMode::ZeroPage
                },

                5 => {
                    let nth: char = to_analyze.chars().nth(4).unwrap();

                    match nth {
                        'X' => AddressingMode::ZeroPageX,
                        'Y' => AddressingMode::ZeroPageY,
                        
                        _ => AddressingMode::Absolute
                    }
                },

                7 => {
                    let nth: char = to_analyze.chars().nth(6).unwrap();

                    match nth {
                        'X' => AddressingMode::AbsoluteX,
                        'Y' => AddressingMode::AbsoluteY,
                        
                        _ => panic!("Error in analyze_code.rs at get_addressing_mode")
                    }
                }

                _ => panic!("Error in analyze_code.rs at get_addressing_mode")
            }
        },

        '(' => {
            let nth = to_analyze.chars().nth(6).unwrap();

            match nth {
                ')' => AddressingMode::IndirectX,
                'Y' => AddressingMode::IndirectY,
                _ => panic!("Error in analyze_code.rs at get_addressing_mode")
            }
        },

        _ => panic!("Error in analyze_code.rs at get_addressing_mode")
    }
}

fn remove_whitespaces(input: &str) -> String {
    input.replace(" ", "")
}

fn get_operand_value(parameters: &str, addressing_mode: AddressingMode) -> u16 {
    u16::from_str_radix({
        match addressing_mode {
            AddressingMode::Immediate => &parameters[2..],
            AddressingMode::ZeroPage | AddressingMode::Absolute => &parameters[1..],
            AddressingMode::ZeroPageX | AddressingMode::ZeroPageY | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => &parameters[1..].split(&",".to_string()).collect::<Vec<&str>>()[0],
            AddressingMode::IndirectX => &parameters[2..].split(&",".to_string()).collect::<Vec<&str>>()[0],
            AddressingMode::IndirectY => &parameters[1..].split(&")".to_string()).collect::<Vec<&str>>()[0],
            
            _ => "FFFF"
        }
    }, 16).unwrap()
}