pub struct Gpu {
    pub lcd_control: u8,
    pub lcd_status: u8,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub ly: u8,
    pub lyc: u8,
    pub bg_palette: u8,
    pub sprite_palette_0: u8,
    pub sprite_palette_1: u8,
    pub window_x: u8,
    pub window_y: u8,

    pub oam: [u8; 0xA0],
    pub vram: [u8; 0x2000],
    // TODO: DMACONT
}

// 0x40 => {} // LDCCONT - LCD Control
// 0x41 => {} // LCDSTAT - LCD Status
// 0x42 => {} // SCROLLY - Scroll Y
// 0x43 => {} // SCROLLX - Scroll X
// 0x44 => {} // CURLINE - Current scanline
// 0x45 => {} // CMPLINE - Scanline comparison
// 0x47 => {} // BGRDPAL - Background palette
// 0x48 => {} // OBJ0PAL - Sprite palette #0
// 0x49 => {} // OBJ1PAL - Sprite palette #1
// 0x4A => {} // WNDPOSY - Window Y position
// 0x4B => {} // WNDPOSX - Window X position
// 0x46 => {} // DMACONT - DMA Transfer Controller
impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            lcd_control: 0,
            lcd_status: 0,
            scroll_x: 0,
            scroll_y: 0,
            ly: 148,
            lyc: 0,
            bg_palette: 0,
            sprite_palette_0: 0,
            sprite_palette_1: 0,
            window_x: 0,
            window_y: 0,
            oam: [0; 0xA0],
            vram: [0; 0x2000],
        }
    }

    // pub fn vram_load(&mut self, address: u16) -> u8

    pub fn vram_store(&mut self, address: u16, value: u8) {
        self.vram[address as usize] = value;
    }

    pub fn load(&mut self, address: u8) -> u8 {
        match address {
            0x40 => self.lcd_control,
            0x41 => self.lcd_status,
            0x42 => self.scroll_x,
            0x43 => self.scroll_y,
            0x44 => self.ly,
            // 0x45 => {} // CMPLINE - Scanline comparison
            0x47 => self.bg_palette,
            // 0x46 => {} // DMACONT - DMA Transfer Controller
            // 0x48 => {} // OBJ0PAL - Sprite palette #0
            // 0x49 => {} // OBJ1PAL - Sprite palette #1
            // 0x4A => {} // WNDPOSY - Window Y position
            // 0x4B => {} // WNDPOSX - Window X position
            _ => panic!("Can't load from GPU yet (0x{:02X})", address)
        }
        // panic!("Can't load from GPU yet ({:02X})", address);
    }

    pub fn store(&mut self, address: u8, value: u8) {
        match address {
            0x40 => { self.lcd_control = value }, // FIXME some bits are read only
            0x41 => { self.lcd_status = value }, // FIXME some bits are read only
            0x42 => { self.scroll_x = value },
            0x43 => { self.scroll_y = value },
            0x44 => { self.ly = 0 },
            // 0x45 => {} // CMPLINE - Scanline comparison
            // 0x46 => {} // DMACONT - DMA Transfer Controller
            0x47 => { self.bg_palette = value },
            0x48 => { self.sprite_palette_0 = value },
            0x49 => { self.sprite_palette_1 = value },
            // 0x4A => {} // WNDPOSY - Window Y position
            // 0x4B => {} // WNDPOSX - Window X position
            _ => panic!("Can't store in GPU yet (0x{:02X})", address)
        }
    }
}
