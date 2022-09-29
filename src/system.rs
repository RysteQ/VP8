pub mod system {
    #[derive(Clone, Copy)]
    pub struct Registers {
        acc: u8,
        x: u8,
        y: u8,
        sp: u8
    }
    
    impl Registers {
        pub fn init() -> Registers {
            Registers { 
                acc: 0,
                x: 0,
                y: 0,
                sp: 255
            }
        }

        pub fn get_acc(&self) -> u8 {
            self.acc
        }
    
        pub fn get_x(&self) -> u8 {
            self.x
        }
    
        pub fn get_y(&self) -> u8 {
            self.y
        }
    
        pub fn get_sp(&self) -> u8 {
            self.sp
        }

        pub fn set_acc(&mut self, value: u8) {
            self.acc = value
        }
        
        pub fn set_x(&mut self, value: u8) {
            self.x = value
        }
    
        pub fn set_y(&mut self, value: u8) {
            self.y = value
        }

        pub fn set_sp(&mut self, value: u8) {
            self.sp = value;
        }
    }
    
    // I am not going to implement the B flag
    #[derive(Clone, Copy)]
    pub struct Flags {
        carry: bool,
        zero: bool,
        interrupt_disable: bool,
        decimal: bool,
        overflow: bool,
        negative: bool
    }

    impl Flags {
        pub fn init() -> Flags {
            Flags { 
                carry: false,
                zero: false,
                interrupt_disable: false,
                decimal: false,
                overflow: false,
                negative: false
            }
        }

        pub fn set_carry_flag(&mut self, value: bool) {
            self.carry = value;
        }

        pub fn set_zerro_flag(&mut self, value: bool) {
            self.zero = value;
        }

        pub fn set_interrupt_disable_flag(&mut self, value: bool) {
            self.interrupt_disable = value;
        }

        pub fn set_decimal_flag(&mut self, value: bool) {
            self.decimal = value;
        }

        pub fn set_overflow_flag(&mut self, value: bool) {
            self.overflow = value;
        }

        pub fn set_negative_flag(&mut self, value: bool) {
            self.negative = value;
        }

        pub fn get_carry_flag(&self) -> bool {
            self.carry
        }

        pub fn get_zerro_flag(&self) -> bool {
            self.zero
        }

        pub fn get_overflow_flag(&self) -> bool {
            self.overflow
        }

        pub fn get_negative_flag(&self) -> bool {
            self.negative
        }
    }
    
    #[derive(Clone, Copy)]
    pub struct Memory {
        mem_cell: [u8; 65536],
        stack_pointer: u16
    }

    impl Memory {
        pub fn init() -> Memory {
            Memory { 
                mem_cell: [00; 65536],
                stack_pointer: 0x01ff
            }
        }

        pub fn get_mem_cell_value(&self, index: usize) -> u8 {
            self.mem_cell[index]
        }

        pub fn get_stack_pointer(&self) -> u16 {
            self.stack_pointer
        }

        pub fn set_mem_cell_value(&mut self, index: usize, value: u8) {
            self.mem_cell[index] = value;
        }

        pub fn increment_stack_pointer(&mut self) {
            self.stack_pointer = self.stack_pointer + 1;
        }

        pub fn decrement_stack_pointer(&mut self) {
            self.stack_pointer = self.stack_pointer - 1;
        }
    }
}