use std::cmp::Ordering;

use crate::analyze_code::AddressingMode;
use crate::system::system;

pub fn adc(address: u16, addressing_mode: AddressingMode, registers: &mut system::Registers, flags: &mut system::Flags, memory: system::Memory) {
    let mut result: u16 = registers.get_acc() as u16;
    
    match addressing_mode {
        AddressingMode::Immediate => result += address, 
        AddressingMode::ZeroPage | AddressingMode::Absolute => result += memory.get_mem_cell_value(address as usize) as u16,
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => result += memory.get_mem_cell_value(address as usize + registers.get_x() as usize) as u16,
        AddressingMode::AbsoluteY => result += memory.get_mem_cell_value(address as usize + registers.get_y() as usize) as u16,
        AddressingMode::IndirectX => result += memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_x())) as u16,
        AddressingMode::IndirectY => result += memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_y())) as u16,

        _ => { }
    }

    if result > 255 {
        flags.set_carry_flag(true);
    }

    registers.set_acc(result as u8);
}

pub fn and(address: u16, addressing_mode: AddressingMode, registers: &mut system::Registers, memory: system::Memory) {
    let mut result: u8 = registers.get_acc();
    
    match addressing_mode {
        AddressingMode::Immediate => result &= address as u8, 
        AddressingMode::ZeroPage | AddressingMode::Absolute => result &= memory.get_mem_cell_value(address as usize),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => result &= memory.get_mem_cell_value(address as usize + registers.get_x() as usize),
        AddressingMode::AbsoluteY => result &= memory.get_mem_cell_value(address as usize + registers.get_y() as usize),
        AddressingMode::IndirectX => result &= memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_x())),
        AddressingMode::IndirectY => result &= memory.get_mem_cell_value(indirect_indexed_address(memory, address, registers.get_y())),

        _ => { }
    }

    registers.set_acc(result);
}

