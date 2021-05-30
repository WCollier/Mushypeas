use crate::{cpu::MAX_INSTRS, EmulatorError};

use std::{fs::File, io::Read};
use clap::Clap;
use std::io::Error;

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

impl From<std::io::Error> for EmulatorError {
    fn from(e: Error) -> Self {
        EmulatorError::IOError(e)
    }
}