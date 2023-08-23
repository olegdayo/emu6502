use std::ops::{Index, IndexMut};

use crate::types::{Byte, Word};

pub const MAXIMUM_MEMORY_SIZE: usize = 65536;

#[derive(Debug, Clone, Copy)]
pub struct Memory {
    data: [Byte; MAXIMUM_MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [Byte::default(); MAXIMUM_MEMORY_SIZE],
        }
    }

    pub fn reset(&mut self) {
        self.data = [Byte::default(); MAXIMUM_MEMORY_SIZE];
    }
}

impl Index<Word> for Memory {
    type Output = Byte;

    fn index(&self, index: Word) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl IndexMut<Word> for Memory {
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}