pub fn asl(address: u16, addressing_mode: AddressingMode, registers: &mut system::Registers, _flags: &mut system::Flags, memory: &mut system::Memory) {
    let mut shifted_value: u8 = 0;
    
    match addressing_mode {
        AddressingMode::Implied => shifted_value = registers.get_acc() << 1,
        AddressingMode::ZeroPage | AddressingMode::Absolute => shifted_value = memory.get_mem_cell_value(address as usize) << 1,
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => shifted_value = memory.get_mem_cell_value(address as usize + registers.get_x() as usize) << 1,

        _ => { }
    }

    if addressing_mode == AddressingMode::Implied {
        registers.set_acc(shifted_value);
    } else if addressing_mode == AddressingMode::ZeroPage || addressing_mode == AddressingMode::Absolute {
        memory.set_mem_cell_value(address as usize, shifted_value);
    } else {
        memory.set_mem_cell_value(address as usize + registers.get_x() as usize, shifted_value);
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

pub fn bpl(current_index: usize, flags: system::Flags, label_name: String, labels: Vec<(String, usize)>) -> usize {
    branch(flags.get_negative_flag(), false, label_name, labels, current_index)
}

pub fn bmi(current_index: usize, flags: system::Flags, label_name: String, labels: Vec<(String, usize)>) -> usize {
    branch(flags.get_negative_flag(), true, label_name, labels, current_index)
}

pub fn bvc(current_index: usize, flags: system::Flags, label_name: String, labels: Vec<(String, usize)>) -> usize {
    branch(flags.get_overflow_flag(), false, label_name, labels, current_index)
}

pub fn bvs(current_index: usize, flags: system::Flags, label_name: String, labels: Vec<(String, usize)>) -> usize {
    branch(flags.get_overflow_flag(), true, label_name, labels, current_index)
}

pub fn bcc(current_index: usize, flags: system::Flags, label_name: String, labels: Vec<(String, usize)>) -> usize {
    branch(flags.get_carry_flag(), false, label_name, labels, current_index)
}

pub fn bcs(current_index: usize, flags: system::Flags, label_name: String, labels: Vec<(String, usize)>) -> usize {
    branch(flags.get_carry_flag(), true, label_name, labels, current_index)
}

pub fn bne(current_index: usize, flags: system::Flags, label_name: String, labels: Vec<(String, usize)>) -> usize {
    branch(flags.get_zerro_flag(), false, label_name, labels, current_index)
}

pub fn beq(current_index: usize, flags: system::Flags, label_name: String, labels: Vec<(String, usize)>) -> usize {
    branch(flags.get_zerro_flag(), true, label_name, labels, current_index)
}

pub fn clc(flags: &mut system::Flags) {
    flags.set_carry_flag(false);
}

pub fn sec(flags: &mut system::Flags) {
    flags.set_carry_flag(true);
}

pub fn cli(flags: &mut system::Flags) {
    flags.set_interrupt_disable_flag(false);
}

pub fn sei(flags: &mut system::Flags) {
    flags.set_interrupt_disable_flag(true);
}

pub fn clv(flags: &mut system::Flags) {
    flags.set_overflow_flag(false);
}

pub fn cld(flags: &mut system::Flags) {
    flags.set_decimal_flag(false);
}

pub fn sed(flags: &mut system::Flags) {
    flags.set_decimal_flag(true);
}

pub fn sta(address: u16, addressing_mode: AddressingMode, registers: system::Registers, memory: &mut system::Memory) {
    match addressing_mode {
        AddressingMode::ZeroPage | AddressingMode::Absolute => memory.set_mem_cell_value(address as usize, registers.get_acc()),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => memory.set_mem_cell_value(address as usize, registers.get_acc()),
        AddressingMode::AbsoluteY => memory.set_mem_cell_value(address as usize + registers.get_y() as usize, registers.get_acc()),
        AddressingMode::IndirectX => memory.set_mem_cell_value(indexed_indirect_address(*memory, address, registers.get_x()), registers.get_acc()),
        AddressingMode::IndirectY => memory.set_mem_cell_value(indirect_indexed_address(*memory, address, registers.get_y()), registers.get_acc()),
        
        _ => { }
    }
}

pub fn stx(address: u16, addressing_mode: AddressingMode, registers: system::Registers, memory: &mut system::Memory) {
    match addressing_mode {
        AddressingMode::ZeroPage | AddressingMode::Absolute => memory.set_mem_cell_value(address as usize, registers.get_x()),
        AddressingMode::ZeroPageY => memory.set_mem_cell_value(address as usize + registers.get_y() as usize, registers.get_x()),

        _ => { }
    }
}

pub fn sty(address: u16, addressing_mode: AddressingMode, registers: system::Registers, memory: &mut system::Memory) {
    match addressing_mode {
        AddressingMode::ZeroPage | AddressingMode::Absolute => memory.set_mem_cell_value(address as usize, registers.get_y()),
        AddressingMode::ZeroPageX => memory.set_mem_cell_value(address as usize + registers.get_x() as usize, registers.get_y()),

        _ => { }
    }
}

pub fn tax(registers: &mut system::Registers) {
    registers.set_x(registers.get_acc());
}

pub fn tay(registers: &mut system::Registers) {
    registers.set_y(registers.get_acc());
}

pub fn tsx(registers: &mut system::Registers) {
    registers.set_x(registers.get_sp());
}

pub fn txa(registers: &mut system::Registers) {
    registers.set_acc(registers.get_x());
}

pub fn txs(registers: &mut system::Registers) {
    registers.set_sp(registers.get_x());
}

pub fn tya(registers: &mut system::Registers) {
    registers.set_acc(registers.get_y());
}

pub fn cmp(address: u16, addressing_mode: AddressingMode, registers: system::Registers, flags: &mut system::Flags, memory: system::Memory) {
    match addressing_mode {
        AddressingMode::Immediate => compare(registers.get_acc(), address as u8, flags),
        AddressingMode::ZeroPage | AddressingMode::Absolute => compare(registers.get_acc(), memory.get_mem_cell_value(address as usize), flags),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => compare(registers.get_acc(), memory.get_mem_cell_value(address as usize + registers.get_x() as usize), flags),
        AddressingMode::AbsoluteY => compare(registers.get_acc(), memory.get_mem_cell_value(address as usize + registers.get_y() as usize), flags),
        AddressingMode::IndirectX => compare(registers.get_acc(), memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_x())), flags),
        AddressingMode::IndirectY => compare(registers.get_acc(), memory.get_mem_cell_value(indirect_indexed_address(memory, address, registers.get_y())), flags),

        _ => { }
    }
}

