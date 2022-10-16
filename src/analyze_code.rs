use parse_display::{Display, FromStr};
use std::u16;

macro_rules! remove_whitespaces {
    ($to_remove: expr) => {
        $to_remove.replace(" ", "")
    };
}

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq, Eq)]
pub enum Opcode {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI,
    BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI,
    CLV, CMP, CPX, CPY, DEC, DEX, DEY, EOR,
    INC, INX, INY, JMP, JSR, LDA, LDX, LDY,
    LSR, NOP, ORA, PHA, PLA, PHP, PLP, ROL, 
    ROR, RTS, SBC, SEC, SED, SEI, STA, STX, 
    STY, TAX, TAY, TSX, TXA, TXS, TYA,

    LABEL, DRW
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Display)]
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
        let opcode: Opcode = get_opcode(opcode_str, instructions[i].as_str());
        
        if opcode != Opcode::LABEL {
            let addressing_mode: AddressingMode = get_addressing_mode(operand, opcode);
            let value: u16 = get_operand_value(operand, addressing_mode);
            let mut label_name: String = String::new();

            if instructions[i].len() > 3 {
                label_name = instructions[i][4..instructions[i].len()].to_string();
            }

            to_return.push(Instruction {
                opcode,
                addressing_mode,
                value,
                label_name
            });
        } else {
            to_return.push(Instruction { 
                opcode, 
                addressing_mode: AddressingMode::Absolute, 
                value: 0,
                label_name: instructions[i][0..instructions[i].len() - 1].to_string()
            });
        }
    }
    
    to_return
}

fn get_opcode(opcode_to_analyze: &str, original_version: &str) -> Opcode {
    if original_version.ends_with(':') == false{
        opcode_to_analyze.parse().unwrap()
    } else {
        Opcode::LABEL
    }
}

fn get_addressing_mode(parameters_to_analyze: &str, opcode: Opcode) -> AddressingMode {
    let to_analyze: String = remove_whitespaces!(parameters_to_analyze);

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
            let nth: char = to_analyze.chars().nth(6).unwrap();

            match nth {
                ')' => AddressingMode::IndirectX,
                'Y' => AddressingMode::IndirectY,
                _ => panic!("Error in analyze_code.rs at get_addressing_mode")
            }
        },

        _ => {
            let jump_operations: [Opcode; 10] = [
                Opcode::JMP, Opcode::JSR, Opcode::BPL, Opcode::BMI, Opcode::BVC,
                Opcode::BVS, Opcode::BCC, Opcode::BCS, Opcode::BNE, Opcode::BEQ
            ];
            
            if opcode == Opcode::LABEL || jump_operations.contains(&opcode) {
                return AddressingMode::Relative
            }
            
            panic!("Error in analyze_code.rs at get_addressing_mode, operand value = {parameters_to_analyze}")
        }
    }
}

fn get_operand_value(parameters: &str, addressing_mode: AddressingMode) -> u16 {
    u16::from_str_radix({
        match addressing_mode {
            AddressingMode::Immediate => &parameters[2..],
            AddressingMode::ZeroPage | AddressingMode::Absolute => &parameters[1..],
            AddressingMode::ZeroPageX | AddressingMode::ZeroPageY | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => &parameters[1..].split(&",".to_string()).collect::<Vec<&str>>()[0],
            AddressingMode::IndirectX => &parameters[2..].split(&",".to_string()).collect::<Vec<&str>>()[0],
            AddressingMode::IndirectY => &parameters[2..].split(&")".to_string()).collect::<Vec<&str>>()[0],
            
            _ => "FFFF"
        }
    }, 16).unwrap()
}