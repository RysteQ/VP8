use num_derive::{FromPrimitive, ToPrimitive};
use parse_display::{Display, FromStr};


#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, Display, FromStr)]
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

enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY
}

pub struct Instruction {
    opcode: Opcode,
    addressing_mode: AddressingMode,
    value: u16,
    size: u8,
    label_name: String
}

pub fn get_instructions(instructions: Vec<String>) -> Vec<Instruction> {
    let mut _to_: Vec<Instruction>;

    for _i in 0..instructions.len() {
        // TODO
    }

    // return to_return
    todo!();
}

fn get_opcode(opcode_to_analyze: String) -> Opcode {
    if !opcode_to_analyze.ends_with(':') {
        opcode_to_analyze.parse().unwrap()
    } else {
        Opcode::LABEL
    }
}

fn get_addressing_mode(parameters_to_analyze: String) -> AddressingMode {
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

fn inform_error_and_exit(msg: &str, exit_code: i32) -> ! {
    println!("{msg}");
    std::process::exit(exit_code);
}