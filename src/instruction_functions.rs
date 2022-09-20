use crate::analyze_code::AddressingMode;
use crate::system::system;

pub fn adc(address: u16, addressing_mode: AddressingMode, registers: &mut system::Registers, flags: &mut system::Flags, memory: system::Memory) {
    let mut result: u16 = registers.get_acc() as u16;
    
    match addressing_mode {
        AddressingMode::Immediate => result = result + address, 
        AddressingMode::ZeroPage | AddressingMode::Absolute => result = result + memory.get_mem_cell_value(address as usize) as u16,
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => result = result + memory.get_mem_cell_value(address as usize + registers.get_x() as usize) as u16,
        AddressingMode::AbsoluteY => result = result + memory.get_mem_cell_value(address as usize + registers.get_y() as usize) as u16,
        AddressingMode::IndirectX => result = result + memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_x())) as u16,
        AddressingMode::IndirectY => result = result + memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_y())) as u16,

        _ => { }
    }

    if result > 255 {
        flags.set_carry_flag(true);
        registers.set_acc(result as u8);
    } else {
        registers.set_acc(result as u8);
    }
}

pub fn and(address: u16, addressing_mode: AddressingMode, registers: &mut system::Registers, memory: system::Memory) {
    let mut result: u8 = registers.get_acc();
    
    match addressing_mode {
        AddressingMode::Immediate => result = result & address as u8, 
        AddressingMode::ZeroPage | AddressingMode::Absolute => result = result & memory.get_mem_cell_value(address as usize),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => result = result & memory.get_mem_cell_value(address as usize + registers.get_x() as usize),
        AddressingMode::AbsoluteY => result = result & memory.get_mem_cell_value(address as usize + registers.get_y() as usize),
        AddressingMode::IndirectX => result = result & memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_x())),
        AddressingMode::IndirectY => result = result & memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_y())),

        _ => { }
    }

    registers.set_acc(result);
}

pub fn asl(address: u16, addressing_mode: AddressingMode, registers: &mut system::Registers, flags: &mut system::Flags, memory: &mut system::Memory) {
    let mut shifted_value: u8 = 0;
    
    match addressing_mode {
        AddressingMode::Implied => shifted_value = registers.get_acc() << 1,
        AddressingMode::ZeroPage | AddressingMode::Absolute => shifted_value = memory.get_mem_cell_value(address as usize) << 1,
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => shifted_value = memory.get_mem_cell_value(address as usize + registers.get_x() as usize) << 1,

        _ => { }
    }

    if addressing_mode == AddressingMode::Implied {
        registers.set_acc(shifted_value);
    } else {
        if addressing_mode == AddressingMode::ZeroPage || addressing_mode == AddressingMode::Absolute {
            memory.set_mem_cell_value(address as usize, shifted_value);
        } else {
            memory.set_mem_cell_value(address as usize + registers.get_x() as usize, shifted_value);
        }
    }
}

pub fn bit(address: u16, addressing_mode: AddressingMode, flags: &mut system::Flags, memory: system::Memory) {
    match addressing_mode {
        AddressingMode::ZeroPage | AddressingMode::Absolute => {
            let mem_cell_data: u8 = memory.get_mem_cell_value(address as usize);
            let new_n_flag_value: bool = (mem_cell_data & 128) == 1;
            let new_v_flag_value: bool = (mem_cell_data & 64) == 1;

            flags.set_negative_flag(new_n_flag_value);
            flags.set_overflow_flag(new_v_flag_value);
        }

        _ => { }
    }
}

fn indexed_indirect_address(memory: system::Memory, address: u16, x_register: u8) -> usize {
    let low: u8 = memory.get_mem_cell_value(address as usize + x_register as usize);
    let high: u8 = memory.get_mem_cell_value(address as usize + x_register as usize + 1);
    
    ((high as u16) << 8 | low as u16) as usize
}

fn indirect_indexed_address(memory: system::Memory, address: u16, y_register: u8) -> usize {
    let low: u8 = memory.get_mem_cell_value(address as usize);
    let high: u8 = memory.get_mem_cell_value(address as usize + 1);
    
    (((high as u16) << 8 | low as u16) + y_register as u16) as usize
}