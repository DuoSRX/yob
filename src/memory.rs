pub struct Memory {
    pub ram: Ram,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: Ram::new(),
        }
    }

    pub fn load(&mut self, address: u16) -> u8 {
        self.ram.load(address)
    }

    pub fn store(&mut self, address: u16, value: u8) {
        self.ram.store(address, value);
    }
}

pub struct Ram {
    val: [u8; 0xFFFF]
}

impl Ram {
    pub fn new() -> Ram {
        Ram { val: [0; 0xFFFF] }
    }

    pub fn load(&self, address: u16) -> u8 {
        self.val[address as usize]
    }

    pub fn store(&mut self, address: u16, value: u8) {
        self.val[address as usize] = value;
    }
}
