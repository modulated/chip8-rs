#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]

pub enum OpCode {
    CLS,                            // 00E0
    RET,                            // 00EE
    JMP(u16),                       // 1NNN
    CALL(u16),                      // 2NNN
    SE { reg: u8, value: u8 },      // 3XNN
    SNE { reg: u8, value: u8 },     // 4XNN
    RSE { reg_x: u8, reg_y: u8 },   // 5XY0
    SET { reg: u8, value: u8 },     // 6NNN
    ADD { reg: u8, value: u8 },     // 7XNN
    RLD { reg_x: u8, reg_y: u8 },   // 8XY0
    ROR { reg_x: u8, reg_y: u8 },   // 8XY1
    RAND { reg_x: u8, reg_y: u8 },  // 8XY2
    RXOR { reg_x: u8, reg_y: u8 },  // 8XY3
    RADD { reg_x: u8, reg_y: u8 },  // 8XY4
    RSUB { reg_x: u8, reg_y: u8 },  // 8XY5
    RSHR(u8),                       // 8XY6
    RSUBN { reg_x: u8, reg_y: u8 }, // 8XY7
    RSHL(u8),                       // 8XYE
    RSNE { reg_x: u8, reg_y: u8 },  // 9XY0
    LD(u16),                        // ANNN
    JP(u16),                        // BNNN
    RND { reg: u8, value: u8 },     // CXNN
    DRW { x: u8, y: u8, n: u8 },    // DXYN
    SKP(u8),                        // EX9E
    SKNP(u8),                       // EXA1
    LDT(u8),                        // FX07
    KPR(u8),                        // FX0A
    SETDT(u8),                      // FX15
    SETST(u8),                      // FX18
    ADDI(u8),                       // FX1E
    LDSPR(u8),                      // FX29
    STBCD(u8),                      // FX33
    STORE(u8),                      // FX55
    READ(u8),                       // FX65
    Unknown(u16),
}

impl OpCode {
    #[must_use]
    pub fn from_bytes(bytes: (u8, u8)) -> Self {
        use OpCode::{
            Unknown, ADD, ADDI, CALL, CLS, DRW, JMP, JP, KPR, LD, LDSPR, LDT, RADD, RAND, READ,
            RET, RLD, RND, ROR, RSE, RSHL, RSHR, RSNE, RSUB, RSUBN, RXOR, SE, SET, SETDT, SETST,
            SKNP, SKP, SNE, STBCD, STORE,
        };
        match (bytes.0, bytes.1) {
            (0x00, 0xE0) => CLS,
            (0x00, 0xEE) => RET,
            (0x10..=0x1F, _) => JMP(0x1000 ^ (((bytes.0 as u16) << 8) + bytes.1 as u16)),
            (0x20..=0x2F, _) => CALL(0x2000 ^ (((bytes.0 as u16) << 8) + bytes.1 as u16)),
            (0x30..=0x3F, _) => {
                let reg = bytes.0 ^ 0x30;
                let value = bytes.1;
                SE { reg, value }
            }
            (0x40..=0x4F, _) => {
                let reg = bytes.0 ^ 0x40;
                let value = bytes.1;
                SNE { reg, value }
            }
            (0x50..=0x5F, _) => {
                let reg_x = bytes.0 ^ 0x50;
                let reg_y = bytes.1 >> 4;
                RSE { reg_x, reg_y }
            }
            (0x60..=0x6F, _) => {
                let reg = bytes.0 ^ 0x60;
                let value = bytes.1;
                SET { reg, value }
            }
            (0x70..=0x7F, _) => {
                let reg = bytes.0 ^ 0x70;
                let value = bytes.1;
                ADD { reg, value }
            }
            (0x80..=0x8F, _) => {
                let reg_x = bytes.0 ^ 0x80;
                let reg_y = bytes.1 >> 4;
                let suffix = bytes.1 & 0x0F;
                match suffix {
                    0x00 => RLD { reg_x, reg_y },
                    0x01 => ROR { reg_x, reg_y },
                    0x02 => RAND { reg_x, reg_y },
                    0x03 => RXOR { reg_x, reg_y },
                    0x04 => RADD { reg_x, reg_y },
                    0x05 => RSUB { reg_x, reg_y },
                    0x06 => RSHR(reg_x),
                    0x07 => RSUBN { reg_x, reg_y },
                    0x0E => RSHL(reg_x),
                    _ => unreachable!("OpCode 0x8XY{:1X}", suffix),
                }
            }
            (0x90..=0x9F, _) => {
                let reg_x = bytes.0 ^ 0x90;
                let reg_y = bytes.1 >> 4;
                RSNE { reg_x, reg_y }
            }
            (0xA0..=0xAF, _) => LD(0xA000 ^ (((bytes.0 as u16) << 8) + bytes.1 as u16)),
            (0xB0..=0xBF, _) => JP(0xB000 ^ (((bytes.0 as u16) << 8) + bytes.1 as u16)),
            (0xC0..=0xCF, _) => {
                let reg = bytes.0 ^ 0xC0;
                let value = bytes.1;
                RND { reg, value }
            }
            (0xD0..=0xDF, _) => {
                let x = bytes.0 ^ 0xD0;
                let y = bytes.1 >> 4;
                let n = bytes.1 & 0x0F;
                DRW { x, y, n }
            }
            (0xE0..=0xEF, 0x9E) => SKP(bytes.0 ^ 0xE0),
            (0xE0..=0xEF, 0xA1) => SKNP(bytes.0 ^ 0xE0),
            (0xF0..=0xFF, 0x07) => LDT(bytes.0 ^ 0xF0),
            (0xF0..=0xFF, 0x0A) => KPR(bytes.0 ^ 0xF0),
            (0xF0..=0xFF, 0x15) => SETDT(bytes.0 ^ 0xF0),
            (0xF0..=0xFF, 0x18) => SETST(bytes.0 ^ 0xF0),
            (0xF0..=0xFF, 0x1E) => ADDI(bytes.0 ^ 0xF0),
            (0xF0..=0xFF, 0x29) => LDSPR(bytes.0 ^ 0xF0),
            (0xF0..=0xFF, 0x33) => STBCD(bytes.0 ^ 0xF0),
            (0xF0..=0xFF, 0x55) => STORE(bytes.0 ^ 0xF0),
            (0xF0..=0xFF, 0x65) => READ(bytes.0 ^ 0xF0),

            (_, _) => Unknown(((bytes.0 as u16) << 8) + bytes.1 as u16),
        }
    }
}

