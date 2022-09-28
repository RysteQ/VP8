use crate::analyze_code::AddressingMode;
use crate::system::system;
use crate::{analyze_code::{Instruction, Opcode}, instruction_functions};

use rand::Rng;

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
    let mut index: usize = 0;
    let labels: Vec<(String, usize)> = get_labels(instructions.clone());

    while index < instructions.len() {
        let address: u16 = instructions[index].value;
        let addressing_mode: AddressingMode = instructions[index].addressing_mode;
        let label_name: String = instructions[index].label_name.clone();

        let mut routines: Vec<usize> = vec![];

        match instructions[index].opcode {
            Opcode::ADC => instruction_functions::adc(address, addressing_mode, &mut registers, &mut flags, memory),
            Opcode::AND => instruction_functions::and(address, addressing_mode, &mut registers, memory),
            Opcode::ASL => instruction_functions::asl(address, addressing_mode, &mut registers, &mut flags, &mut memory),
            Opcode::BIT => instruction_functions::bit(address, addressing_mode, &mut flags, memory),
            Opcode::BCC => index = instruction_functions::bcc(index, flags, label_name, labels.clone()),
            Opcode::BCS => index = instruction_functions::bcs(index, flags, label_name, labels.clone()),
            Opcode::BEQ => index = instruction_functions::beq(index, flags, label_name, labels.clone()),
            Opcode::BMI => index = instruction_functions::bmi(index, flags, label_name, labels.clone()),
            Opcode::BNE => index = instruction_functions::bne(index, flags, label_name, labels.clone()),
            Opcode::BPL => index = instruction_functions::bpl(index, flags, label_name, labels.clone()),
            Opcode::BVC => index = instruction_functions::bvc(index, flags, label_name, labels.clone()),
            Opcode::BVS => index = instruction_functions::bvs(index, flags, label_name, labels.clone()),
            Opcode::BRK => break,
            Opcode::CLC => instruction_functions::clc(&mut flags),
            Opcode::CLD => instruction_functions::cld(&mut flags),
            Opcode::CLI => instruction_functions::cli(&mut flags),
            Opcode::CLV => instruction_functions::clv(&mut flags),
            Opcode::CMP => instruction_functions::cmp(address, addressing_mode, registers, &mut flags, memory),
            Opcode::CPX => instruction_functions::cpx(address, addressing_mode, registers, &mut flags, memory),
            Opcode::CPY => instruction_functions::cpy(address, addressing_mode, registers, &mut flags, memory),
            Opcode::DEC => instruction_functions::dec(address, addressing_mode, registers, &mut memory),
            Opcode::DEX => instruction_functions::dex(&mut registers),
            Opcode::DEY => instruction_functions::dey(&mut registers),
            Opcode::EOR => instruction_functions::eor(address, addressing_mode, memory, &mut registers),
            Opcode::INC => instruction_functions::inc(address, addressing_mode, registers, &mut memory),
            Opcode::INX => instruction_functions::inx(&mut registers),
            Opcode::INY => instruction_functions::iny(&mut registers),
            Opcode::JMP => index = instruction_functions::jmp(instructions[index].label_name.clone(), labels.clone()),
            Opcode::LDA => instruction_functions::lda(address, addressing_mode, memory, &mut registers),
            Opcode::LDX => instruction_functions::ldx(address, addressing_mode, memory, &mut registers),
            Opcode::LDY => instruction_functions::ldy(address, addressing_mode, memory, &mut registers),
            Opcode::LSR => instruction_functions::lsr(address, addressing_mode, &mut memory, &mut registers),
            Opcode::NOP => continue,
            Opcode::ORA => instruction_functions::ora(address, addressing_mode, &mut registers, memory),
            Opcode::PLA => instruction_functions::pla(&mut registers, &mut memory),
            Opcode::PHA => instruction_functions::pha(registers, &mut memory),
            Opcode::ROL => instruction_functions::rol(address, addressing_mode, &mut registers, memory, &mut flags),
            Opcode::ROR => instruction_functions::ror(address, addressing_mode, &mut registers, memory, &mut flags),
            Opcode::SBC => instruction_functions::sbc(address, addressing_mode, &mut registers, &mut flags, memory),
            Opcode::SEC => instruction_functions::sec(&mut flags),
            Opcode::SED => instruction_functions::sed(&mut flags),
            Opcode::SEI => instruction_functions::sei(&mut flags),
            Opcode::STA => instruction_functions::sta(address, addressing_mode, registers, &mut memory),
            Opcode::STX => instruction_functions::stx(address, addressing_mode, registers, &mut memory),
            Opcode::STY => instruction_functions::sty(address, addressing_mode, registers, &mut memory),
            Opcode::TAX => instruction_functions::tax(&mut registers),
            Opcode::TAY => instruction_functions::tay(&mut registers),
            Opcode::TSX => instruction_functions::tsx(&mut registers),
            Opcode::TXA => instruction_functions::txa(&mut registers),
            Opcode::TXS => instruction_functions::txs(&mut registers),
            Opcode::TYA => instruction_functions::tya(&mut registers),
            Opcode::LABEL => continue,

            Opcode::JSR => routines.push(instruction_functions::jsr(instructions[index].label_name.clone(), labels.clone())),

            Opcode::RTS => {
                if routines.len() == 0 {
                    panic!("No routine ro return from");
                }

                index = routines.pop().unwrap();
            }
        }

        let random_number: u8 = rand::thread_rng().gen();
        memory.set_mem_cell_value(0xfe, random_number);

        index += 1;
    }
}