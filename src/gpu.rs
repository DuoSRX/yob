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

    pub new_frame: bool,
    pub frame_content: [u8; 160 * 144 * 3],
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            lcd_control: 0x91,
            lcd_status: 0,
            scroll_x: 0,
            scroll_y: 0,
            ly: 0x91,
            lyc: 0,
            bg_palette: 0xFC,
            sprite_palette_0: 0xFF,
            sprite_palette_1: 0xFF,
            window_x: 0,
            window_y: 0,
            oam: [0; 0xA0],
            vram: [0; 0x2000],
            new_frame: false,
            frame_content: [0xFF; 160 * 144 * 3],
        }
    }

    pub fn step(&mut self, _cycles: u64) {
        for x in 0..16 {
            for y in 0..16 {
                self.draw_tile(x, y);
                // println!("{},{}", x, y);
            }
        }

        self.new_frame = true;
    }

    fn draw_tile(&mut self, x_offset: u16, y_offset: u16) {
        for y in 0..8 {
            let plane0 = self.vram_load(y as u16 * 2 + (x_offset * 16) + (y_offset * 16 * 16));
            let plane1 = self.vram_load(y as u16 * 2 + 1 + (x_offset * 16) + (y_offset * 16 * 16));
            // println!("{:08b}", plane0);
            // println!("{:08b}", plane0);

            for x in 0..8 {
                // let bit0 = (plane0 >> x) & 1;
                // let bit1 = (plane1 >> x) & 1;
                let bit0 = (plane0 >> ((7 - ((x % 8) as u8)) as usize)) & 1;
                let bit1 = (plane1 >> ((7 - ((x % 8) as u8)) as usize)) & 1;
                let result = (bit1 << 1) | bit0;
                let c = (result << 6) as u32;
                self.set_pixel(x as u32 + (x_offset as u32 * 8), y + (y_offset as u32 * 8), (c << 8) | (c << 16) | (c << 24));
            }
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        self.frame_content[((y * 160 + x) * 3 + 0) as usize] = (color >> 24) as u8;
        self.frame_content[((y * 160 + x) * 3 + 1) as usize] = (color >> 16) as u8;
        self.frame_content[((y * 160 + x) * 3 + 2) as usize] = (color >> 8) as u8;
    }

    pub fn reset(&mut self) {
    }

    pub fn vram_load(&mut self, address: u16) -> u8 {
        self.vram[address as usize]
    }

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
            // 0x4B => {} // WNDPOSX - Window X position
            _ => panic!("Can't load from GPU yet (0x{:02X})", address)
        }
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
            0x4A => { self.window_x = value },
            0x4B => { self.window_y = value },
            _ => panic!("Can't store in GPU yet (0x{:02X})", address)
        }
    }
}
