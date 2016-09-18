use std::fs::File;

use cartridge::Cartridge;
use gpu::Gpu;

pub struct Memory {
    pub gpu: Gpu,
    pub rom: Vec<u8>,
    pub work_ram: [u8; 0x2000], // 8 kB of RAM
    pub high_ram: [u8; 0x7F], // from 0xFF80 to 0xFFFF

    interrupt_flags: u8,
}

// http://gbdev.gg8.se/wiki/articles/Memory_Map
impl Memory {
    pub fn new() -> Memory {
        let mut file = File::open("roms/tetris.gb").unwrap();
        let cartridge = Cartridge::load(&mut file);

        Memory {
            gpu: Gpu::new(),
            rom: cartridge.rom,
            high_ram: [0; 0x7F],
            work_ram: [0; 0x2000],
            interrupt_flags: 0,
        }
    }

    pub fn load(&mut self, address: u16) -> u8 {
        if address < 0x3FFF {
            self.rom[address as usize & 0x3FFF]
        } else if address <= 0xDFFF {
            self.work_ram[address as usize & 0xDFFF]
        } else if address < 0xFF80 {
            self.read_io(address)
        } else if address < 0xFFFF {
            self.high_ram[address as usize & 0x7F]
        } else {
            //panic!("Can't read memory at {:04x}", address);
            self.interrupt_flags
        }
    }

    pub fn store(&mut self, address: u16, value: u8) {
        if address < 0x3FFF {
            self.rom[address as usize & 0x3FFF] = value;
        } else if address <= 0xDFFF {
            self.work_ram[address as usize & 0x0FFF] = value;
        } else if address < 0xFF80 {
            self.write_io(address, value);
        } else if address < 0xFFFF {
            self.high_ram[address as usize & 0x7F] = value;
        } else {
            self.interrupt_flags = value;
            //panic!("Can't store {:02x} at {:04x}", value, address);
        }
    }

    pub fn read_io(&mut self, address: u16) -> u8 {
        match address & 0xFF {
            // 0x00 => {} // Joypad
            // 0x01 | 0x02 => {} // Serial
            // 0x04 => {} // Divider (???)
            // 0x05...0x8 => {} // Timer
            0x0F => self.interrupt_flags, // Interrupt flags
            // 0x10...0x27 => {} // Sound
            0x40...0x47 => self.gpu.load((address as u8) & 0xFF),
            _    => { panic!("Can't write to unknown IO register {:04X}", address) }
        }
    }

    // http://fms.komkon.org/GameBoy/Tech/Software.html
    pub fn write_io(&mut self, address: u16, value: u8) {
        match address & 0xFF {
            // 0x00 => {} // Joypad
            0x01 | 0x02 => {} // TODO: Serial
            // 0x04 => {} // Divider (???)
            // 0x05...0x8 => {} // Timer
            0x0F => { self.interrupt_flags = value } // Interrupt flags
            // 0x10...0x27 => {} // Sound
            0x40...0x47 => self.gpu.store((address as u8) & 0xFF, value),
            _    => { panic!("Can't write to unknown IO register {:04X}", address) }
        }
    }
}
