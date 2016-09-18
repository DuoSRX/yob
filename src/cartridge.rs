#[allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;

// pub struct Headers {
//     garbage: [u8; 100],
// }
pub struct Cartridge {
    // headers: Headers
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn load(mut file: &mut File) -> Cartridge {
        // let headers = Headers {
        //     garbage: [0xFF; 100],
        // };

        let mut rom = Vec::new();
        file.read_to_end(&mut rom).expect("Cannot read file");

        Cartridge {
            rom: rom,
        }
    }
}
