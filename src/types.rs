pub type Byte = u8;
pub type Word = u16;
#[derive(Debug, Default)]
pub struct BitFields(u8);

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
    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

pub const ALL_FALSE: u8 = 0b00000000;
pub const ALL_TRUE: u8 = 0b01111111;

impl BitFields {
    pub fn new(num: u8) -> Self {
        Self(num)
    }

    pub fn get_flag(&self, flag: &Flag) -> bool {
        self.0 & flag.to_u8() == flag.to_u8()
    }

    pub fn set_flag(&mut self, flag: &Flag, val: bool) {
        if val {
            self.0 = self.0 | flag.to_u8();
            return;
        }
        self.0 = self.0 & (ALL_TRUE - flag.to_u8());
    }

    pub fn invert_flag(&mut self, flag: &Flag) {
        self.set_flag(flag, !self.get_flag(flag));
    }
}
