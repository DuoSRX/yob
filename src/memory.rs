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
        match address {
            0x0000...0x7FFF => self.rom[address as usize],
            0x8000...0x9FFF => self.gpu.vram_load(address - 0x8000),
            0xC000...0xDFFF => self.work_ram[address as usize - 0xC000],
            // 0xC000...0xCFFF => self.work_ram[address as usize - 0xC000],
            // 0xD000...0xDFFF => self.work_ram[address as usize - 0xD000],
            0xE000...0xFDFF => self.rom[address as usize - 0xE000],
            0xFF00...0xFF80 => self.read_io(address),
            0xFF80...0xFFFE => self.high_ram[address as usize & 0x7F],
            0xFFFF => self.interrupt_flags,
            _ => panic!("Can't read memory at {:04x}", address),
        }
    }

    pub fn store(&mut self, address: u16, value: u8) {
        match address {
            0x0000...0x7FFF => { self.rom[address as usize] = value }
            0x8000...0x9FFF => { self.gpu.vram_store(address - 0x8000, value) },
            0xC000...0xDFFF => { self.work_ram[address as usize - 0xC000] = value },
            //0xC000...0xCFFF => { self.work_ram[address as usize - 0xC000] = value },
            //0xD000...0xDFFF => { self.work_ram[address as usize - 0xD000] = value },
            0xFE00...0xFE9F => { self.gpu.oam[address as usize - 0xFE00] = value },
            0xFEA0...0xFEFF => { } // Unusable... weird
            0xFF00...0xFF80 => { self.write_io(address, value) },
            0xFF80...0xFFFE => { self.high_ram[address as usize & 0x7F] = value },
            0xFFFF => { self.interrupt_flags = value },
            _ => panic!("Can't write memory at {:04x}", address),
        }
        // } else if address < 0xFF00 {
        //     // The wiki says it's unusable... okay?
    }

    pub fn read_io(&mut self, address: u16) -> u8 {
        match address & 0xFF {
            0x00 => 0u8, // TODO: Joypad
            // 0x01 | 0x02 => {} // Serial
            // 0x04 => {} // Divider (???)
            // 0x05...0x8 => {} // Timer
            0x0F => self.interrupt_flags, // Interrupt flags
            // 0x10...0x27 => {} // Sound
            0x40...0x4C => self.gpu.load((address as u8) & 0xFF),
            0x4C...0xFF => { 0 },
            _    => { panic!("Can't read unknown IO register {:04X}", address) }
        }
    }

    // http://fms.komkon.org/GameBoy/Tech/Software.html
    pub fn write_io(&mut self, address: u16, value: u8) {
        match address & 0xFF {
            0x00 => {} // TODO: Joypad
            0x01...0x02 => {} // TODO: Serial
            // 0x04 => {} // Divider (???)
            0x05...0x7 => {} // TODO: Timer
            0x0F => { self.interrupt_flags = value } // Interrupt flags
            0x10...0x26 => {} // TODO: Sound
            0x40...0x4B => self.gpu.store((address as u8) & 0xFF, value),
            0x4C...0xFF => {},
            _ => { panic!("Can't write to unknown IO register {:04X}", address) }
        }
    }
}
