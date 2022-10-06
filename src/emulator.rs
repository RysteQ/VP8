pub mod emulator {
    use crate::analyze_code::analyzer::AddressingMode;
    use crate::system::system;
    use crate::window::window::Window;
    use crate::{analyze_code::analyzer::{Instruction, Opcode}, instruction_functions::instructions};
    
    use glutin_window::GlutinWindow;
    use piston::event_loop::{EventSettings, Events};
    use piston::input::RenderEvent;
    use piston::window::WindowSettings;
    use opengl_graphics::OpenGL;
    use rand::Rng;
    
    struct App {
        pub window: GlutinWindow,
        pub game_window: Window,
        pub event_control: Events,
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
    
        let mut gui_application: App = App {
            window: WindowSettings::new(
                "Virtual Processor 8",
                [256, 256]
            ).opengl(OpenGL::V3_2).exit_on_esc(true).build().unwrap(),
            
            game_window: Window::init(OpenGL::V3_2),
            event_control: Events::new(EventSettings::new())
        };

        let labels: Vec<(String, usize)> = get_labels(instructions.clone());
        let mut routines: Vec<usize> = vec![];
        let mut index: usize = 0;
    
        while let Some(e) = gui_application.event_control.next(&mut gui_application.window) {
            let address: u16 = instructions[index].value;
            let addressing_mode: AddressingMode = instructions[index].addressing_mode;
            let label_name: String = instructions[index].label_name.clone();
            
            match instructions[index].opcode {
                Opcode::ADC => instructions::adc(address, addressing_mode, &mut vp8.registers, &mut vp8.flags, vp8.memory),
                Opcode::AND => instructions::and(address, addressing_mode, &mut vp8.registers, vp8.memory),
                Opcode::ASL => instructions::asl(address, addressing_mode, &mut vp8.registers, &mut vp8.flags, &mut vp8.memory),
                Opcode::BIT => instructions::bit(address, addressing_mode, &mut vp8.flags, vp8.memory),
                Opcode::BCC => index = instructions::bcc(index, vp8.flags, label_name, labels.clone()),
                Opcode::BCS => index = instructions::bcs(index, vp8.flags, label_name, labels.clone()),
                Opcode::BEQ => index = instructions::beq(index, vp8.flags, label_name, labels.clone()),
                Opcode::BMI => index = instructions::bmi(index, vp8.flags, label_name, labels.clone()),
                Opcode::BNE => index = instructions::bne(index, vp8.flags, label_name, labels.clone()),
                Opcode::BPL => index = instructions::bpl(index, vp8.flags, label_name, labels.clone()),
                Opcode::BVC => index = instructions::bvc(index, vp8.flags, label_name, labels.clone()),
                Opcode::BVS => index = instructions::bvs(index, vp8.flags, label_name, labels.clone()),
                Opcode::CLC => instructions::clc(&mut vp8.flags),
                Opcode::CLD => instructions::cld(&mut vp8.flags),
                Opcode::CLI => instructions::cli(&mut vp8.flags),
                Opcode::CLV => instructions::clv(&mut vp8.flags),
                Opcode::CMP => instructions::cmp(address, addressing_mode, vp8.registers, &mut vp8.flags, vp8.memory),
                Opcode::CPX => instructions::cpx(address, addressing_mode, vp8.registers, &mut vp8.flags, vp8.memory),
                Opcode::CPY => instructions::cpy(address, addressing_mode, vp8.registers, &mut vp8.flags, vp8.memory),
                Opcode::DEC => instructions::dec(address, addressing_mode, vp8.registers, &mut vp8.memory),
                Opcode::DEX => instructions::dex(&mut vp8.registers),
                Opcode::DEY => instructions::dey(&mut vp8.registers),
                Opcode::EOR => instructions::eor(address, addressing_mode, vp8.memory, &mut vp8.registers),
                Opcode::INC => instructions::inc(address, addressing_mode, vp8.registers, &mut vp8.memory),
                Opcode::INX => instructions::inx(&mut vp8.registers),
                Opcode::INY => instructions::iny(&mut vp8.registers),
                Opcode::JMP => index = instructions::jmp(instructions[index].label_name.clone(), labels.clone()),
                Opcode::LDA => instructions::lda(address, addressing_mode, vp8.memory, &mut vp8.registers),
                Opcode::LDX => instructions::ldx(address, addressing_mode, vp8.memory, &mut vp8.registers),
                Opcode::LDY => instructions::ldy(address, addressing_mode, vp8.memory, &mut vp8.registers),
                Opcode::LSR => instructions::lsr(address, addressing_mode, &mut vp8.memory, &mut vp8.registers),
                Opcode::ORA => instructions::ora(address, addressing_mode, &mut vp8.registers, vp8.memory),
                Opcode::PLA => instructions::pla(&mut vp8.registers, &mut vp8.memory),
                Opcode::PHA => instructions::pha(vp8.registers, &mut vp8.memory),
                Opcode::ROL => instructions::rol(address, addressing_mode, &mut vp8.registers, vp8.memory, &mut vp8.flags),
                Opcode::ROR => instructions::ror(address, addressing_mode, &mut vp8.registers, vp8.memory, &mut vp8.flags),
                Opcode::SBC => instructions::sbc(address, addressing_mode, &mut vp8.registers, &mut vp8.flags, vp8.memory),
                Opcode::SEC => instructions::sec(&mut vp8.flags),
                Opcode::SED => instructions::sed(&mut vp8.flags),
                Opcode::SEI => instructions::sei(&mut vp8.flags),
                Opcode::STA => instructions::sta(address, addressing_mode, vp8.registers, &mut vp8.memory),
                Opcode::STX => instructions::stx(address, addressing_mode, vp8.registers, &mut vp8.memory),
                Opcode::STY => instructions::sty(address, addressing_mode, vp8.registers, &mut vp8.memory),
                Opcode::TAX => instructions::tax(&mut vp8.registers),
                Opcode::TAY => instructions::tay(&mut vp8.registers),
                Opcode::TSX => instructions::tsx(&mut vp8.registers),
                Opcode::TXA => instructions::txa(&mut vp8.registers),
                Opcode::TXS => instructions::txs(&mut vp8.registers),
                Opcode::TYA => instructions::tya(&mut vp8.registers),

                Opcode::BRK => index -= 1,

                Opcode::LABEL => { },
                Opcode::NOP => { },
                
                Opcode::JSR => {
                    routines.push(index);
                    index = instructions::jmp(instructions[index].label_name.clone(), labels.clone());
                },

                Opcode::RTS => {
                    if routines.len() == 0 {
                        panic!("No routine ro return from, instruction {:?} at {index}", instructions[index]);
                    }
    
                    index = routines.pop().unwrap();
                }
            }

            if let Some(r) = e.render_args() {
                gui_application.game_window.set_screen_memory_data(vp8.memory.get_screen_memory());
                gui_application.game_window.update(r);
            }
    
            index = increment_instruction_index(index, instructions.len());
            random_number_in_memory(&mut vp8.memory);
        }
    }

    fn random_number_in_memory(memory: &mut system::Memory) {
        let random_number: u8 = rand::thread_rng().gen();
        memory.set_mem_cell_value(0xfe, random_number);
    }

    fn increment_instruction_index(index: usize, instructions_length: usize) -> usize {
        if index != instructions_length - 1 {
            index + 1
        } else {
            index
        }
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
}