pub fn cpx(address: u16, addressing_mode: AddressingMode, registers: system::Registers, flags: &mut system::Flags, memory: system::Memory) {
    match addressing_mode {
        AddressingMode::Immediate => compare(registers.get_x(), address as u8, flags),
        AddressingMode::ZeroPage | AddressingMode::Absolute => compare(registers.get_x(), memory.get_mem_cell_value(address as usize), flags),
        _ => { }
    }
}

pub fn cpy(address: u16, addressing_mode: AddressingMode, registers: system::Registers, flags: &mut system::Flags, memory: system::Memory) {
    match addressing_mode {
        AddressingMode::Immediate => compare(registers.get_y(), address as u8, flags),
        AddressingMode::ZeroPage | AddressingMode::Absolute => compare(registers.get_y(), memory.get_mem_cell_value(address as usize), flags),
        
        _ => { }
    } 
}

pub fn dec(address: u16, addressing_mode: AddressingMode, registers: system::Registers, memory: &mut system::Memory) {
    match addressing_mode {
        AddressingMode::ZeroPage | AddressingMode::Absolute => memory.set_mem_cell_value(address as usize, memory.get_mem_cell_value(address as usize) - 1),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => memory.set_mem_cell_value(address as usize + registers.get_x() as usize, memory.get_mem_cell_value(address as usize) - 1),

        _ => { }
    }
}

pub fn dex(registers: &mut system::Registers) {
    if registers.get_x() == 0 {
        registers.set_x(255);
    } else {
        registers.set_x(registers.get_x() - 1);
    }
}

pub fn dey(registers: &mut system::Registers) {
    if registers.get_x() == 0 {
        registers.set_x(255);
    } else {
        registers.set_x(registers.get_x() - 1);
    }
}

pub fn inc(address: u16, addressing_mode: AddressingMode, registers: system::Registers, memory: &mut system::Memory) {
    match addressing_mode {
        AddressingMode::ZeroPage | AddressingMode::Absolute => memory.set_mem_cell_value(address as usize, memory.get_mem_cell_value(address as usize) + 1),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => memory.set_mem_cell_value(address as usize + registers.get_x() as usize, memory.get_mem_cell_value(address as usize) + 1),

        _ => { }
    }
}

pub fn inx(registers: &mut system::Registers) {
    if registers.get_x() as u16 + 1 != 256 {
        registers.set_x(registers.get_x() + 1);
    } else {
        registers.set_x(0);
    }
}

pub fn iny(registers: &mut system::Registers) {
    if registers.get_y() as u16 + 1 != 256 {
        registers.set_y(registers.get_y() + 1);
    } else {
        registers.set_y(0);
    }
}

pub fn eor(address: u16, addressing_mode: AddressingMode, memory: system::Memory, registers: &mut system::Registers) {
    match addressing_mode {
        AddressingMode::ZeroPage | AddressingMode::Absolute => registers.set_acc(registers.get_acc() ^ memory.get_mem_cell_value(address as usize)),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => registers.set_acc(registers.get_acc() ^ memory.get_mem_cell_value(address as usize + registers.get_acc() as usize)),
        AddressingMode::AbsoluteY => registers.set_acc(registers.get_acc() ^ memory.get_mem_cell_value(address as usize + registers.get_y() as usize)),
        AddressingMode::IndirectX => registers.set_acc(registers.get_acc() ^ memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_x()))),
        AddressingMode::IndirectY => registers.set_acc(registers.get_acc() ^ memory.get_mem_cell_value(indirect_indexed_address(memory, address, registers.get_y()))),

        _ => { }
    }
}

