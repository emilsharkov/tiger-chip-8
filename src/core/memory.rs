const MEMORY_SIZE: usize = 0x1000;

pub struct Memory {
    map: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self { map: [0; MEMORY_SIZE] }
    }

    pub fn read(&self, address: usize) -> u8 {
        self.map[address]
    }

    pub fn write(&mut self, address: usize, value: u8) {
        self.map[address] = value;
    }
}

