extern crate yob;
extern crate sdl2;

use std::fs::File;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use yob::cartridge::Cartridge;
use yob::cpu::Cpu;

fn main() {
    // let mut file = File::open("roms/tetris.gb").unwrap();
    // let cartridge = Cartridge::load(&mut file);
    // println!("{:?}", cartridge.rom);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("yob", 160, 144)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();

    renderer.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut texture = renderer.create_texture_target(PixelFormatEnum::BGR24, 256, 240).unwrap();

    let mut cpu = Cpu::new();
    cpu.reset();
    let mut cycles = 0u64;

    'running: loop {
        cpu.step();
        cycles += 1;
        // if cycles >= 10000 {
        //     break 'running;
        // }
            // println!("{:04x} {:02x}", cpu.registers.pc, cpu.memory.gpu.vram[0]);
            // for i in 0x8000..0x8011 {
            //     println!("{:02x}", cpu.load_byte(i));
            // }
        //     break 'running;
        // }


        while let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("{:?}", cpu);
                    cpu.step();
                    println!("{:?}", cpu);
                    cpu.step();
                    println!("{:?}", cpu);
                    cpu.step();
                    println!("{:?}", cpu);
                    cpu.step();
                    println!("{:?}", cpu);
                    cpu.step();
                    println!("{:?}", cpu);
                    cpu.step();
                    println!("{:?}", cpu);
                    println!("{:04X}", cpu.pop_word());
                    // for i in 0x99E0..0x99EB {
                    //     println!("{:02x}", cpu.load_byte(i));
                    // }
                    break 'running
                },
                _ => ()
            }
        }
    }
}
