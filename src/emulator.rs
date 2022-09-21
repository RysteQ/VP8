use crate::system::system;
use crate::{analyze_code::{Instruction, Opcode}, instruction_functions};

fn get_labels(instructions: Vec<Instruction>) -> Vec<(String, usize)> {
    let mut to_return: Vec<(String, usize)> = vec![];

    for i in 0..instructions.len() {
        if instructions[i].opcode == Opcode::LABEL {
            to_return.push((instructions[i].label_name.clone(), i));
        }
    }

    to_return
}

pub fn start_emulator(instructions: Vec<Instruction>) {
    let mut registers: system::Registers = system::Registers::init();
    let mut flags: system::Flags = system::Flags::init();
    let mut memory: system::Memory = system::Memory::init();
    let labels: Vec<(String, usize)> = get_labels(instructions.clone());

    for mut i in 0..instructions.len() {
        match instructions[i].opcode {
            // todo macro this
            Opcode::ADC => instruction_functions::adc(instructions[i].value, instructions[i].addressing_mode, &mut registers, &mut flags, memory),
            Opcode::AND => instruction_functions::and(instructions[i].value, instructions[i].addressing_mode, &mut registers, memory),
            Opcode::ASL => instruction_functions::asl(instructions[i].value, instructions[i].addressing_mode, &mut registers, &mut flags, &mut memory),
            Opcode::BIT => instruction_functions::bit(instructions[i].value, instructions[i].addressing_mode, &mut flags, memory),
            Opcode::BCC => i = instruction_functions::bcc(i, flags, instructions[i].label_name.clone(), labels.clone()),
            Opcode::BCS => i = instruction_functions::bcs(i, flags, instructions[i].label_name.clone(), labels.clone()),
            Opcode::BEQ => i = instruction_functions::beq(i, flags, instructions[i].label_name.clone(), labels.clone()),
            Opcode::BMI => i = instruction_functions::bmi(i, flags, instructions[i].label_name.clone(), labels.clone()),
            Opcode::BNE => i = instruction_functions::bne(i, flags, instructions[i].label_name.clone(), labels.clone()),
            Opcode::BPL => i = instruction_functions::bpl(i, flags, instructions[i].label_name.clone(), labels.clone()),
            Opcode::BVC => i = instruction_functions::bvc(i, flags, instructions[i].label_name.clone(), labels.clone()),
            Opcode::BVS => i = instruction_functions::bvs(i, flags, instructions[i].label_name.clone(), labels.clone()),
            Opcode::BRK => break,
            Opcode::CLC => instruction_functions::clc(&mut flags),
            Opcode::CLD => instruction_functions::cld(&mut flags),
            Opcode::CLI => instruction_functions::cli(&mut flags),
            Opcode::CLV => instruction_functions::clv(&mut flags),
            Opcode::CMP => instruction_functions::cmp(instructions[i].value, instructions[i].addressing_mode, registers, &mut flags, memory),
            Opcode::CPX => instruction_functions::cmp(instructions[i].value, instructions[i].addressing_mode, registers, &mut flags, memory),
            Opcode::CPY => instruction_functions::cmp(instructions[i].value, instructions[i].addressing_mode, registers, &mut flags, memory),

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

            Opcode::SEC => instruction_functions::sec(&mut flags),
            Opcode::SED => instruction_functions::sed(&mut flags),
            Opcode::SEI => instruction_functions::sei(&mut flags),
            Opcode::STA => instruction_functions::sta(instructions[i].value, instructions[i].addressing_mode, registers, &mut memory),
            Opcode::STX => instruction_functions::stx(instructions[i].value, instructions[i].addressing_mode, registers, &mut memory),
            Opcode::STY => instruction_functions::sty(instructions[i].value, instructions[i].addressing_mode, registers, &mut memory),
            Opcode::TAX => instruction_functions::tax(&mut registers),
            Opcode::TAY => instruction_functions::tay(&mut registers),
            Opcode::TSX => instruction_functions::tsx(&mut registers),
            Opcode::TXA => instruction_functions::txa(&mut registers),
            Opcode::TXS => instruction_functions::txs(&mut registers),
            Opcode::TYA => instruction_functions::tya(&mut registers),
            Opcode::LABEL => continue
        }
    }
}