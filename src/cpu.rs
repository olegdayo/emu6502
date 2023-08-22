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

    memory: Memory,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            stack_pointer: 0x00ff,
            program_counter: 0xfffc,
            processor_status: BitFields::default(),
            accumulator: 0,
            x: 0,
            y: 0,
            memory: Memory::new(),
        }
    }

    pub fn reset(&mut self) {
        self.stack_pointer = 0x00ff;
        self.program_counter = 0xfffc;
        self.processor_status = BitFields::default();

        self.accumulator = 0;
        self.x = 0;
        self.y = 0;

        self.memory.reset();
    }

    pub fn exec(&mut self, mut cycles_number: u32) {
        while cycles_number > 0 {
            let instruction = self.fetch_byte(&mut cycles_number);
        }
    }

    fn fetch_byte(&mut self, cycles_number: &mut u32) -> Byte {
        let data = self.memory.data[self.program_counter as usize];
        self.program_counter += 1;
        *cycles_number -= 1;
        data
    }
}
