pub mod instruction;

/// Memory Address
pub struct Addr(u16);

impl Addr {
    const fn new(addr: u16) -> Self {
        Self(addr & 0xfff)
    }
}

/// V-Register Index
pub struct RegisterIndex(u8);

impl RegisterIndex {
    const fn new(index: u8) -> Self {
        Self(index & 0xf)
    }
}
