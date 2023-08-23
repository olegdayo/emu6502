use std::fmt::Debug;

pub type Byte = u8;
pub type Word = u16;

#[derive(Copy, Clone)]
pub enum Flag {
    Carry = 1,
    Zero = 2,
    Interrupt = 4,
    Decimal = 8,
    Break = 16,
    Overflow = 32,
    Negative = 64,
}

impl Flag {
    pub fn to_byte(&self) -> Byte {
        *self as Byte
    }
}

#[derive(Default)]
pub struct BitFields(Byte);

pub const ALL_FALSE: Byte = 0b00000000;
pub const ALL_TRUE: Byte = 0b01111111;

impl BitFields {
    pub fn new(num: u8) -> Self {
        Self(num)
    }

    pub fn get_flag(&self, flag: &Flag) -> bool {
        self.0 & flag.to_byte() == flag.to_byte()
    }

    pub fn set_flag(&mut self, flag: &Flag, val: bool) {
        if val {
            self.0 = self.0 | flag.to_byte();
            return;
        }
        self.0 = self.0 & (ALL_TRUE - flag.to_byte());
    }

    pub fn invert_flag(&mut self, flag: &Flag) {
        self.set_flag(flag, !self.get_flag(flag));
    }
}

impl Debug for BitFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitFields")
            .field("value", &format!("{:x}", self.0))
            .field("carry", &self.get_flag(&Flag::Carry))
            .field("zero", &self.get_flag(&Flag::Zero))
            .field("interrupt", &self.get_flag(&Flag::Interrupt))
            .field("decimal", &self.get_flag(&Flag::Decimal))
            .field("break", &self.get_flag(&Flag::Break))
            .field("overflow", &self.get_flag(&Flag::Overflow))
            .field("negative", &self.get_flag(&Flag::Negative))
            .finish()
    }
}
