use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, VM};

impl VM {
    // CLS - 00E0
    pub fn clear_display(&mut self) {
        self.screen.iter_mut().for_each(|x| *x = false);
    }

    // RET - 00EE
    pub fn return_subroutine(&mut self) {
        self.program_counter = self.pop();
    }

    // JMP - 1NNN
    pub fn jump(&mut self, addr: u16) {
        self.program_counter = addr;
    }

    // CALL - 2NNN
    pub fn call(&mut self, addr: u16) {
        self.push(self.program_counter);
        self.program_counter = addr;
    }

    // SE - 3XNN
    pub fn skip_equal(&mut self, register: u8, value: u8) {
        if self.reg[register as usize] == value {
            self.program_counter += 2;
        }
    }

    // SNE - 4XNN
    pub fn skip_not_equal(&mut self, register: u8, value: u8) {
        if self.reg[register as usize] != value {
            self.program_counter += 2;
        }
    }

    // RSE - 5XY0
    pub fn reg_skip_equal(&mut self, reg1: u8, reg2: u8) {
        if self.reg[reg1 as usize] == self.reg[reg2 as usize] {
            self.program_counter += 2;
        }
    }

    // SET - 6NNN
    pub fn set_register(&mut self, register: u8, value: u8) {
        self.reg[register as usize] = value;
    }

    // ADD - 7XNN
    pub fn add(&mut self, register: u8, value: u8) {
        self.reg[register as usize] = self.reg[register as usize].wrapping_add(value);
    }

    // RLD - 8XY0
    pub fn reg_load(&mut self, reg_x: u8, reg_y: u8) {
        self.set_register(reg_x, self.reg[reg_y as usize]);
    }

    // ROR - 8XY1
    pub fn reg_or(&mut self, reg_x: u8, reg_y: u8) {
        self.reg[reg_x as usize] |= self.reg[reg_y as usize];
    }

    // RAND - 8XY2
    pub fn reg_and(&mut self, reg_x: u8, reg_y: u8) {
        self.reg[reg_x as usize] &= self.reg[reg_y as usize];
    }

    // RXOR - 8XY3
    pub fn reg_xor(&mut self, reg_x: u8, reg_y: u8) {
        self.reg[reg_x as usize] ^= self.reg[reg_y as usize];
    }

    // RADD - 8XY4
    pub fn reg_add(&mut self, reg_x: u8, reg_y: u8) {
        let r = self.reg[reg_x as usize].checked_add(self.reg[reg_y as usize]);
        match r {
            None => {
                self.reg[reg_x as usize] =
                    self.reg[reg_x as usize].wrapping_add(self.reg[reg_y as usize]);
                self.set_carry_flag(1);
            }
            Some(x) => {
                self.reg[reg_x as usize] = x;
                self.set_carry_flag(0);
            }
        }
    }

    // RSUB - 8XY5
    pub fn reg_sub(&mut self, reg_x: u8, reg_y: u8) {
        let x = self.reg[reg_x as usize];
        let y = self.reg[reg_y as usize];

        self.reg[reg_x as usize] = x.wrapping_sub(y);

        if x > y {
            self.set_carry_flag(1);
        } else {
            self.set_carry_flag(0);
        }
    }

    // RSHR - 8XY6
    pub fn reg_shift_right(&mut self, reg: u8) {
        let v = self.reg[reg as usize];
        self.reg[reg as usize] = v >> 1;

        if 0b0000_0001 & v == 1 {
            self.set_carry_flag(1);
        } else {
            self.set_carry_flag(0);
        }
    }

    // RSUBN - 8XY7
    pub fn reg_sub_not_borrow(&mut self, reg_x: u8, reg_y: u8) {
        let x = self.reg[reg_x as usize];
        let y = self.reg[reg_y as usize];

        self.reg[reg_x as usize] = y.wrapping_sub(x);

        if y > x {
            self.set_carry_flag(1);
        } else {
            self.set_carry_flag(0);
        }
    }