pub fn lda(address: u16, addressing_mode: AddressingMode, memory: system::Memory, registers: &mut system::Registers) {
    match addressing_mode {
        AddressingMode::Immediate => registers.set_acc(address as u8),
        AddressingMode::ZeroPage | AddressingMode::Absolute => registers.set_acc(memory.get_mem_cell_value(address as usize)),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => registers.set_acc(memory.get_mem_cell_value(address as usize + registers.get_x() as usize)),
        AddressingMode::AbsoluteY => registers.set_acc(memory.get_mem_cell_value(address as usize + registers.get_y() as usize)),
        AddressingMode::IndirectX => registers.set_acc(memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_x()))),
        AddressingMode::IndirectY => registers.set_acc(memory.get_mem_cell_value(indirect_indexed_address(memory, address, registers.get_y()))),

        _ => { }
    }
}

pub fn ldx(address: u16, addressing_mode: AddressingMode, memory: system::Memory, registers: &mut system::Registers) {
    match addressing_mode {
        AddressingMode::Immediate => registers.set_x(address as u8),
        AddressingMode::ZeroPage | AddressingMode::Absolute => registers.set_x(memory.get_mem_cell_value(address as usize)),
        AddressingMode::ZeroPageY | AddressingMode::AbsoluteY => registers.set_x(memory.get_mem_cell_value(address as usize + registers.get_y() as usize)),

        _ => { }
    }
}

pub fn ldy(address: u16, addressing_mode: AddressingMode, memory: system::Memory, registers: &mut system::Registers) {
    match addressing_mode {
        AddressingMode::Immediate => registers.set_y(address as u8),
        AddressingMode::ZeroPage | AddressingMode::Absolute => registers.set_y(memory.get_mem_cell_value(address as usize)),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => registers.set_y(memory.get_mem_cell_value(address as usize + registers.get_x() as usize)),

        _ => { }
    }
}

pub fn jmp(label_name: String, label_names: Vec<(String, usize)>) -> usize {
    for i in label_names.iter() {
        if label_name == i.0 {
            return i.1
        }
    }

    panic!("Label {{{label_name}}} not found");
}

pub fn lsr(address: u16, addressing_mode: AddressingMode, memory: &mut system::Memory, registers: &mut system::Registers) {
    match addressing_mode {
        AddressingMode::Implied => registers.set_acc(registers.get_acc() >> 1),
        AddressingMode::ZeroPage | AddressingMode::Absolute => memory.set_mem_cell_value(address as usize, memory.get_mem_cell_value(address as usize) >> 1),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => memory.set_mem_cell_value(address as usize + registers.get_x() as usize, memory.get_mem_cell_value(address as usize + registers.get_x() as usize) >> 1),

        _ => { }
    }
}

pub fn pha(registers: system::Registers, memory: &mut system::Memory) {
    if memory.get_stack_pointer() == 255 {
        panic!("Stack limit reached");
    }

    memory.set_mem_cell_value(memory.get_stack_pointer() as usize, registers.get_acc());
    memory.decrement_stack_pointer();
}

pub fn pla(registers: &mut system::Registers, memory: &mut system::Memory) {
    if memory.get_stack_pointer() == 0x01ff {
        panic!("Stack limit reached");   
    }

    registers.set_acc(memory.get_mem_cell_value(memory.get_stack_pointer() as usize));
    memory.increment_stack_pointer();
}

pub fn ora(address: u16, addressing_mode: AddressingMode, registers: &mut system::Registers, memory: system::Memory) {
    match addressing_mode {
        AddressingMode::Immediate => registers.set_acc(registers.get_acc() | address as u8),
        AddressingMode::ZeroPage | AddressingMode::Absolute => registers.set_acc(registers.get_acc() | memory.get_mem_cell_value(address as usize)),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => registers.set_acc(registers.get_acc() | memory.get_mem_cell_value(address as usize + registers.get_x() as usize)),
        AddressingMode::AbsoluteY => registers.set_acc(registers.get_acc() | memory.get_mem_cell_value(address as usize + registers.get_y() as usize)),
        AddressingMode::IndirectX => registers.set_acc(registers.get_acc() | memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_x()))),
        AddressingMode::IndirectY => registers.set_acc(registers.get_acc() | memory.get_mem_cell_value(indirect_indexed_address(memory, address, registers.get_y()))),

        _ => { }
    }
}

