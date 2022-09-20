use crate::system::system;
use crate::{analyze_code::{Instruction, Opcode}, instruction_functions};

pub fn start_emulator(instructions: Vec<Instruction>) {
    let mut registers: system::Registers = system::Registers::init();
    let mut flags: system::Flags = system::Flags::init();
    let mut memory: system::Memory = system::Memory::init();

    for _i in 0..instructions.len() {
        match instructions[_i].opcode {
            Opcode::ADC => instruction_functions::ADC(instructions[_i].value, instructions[_i].addressing_mode.clone(), &mut registers, &mut flags, memory),
            Opcode::AND => instruction_functions::AND(instructions[_i].value, instructions[_i].addressing_mode.clone(), &mut registers, memory),
            Opcode::ASL => instruction_functions::ASL(instructions[_i].value, instructions[_i].addressing_mode.clone(), &mut registers, &mut flags, memory),
            Opcode::BIT => instruction_functions::BIT(instructions[_i].value, instructions[_i].addressing_mode.clone(), &mut flags, memory),

            Opcode::ASL => {

            } 	

            Opcode::BCC => {

            } 	

            Opcode::BCS => {

            } 	

            Opcode::BEQ => {

            } 	


            Opcode::BMI => {

            } 	

            Opcode::BNE => {

            } 	

            Opcode::BPL => {

            } 	

            Opcode::BRK => {

            } 	

            Opcode::BVC => {

            } 	

            Opcode::BVS => {

            } 	

            Opcode::CLC => {

            }

            Opcode::CLD => {

            } 	

            Opcode::CLI => {

            } 	

            Opcode::CLV => {

            } 	

            Opcode::CMP => {

            } 	

            Opcode::CPX => {

            } 	

            Opcode::CPY => {

            } 	

            Opcode::DEC => {

            } 	

            Opcode::DEX => {

            } 	

            Opcode::DEY => {

            } 	

            Opcode::EOR => {

            } 	

            Opcode::INC => {

            } 	

            Opcode::INX => {

            } 	

            Opcode::INY => {

            } 	

            Opcode::JMP => {

            }

            Opcode::JSR => {

            } 	

            Opcode::LDA => {

            } 	

            Opcode::LDX => {

            } 	

            Opcode::LDY => {

            } 	

            Opcode::LSR => {

            } 	

            Opcode::NOP => {

            } 	

            Opcode::ORA => {

            } 	

            Opcode::PHA => {

            } 	

            Opcode::PHP => {

            } 	

            Opcode::PLA => {

            } 	

            Opcode::PLP => {

            } 	

            Opcode::ROL => {

            } 	

            Opcode::ROR => {

            } 	

            Opcode::RTI => {

            }

            Opcode::RTS => {

            } 	

            Opcode::SBC => {

            } 	

            Opcode::SEC => {

            } 	

            Opcode::SED => {

            } 	

            Opcode::SEI => {

            } 	

            Opcode::STA => {

            } 	

            Opcode::STX => {

            } 	

            Opcode::STY => {

            } 	

            Opcode::TAX => {

            } 	

            Opcode::TAY => {

            } 	

            Opcode::TSX => {

            } 	

            Opcode::TXA => {

            } 	

            Opcode::TXS => {

            } 	

            Opcode::TYA => {

            }

            Opcode::LABEL => continue
        }
    }
}