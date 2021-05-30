use crate::{
    emulator::Emulator,
    opcode::Opcode,
    config::Config,
};

use clap::Clap;

mod cpu;
mod config;
mod emulator;
mod fonts;
mod instr;
mod opcode;

pub(crate) type Result<T = ()> = std::result::Result<T, EmulatorError>;

#[derive(Debug)]
enum EmulatorError {
    UnknownOpcode(Opcode),
    IOError(std::io::Error),
}

fn main() -> Result {
    let config = Config::parse();

    let rom = config.load_rom()?;

    let mut emu = Emulator::new(config);

    emu.run(&rom);

    Ok(())
}
