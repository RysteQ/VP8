pub mod emulator {
    use crate::analyze_code::AddressingMode;
    use crate::system::system;
    use crate::window::window::Window;
    use crate::{analyze_code::{Instruction, Opcode}, instruction_functions};
    
    use glutin_window::GlutinWindow;
    use piston::event_loop::{EventSettings, Events};
    use piston::input::RenderEvent;
    use piston::window::WindowSettings;
    
    use opengl_graphics::OpenGL;
    
    use rand::Rng;
    
    struct App {
        pub open_gl: OpenGL,
        pub window: GlutinWindow,
        pub game_window: Window,
        pub event_control: Events,
    }

    struct Vp8System {
        pub registers: system::Registers,
        pub memory: system::Memory,
        pub flags: system::Flags,
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
    
    pub fn start_emulator(instructions: Vec<Instruction>) {
        let mut vp8: Vp8System = Vp8System { 
            registers: system::Registers::init(),
            flags: system::Flags::init(),
            memory: system::Memory::init()
        };
    
        let mut gui_application: App = App {
            open_gl: OpenGL::V3_2,

            window: WindowSettings::new(
                "Virtual Processor 8",
                [512, 512]
            ).opengl(OpenGL::V3_2).exit_on_esc(true).build().unwrap(),
            
            game_window: Window::init(OpenGL::V3_2),

            event_control: Events::new(EventSettings::new())
        };

        let labels: Vec<(String, usize)> = get_labels(instructions.clone());
        let mut index: usize = 0;
    
        while let Some(e) = gui_application.event_control.next(&mut gui_application.window) {
            let address: u16 = instructions[index].value;
            let addressing_mode: AddressingMode = instructions[index].addressing_mode;
            let label_name: String = instructions[index].label_name.clone();
            let mut routines: Vec<usize> = vec![];
            
            match instructions[index].opcode {
                Opcode::ADC => instruction_functions::adc(address, addressing_mode, &mut vp8.registers, &mut vp8.flags, vp8.memory),
                Opcode::AND => instruction_functions::and(address, addressing_mode, &mut vp8.registers, vp8.memory),
                Opcode::ASL => instruction_functions::asl(address, addressing_mode, &mut vp8.registers, &mut vp8.flags, &mut vp8.memory),
                Opcode::BIT => instruction_functions::bit(address, addressing_mode, &mut vp8.flags, vp8.memory),
                Opcode::BCC => index = instruction_functions::bcc(index, vp8.flags, label_name, labels.clone()),
                Opcode::BCS => index = instruction_functions::bcs(index, vp8.flags, label_name, labels.clone()),
                Opcode::BEQ => index = instruction_functions::beq(index, vp8.flags, label_name, labels.clone()),
                Opcode::BMI => index = instruction_functions::bmi(index, vp8.flags, label_name, labels.clone()),
                Opcode::BNE => index = instruction_functions::bne(index, vp8.flags, label_name, labels.clone()),
                Opcode::BPL => index = instruction_functions::bpl(index, vp8.flags, label_name, labels.clone()),
                Opcode::BVC => index = instruction_functions::bvc(index, vp8.flags, label_name, labels.clone()),
                Opcode::BVS => index = instruction_functions::bvs(index, vp8.flags, label_name, labels.clone()),
                Opcode::BRK => break,
                Opcode::CLC => instruction_functions::clc(&mut vp8.flags),
                Opcode::CLD => instruction_functions::cld(&mut vp8.flags),
                Opcode::CLI => instruction_functions::cli(&mut vp8.flags),
                Opcode::CLV => instruction_functions::clv(&mut vp8.flags),
                Opcode::CMP => instruction_functions::cmp(address, addressing_mode, vp8.registers, &mut vp8.flags, vp8.memory),
                Opcode::CPX => instruction_functions::cpx(address, addressing_mode, vp8.registers, &mut vp8.flags, vp8.memory),
                Opcode::CPY => instruction_functions::cpy(address, addressing_mode, vp8.registers, &mut vp8.flags, vp8.memory),
                Opcode::DEC => instruction_functions::dec(address, addressing_mode, vp8.registers, &mut vp8.memory),
                Opcode::DEX => instruction_functions::dex(&mut vp8.registers),
                Opcode::DEY => instruction_functions::dey(&mut vp8.registers),
                Opcode::EOR => instruction_functions::eor(address, addressing_mode, vp8.memory, &mut vp8.registers),
                Opcode::INC => instruction_functions::inc(address, addressing_mode, vp8.registers, &mut vp8.memory),
                Opcode::INX => instruction_functions::inx(&mut vp8.registers),
                Opcode::INY => instruction_functions::iny(&mut vp8.registers),
                Opcode::JMP => index = instruction_functions::jmp(instructions[index].label_name.clone(), labels.clone()),
                Opcode::LDA => instruction_functions::lda(address, addressing_mode, vp8.memory, &mut vp8.registers),
                Opcode::LDX => instruction_functions::ldx(address, addressing_mode, vp8.memory, &mut vp8.registers),
                Opcode::LDY => instruction_functions::ldy(address, addressing_mode, vp8.memory, &mut vp8.registers),
                Opcode::LSR => instruction_functions::lsr(address, addressing_mode, &mut vp8.memory, &mut vp8.registers),
                Opcode::NOP => continue,
                Opcode::ORA => instruction_functions::ora(address, addressing_mode, &mut vp8.registers, vp8.memory),
                Opcode::PLA => instruction_functions::pla(&mut vp8.registers, &mut vp8.memory),
                Opcode::PHA => instruction_functions::pha(vp8.registers, &mut vp8.memory),
                Opcode::ROL => instruction_functions::rol(address, addressing_mode, &mut vp8.registers, vp8.memory, &mut vp8.flags),
                Opcode::ROR => instruction_functions::ror(address, addressing_mode, &mut vp8.registers, vp8.memory, &mut vp8.flags),
                Opcode::SBC => instruction_functions::sbc(address, addressing_mode, &mut vp8.registers, &mut vp8.flags, vp8.memory),
                Opcode::SEC => instruction_functions::sec(&mut vp8.flags),
                Opcode::SED => instruction_functions::sed(&mut vp8.flags),
                Opcode::SEI => instruction_functions::sei(&mut vp8.flags),
                Opcode::STA => instruction_functions::sta(address, addressing_mode, vp8.registers, &mut vp8.memory),
                Opcode::STX => instruction_functions::stx(address, addressing_mode, vp8.registers, &mut vp8.memory),
                Opcode::STY => instruction_functions::sty(address, addressing_mode, vp8.registers, &mut vp8.memory),
                Opcode::TAX => instruction_functions::tax(&mut vp8.registers),
                Opcode::TAY => instruction_functions::tay(&mut vp8.registers),
                Opcode::TSX => instruction_functions::tsx(&mut vp8.registers),
                Opcode::TXA => instruction_functions::txa(&mut vp8.registers),
                Opcode::TXS => instruction_functions::txs(&mut vp8.registers),
                Opcode::TYA => instruction_functions::tya(&mut vp8.registers),
                Opcode::LABEL => continue,
                
                Opcode::JSR => routines.push(instruction_functions::jsr(instructions[index].label_name.clone(), labels.clone())),
                Opcode::RTS => {
                    if routines.len() == 0 {
                        panic!("No routine ro return from");
                    }
    
                    index = routines.pop().unwrap();
                }
            }

            if let Some(r) = e.render_args() {
                gui_application.game_window.set_screen_memory_data(vp8.memory.get_screen_memory());
                gui_application.game_window.update(r);
            }
    
            if index != instructions.len() - 1 {
                index += 1;
            } else {
                println!("Reached end of instructions")
            }

            let random_number: u8 = rand::thread_rng().gen();
            vp8.memory.set_mem_cell_value(0xfe, random_number);
        }
    }
}