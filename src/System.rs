pub mod System {
    #[derive(Clone, Copy)]
    pub struct Registers {
        acc: u8,
        x: u8,
        y: u8
    }
    
    impl Registers {
        pub fn init() -> Registers {
            Registers { 
                acc: 0,
                x: 0,
                y: 0
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
    
        pub fn set_acc(&mut self, value: u8) {
            self.acc = value
        }
        
        pub fn set_x(&mut self, value: u8) {
            self.x = value
        }
    
        pub fn set_y(&mut self, value: u8) {
            self.y = value
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
    }
    
    #[derive(Clone, Copy)]
    pub struct Memory {
        mem_cell: [u8; 65536]
    }

    impl Memory {
        pub fn init() -> Memory {
            Memory { 
                mem_cell: [00; 65536]
            }
        }

        pub fn get_mem_cell_value(&self, index: usize) -> u8 {
            self.mem_cell[index]
        }

        pub fn set_mem_cell_value(&mut self, index: usize, value: u8) {
            self.mem_cell[index] = value;
        }
    }
}