pub fn rol(address: u16, addressing_mode: AddressingMode, registers: &mut system::Registers, memory: system::Memory, flags: &mut system::Flags) {
    let carry_flag_bit: bool = flags.get_carry_flag();
    let mut to_rotate: u8 = 0;

    match addressing_mode {
        AddressingMode::Implied => to_rotate = registers.get_acc(),
        AddressingMode::ZeroPage | AddressingMode::Absolute => to_rotate = memory.get_mem_cell_value(address as usize),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => to_rotate = memory.get_mem_cell_value(address as usize + registers.get_x() as usize),

        _ => { }
    }

    let new_carry_flag_bit: u8 = to_rotate & 0b10000000;

    // Check if the value is zero and save the opposite of the value to the carry flag
    flags.set_carry_flag(!(new_carry_flag_bit == 0));
    registers.set_acc((to_rotate << 1) | (carry_flag_bit as u8 >> 7));
}

pub fn ror(address: u16, addressing_mode: AddressingMode, registers: &mut system::Registers, memory: system::Memory, flags: &mut system::Flags) {
    let carry_flag_bit: bool = flags.get_carry_flag();
    let mut to_rotate: u8 = 0;

    match addressing_mode {
        AddressingMode::Implied => to_rotate = registers.get_acc(),
        AddressingMode::ZeroPage | AddressingMode::Absolute => to_rotate = memory.get_mem_cell_value(address as usize),
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => to_rotate = memory.get_mem_cell_value(address as usize + registers.get_x() as usize),

        _ => { }
    }

    let new_carry_flag_bit: u8 = to_rotate & 0b00000001;

    // Check if the value is zero and save the opposite of the value to the carry flag
    flags.set_carry_flag(!(new_carry_flag_bit == 0));
    registers.set_acc((to_rotate >> 1) | ((carry_flag_bit as u8) << 1));
}

pub fn sbc(address: u16, addressing_mode: AddressingMode, registers: &mut system::Registers, flags: &mut system::Flags, memory: system::Memory) {
    let mut result: u16 = registers.get_acc() as u16;
    
    match addressing_mode {
        AddressingMode::Immediate => result -= address, 
        AddressingMode::ZeroPage | AddressingMode::Absolute => result -= memory.get_mem_cell_value(address as usize) as u16,
        AddressingMode::ZeroPageX | AddressingMode::AbsoluteX => result -= memory.get_mem_cell_value(address as usize + registers.get_x() as usize) as u16,
        AddressingMode::AbsoluteY => result -= memory.get_mem_cell_value(address as usize + registers.get_y() as usize) as u16,
        AddressingMode::IndirectX => result -= memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_x())) as u16,
        AddressingMode::IndirectY => result -= memory.get_mem_cell_value(indexed_indirect_address(memory, address, registers.get_y())) as u16,

        _ => { }
    }

    if result > 255 {
        flags.set_carry_flag(true);
    }

    registers.set_acc(result as u8);
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

fn branch(flag_to_check: bool, expected_value: bool, label_name: String, labels: Vec<(String, usize)>, current_index: usize) -> usize {
    if flag_to_check == expected_value {
        for i in labels.iter() {
            if label_name == i.0 {
                return i.1
            }
        }

        panic!("Label {{{label_name}}} does not exist");
    }

    current_index
}

fn compare(register_value: u8, expected_value: u8, flags: &mut system::Flags) {
    let order_value: Ordering = register_value.cmp(&expected_value);

    match order_value {
        Ordering::Less => flags.set_negative_flag(false),

        Ordering::Equal => {
            flags.set_zerro_flag(true);
            flags.set_carry_flag(true);
        }

        Ordering::Greater => {
            flags.set_negative_flag(true);
            flags.set_carry_flag(true);
        }
    }
}