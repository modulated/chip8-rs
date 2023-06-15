use crate::{OpCode, VM};

impl VM {
    #[allow(clippy::missing_panics_doc)]
    pub fn execute_op(&mut self, op: &OpCode) {
        use OpCode::{
            Unknown, ADD, ADDI, CALL, CLS, DRW, JMP, JP, KPR, LD, LDSPR, LDT, RADD, RAND, READ,
            RET, RLD, RND, ROR, RSE, RSHL, RSHR, RSNE, RSUB, RSUBN, RXOR, SE, SET, SETDT, SETST, SKNP,
            SKP, SNE, STBCD, STORE,
        };

        // let start = std::time::Instant::now();

        match op {
            CLS => self.clear_display(),
            RET => self.return_subroutine(),
            JMP(x) => self.jump(*x),
            CALL(x) => self.call(*x),
            SE { reg, value } => self.skip_equal(*reg, *value),
            SNE { reg, value } => self.skip_not_equal(*reg, *value),
            RSE { reg_x, reg_y } => self.reg_skip_equal(*reg_x, *reg_y),
            RSNE { reg_x, reg_y } => self.reg_skip_not_equal(*reg_x, *reg_y),
            SET { reg, value } => self.set_register(*reg, *value),
            ADD { reg, value } => self.add(*reg, *value),
            RLD { reg_x, reg_y } => self.reg_load(*reg_x, *reg_y),
            ROR { reg_x, reg_y } => self.reg_or(*reg_x, *reg_y),
            RAND { reg_x, reg_y } => self.reg_and(*reg_x, *reg_y),
            RXOR { reg_x, reg_y } => self.reg_xor(*reg_x, *reg_y),
            RADD { reg_x, reg_y } => self.reg_add(*reg_x, *reg_y),
            RSUB { reg_x, reg_y } => self.reg_sub(*reg_x, *reg_y),
            RSHR(reg) => self.reg_shift_right(*reg),
            RSUBN { reg_x, reg_y } => self.reg_sub_not_borrow(*reg_x, *reg_y),
            RSHL(reg) => self.reg_shift_left(*reg),
            LD(x) => self.set_index(*x),
            JP(x) => self.jump_location(*x),
            RND { reg, value } => self.random(*reg, *value),
            DRW { x, y, n } => self.draw(*x, *y, *n),
            SKP(x) => self.skip_if_key(*x),
            SKNP(x) => self.skip_if_no_key(*x),
            LDT(x) => self.load_delay_timer(*x),
            KPR(x) => self.await_keypress(*x),
            SETDT(x) => self.set_delay_timer(*x),
			SETST(x) => self.set_sound_timer(*x),
            ADDI(x) => self.add_i(*x),
            LDSPR(x) => self.load_sprite(*x),
            STBCD(x) => self.store_bcd(*x),
            STORE(x) => self.store_registers(*x),
            READ(x) => self.read_registers(*x),

            Unknown(x) => panic!("Unknown {x:#06X}"),
        }

        // let end = std::time::Instant::now();
        // let dif = end - start;
        // println!("{op}: {}", dif.as_micros());
    }
}