    // RSHL - 8XYE
    pub fn reg_shift_left(&mut self, reg: u8) {
        let v = self.reg[reg as usize];

        self.reg[reg as usize] = v << 1;

        if 0b1000_0000 & v == 0b1000_0000 {
            self.set_carry_flag(1);
        } else {
            self.set_carry_flag(0);
        }
    }

    // RSNE - 9XY0
    pub fn reg_skip_not_equal(&mut self, reg1: u8, reg2: u8) {
        if self.reg[reg1 as usize] != self.reg[reg2 as usize] {
            self.program_counter += 2;
        }
    }

    // LD - ANNN
    pub fn set_index(&mut self, val: u16) {
        self.i = val;
    }

    // JP - BNNN
    pub fn jump_location(&mut self, value: u16) {
        self.program_counter = self.reg[0x0] as u16 + value;
    }

    // RND - CXNN
    pub fn random(&mut self, reg: u8, value: u8) {
        let r = macroquad::rand::gen_range(0, 255);
        self.reg[reg as usize] = r & value;
    }

    // DRW - DXYN
    pub fn draw(&mut self, x: u8, y: u8, n: u8) {
        let addr = self.i;
        let x = self.reg[x as usize] % SCREEN_WIDTH as u8;
        let y = self.reg[y as usize] % SCREEN_HEIGHT as u8;
        for i in 0..n {
            let data = self.memory[addr as usize + i as usize];
            for j in 0..8 {
                let x = (x + j) % SCREEN_WIDTH as u8;
                let y = (y + i) % SCREEN_HEIGHT as u8;
                if 0b1000_0000 >> j & data != 0 && self.set_pixel(x, y) {
                    self.set_carry_flag(1);
                }
            }
        }
    }

    // SKP - EX9E
    pub fn skip_if_key(&mut self, reg: u8) {
        let key = self.reg[reg as usize];
        if self.key[key as usize] {
            self.program_counter += 2;
        }
    }

    // SKNP - EXA1
    pub fn skip_if_no_key(&mut self, reg: u8) {
        let key = self.reg[reg as usize];
        if !self.key[key as usize] {
            self.program_counter += 2;
        }
    }

    // LDT - FX07
    pub fn load_delay_timer(&mut self, reg: u8) {
        self.reg[reg as usize] = self.delay_timer;
    }

    // KPR - FX0A
    pub fn await_keypress(&mut self, reg: u8) {
        for (i, b) in self.key.iter().enumerate() {
            if *b {
                self.reg[reg as usize] = i as u8;
                return;
            }
        }
        self.program_counter -= 2;
    }

    // SETDT - FX15
    pub fn set_delay_timer(&mut self, reg: u8) {
        self.delay_timer = self.reg[reg as usize];
    }

    // SETST - FX18
    pub fn set_sound_timer(&mut self, reg: u8) {
        self.sound_timer = self.reg[reg as usize];
    }

    // ADDI - FX1E
    pub fn add_i(&mut self, reg: u8) {
        self.i += self.reg[reg as usize] as u16;
    }

    // LDSPR - FX29
    pub fn load_sprite(&mut self, reg: u8) {
        let val = self.reg[reg as usize];
        self.i = self.memory[val as usize * 5] as u16;
    }

    // STBCD - FX33
    pub fn store_bcd(&mut self, reg: u8) {
        let val = self.reg[reg as usize];
        let addr = self.i as usize;
        let d1 = val / 100;
        let d2 = (val % 100) / 10;
        let d3 = val % 10;
        self.memory[addr] = d1;
        self.memory[addr + 1] = d2;
        self.memory[addr + 2] = d3;
    }

    // STORE - FX55
    pub fn store_registers(&mut self, reg: u8) {
        let addr = self.i;
        for i in 0..=(reg as usize) {
            self.memory[(addr as usize) + i] = self.reg[i];
        }
    }

    // READ - FX65
    pub fn read_registers(&mut self, reg: u8) {
        let addr = self.i;
        for i in 0..=(reg as usize) {
            self.reg[i] = self.memory[(addr as usize) + i];
        }
    }
}
