#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    /// Represents the B,C pair with B as the high-order
    /// register and C as the low-order register.
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    /// Represents the D,E pair with D as the high-order
    /// register and E as the low-order register
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    /// Represents the H,L pair with H as the high-order
    /// register and L as the low-order register
    pub fn m(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
}
