use crate::analyze_code::AddressingMode;
use crate::System::System;

pub fn ADC(address: u16, addressing_mode: AddressingMode, mut registers: System::Registers, flags: System::Flags, memory: System::Memory) {
    let mut result: u16 = registers.get_acc() as u16;
    
    match addressing_mode {
        AddressingMode::Immediate => result = result + address, 
        AddressingMode::ZeroPage | AddressingMode::Absolute => result = result + memory.get_mem_cell_value(address as usize) as u16,
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => result = result + memory.get_mem_cell_value(address as usize + registers.get_x() as usize) as u16,
        AddressingMode::Absolute => result = result + memory.get_mem_cell_value(address as usize) as u16,
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

fn indexed_indirect_address(memory: System::Memory, address: u16, x_register: u8) -> usize {
    let low: u8 = memory.get_mem_cell_value(address as usize + x_register as usize);
    let high: u8 = memory.get_mem_cell_value(address as usize + x_register as usize + 1);
    
    ((high << 8) as u16 | low as u16) as usize
}

fn indirect_indexed_address(memory: System::Memory, address: u16, y_register: u8) -> usize {
    let low: u8 = memory.get_mem_cell_value(address as usize);
    let high: u8 = memory.get_mem_cell_value(address as usize + 1);
    
    (((high << 8) as u16 | low as u16) + y_register as u16) as usize
}