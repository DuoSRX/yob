use std::fs::File;
use cartridge::Cartridge;

pub struct Memory {
    pub rom: Vec<u8>,
    pub work_ram: [u8; 0x2000], // 8 kB of RAM
    pub high_ram: [u8; 0x7F], // from 0xFF80 to 0xFFFF
}

// http://gbdev.gg8.se/wiki/articles/Memory_Map
impl Memory {
    pub fn new() -> Memory {
        let mut file = File::open("roms/tetris.gb").unwrap();
        let cartridge = Cartridge::load(&mut file);

        Memory {
            rom: cartridge.rom,
            high_ram: [0; 0x7F],
            work_ram: [0; 0x2000],
        }
    }

    pub fn load(&mut self, address: u16) -> u8 {
        if address < 0x3FFF {
            self.rom[address as usize & 0x3FFF]
        } else if address <= 0xDFFF {
            self.work_ram[address as usize & 0xDFFF]
        } else if address >= 0xFF80 {
            self.high_ram[address as usize & 0x7F]
        } else {
            panic!("Can't read memory at {:04x}", address);
        }
    }

    pub fn store(&mut self, address: u16, value: u8) {
        if address < 0x3FFF {
            self.rom[address as usize & 0x3FFF] = value;
        } else if address <= 0xDFFF {
            self.work_ram[address as usize & 0x0FFF] = value;
        } else if address >= 0xFF80 {
            self.high_ram[address as usize & 0x7F] = value;
        } else {
            panic!("Can't store {:02x} at {:04x}", value, address);
        }
    }
}
