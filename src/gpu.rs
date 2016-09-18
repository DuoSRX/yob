pub struct Gpu {
    pub lcd_control: u8,
    pub lcd_status: u8,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub scanline: u8,
    pub cmpline: u8,
    pub bg_palette: u8,
    pub sprite_palette_0: u8,
    pub sprite_palette_1: u8,
    pub window_x: u8,
    pub window_y: u8,
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
            scanline: 0,
            cmpline: 0,
            bg_palette: 0,
            sprite_palette_0: 0,
            sprite_palette_1: 0,
            window_x: 0,
            window_y: 0,
        }
    }

    pub fn load(&mut self, address: u8) -> u8 {
        match address {
            0x40 => self.lcd_control,
            0x41 => self.lcd_status,
            0x42 => self.scroll_x,
            0x43 => self.scroll_y,
            0x44 => self.scanline,
            // 0x45 => {} // CMPLINE - Scanline comparison
            // 0x47 => {} // BGRDPAL - Background palette
            // 0x48 => {} // OBJ0PAL - Sprite palette #0
            // 0x49 => {} // OBJ1PAL - Sprite palette #1
            // 0x4A => {} // WNDPOSY - Window Y position
            // 0x4B => {} // WNDPOSX - Window X position
            // 0x46 => {} // DMACONT - DMA Transfer Controller
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
            0x44 => { self.scanline = 0 },
            // 0x45 => {} // CMPLINE - Scanline comparison
            // 0x47 => {} // BGRDPAL - Background palette
            // 0x48 => {} // OBJ0PAL - Sprite palette #0
            // 0x49 => {} // OBJ1PAL - Sprite palette #1
            // 0x4A => {} // WNDPOSY - Window Y position
            // 0x4B => {} // WNDPOSX - Window X position
            // 0x46 => {} // DMACONT - DMA Transfer Controller
            _ => panic!("Can't store in GPU yet (0x{:02X})", address)
        }
    }
}
