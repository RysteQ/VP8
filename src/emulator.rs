use crate::analyze_code::AddressingMode;
use crate::system::system;
use crate::window::Window;
use crate::{analyze_code::{Instruction, Opcode}, instruction_functions as instruction};
use rand::Rng;

macro_rules! increment_instruction_index {
    ($index: expr, $instruction_vector_length: expr) => {
        if $index != $instruction_vector_length - 1 {
            $index + 1
        } else {
            $index
        }
    };
}

struct Vp8System {
    pub registers: system::Registers,
    pub memory: system::Memory,
    pub flags: system::Flags,
}

pub fn start_emulator(instructions: Vec<Instruction>) {
    let mut vp8: Vp8System = Vp8System { 
        registers: system::Registers::init(),
        flags: system::Flags::init(),
        memory: system::Memory::init()
    };

    let mut game_window: Window = Window::init();

    let labels: Vec<(String, usize)> = get_labels(instructions.clone());
    let mut routines: Vec<usize> = vec![];
    let mut index: usize = 0;

    while let Some(event) = game_window.get_window_next() {
        execute_code(&instructions, &mut index, &mut vp8, labels.clone(), &mut routines);

        game_window.set_screen_memory_data(vp8.memory.get_screen_memory());
        game_window.update(event);

        index = increment_instruction_index!(index, instructions.len());
    }
}

