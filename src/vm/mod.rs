use macroquad::prelude::*;
mod opcodes;
pub use opcodes::OpCode;
mod execute;
mod input;
mod operations;
mod screen;
mod stack;
mod timer;

#[derive(Debug)]
#[allow(dead_code)]
pub struct VM {
    memory: [u8; 4096],
    program_counter: u16,
    i: u16,
    reg: [u8; 16],
    stack: [u16; crate::STACK_SIZE],
    stack_pointer: i8,
    key: [bool; 16],
    screen: [bool; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    sound_playing: bool,
    key_pressed: Option<u8>,
}

impl VM {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_program(&mut self, file: &str) -> Result<(), std::io::Error> {
        use std::io::Read;
        let mut f = std::fs::File::open(file)?;
        let mut buf = vec![];
        f.read_to_end(&mut buf)?;

        self.load_bytes(&buf, 0x200);
		self.dump_memory();
        Ok(())
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn load_bytes(&mut self, buf: &[u8], offset: u16) {
        assert!(
            buf.len() <= 4096,
            "ROM larger than memory. Tried to read {} bytes",
            buf.len()
        );

        for (i, b) in buf.iter().enumerate() {
            self.memory[offset as usize + i] = *b;
        }
    }

    pub fn get_instruction(&mut self) -> (u8, u8) {
        self.program_counter += 2;
        (
            self.memory[self.program_counter as usize - 2],
            self.memory[self.program_counter as usize - 1],
        )
    }

    pub async fn run(&mut self) {
        const CPU_TICK_NANOS: u128 = 1_000_000_000 / 500_000_000; // 500 MHz
        const UPDATE_TIMESTEP_MICROS: u128 = 1_000_000 / 60; // 60Hz

        let mut start_tick = std::time::Instant::now();
        let mut start_timestep = std::time::Instant::now();

        loop {
            let end_tick = std::time::Instant::now();
            let dif_tick = (end_tick - start_tick).as_micros();

            if dif_tick > CPU_TICK_NANOS {
                // should run 500 MHz
                let instruction = self.get_instruction(); // get instruction and increments IP by 2
                let op = OpCode::from_bytes(instruction);
                self.execute_op(&op);
                start_tick = std::time::Instant::now();
            }

            let end_timestep = std::time::Instant::now();
            let dif_timestep = (end_timestep - start_timestep).as_micros();
            if dif_timestep > UPDATE_TIMESTEP_MICROS {
                // should run at 60 Hz
                self.get_input();
                self.draw_screen();
                self.run_timers();
                let fps = get_fps();
                draw_text(&format!("FPS: {fps}"), 80.0, 20.0, 20.0, WHITE);
                macroquad::prelude::next_frame().await;
                start_timestep = std::time::Instant::now();
            }
        }
    }

    pub fn dump_memory(&self) {
        for (i, b) in self.memory.iter().enumerate() {
            if i % 32 == 0 {
                println!();
                print!("{i:#06X}: ");
            }
			if i % 8 == 0 {
				print!("  ");
			}
            print!("{b:02X} ");
        }
    }

    pub fn set_carry_flag(&mut self, value: u8) {
        self.reg[0xF] = value;
    }

    pub fn set_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

impl Default for VM {
    fn default() -> Self {
        let mut vm = Self {
            memory: [0; 4096],
            program_counter: 0x200,
            i: 0,
            reg: [0; 16],
            stack: [0; 16],
            stack_pointer: -1,
            key: [false; 16],
            screen: [false; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            sound_playing: false,
            key_pressed: None,
        };
        vm.load_bytes(&FONTSET, 0);
        vm
    }
}

#[cfg(test)]
mod test {
    use crate::VM;

    #[test]
    fn test_load_bytes() {
        let mut vm = VM::new();
        let buf = vec![0x00, 0xEE, 0x00, 0xE0, 0x10, 0x20];
        vm.load_bytes(&buf, 0x200);
        assert_eq!(vm.memory[0x200], 0x00);
        assert_eq!(vm.memory[0x201], 0xEE);
    }

	#[test]
    fn test_load_font() {
		let mut vm = VM::new();
       
        vm.load_bytes(&super::FONTSET, 0x0);
        assert_eq!(vm.memory[0x00], 0xF0);
        assert_eq!(vm.memory[0x01], 0x90);		
        assert_eq!(vm.memory[5 * 4], 0x90);
	}
}

static FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
