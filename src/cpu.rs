use crate::types::{BitFields, Byte, Word};

pub struct CPU {
    pub program_counter: Word,
    pub stack_pointer: Byte,
    pub processor_status: BitFields,

    pub accumulator: Byte,
    pub x: Byte,
    pub y: Byte,
}