fn execute_code(instructions: &Vec<Instruction>, index: &mut usize, vp8: &mut Vp8System, labels: Vec<(String, usize)>, routines: &mut Vec<usize>) {
    loop {
        let address: u16 = instructions[*index].value;
        let addressing_mode: AddressingMode = instructions[*index].addressing_mode;
        let label_name: String = instructions[*index].label_name.clone();

        match instructions[*index].opcode {
            Opcode::ADC => instruction::adc(address, addressing_mode, &mut vp8.registers, &mut vp8.flags, vp8.memory),
            Opcode::AND => instruction::and(address, addressing_mode, &mut vp8.registers, vp8.memory),
            Opcode::ASL => instruction::asl(address, addressing_mode, &mut vp8.registers, &mut vp8.flags, &mut vp8.memory),
            Opcode::BIT => instruction::bit(address, addressing_mode, &mut vp8.flags, vp8.memory),
            Opcode::BCC => *index = instruction::bcc(*index, &mut vp8.flags, label_name.clone(), labels.clone()),
            Opcode::BCS => *index = instruction::bcs(*index, &mut vp8.flags, label_name.clone(), labels.clone()),
            Opcode::BEQ => *index = instruction::beq(*index, &mut vp8.flags, label_name.clone(), labels.clone()),
            Opcode::BMI => *index = instruction::bmi(*index, &mut vp8.flags, label_name.clone(), labels.clone()),
            Opcode::BNE => *index = instruction::bne(*index, &mut vp8.flags, label_name.clone(), labels.clone()),
            Opcode::BPL => *index = instruction::bpl(*index, &mut vp8.flags, label_name.clone(), labels.clone()),
            Opcode::BVC => *index = instruction::bvc(*index, &mut vp8.flags, label_name.clone(), labels.clone()),
            Opcode::BVS => *index = instruction::bvs(*index, &mut vp8.flags, label_name.clone(), labels.clone()),
            Opcode::CLC => instruction::clc(&mut vp8.flags),
            Opcode::CLD => instruction::cld(&mut vp8.flags),
            Opcode::CLI => instruction::cli(&mut vp8.flags),
            Opcode::CLV => instruction::clv(&mut vp8.flags),
            Opcode::CMP => instruction::cmp(address, addressing_mode, vp8.registers, &mut vp8.flags, vp8.memory),
            Opcode::CPX => instruction::cpx(address, addressing_mode, vp8.registers, &mut vp8.flags, vp8.memory),
            Opcode::CPY => instruction::cpy(address, addressing_mode, vp8.registers, &mut vp8.flags, vp8.memory),
            Opcode::DEC => instruction::dec(address, addressing_mode, vp8.registers, &mut vp8.memory),
            Opcode::DEX => instruction::dex(&mut vp8.registers),
            Opcode::DEY => instruction::dey(&mut vp8.registers),
            Opcode::EOR => instruction::eor(address, addressing_mode, vp8.memory, &mut vp8.registers),
            Opcode::INC => instruction::inc(address, addressing_mode, vp8.registers, &mut vp8.memory),
            Opcode::INX => instruction::inx(&mut vp8.registers),
            Opcode::INY => instruction::iny(&mut vp8.registers),
            Opcode::JMP => *index = instruction::jmp(instructions[*index].label_name.clone(), labels.clone()),
            Opcode::LDA => instruction::lda(address, addressing_mode, vp8.memory, &mut vp8.registers),
            Opcode::LDX => instruction::ldx(address, addressing_mode, vp8.memory, &mut vp8.registers),
            Opcode::LDY => instruction::ldy(address, addressing_mode, vp8.memory, &mut vp8.registers),
            Opcode::LSR => instruction::lsr(address, addressing_mode, &mut vp8.memory, &mut vp8.registers),
            Opcode::ORA => instruction::ora(address, addressing_mode, &mut vp8.registers, vp8.memory),
            Opcode::PLA => instruction::pla(&mut vp8.registers, &mut vp8.memory),
            Opcode::PHA => instruction::pha(vp8.registers, &mut vp8.memory),
            Opcode::PHP => instruction::php(vp8.flags, &mut vp8.memory),
            Opcode::PLP => instruction::plp(&mut vp8.flags, &mut vp8.memory),
            Opcode::ROL => instruction::rol(address, addressing_mode, &mut vp8.registers, vp8.memory, &mut vp8.flags),
            Opcode::ROR => instruction::ror(address, addressing_mode, &mut vp8.registers, vp8.memory, &mut vp8.flags),
            Opcode::SBC => instruction::sbc(address, addressing_mode, &mut vp8.registers, &mut vp8.flags, vp8.memory),
            Opcode::SEC => instruction::sec(&mut vp8.flags),
            Opcode::SED => instruction::sed(&mut vp8.flags),
            Opcode::SEI => instruction::sei(&mut vp8.flags),
            Opcode::STA => instruction::sta(address, addressing_mode, vp8.registers, &mut vp8.memory),
            Opcode::STX => instruction::stx(address, addressing_mode, vp8.registers, &mut vp8.memory),
            Opcode::STY => instruction::sty(address, addressing_mode, vp8.registers, &mut vp8.memory),
            Opcode::TAX => instruction::tax(&mut vp8.registers),
            Opcode::TAY => instruction::tay(&mut vp8.registers),
            Opcode::TSX => instruction::tsx(&mut vp8.registers),
            Opcode::TXA => instruction::txa(&mut vp8.registers),
            Opcode::TXS => instruction::txs(&mut vp8.registers),
            Opcode::TYA => instruction::tya(&mut vp8.registers),
            Opcode::BRK => *index -= 1,
            
            Opcode::DRW => break,
            
            Opcode::JSR => {
                routines.push(*index);
                *index = instruction::jmp(instructions[*index].label_name.clone(), labels.clone());
            },

            Opcode::RTS => {
                if routines.len() == 0 {
                    panic!("No routine ro return from, instruction {:?} at {}", instructions[*index], *index);
                }

                *index = routines.pop().unwrap();
            },

            _ => { }
        }
        
        *index = increment_instruction_index!(*index, instructions.len());
        random_number_in_memory(&mut vp8.memory);
    }
}

fn random_number_in_memory(memory: &mut system::Memory) {
    let random_number: u8 = rand::thread_rng().gen();
    memory.set_mem_cell_value(0xfe, random_number);
}

fn get_labels(instructions: Vec<Instruction>) -> Vec<(String, usize)> {
    let mut to_return: Vec<(String, usize)> = vec![];

    for i in 0..instructions.len() {
        if instructions[i].opcode == Opcode::LABEL {
            to_return.push((instructions[i].label_name.clone(), i));
        }
    }

    to_return
}