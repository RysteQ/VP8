use crate::analyze_code::{Instruction, Opcode, AddressingMode};

pub fn verify_data(instructions: Vec<Instruction>) -> bool {
    let _only_implied_mode: [Opcode; 25] = [
        Opcode::TXS, Opcode::TSX, Opcode::PHA, Opcode::PLA, Opcode::RTS, 
        Opcode::RTI, Opcode::TAX, Opcode::TXA, Opcode::DEX, Opcode::INX, 
        Opcode::TAY, Opcode::TYA, Opcode::DEY, Opcode::INY, Opcode::NOP, 
        Opcode::CLC, Opcode::SEC, Opcode::CLI, Opcode::SEI, Opcode::CLV, 
        Opcode::CLD, Opcode::SED, Opcode::BRK, Opcode::JMP, Opcode::JSR
    ];

    let _branch_instructions: [Opcode; 8] = [
        Opcode::BPL, Opcode::BMI, Opcode::BVC, Opcode::BVS, Opcode::BCC,
        Opcode::BCS, Opcode::BNE, Opcode::BEQ
    ];

    for _i in 0..instructions.len() {
        let _opcode_to_check: Opcode = instructions[_i].opcode;
        let _addressing_mode: AddressingMode = instructions[_i].addressing_mode;
        let mut allowed_command: bool = true;

        match _opcode_to_check {
            Opcode::ASL | Opcode::LSR | Opcode::ROR | Opcode::ROL => allowed_command = allowed_addressing_mode_check(_addressing_mode, vec![AddressingMode::Implied, AddressingMode::ZeroPage, AddressingMode::ZeroPageX, AddressingMode::Absolute, AddressingMode::AbsoluteX]),
            Opcode::DEC | Opcode::INC => allowed_command = allowed_addressing_mode_check(_addressing_mode, vec![AddressingMode::ZeroPage, AddressingMode::ZeroPageX, AddressingMode::Absolute, AddressingMode::AbsoluteX]),
            Opcode::CPX | Opcode::CPY => allowed_command = allowed_addressing_mode_check(_addressing_mode, vec![AddressingMode::Immediate, AddressingMode::ZeroPage, AddressingMode::Absolute]),
            Opcode::STX => allowed_command = allowed_addressing_mode_check(_addressing_mode, vec![AddressingMode::ZeroPage, AddressingMode::IndirectY, AddressingMode::Absolute, AddressingMode::AbsoluteX]),
            Opcode::STY => allowed_command = allowed_addressing_mode_check(_addressing_mode, vec![AddressingMode::ZeroPage, AddressingMode::ZeroPageY, AddressingMode::Absolute, AddressingMode::AbsoluteX]),
            Opcode::BIT => allowed_command = allowed_addressing_mode_check(_addressing_mode, vec![AddressingMode::ZeroPage, AddressingMode::Absolute]),

            _ =>  {
                if _only_implied_mode.contains(&_opcode_to_check) && _addressing_mode != AddressingMode::Implied {
                    allowed_command = false;
                }
        
                if _branch_instructions.contains(&_opcode_to_check) && _addressing_mode != AddressingMode::Relative {
                    allowed_command = false;
                }
            }
        }

        if allowed_command == false {
            return false;
        }
    }

    true
}

fn allowed_addressing_mode_check(_addressing_mode: AddressingMode, _allowed_addressing_modes: Vec<AddressingMode>) -> bool {
    if !_allowed_addressing_modes.contains(&_addressing_mode) {
        false;
    } 

    true
}