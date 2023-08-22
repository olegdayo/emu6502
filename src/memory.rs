use crate::types::Byte;

pub const MAXIMUM_MEMORY_SIZE: usize = 65536;

pub struct Memory {
    pub data: [Byte; MAXIMUM_MEMORY_SIZE],
}

impl Memory {
    pub fn init(&mut self) {
        self.data = [Byte::default(); MAXIMUM_MEMORY_SIZE];
    }
}
