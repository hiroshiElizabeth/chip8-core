use crate::{Addr, RegisterIndex};

type RI = RegisterIndex;
type Byte = u8;
type Nibble = u8;

pub enum OpcodeFormat {
    I0nnn(Addr),
    I00e0,
    I00ee,
    I1nnn(Addr),
    I2nnn(Addr),
    I3xkk(RI, Byte),
    I4xkk(RI, Byte),
    I5xy0(RI, RI),
    I6xkk(RI, Byte),
    I7xkk(RI, Byte),
    I8xy0(RI, RI),
    I8xy1(RI, RI),
    I8xy2(RI, RI),
    I8xy3(RI, RI),
    I8xy4(RI, RI),
    I8xy5(RI, RI),
    I8xy6(RI),
    I8xy7(RI, RI),
    I8xye(RI),
    I9xy0(RI, RI),
    Iannn(Addr),
    Ibnnn(Addr),
    Icxkk(RI, Byte),
    Idxyn(RI, RI, Nibble),
    Iex9e(RI),
    Iexa1(RI),
    Ifx07(RI),
    Ifx0a(RI),
    Ifx15(RI),
    Ifx18(RI),
    Ifx1e(RI),
    Ifx29(RI),
    Ifx33(RI),
    Ifx55(RI),
    Ifx65(RI),
}

impl From<(u8, u8)> for OpcodeFormat {
    fn from(bytes: (u8, u8)) -> Self {
        let inst = ByteInst(bytes.0, bytes.1);
        match inst.first_nibble() {
            0x0 => match inst.n() {
                0x0 => Self::I00e0,
                0xe => Self::I00ee,
                _ => Self::I0nnn(inst.nnn()),
            },
            0x1 => Self::I1nnn(inst.nnn()),
            0x2 => Self::I2nnn(inst.nnn()),
            0x3 => Self::I3xkk(inst.x(), inst.nn()),
            0x4 => Self::I4xkk(inst.x(), inst.nn()),
            0x5 => Self::I5xy0(inst.x(), inst.y()),
            0x6 => Self::I6xkk(inst.x(), inst.nn()),
            0x7 => Self::I7xkk(inst.x(), inst.nn()),
            0x8 => match inst.n() {
                0x0 => Self::I8xy0(inst.x(), inst.y()),
                0x1 => Self::I8xy1(inst.x(), inst.y()),
                0x2 => Self::I8xy2(inst.x(), inst.y()),
                0x3 => Self::I8xy3(inst.x(), inst.y()),
                0x4 => Self::I8xy4(inst.x(), inst.y()),
                0x5 => Self::I8xy5(inst.x(), inst.y()),
                0x6 => Self::I8xy6(inst.x()),
                0x7 => Self::I8xy7(inst.x(), inst.y()),
                0xe => Self::I8xye(inst.x()),
                _ => unreachable!(),
            },
            0x9 => Self::I9xy0(inst.x(), inst.y()),
            0xa => Self::Iannn(inst.nnn()),
            0xb => Self::Ibnnn(inst.nnn()),
            0xc => Self::Icxkk(inst.x(), inst.nn()),
            0xd => Self::Idxyn(inst.x(), inst.y(), inst.n()),
            0xe => match inst.nn() {
                0x9e => Self::Iex9e(inst.x()),
                0xa1 => Self::Iexa1(inst.x()),
                _ => unreachable!(),
            },
            0xf => match inst.nn() {
                0x07 => Self::Ifx07(inst.x()),
                0x0a => Self::Ifx0a(inst.x()),
                0x15 => Self::Ifx15(inst.x()),
                0x18 => Self::Ifx18(inst.x()),
                0x1e => Self::Ifx1e(inst.x()),
                0x29 => Self::Ifx29(inst.x()),
                0x33 => Self::Ifx33(inst.x()),
                0x55 => Self::Ifx55(inst.x()),
                0x65 => Self::Ifx65(inst.x()),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

struct ByteInst(u8, u8);

impl ByteInst {
    const fn first_nibble(&self) -> Nibble {
        self.0 >> 4
    }
    const fn x(&self) -> RI {
        RI::new(self.0)
    }
    const fn y(&self) -> RI {
        RI::new(self.1 >> 4)
    }
    const fn n(&self) -> Nibble {
        self.1 & 0xf
    }
    const fn nn(&self) -> Byte {
        self.1
    }
    const fn nnn(&self) -> Addr {
        Addr::new((self.0 as u16) << 8 | self.1 as u16)
    }
}
