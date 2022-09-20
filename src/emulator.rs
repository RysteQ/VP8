use crate::system::system;
use crate::{analyze_code::{Instruction, Opcode}, instruction_functions};

pub fn start_emulator(instructions: Vec<Instruction>) {
    let mut registers: system::Registers = system::Registers::init();
    let mut flags: system::Flags = system::Flags::init();
    let mut memory: system::Memory = system::Memory::init();

    for i in 0..instructions.len() {
        match instructions[i].opcode {
            // todo macro this
            Opcode::ADC => instruction_functions::adc(instructions[i].value, instructions[i].addressing_mode.clone(), &mut registers, &mut flags, memory),
            Opcode::AND => instruction_functions::and(instructions[i].value, instructions[i].addressing_mode.clone(), &mut registers, memory),
            Opcode::ASL => instruction_functions::asl(instructions[i].value, instructions[i].addressing_mode.clone(), &mut registers, &mut flags, &mut memory),
            Opcode::BIT => instruction_functions::bit(instructions[i].value, instructions[i].addressing_mode.clone(), &mut flags, memory),

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