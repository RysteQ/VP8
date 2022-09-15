

#[derive(Debug, Clone, Copy)]
enum Opcode {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI,
    BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI,
    CLV, CMP, CPX, CPY, DEC, DEX, DEY, EOR,
    INC, INX, INY, JMP, JSR, LDA, LDX, LDY,
    LSR, NOP, ORA, PHA, PHP, PLA, LPL, ROL,
    ROR, RTI, RTS, SBC, SEC, SED, SEI, STA,
    STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,

    LABEL
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
    let mut _to_return: Vec<Instruction>;

    for _i in 0..instructions.len() {
        // TODO
    }

    // return to_return
    todo!();
}

fn get_opcode(opcode_to_analyze: String) -> Opcode {
    let opcode_syllables: [&str; 56] = [
        "ADC", "AND", "ASL", "BCC", "BCS", "BEQ", "BIT", "BMI", "BNE", "BPL", "BRK", "BVC", "BVS", "CLC",
        "CLD", "CLI", "CLV", "CMP", "CPX", "CPY", "DEC", "DEX", "DEY", "EOR", "INC", "INX", "INY", "JMP",
        "JSR", "LDA", "LDX", "LDY", "LSR", "NOP", "ORA", "PHA", "PHP", "PLA", "PLP", "ROL", "ROR", "RTI",
        "RTS", "SBC", "SEC", "SED", "SEI", "STA", "STX", "STY", "TAX", "TAY", "TSX", "TXA", "TXS", "TYA"
    ];

    let opcode_values: [Opcode; 56] = [
        Opcode::ADC, Opcode::AND, Opcode::ASL, Opcode::BCC, Opcode::BCS, Opcode::BEQ, Opcode::BIT, Opcode::BMI,
        Opcode::BNE, Opcode::BPL, Opcode::BRK, Opcode::BVC, Opcode::BVS, Opcode::CLC, Opcode::CLD, Opcode::CLI,
        Opcode::CLV, Opcode::CMP, Opcode::CPX, Opcode::CPY, Opcode::DEC, Opcode::DEX, Opcode::DEY, Opcode::EOR,
        Opcode::INC, Opcode::INX, Opcode::INY, Opcode::JMP, Opcode::JSR, Opcode::LDA, Opcode::LDX, Opcode::LDY,
        Opcode::LSR, Opcode::NOP, Opcode::ORA, Opcode::PHA, Opcode::PHP, Opcode::PLA, Opcode::LPL, Opcode::ROL,
        Opcode::ROR, Opcode::RTI, Opcode::RTS, Opcode::SBC, Opcode::SEC, Opcode::SED, Opcode::SEI, Opcode::STA,
        Opcode::STX, Opcode::STY, Opcode::TAX, Opcode::TAY, Opcode::TSX, Opcode::TXA, Opcode::TXS, Opcode::TYA
    ];

    for index in 0..56 {
        if opcode_to_analyze.as_str() == opcode_syllables[index] {
            return opcode_values[index];
        }
    }

    if opcode_to_analyze.ends_with(':') {
        return Opcode::LABEL;
    }

    // TODO: Inform the user of the opcode and the line
    println!("Wrong opcode");
    std::process::exit(-1);
}

fn GetAddressingMode(parameters_to_analyze: String) -> AddressingMode {
    match parameters_to_analyze.chars().next().unwrap() {
        '#' => {
            if parameters_to_analyze.len() == 4 {
                return AddressingMode::Immediate;
            }
        },
        
        '$' => {
            match parameters_to_analyze.len() {
                3 => {
                    return AddressingMode::ZeroPage; 
                },

                5 => {
                    if parameters_to_analyze.chars().nth(4).unwrap() == 'X' {
                        return AddressingMode::ZeroPageX;
                    } else {
                        return AddressingMode::Absolute;
                    }
                },

                7 => {
                    if parameters_to_analyze.chars().nth(6).unwrap() == 'X' {
                        return AddressingMode::AbsoluteX;
                    } else if parameters_to_analyze.chars().nth(6).unwrap() == 'Y' {
                        return AddressingMode::AbsoluteY;
                    }
                }

                _ => InformErrorAndExit("Error analyzing parameters", -1)
            }
        },

        '(' => {
            if parameters_to_analyze.chars().nth(6).unwrap() == ')' {
                return AddressingMode::IndirectX;
            } else if parameters_to_analyze.chars().nth(6).unwrap() == 'Y' {
                return AddressingMode::IndirectY;
            }
        },

        _ => InformErrorAndExit("Error analyzing parameters", -1)
    }

    println!("Error analyzing parameters");
    std::process::exit(-1);
}

fn InformErrorAndExit(msg: &str, exit_code: i32) {
    println!("{msg}");
    std::process::exit(exit_code);
}