use crate::{
    memory::Memory,
    types::{BitFields, Byte, Word},
};

pub struct CPU {
    pub program_counter: Word,
    pub stack_pointer: Byte,
    pub processor_status: BitFields,

    pub accumulator: Byte,
    pub x: Byte,
    pub y: Byte,
}

impl CPU {
    pub fn reset(&mut self, memory: &mut Memory) {
        self.stack_pointer = 0x00ff;
        self.program_counter = 0xfffc;
        self.processor_status = BitFields::default();

        self.accumulator = 0;
        self.x = 0;
        self.y = 0;

        memory.init();
    }
}
