use crate::{
    instructions::group2,
    memory::Memory,
    types::{BitFields, Byte, Word},
};

#[derive(Debug)]
pub struct CPU {
    pub program_counter: Word,
    pub stack_pointer: Byte,
    pub processor_status: BitFields,

    pub accumulator: Byte,
    pub x: Byte,
    pub y: Byte,
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
        }
    }

    pub fn reset(&mut self, memory: &mut Memory) {
        self.stack_pointer = 0x00ff;
        self.program_counter = 0xfffc;
        self.processor_status = BitFields::default();

        self.accumulator = 0;
        self.x = 0;
        self.y = 0;

        memory.reset();
    }

    pub fn exec(&mut self, memory: &mut Memory, mut cycles_number: u32) {
        while cycles_number > 0 {
            let instruction = self.fetch_byte(memory, &mut cycles_number);
            match instruction {
                group2::LDA => {
                    let val = self.fetch_byte(memory, &mut cycles_number);
                    self.accumulator = val;
                    self.processor_status
                        .set_flag(&crate::types::Flag::Zero, self.accumulator == 0);
                    self.processor_status.set_flag(
                        &crate::types::Flag::Negative,
                        self.accumulator & 0b10000000 > 0,
                    );
                }

                _ => {}
            }
        }
    }

    fn fetch_byte(&mut self, memory: &mut Memory, cycles_number: &mut u32) -> Byte {
        let data = memory[self.program_counter];
        self.program_counter += 1;
        *cycles_number -= 1;
        data
    }
}