impl std::fmt::Display for OpCode {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OpCode::{
            Unknown, ADD, ADDI, CALL, CLS, DRW, JMP, JP, KPR, LD, LDSPR, LDT, RADD, RAND, READ,
            RET, RLD, RND, ROR, RSE, RSHL, RSHR, RSNE, RSUB, RSUBN, RXOR, SE, SET, SETDT, SETST,
            SKNP, SKP, SNE, STBCD, STORE,
        };
        match self {
            CLS => Ok(write!(f, "CLS - {:#06X}", 0x00E0)?),
            RET => Ok(write!(f, "RET - {:#06X}", 0x00EE)?),
            JMP(x) => Ok(write!(f, "JMP - {:#06X}", x ^ 0x1000)?),
            CALL(x) => Ok(write!(f, "CALL - {:#06X}", x ^ 0x2000)?),
            SE { reg, value } => Ok(write!(
                f,
                "SE - {:#06X}",
                (((*reg as u16) << 8) + *value as u16) ^ 0x3000
            )?),
            SNE { reg, value } => Ok(write!(
                f,
                "SNE - {:#06X}",
                (((*reg as u16) << 8) + *value as u16) ^ 0x4000
            )?),
            RSE { reg_x, reg_y } => Ok(write!(
                f,
                "RSE - {:#06X}",
                (((*reg_x as u16) << 8) + (reg_y << 4) as u16) ^ 0x4000
            )?),
            RSNE { reg_x, reg_y } => Ok(write!(
                f,
                "RSNE - {:#06X}",
                (((*reg_x as u16) << 8) + (reg_y << 4) as u16) ^ 0x9000
            )?),
            SET { reg, value } => Ok(write!(
                f,
                "SET - {:#06X}",
                (((*reg as u16) << 8) + *value as u16) ^ 0x6000
            )?),
            ADD { reg, value } => Ok(write!(
                f,
                "ADD - {:#06X}",
                (((*reg as u16) << 8) + *value as u16) ^ 0x7000
            )?),
            RLD { reg_x, reg_y } => Ok(write!(
                f,
                "RLD - {:#06X}",
                0x8000 ^ (((*reg_x as u16) << 8) + ((*reg_y as u16) << 4))
            )?),
            ROR { reg_x, reg_y } => Ok(write!(
                f,
                "ROR - {:#06X}",
                0x8000 ^ (((*reg_x as u16) << 8) + ((*reg_y as u16) << 4) + 0x01)
            )?),
            RAND { reg_x, reg_y } => Ok(write!(
                f,
                "RAND - {:#06X}",
                0x8000 ^ (((*reg_x as u16) << 8) + ((*reg_y as u16) << 4) + 0x02)
            )?),
            RXOR { reg_x, reg_y } => Ok(write!(
                f,
                "RXOR - {:#06X}",
                0x8000 ^ (((*reg_x as u16) << 8) + ((*reg_y as u16) << 4) + 0x03)
            )?),
            RADD { reg_x, reg_y } => Ok(write!(
                f,
                "RADD - {:#06X}",
                0x8000 ^ (((*reg_x as u16) << 8) + ((*reg_y as u16) << 4) + 0x04)
            )?),
            RSUB { reg_x, reg_y } => Ok(write!(
                f,
                "RSUB - {:#06X}",
                0x8000 ^ (((*reg_x as u16) << 8) + ((*reg_y as u16) << 4) + 0x05)
            )?),
            RSHR(reg) => Ok(write!(
                f,
                "RSHR - {:#06X}",
                0x8000 ^ (((*reg as u16) << 8) + 0x06)
            )?),
            RSUBN { reg_x, reg_y } => Ok(write!(
                f,
                "RSUBN - {:#06X}",
                0x8000 ^ (((*reg_x as u16) << 8) + ((*reg_y as u16) << 4) + 0x07)
            )?),
            RSHL(reg) => Ok(write!(
                f,
                "RSHL - {:#06X}",
                0x8000 ^ (((*reg as u16) << 8) + 0x0E)
            )?),
            LD(x) => Ok(write!(f, "LD - {:#06X}", x ^ 0xA000)?),
            JP(x) => Ok(write!(f, "JP - {:#06X}", x ^ 0xB000)?),
            RND { reg, value } => Ok(write!(
                f,
                "RND - {:#06X}",
                (((*reg as u16) << 8) + *value as u16) ^ 0xC000
            )?),
            DRW { x, y, n } => Ok(write!(
                f,
                "DRW - {:#06X}",
                (((*x as u16) << 8) + ((y << 4) + n) as u16) ^ 0xD000
            )?),
            SKP(x) => Ok(write!(f, "SKP - {:#06X}", ((*x as u16) << 8) ^ 0xE09E)?),
            SKNP(x) => Ok(write!(f, "SKNP - {:#06X}", ((*x as u16) << 8) ^ 0xE0A1)?),
            LDT(x) => Ok(write!(f, "LDT - {:#06X}", ((*x as u16) << 8) ^ 0xF007)?),
            KPR(x) => Ok(write!(f, "KPR - {:#06X}", ((*x as u16) << 8) ^ 0xF00A)?),

            SETDT(x) => Ok(write!(f, "SETDT - {:#06X}", ((*x as u16) << 8) ^ 0xF015)?),
            SETST(x) => Ok(write!(f, "SETST - {:#06X}", ((*x as u16) << 8) ^ 0xF018)?),
            ADDI(x) => Ok(write!(f, "ADDI - {:#06X}", ((*x as u16) << 8) ^ 0xF01E)?),
            LDSPR(x) => Ok(write!(f, "LDSPR - {:#06X}", ((*x as u16) << 8) ^ 0xF029)?),
            STBCD(x) => Ok(write!(f, "STBCD - {:#06X}", ((*x as u16) << 8) ^ 0xF033)?),
            STORE(x) => Ok(write!(f, "STORE - {:#06X}", ((*x as u16) << 8) ^ 0xF055)?),
            READ(x) => Ok(write!(f, "READ - {:#06X}", ((*x as u16) << 8) ^ 0xF065)?),

            Unknown(x) => Ok(write!(f, "UNK - {x:#06X}")?),
        }
    }
}
