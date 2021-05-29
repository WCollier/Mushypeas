use crate::{
    emulator::Emulator,
    cpu::MAX_INSTRS,
};

use std::{fs::File, io::Read};
use clap::Clap;

#[derive(Clone, Debug, Clap)]
#[clap(name = "mushypeas")]
pub(crate) struct Config {
    #[clap(short, long)]
    pub(crate) mute: bool,

    #[clap(short, long)]
    rom: String,
}

impl Config {
    pub(crate) fn load_rom(&self) -> std::io::Result<[u8; MAX_INSTRS]> {
        let mut f = File::open(&self.rom)?;

        let mut buffer = [0; MAX_INSTRS];

        f.read(&mut buffer)?;

        Ok(buffer)
    }
}