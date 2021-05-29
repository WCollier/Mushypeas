use crate::{
    emulator::Emulator,
    opcode::Opcode,
};

mod cpu;
mod emulator;
mod fonts;
mod instr;
mod opcode;

pub(crate) type Result<T = ()> = std::result::Result<T, EmulatorError>;

#[derive(Copy, Clone, Debug)]
enum EmulatorError {
    UnknownOpcode(Opcode),
}

fn main() -> Result {
    let bytes = include_bytes!("br8kout.ch8");

    let mut emu = Emulator::new();

    emu.run(bytes);

    /*
    let mut cpu = Cpu::new();

    //let bytes = include_bytes!("breakout.ch8");

    let bytes = include_bytes!("breakout.ch8");

    let mut window = Window::new("Example", 640, 320, WindowOptions::default())
        .expect("Unable to create window");

    let mut buffer: Vec<u32> = vec![0x0; SCREEN_WIDTH * SCREEN_HEIGHT];

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    cpu.load(bytes);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.run();

        cpu.keys[0] = window.is_key_down(Key::Key1);

        cpu.keys[1] = window.is_key_down(Key::Key2);

        cpu.keys[2] = window.is_key_down(Key::Key3);

        cpu.keys[3] = window.is_key_down(Key::Key4);

        cpu.keys[4] = window.is_key_down(Key::Q);

        cpu.keys[5] = window.is_key_down(Key::W);

        cpu.keys[6] = window.is_key_down(Key::E);

        cpu.keys[7] = window.is_key_down(Key::R);

        cpu.keys[8] = window.is_key_down(Key::A);

        cpu.keys[9] = window.is_key_down(Key::S);

        cpu.keys[10] = window.is_key_down(Key::D);

        cpu.keys[11] = window.is_key_down(Key::F);

        cpu.keys[12] = window.is_key_down(Key::Z);

        cpu.keys[13] = window.is_key_down(Key::X);

        cpu.keys[14] = window.is_key_down(Key::C);

        cpu.keys[15] = window.is_key_down(Key::V);

        if !cpu.should_rerender {
            continue
        }

        for (i, pixel) in buffer.iter_mut().enumerate() {
            if cpu.display[i] == 0 {
                *pixel = 0x0;

            } else {
                *pixel = 0xFFFFFF;
            }
        }

        window.update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT);
    }
     */

    Ok(())
}
